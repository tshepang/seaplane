use reqwest::Url;
use seaplane::{
    api::{
        v1::formations::{
            ActiveConfigurations as ActiveConfigurationsModel, Container as ContainerModel,
            Containers as ContainersModel, FormationConfiguration as FormationConfigurationModel,
            FormationMetadata as FormationMetadataModel, FormationNames as FormationNamesModel,
            FormationsRequest,
        },
        AccessToken, ApiErrorKind,
    },
    error::SeaplaneError,
};
use uuid::Uuid;

use crate::{
    api::request_token,
    context::Ctx,
    error::{CliError, Context, Result},
    ops::formation::{Formation, FormationConfiguration, Formations},
    printer::{Color, Pb},
};

/// Wraps an SDK `FormationsRequest` where we do additional things like re-use request access
/// tokens, allow changing the Formation this request is pointed to, and map errors appropriately.
#[derive(Debug)]
pub struct FormationsReq {
    api_key: String,
    name: Option<String>,
    token: Option<AccessToken>,
    inner: Option<FormationsRequest>,
    identity_url: Option<Url>,
    compute_url: Option<Url>,
}

impl FormationsReq {
    /// Builds a FormationsRequest and immediately requests an access token using the given API key.
    ///
    /// If the `name` is `None` it should be noted that the only request that can be made without
    /// error is `FormationsRequest::list_names`
    pub fn new<S: Into<String>>(ctx: &Ctx, name: Option<S>) -> Result<Self> {
        let mut this = Self::new_delay_token(ctx)?;
        this.name = name.map(Into::into);
        this.refresh_token()?;
        Ok(this)
    }

    /// Builds a FormationsRequest but *does not* request an access token using the given API key.
    ///
    /// You must call `refresh_token` to have the access token requested.
    pub fn new_delay_token(ctx: &Ctx) -> Result<Self> {
        Ok(Self {
            api_key: ctx.args.api_key()?.into(),
            name: None,
            token: None,
            inner: None,
            identity_url: ctx.identity_url.clone(),
            compute_url: ctx.compute_url.clone(),
        })
    }

    /// Request a new Access Token
    pub fn refresh_token(&mut self) -> Result<()> {
        self.token = Some(request_token(&self.api_key, self.identity_url.as_ref())?);
        Ok(())
    }

    /// Re-build the inner `FormationsRequest`. This is mostly useful when one wants to point at a
    /// different Formation than the original request was pointed at (i.e. via `set_name`). This
    /// method will also refresh the access token, only if required.
    fn refresh_inner(&mut self) -> Result<()> {
        let mut builder = FormationsRequest::builder().token(self.token_or_refresh()?);
        if let Some(url) = &self.compute_url {
            builder = builder.base_url(url);
        }

        if let Some(name) = &self.name {
            builder = builder.name(name);
        }

        self.inner = Some(builder.build().map_err(CliError::from)?);
        Ok(())
    }

    /// Retrieves the JWT access token, requesting a new one if required.
    pub fn token_or_refresh(&mut self) -> Result<&str> {
        if self.token.is_none() {
            self.refresh_token()?;
        }
        Ok(&self.token.as_ref().unwrap().token)
    }

    /// Sets the Formation name and re-builds the inner FormationsRequest also requesting a new
    /// access token if required
    pub fn set_name<S: Into<String>>(&mut self, name: S) -> Result<()> {
        self.name = Some(name.into());
        self.refresh_inner()
    }

    /// Retrieves all Formations and their Formation Configurations (both active and inactive) from
    /// the Compute API. Makes multiple calls against the API to gather the info and returns a
    /// `Formations` struct.
    ///
    /// It should be noted that the local IDs associated with all the items in the Formations
    /// struct are generated unique after retrieval from the compute API. i.e. they do not match
    /// anything existing in the local DB even if the contents are otherwise identical.
    pub fn get_all_formations<S: AsRef<str>>(
        &mut self,
        formation_names: &[S],
        pb: &Pb,
    ) -> Result<Formations> {
        let mut formations = Formations::default();
        for name in formation_names {
            let name = name.as_ref();
            self.set_name(name)?;
            pb.set_message(format!("Syncing Formation {name}..."));
            let mut formation = Formation::new(name);

            let cfg_uuids = self
                .list_configuration_ids()
                .context("Context: failed to retrieve Formation Configuration IDs\n")?;
            let active_cfgs = self
                .get_active_configurations()
                .context("Context: failed to retrieve Active Formation Configurations\n")?;

            pb.set_message(format!("Syncing Formation {name} Configurations..."));
            for uuid in cfg_uuids.into_iter() {
                let cfg_model = self
                    .get_configuration(uuid)
                    .context("Context: failed to retrieve Formation Configuration\n\tUUID: ")
                    .with_color_context(|| (Color::Yellow, format!("{uuid}\n")))?;

                let cfg = FormationConfiguration::with_uuid(uuid, cfg_model);
                let is_active = active_cfgs.iter().any(|ac| ac.uuid() == &uuid);
                formation.local.insert(cfg.id);
                if is_active {
                    formation.in_air.insert(cfg.id);
                } else {
                    formation.grounded.insert(cfg.id);
                }
                formations.configurations.push(cfg);
            }

            if !formation.is_empty() {
                formations.formations.push(formation);
            }
        }

        Ok(formations)
    }

    /// Return a `Vec` of all known formation names if this `FormationsReq` currently has no `name`
    /// associated with it. Otherwise it returns the single `name` associated with this
    /// `FormationsReq` (returned in a `Vec`). This is used when the CLI supports either doing
    /// something to all formations, or just a single one that is passed in by the user.
    pub fn get_formation_names(&mut self) -> Result<Vec<String>> {
        Ok(if let Some(name) = &self.name {
            vec![name.to_owned()]
        } else {
            // First download all formation names
            self.list_names()
                .context("Context: failed to retrieve Formation Instance names\n")?
                .into_inner()
        })
    }
}

/// Performs the wrapped method request against the Compute API. If the response is that the access
/// token is expired, it will refresh the access token and try again. All other errors are mapped
/// to the CliError type.
macro_rules! maybe_retry {
    ($this:ident . $fn:ident ( $($arg:expr),* ) ) => {{
        if $this.inner.is_none() {
            $this.refresh_inner()?;
        }
        let req =  $this.inner.as_ref().unwrap();

        let res = match req.$fn($( $arg ),*) {
            Ok(ret) => Ok(ret),
            Err(SeaplaneError::ApiResponse(ae))
                if ae.kind == ApiErrorKind::Unauthorized =>
            {
                $this.token = Some(request_token(&$this.api_key, $this.identity_url.as_ref())?);
                Ok(req.$fn($( $arg ,)*)?)
            }
            Err(e) => Err(e),
        };
        res.map_err(CliError::from)
    }};
}
// Wrapped FormationsRequest methods to handle expired token retries
//
impl FormationsReq {
    pub fn list_names(&mut self) -> Result<FormationNamesModel> { maybe_retry!(self.list_names()) }

    pub fn get_metadata(&mut self) -> Result<FormationMetadataModel> {
        maybe_retry!(self.get_metadata())
    }
    pub fn create(
        &mut self,
        configuration: &FormationConfigurationModel,
        active: bool,
    ) -> Result<Vec<Uuid>> {
        maybe_retry!(self.create(configuration, active))
    }
    pub fn clone_from(&mut self, source_name: &str, active: bool) -> Result<Vec<Uuid>> {
        maybe_retry!(self.clone_from(source_name, active))
    }
    pub fn delete(&mut self, force: bool) -> Result<Vec<Uuid>> { maybe_retry!(self.delete(force)) }
    pub fn get_active_configurations(&mut self) -> Result<ActiveConfigurationsModel> {
        maybe_retry!(self.get_active_configurations())
    }
    pub fn stop(&mut self) -> Result<()> { maybe_retry!(self.stop()) }
    pub fn set_active_configurations(
        &mut self,
        configs: &ActiveConfigurationsModel,
        force: bool,
    ) -> Result<()> {
        maybe_retry!(self.set_active_configurations(configs, force))
    }
    pub fn get_containers(&mut self) -> Result<ContainersModel> {
        maybe_retry!(self.get_containers())
    }
    pub fn get_container(&mut self, container_id: Uuid) -> Result<ContainerModel> {
        maybe_retry!(self.get_container(container_id))
    }
    pub fn get_configuration(&mut self, uuid: Uuid) -> Result<FormationConfigurationModel> {
        maybe_retry!(self.get_configuration(uuid))
    }
    pub fn list_configuration_ids(&mut self) -> Result<Vec<Uuid>> {
        maybe_retry!(self.list_configuration_ids())
    }
    pub fn remove_configuration(&mut self, uuid: Uuid, force: bool) -> Result<Uuid> {
        maybe_retry!(self.remove_configuration(uuid, force))
    }
    pub fn add_configuration(
        &mut self,
        configuration: &FormationConfigurationModel,
        active: bool,
    ) -> Result<Uuid> {
        maybe_retry!(self.add_configuration(configuration, active))
    }
}
