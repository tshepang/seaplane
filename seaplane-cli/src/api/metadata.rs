use reqwest::Url;
use seaplane::{
    api::{
        v1::{
            config::{
                ConfigRequest, KeyValue as KeyValueModel, KeyValueRange as KeyValueRangeModel,
                Value as ValueModel,
            },
            Key, RangeQueryContext,
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

/// Wraps an SDK `ConfigRequest` where we do additional things like re-use request access
/// tokens, allow changing the Formation this request is pointed to, and map errors appropriately.
#[derive(Debug)]
pub struct ConfigReq {
    api_key: String,
    key: Option<String>,
    range: Option<RangeQueryContext<Key>>,
    token: Option<AccessToken>,
    inner: Option<ConfigRequest>,
    identity_url: Option<Url>,
    metadata_url: Option<Url>,
}

impl ConfigReq {
    pub fn new(ctx: &Ctx) -> Result<Self> {
        Ok(Self {
            api_key: ctx.args.api_key()?.into(),
            key: None,
            range: None,
            token: None,
            inner: None,
            identity_url: ctx.identity_url.clone(),
            metadata_url: ctx.metadata_url.clone(),
        })
    }

    pub fn set_key<S: Into<String>>(&mut self, key: S) -> Result<()> {
        self.key = Some(key.into());
        self.range = None;
        self.refresh_inner()
    }

    pub fn set_dir(&mut self, dir: RangeQueryContext<Key>) -> Result<()> {
        self.range = Some(dir);
        self.key = None;
        self.refresh_inner()
    }

    /// Request a new Access Token
    pub fn refresh_token(&mut self) -> Result<()> {
        self.token = Some(request_token(&self.api_key, self.identity_url.as_ref())?);
        Ok(())
    }

    /// Re-build the inner `ConfigRequest`. This is mostly useful when one wants to point at
    /// different Metadata than the original request was pointed at. This method will also refresh
    /// the access token, only if required.
    fn refresh_inner(&mut self) -> Result<()> {
        let mut builder = ConfigRequest::builder().token(self.token_or_refresh()?);

        if let Some(url) = &self.metadata_url {
            builder = builder.base_url(url);
        }

        if let Some(key) = &self.key {
            builder = builder.encoded_key(key);
        }

        if let Some(range) = &self.range {
            builder = builder.range(range.clone());
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
}

/// Performs the wrapped method request against the Compute API. If the response is that the access
/// token is expired, it will refresh the access token and try again. All other errors are mapped
/// to the CliError type.
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
// Wrapped ConfigRequest methods to handle expired token retries
//
impl ConfigReq {
    pub fn get_value(&mut self) -> Result<ValueModel> {
        maybe_retry!(self.get_value())
    }
    pub fn put_value_unencoded<S: AsRef<[u8]> + Clone>(&mut self, value: S) -> Result<()> {
        maybe_retry!(self.put_value_unencoded(value))
    }
    pub fn put_value(&mut self, value: ValueModel) -> Result<()> {
        maybe_retry!(self.put_value(value))
    }
    pub fn delete_value(&mut self) -> Result<()> {
        maybe_retry!(self.delete_value())
    }
    pub fn get_page(&mut self) -> Result<KeyValueRangeModel> {
        maybe_retry!(self.get_page())
    }
    pub fn get_all_pages(&mut self) -> Result<Vec<KeyValueModel>> {
        maybe_retry!(self.get_all_pages())
    }
}
