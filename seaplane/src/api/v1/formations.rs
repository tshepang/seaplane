//! The `/formations` endpoint APIs which allows working with [`FormationConfiguration`]s,
//! [`Flight`]s, and the underlying containers

mod error;
mod models;

pub use error::{map_error, FormationsError, FormationsErrorKind};

use reqwest::{
    blocking,
    header::{self, CONTENT_TYPE},
    Url,
};
use uuid::Uuid;

use crate::{
    api::COMPUTE_API_URL,
    error::{Result, SeaplaneError},
};
pub use models::*;

/// A builder struct for creating a [`FormationsRequest`] which will then be used for making a
/// request against the `/formations` APIs
#[derive(Debug, Default)]
pub struct FormationsRequestBuilder {
    // The name of the formation (not required for GET /formations)
    name: Option<String>,
    // Required for Bearer Auth
    token: Option<String>,
    // Used for testing
    #[doc(hidden)]
    base_url: Option<Url>,
}

impl FormationsRequestBuilder {
    /// Create a new `Default` builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the token used in Bearer Authorization
    ///
    /// **NOTE:** This is required for all endpoints
    #[must_use]
    pub fn token<S: Into<String>>(mut self, token: S) -> Self {
        self.token = Some(token.into());
        self
    }

    /// The name of the Formation to query as part of the request.
    ///
    /// **NOTE:** This is not required for all endpoints
    #[must_use]
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Build a FormationsRequest from the given parameters
    pub fn build(self) -> Result<FormationsRequest> {
        if self.token.is_none() {
            return Err(SeaplaneError::MissingRequestAuthToken);
        }

        let mut headers = header::HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        #[cfg_attr(not(feature = "api_tests"), allow(unused_mut))]
        let mut builder = blocking::Client::builder()
            .default_headers(headers)
            .https_only(true);

        #[cfg(feature = "api_tests")]
        {
            builder = builder.https_only(false);
        }

        let url = if let Some(url) = self.base_url {
            url.join("v1/formations")?
        } else {
            let mut url: Url = COMPUTE_API_URL.parse()?;
            url.set_path("v1/formations");
            url
        };

        Ok(FormationsRequest {
            name: self.name,
            token: self.token.unwrap(),
            client: builder.build()?,
            endpoint_url: url,
        })
    }

    // Used in testing to manually set the URL
    #[cfg(feature = "api_tests")]
    #[cfg_attr(feature = "api_tests", doc(hidden))]
    pub fn base_url<S: AsRef<str>>(mut self, url: S) -> Self {
        self.base_url = Some(url.as_ref().parse().unwrap());
        self
    }
}

/// For making requests against the `/formations` APIs.
#[derive(Debug)]
pub struct FormationsRequest {
    /// The name of the formation
    name: Option<String>,
    token: String, // TODO: probably not a string
    #[doc(hidden)]
    client: reqwest::blocking::Client,
    #[doc(hidden)]
    endpoint_url: Url,
}

impl FormationsRequest {
    /// Create a new request builder
    pub fn builder() -> FormationsRequestBuilder {
        FormationsRequestBuilder::new()
    }

    /// Creates a new nameless formations request.
    ///
    /// **WARNING:** Because this request lacks a formation name, it is *not* valid for all
    /// endpoints. To create a `FormationsRequest` which is valid for all endpoints use
    /// `FormationsRequest::builder()`
    pub fn new<S: Into<String>>(token: S) -> Self {
        FormationsRequest::builder().token(token).build().unwrap()
    }

    //TODO: add the following methods:
    //  - start: sets all current configurations to active
    //  - start_configuration: sets given config to active along with all other already active
    //    configs
    //  - stop_configuration: sets only the given configuration to inactive, all others remain
    //    active

    /// Returns a list of the names of all Formations you have access to
    ///
    /// **NOTE:** This is the only endpoint that does not require a Formation name as part of the
    /// request.
    ///
    /// Uses `GET /formations`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::v1::{FormationsRequest};
    /// let req = FormationsRequest::new("abc123_token");
    ///
    /// let resp = req.list_names().unwrap();
    /// dbg!(resp);
    /// ```
    pub fn list_names(&self) -> Result<FormationNames> {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(self.endpoint_url.clone())
            .bearer_auth(&self.token)
            .send()?;

        map_error(resp, None)?
            .json::<FormationNames>()
            .map_err(Into::into)
    }

    /// Returns metadata about the Formation itself, such as the URL of the Formation.
    ///
    /// Uses `GET /formations/NAME`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::v1::{FormationsRequest};
    /// let req = FormationsRequest::builder().token("abc123_token").name("foo").build().unwrap();
    ///
    /// let resp = req.get_metadata().unwrap();
    /// dbg!(resp);
    /// ```
    pub fn get_metadata(&self) -> Result<FormationMetadata> {
        if self.name.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self
            .endpoint_url
            .join(&format!("formations/{}", self.name()))?;
        let resp = self.client.get(url).bearer_auth(&self.token).send()?;

        map_error(resp, None)?
            .json::<FormationMetadata>()
            .map_err(Into::into)
    }

    /// Create a new Formation and returns the IDs of any created configurations. This differs from
    /// `FormationsRequest::add_configuration` in that the Formation name of this request *must
    /// not* already exists, or an error is returned.
    ///
    /// Uses `POST /formations/NAME`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::v1::{FormationsRequest, FormationConfiguration, Flight};
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let config = FormationConfiguration::builder()
    ///     .add_flight(Flight::builder()
    ///         .name("myflight")
    ///         .image("my/image:latest")
    ///         .build().unwrap())
    ///     .build()
    ///     .unwrap();
    /// let resp = req.create(&config, false).unwrap();
    /// dbg!(resp);
    /// ```
    pub fn create(
        &self,
        configuration: &FormationConfiguration,
        active: bool,
    ) -> Result<Vec<Uuid>> {
        self._post_formation(Some(configuration), active, None)
    }

    /// Clones an existing Formation's (`source`) configuration and optionally sets the given
    /// configuration as active.
    ///
    /// Uses `POST /formations/NAME`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::v1::{FormationsRequest};
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.clone_from("bar", false).unwrap();
    /// dbg!(resp);
    /// ```
    pub fn clone_from(&self, source_name: &str, active: bool) -> Result<Vec<Uuid>> {
        self._post_formation(None, active, Some(source_name))
    }

    // The private internal function to deduplicate create/clone formation
    fn _post_formation(
        &self,
        configuration: Option<&FormationConfiguration>,
        active: bool,
        source: Option<&str>,
    ) -> Result<Vec<Uuid>> {
        if self.name.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let mut url = self
            .endpoint_url
            // We have to add "formations" because that's how URL's join() method works
            .join(&format!("formations/{}?active={active}", self.name()))?;
        if let Some(source) = source {
            url.query_pairs_mut().append_pair("source", source);
        }
        let req = if let Some(ref cfg) = configuration {
            self.client.post(url).bearer_auth(&self.token).json(cfg)
        } else {
            self.client.post(url).bearer_auth(&self.token)
        };
        let resp = req.send()?;
        map_error(
            resp,
            Some((
                FormationsErrorKind::FormationNotFound,
                "'source' not found".into(),
            )),
        )?
        .json::<Vec<Uuid>>()
        .map_err(Into::into)
    }

    /// Deletes a formation
    ///
    /// **WARNING:** Setting `force` to `true` will delete the formation even if it is actively
    /// running.
    ///
    /// Uses `DELETE /formations/NAME`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::v1::{FormationsRequest};
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(req.delete(false).is_ok());
    /// ```
    pub fn delete(&self, force: bool) -> Result<Vec<Uuid>> {
        if self.name.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self
            .endpoint_url
            .join(&format!("formations/{}?force={force}", self.name()))?;
        let resp = self.client.delete(url).bearer_auth(&self.token).send()?;

        map_error(
            resp,
            Some((
                FormationsErrorKind::InvalidRequest,
                "use force=true to override".into(),
            )),
        )?
        .json::<Vec<Uuid>>()
        .map_err(Into::into)
    }

    /// Returns the IDs of all active configurations of a formation, along with their traffic
    /// weights.
    ///
    /// Uses `GET /formations/NAME/activeConfiguration`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::v1::{FormationsRequest, ActiveConfiguration, ActiveConfigurations};
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.get_active_configurations().unwrap();
    /// dbg!(resp);
    /// ```
    pub fn get_active_configurations(&self) -> Result<ActiveConfigurations> {
        if self.name.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self
            .endpoint_url
            .join(&format!("formations/{}/activeConfiguration", self.name()))?;
        let resp = self.client.get(url).bearer_auth(&self.token).send()?;
        map_error(resp, None)?
            .json::<ActiveConfigurations>()
            .map_err(Into::into)
    }

    /// Stops a Formation, spinning down all active Flights
    ///
    /// Uses `DELETE /formations/NAME/activeConfiguration`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::v1::{FormationsRequest, ActiveConfiguration, ActiveConfigurations};
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.stop();
    ///
    /// assert!(resp.is_ok());
    /// ```
    pub fn stop(&self) -> Result<()> {
        if self.name.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self
            .endpoint_url
            .join(&format!("formations/{}/activeConfiguration", self.name()))?;
        let resp = self.client.delete(url).bearer_auth(&self.token).send()?;
        map_error(resp, None)?
            .text()
            .map(|_| ()) // TODO: for now we drop the "success" message to control it ourselves
            .map_err(Into::into)
    }

    /// Sets all active configurations for a particular Formation.
    ///
    /// Uses `PUT /formations/NAME/activeConfiguration`
    ///
    /// **WARNING:** If `ActiveConfigurations` is empty, you are effectively removing *all* active
    /// configurations which brings down the Formation. If this is intentional `force` should be
    /// set to `true` otherwise an error will be returned on an invalid `ActiveConfiguration`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::v1::{FormationsRequest, ActiveConfiguration, ActiveConfigurations};
    /// # use uuid::Uuid;
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.set_active_configurations(
    ///     &ActiveConfigurations::new()
    ///         .add_configuration(ActiveConfiguration::builder()
    ///             .uuid("aa8522e7-06cc-4e35-8966-484ae26e02a9".parse::<Uuid>().unwrap())
    ///             .build()
    ///             .unwrap()
    ///         ),
    ///     false
    /// );
    ///
    /// assert!(resp.is_ok());
    /// ```
    pub fn set_active_configurations(
        &self,
        configs: &ActiveConfigurations,
        force: bool,
    ) -> Result<()> {
        if self.name.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self.endpoint_url.join(&format!(
            "formations/{}/activeConfiguration?force={force}",
            self.name()
        ))?;
        if !force && configs.is_empty() {
            return Err(SeaplaneError::MissingActiveConfiguration);
        }
        let resp = self
            .client
            .put(url)
            .bearer_auth(&self.token)
            .body(serde_json::to_string(&configs)?)
            .send()?;
        map_error(
            resp,
            Some((
                FormationsErrorKind::InvalidRequest,
                "use force=true to override".into(),
            )),
        )?
        .text()
        .map(|_| ()) // TODO: for now we drop the "success" message to control it ourselves
        .map_err(Into::into)
    }

    /// List all containers (both actively running and recently stopped) within a Formation
    ///
    /// Uses `GET /formations/NAME/containers`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::v1::{FormationsRequest, ActiveConfiguration, ActiveConfigurations};
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.get_containers().unwrap();
    /// dbg!(resp);
    /// ```
    pub fn get_containers(&self) -> Result<Containers> {
        if self.name.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self
            .endpoint_url
            .join(&format!("formations/{}/containers", self.name()))?;
        let resp = self.client.get(url).bearer_auth(&self.token).send()?;
        map_error(resp, None)?
            .json::<Containers>()
            .map_err(Into::into)
    }

    /// Returns the status and details of a single containers within a Formation
    ///
    /// Uses `GET /formations/NAME/containers/CONTAINER_UUID`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::v1::FormationsRequest;
    /// # use uuid::Uuid;
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.get_container(
    ///     "aa8522e7-06cc-4e35-8966-484ae26e02a9".parse::<Uuid>().unwrap()
    /// ).unwrap();
    /// dbg!(resp);
    /// ```
    pub fn get_container(&self, container_id: Uuid) -> Result<Container> {
        if self.name.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self.endpoint_url.join(&format!(
            "formations/{}/containers/{container_id}",
            self.name()
        ))?;
        let resp = self.client.get(url).bearer_auth(&self.token).send()?;
        map_error(resp, None)?
            .json::<Container>()
            .map_err(Into::into)
    }

    /// Returns the configuration details for a given configuration UUID within Formation
    ///
    /// Uses `GET /formations/NAME/configurations/UUID`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::v1::{FormationsRequest, ActiveConfiguration, ActiveConfigurations};
    /// # use uuid::Uuid;
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req
    ///     .get_configuration("aa8522e7-06cc-4e35-8966-484ae26e02a9".parse::<Uuid>().unwrap())
    ///     .unwrap();
    ///
    /// dbg!(resp);
    /// ```
    pub fn get_configuration(&self, uuid: Uuid) -> Result<FormationConfiguration> {
        if self.name.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self
            .endpoint_url
            .join(&format!("formations/{}/configurations/{uuid}", self.name()))?;
        let resp = self.client.get(url).bearer_auth(&self.token).send()?;
        map_error(resp, None)?
            .json::<FormationConfiguration>()
            .map_err(Into::into)
    }

    /// Returns all configuration IDs for a given Formation
    ///
    /// Uses `GET /formations/NAME/configurations`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::v1::{FormationsRequest, ActiveConfiguration, ActiveConfigurations};
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.list_configuration_ids().unwrap();
    /// dbg!(resp);
    /// ```
    pub fn list_configuration_ids(&self) -> Result<Vec<Uuid>> {
        if self.name.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self
            .endpoint_url
            .join(&format!("formations/{}/configurations", self.name()))?;
        let resp = self.client.get(url).bearer_auth(&self.token).send()?;
        map_error(resp, None)?
            .json::<Vec<Uuid>>()
            .map_err(Into::into)
    }

    /// Removes a Configuration from a Formation and returns the UUID of the configuration
    ///
    /// **WARNING:** Setting `force` to `true` will delete the formation even if it is actively
    /// running.
    ///
    /// Uses `DELETE /formations/NAME/configurations/UUID`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::v1::{FormationsRequest, ActiveConfiguration, ActiveConfigurations};
    /// # use uuid::Uuid;
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req
    ///     .remove_configuration(
    ///         "aa8522e7-06cc-4e35-8966-484ae26e02a9".parse::<Uuid>().unwrap(),
    ///         false
    ///     )
    ///     .unwrap();
    ///
    /// dbg!(resp);
    /// ```
    pub fn remove_configuration(&self, uuid: Uuid, force: bool) -> Result<Uuid> {
        if self.name.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self.endpoint_url.join(&format!(
            "formations/{}/configurations/{uuid}?force={force}",
            self.name()
        ))?;
        let resp = self.client.delete(url).bearer_auth(&self.token).send()?;
        map_error(
            resp,
            Some((
                FormationsErrorKind::InvalidRequest,
                "use force=true to override".into(),
            )),
        )?
        .json::<Uuid>()
        .map_err(Into::into)
    }

    /// Create a new configuration for this Formation and optionally set it as active. This differs
    /// from `FormationsRequest::create` in that the Formation name of this request *must*
    /// already exists or an error is returned.
    ///
    /// Uses `POST /formations/NAME`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::v1::{FormationsRequest, FormationConfiguration, Flight};
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let config = FormationConfiguration::builder()
    ///     .add_flight(Flight::builder()
    ///         .name("myflight")
    ///         .image("my/image:latest")
    ///         .build().unwrap())
    ///     .build()
    ///     .unwrap();
    /// let resp = req.create(&config, false).unwrap();
    /// dbg!(resp);
    /// ```
    pub fn add_configuration(
        &self,
        configuration: &FormationConfiguration,
        active: bool,
    ) -> Result<Uuid> {
        if self.name.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self.endpoint_url.join(&format!(
            "formations/{}/configurations?active={active}",
            self.name()
        ))?;
        let resp = self
            .client
            .post(url)
            .bearer_auth(&self.token)
            .body(serde_json::to_string(&configuration)?)
            .send()?;
        map_error(
            resp,
            Some((
                FormationsErrorKind::FormationNotFound,
                "'source' not found".into(),
            )),
        )?
        .json::<Uuid>()
        .map_err(Into::into)
    }

    // Internal, only used when can only be a valid name.
    #[inline]
    fn name(&self) -> &str {
        self.name.as_deref().unwrap()
    }
}
