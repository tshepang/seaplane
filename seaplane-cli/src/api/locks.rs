use reqwest::Url;
use seaplane::{
    api::{
        v1::locks::{
            HeldLock as HeldLockModel, LockId, LockInfo as LockInfoModel, LockName, LocksErrorKind,
            LocksRequest, AcquireResponse,
        } ,
        AccessToken,
    },
    error::SeaplaneError,
};

use crate::{
    api::request_token,
    context::Ctx,
    error::{CliError, Result},
};

/// Wraps an SDK `LocksRequest` where we do additional things like re-use request access
/// tokens, allow changing the Locks this request is pointed to, and map errors appropriately.
#[derive(Debug)]
pub struct LocksReq {
    api_key: String,
    lock_id: Option<String>,
    name: Option<String>,
    token: Option<AccessToken>,
    inner: Option<LocksRequest>,
    identity_url: Option<Url>,
    locks_url: Option<Url>,
}

impl LocksReq {
    pub fn new(ctx: &Ctx) -> Result<Self> {
        Ok(Self {
            api_key: ctx.args.api_key()?.into(),
            lock_id: None,
            name: None,
            token: None,
            inner: None,
            identity_url: ctx.identity_url.clone(),
            locks_url: ctx.locks_url.clone(),
        })
    }

    pub fn set_identifiers<S1: Into<String>, S2: Into<String>>(
        &mut self,
        name: Option<S1>,
        lock_id: Option<S2>,
    ) -> Result<()> {
        self.name = name.map(|s| s.into());
        self.lock_id = lock_id.map(|s| s.into());
        self.refresh_inner()
    }

    pub fn set_name<S: Into<String>>(&mut self, name: S) -> Result<()> {
        self.name = Some(name.into());
        self.refresh_inner()
    }

    /// Request a new Access Token
    pub fn refresh_token(&mut self) -> Result<()> {
        self.token = Some(request_token(&self.api_key, self.identity_url.as_ref())?);
        Ok(())
    }

    /// Re-build the inner `LocksRequest`. This is mostly useful when one wants to point at a
    /// different Lock than the original request was pointed at (i.e. via `set_name`). This
    /// method will also refresh the access token, only if required.
    fn refresh_inner(&mut self) -> Result<()> {
        let mut builder = LocksRequest::builder().token(self.token_or_refresh()?);

        if self.name.is_none() && self.lock_id.is_none() {
            panic!("This violates an invariant that all LocksRequests will have either a name or an id,
            however this should not be possible as field are set to 'required' using Clap");
        }

        if let Some(lock_id) = &self.lock_id {
            let default_sequencer_value = 0u32;
            builder = builder.held_lock(HeldLockModel::new(
                LockName::from_encoded(self.name.as_ref().unwrap()),
                LockId::from_encoded(lock_id),
                default_sequencer_value,
            ));
        }

        if self.lock_id.is_none() && self.name.is_some() {
            builder = builder.encoded_lock_name(self.name.as_ref().unwrap());
        }

        if let Some(url) = &self.locks_url {
            builder = builder.base_url(url);
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

/// Performs the wrapped method request against the Locks API. If the response is that the access
/// token is expired, it will refresh the access token and try again. All other errors are mapped
/// to the CliError type.
// TODO: This macro could most likely be moved up to the src/macros.rs level and be de-duplicated
// with the other maybe_retry! macros so it does not get confusing which one we mean.
macro_rules! maybe_retry {
    ($this:ident . $fn:ident ( $($arg:expr),* ) ) => {{
        if $this.inner.is_none() {
            $this.refresh_inner()?;
        }
        let req = &mut $this.inner.as_mut().unwrap();

        let res = match req.$fn($( $arg.clone() ),*) {
            Ok(ret) => Ok(ret),
            Err(SeaplaneError::LocksResponse(fr))
                if fr.kind == LocksErrorKind::NotLoggedIn =>
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
// Wrapped LocksRequest methods to handle expired token retries
//
impl LocksReq {
    pub fn acquire(&mut self, ttl: u32, client_id: &str) -> Result<AcquireResponse> {
        maybe_retry!(self.acquire(ttl, client_id))
    }
    pub fn release(&mut self) -> Result<()> {
        maybe_retry!(self.release())
    }
    pub fn renew(&mut self, ttl: u32) -> Result<()> {
        maybe_retry!(self.renew(ttl))
    }
    pub fn get_lock_info(&mut self) -> Result<LockInfoModel> {
        maybe_retry!(self.get_lock_info())
    }
}
