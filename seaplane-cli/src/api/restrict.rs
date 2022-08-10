use reqwest::Url;
use seaplane::{
    api::{
        v1::{
            RangeQueryContext, RestrictRequest, RestrictedDirectory, Restriction,
            RestrictionDetails,
        },
        AccessToken, ApiErrorKind,
    },
    error::SeaplaneError,
};

use crate::{
    api::request_token,
    context::Ctx,
    error::{CliError, Result},
};

/// Wraps an SDK `RestrictRequest` where we do additional things like re-use
/// request access tokens, and map errors appropriately.
#[derive(Debug)]
pub struct RestrictReq {
    api_key: String,
    api: Option<String>,
    directory: Option<String>,
    // TODO: Remove this when range queries are supported
    #[allow(dead_code)]
    range: Option<RangeQueryContext<RestrictedDirectory>>,
    token: Option<AccessToken>,
    inner: Option<RestrictRequest>,
    identity_url: Option<Url>,
    metadata_url: Option<Url>,
}

impl RestrictReq {
    pub fn new(ctx: &Ctx) -> Result<Self> {
        Ok(Self {
            api_key: ctx.args.api_key()?.into(),
            api: None,
            directory: None,
            range: None,
            token: None,
            inner: None,
            identity_url: ctx.identity_url.clone(),
            metadata_url: ctx.metadata_url.clone(),
        })
    }

    pub fn set_api<S: Into<String>>(&mut self, api: S) -> Result<()> {
        self.api = Some(api.into());
        self.refresh_inner()
    }

    pub fn set_directory<S: Into<String>>(&mut self, dir: S) -> Result<()> {
        self.directory = Some(dir.into());
        self.refresh_inner()
    }

    /// Request a new Access Token
    pub fn refresh_token(&mut self) -> Result<()> {
        self.token = Some(request_token(&self.api_key, self.identity_url.as_ref())?);
        Ok(())
    }

    /// Re-build the inner `RestrictRequest`. This is mostly useful when one
    /// wants to point at different Restriction than the original request was
    /// pointed at. This method will also refresh the access token, only if
    /// required.
    fn refresh_inner(&mut self) -> Result<()> {
        let mut builder = RestrictRequest::builder().token(self.token_or_refresh()?);

        if let Some(url) = &self.metadata_url {
            builder = builder.base_url(url);
        }

        match [&self.api, &self.directory] {
            [Some(api), Some(directory)] => builder = builder.single_restriction(api, directory),
            // TODO: add support for range queries
            [..] => {}
        };

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
}

/// Performs the wrapped method request against the Restrict API. If the
/// response is that the access token is expired, it will refresh the access
/// token and try again. All other errors are mapped to the CliError type.
macro_rules! maybe_retry {
    ($this:ident . $fn:ident ( $($arg:expr),* ) ) => {{
        if $this.inner.is_none() {
            $this.refresh_inner()?;
        }
        let req = &mut $this.inner.as_mut().unwrap();

        let res = match req.$fn($( $arg.clone() ),*) {
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
//
// Wrapped RestrictRequest methods to handle expired token retries
//
impl RestrictReq {
    pub fn get_restriction(&mut self) -> Result<Restriction> {
        maybe_retry!(self.get_restriction())
    }

    pub fn set_restriction(&mut self, details: RestrictionDetails) -> Result<()> {
        maybe_retry!(self.set_restriction(details))
    }
    pub fn delete_restriction(&mut self) -> Result<()> {
        maybe_retry!(self.delete_restriction())
    }
}
