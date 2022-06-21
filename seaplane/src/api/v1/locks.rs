//! The `/locks` endpoint APIs which allows working with [`HeldLock`]s

use models::RequestTarget;
use reqwest::{
    blocking,
    header::{self, CONTENT_TYPE},
    Url,
};
use serde::Deserialize;

use crate::{
    api::METADATA_API_URL,
    base64::add_base64_path_segment,
    error::{Result, SeaplaneError},
};

mod models;
pub use models::*;

mod error;
pub use error::*;

use super::RangeQueryContext;

/// A builder struct for creating a [`LocksRequest`] which will then be used for making a
/// request against the `/locks` APIs
#[derive(Debug, Default)]
pub struct LocksRequestBuilder {
    // The target lock of this request
    target: Option<RequestTarget>,
    // Required for Bearer Auth
    token: Option<String>,
    // Used for testing
    #[doc(hidden)]
    base_url: Option<Url>,
}

/// For making requests against the `/locks` APIs.
#[derive(Debug)]
pub struct LocksRequest {
    target: RequestTarget,
    token: String, // TODO: probably not a string
    #[doc(hidden)]
    client: reqwest::blocking::Client,
    #[doc(hidden)]
    endpoint_url: Url,
}

impl LocksRequestBuilder {
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

    /// The lock name with which to perform operations where you may not be holding the lock.
    /// Encoded in url-safe base64
    ///
    /// **NOTE:** This is not required for all endpoints
    #[must_use]
    pub fn encoded_lock_name<S: Into<String>>(mut self, lock: S) -> Self {
        self.target = Some(RequestTarget::SingleLock(LockName::from_encoded(
            lock.into(),
        )));
        self
    }

    /// The held lock with which to perform held lock operations.
    ///
    /// **NOTE:** This is not required for all endpoints
    pub fn held_lock(mut self, lock: HeldLock) -> Self {
        self.target = Some(RequestTarget::HeldLock(lock));
        self
    }

    /// The context with which to perform a range query
    ///
    /// **NOTE:** This is not required for all endpoints
    #[must_use]
    pub fn range(mut self, context: RangeQueryContext<LockName>) -> Self {
        self.target = Some(RequestTarget::Range(context));
        self
    }

    /// Build a LocksRequest from the given parameters
    pub fn build(self) -> Result<LocksRequest> {
        if self.token.is_none() {
            return Err(SeaplaneError::MissingRequestAuthToken);
        }

        let mut headers = header::HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let target = self
            .target
            .ok_or(SeaplaneError::IncorrectLocksRequestTarget)?;

        #[cfg_attr(not(feature = "api_tests"), allow(unused_mut))]
        let mut builder = blocking::Client::builder()
            .default_headers(headers)
            .https_only(true);

        #[cfg(feature = "api_tests")]
        {
            builder = builder.https_only(false);
        }

        let url = if let Some(url) = self.base_url {
            url.join("v1/locks/")?
        } else {
            let mut url: Url = METADATA_API_URL.parse()?;
            url.set_path("v1/locks/");
            url
        };
        Ok(LocksRequest {
            target,
            token: self.token.unwrap(),
            client: builder.build()?,
            endpoint_url: url,
        })
    }

    // Used in testing and development to manually set the URL
    #[doc(hidden)]
    pub fn base_url<S: AsRef<str>>(mut self, url: S) -> Self {
        self.base_url = Some(url.as_ref().parse().unwrap());
        self
    }
}

impl LocksRequest {
    /// Create a new request builder
    pub fn builder() -> LocksRequestBuilder {
        LocksRequestBuilder::new()
    }

    // Internal method creating the URL for all single key endpoints
    fn single_lock_url(&self) -> Result<Url> {
        match &self.target {
            RequestTarget::HeldLock(_) | RequestTarget::Range(_) => {
                Err(SeaplaneError::IncorrectLocksRequestTarget)
            }
            RequestTarget::SingleLock(l) => Ok(add_base64_path_segment(
                self.endpoint_url.clone(),
                l.encoded(),
            )),
        }
    }

    // Internal method for creating the URL for held lock endpoints
    fn held_lock_url(&self) -> Result<Url> {
        match &self.target {
            RequestTarget::SingleLock(_) | RequestTarget::Range(_) => {
                Err(SeaplaneError::IncorrectLocksRequestTarget)
            }

            RequestTarget::HeldLock(HeldLock { name, id, .. }) => {
                let mut url = add_base64_path_segment(self.endpoint_url.clone(), name.encoded());
                url.set_query(Some(&format!("id={}", id.encoded())));
                Ok(url)
            }
        }
    }

    // Internal method for creating the URL for range endpoints
    fn range_url(&self) -> Result<Url> {
        match &self.target {
            RequestTarget::SingleLock(_) | RequestTarget::HeldLock(_) => {
                Err(SeaplaneError::IncorrectLocksRequestTarget)
            }
            RequestTarget::Range(context) => {
                let mut url = self.endpoint_url.clone();

                if let Some(encoded_dir) = context.directory() {
                    url = add_base64_path_segment(url, encoded_dir.encoded());
                    // A directory is distinguished from a key by the trailing slash
                    url.set_path(&format!("{}/", url.path()));
                }

                if let Some(from) = context.from() {
                    url.set_query(Some(&format!("from=base64:{}", from.encoded())));
                }

                Ok(url)
            }
        }
    }

    // Internal method for getting the lock name
    fn lock_name(&self) -> Result<LockName> {
        match &self.target {
            RequestTarget::HeldLock(_) | RequestTarget::Range(_) => {
                Err(SeaplaneError::IncorrectLocksRequestTarget)
            }
            RequestTarget::SingleLock(l) => Ok(l.clone()),
        }
    }

    /// Attempts to acquire the lock with the given lock name with the given TTL.
    /// Client-ID should identify the client making the request for debugging purposes.
    ///
    /// **NOTE:** This endpoints requires the `RequestTarget` be a `SingleLock`
    ///
    /// # Examples
    /// ```no_run
    /// use seaplane::api::v1::{LocksRequestBuilder,LocksRequest};
    ///
    /// let req = LocksRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .encoded_lock_name("bW9ieQo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.acquire(15, "test-client").unwrap();
    /// dbg!(resp);
    /// ```
    pub fn acquire(self, ttl: u32, client_id: &str) -> Result<HeldLock> {
        let mut url = self.single_lock_url()?;

        url.set_query(Some(&format!("ttl={ttl}&client-id={client_id}")));
        let resp = self.client.put(url).bearer_auth(&self.token).send()?;

        #[derive(Deserialize)]
        struct AcquireResponse {
            id: LockID,
            sequencer: u32,
        }

        let name = self.lock_name()?;
        map_error(dbg!(resp))?
            .json::<AcquireResponse>()
            .map(|AcquireResponse { id, sequencer }| HeldLock {
                name,
                id,
                sequencer,
            })
            .map_err(Into::into)
    }

    /// Attempts to release the given lock.
    ///
    /// **NOTE:** This endpoints requires the `RequestTarget` be a `HeldLock`
    ///
    /// # Examples
    /// ```no_run
    /// use seaplane::api::v1::{LocksRequestBuilder,LocksRequest};
    /// // First we acquire the lock
    /// let req = LocksRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .encoded_lock_name("bW9ieQo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.acquire(15, "test-client").unwrap();
    ///
    /// // Now it can be released
    /// let release_req = LocksRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .held_lock(resp)
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = release_req.release().unwrap();
    /// dbg!(resp)
    /// ```
    pub fn release(self) -> Result<()> {
        let url = self.held_lock_url()?;

        let resp = self.client.delete(url).bearer_auth(&self.token).send()?;

        map_error(resp)?
            .text()
            .map(|_| ()) // TODO: for now we drop the "success" message to control it ourselves
            .map_err(Into::into)
    }

    /// Attempts to renew the given lock, setting the TTL to the given `ttl`
    ///
    /// **NOTE:** This endpoints requires the `RequestTarget` be a `HeldLock`
    ///
    /// # Examples
    /// ```no_run
    /// use seaplane::api::v1::{LocksRequestBuilder,LocksRequest};
    /// // First we acquire the lock
    /// let req = LocksRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .encoded_lock_name("bW9ieQo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.acquire(15, "test-client").unwrap();
    ///
    /// // Now it can be renewed with a new TTL of 20
    /// let renew_req = LocksRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .held_lock(resp)
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = renew_req.renew(20).unwrap();
    /// dbg!(resp)
    /// ```
    pub fn renew(self, ttl: u32) -> Result<()> {
        let mut url = self.held_lock_url()?;

        url.query_pairs_mut().append_pair("ttl", &ttl.to_string());

        let resp = self.client.patch(url).bearer_auth(&self.token).send()?;

        map_error(resp)?
            .text()
            .map(|_| ()) // TODO: for now we drop the "success" message to control it ourselves
            .map_err(Into::into)
    }

    /// Gets information about a single lock.
    ///
    /// **NOTE:** This endpoints requires the `RequestTarget` be a `SingleLock`
    ///
    /// # Examples
    /// ```no_run
    /// use seaplane::api::v1::{LocksRequestBuilder,LocksRequest};
    /// // First we acquire the lock
    /// let req = LocksRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .encoded_lock_name("bW9ieQo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.get_lock_info().unwrap();
    /// dbg!(resp);
    /// ```
    pub fn get_lock_info(self) -> Result<LockInfo> {
        let url = self.single_lock_url()?;

        let resp = self.client.get(url).bearer_auth(&self.token).send()?;

        map_error(resp)?.json::<LockInfo>().map_err(Into::into)
    }

    /// Returns a single page of lock information for the given directory, beginning with the `from` key.
    ///
    /// If no directory is given, the root directory is used.
    /// If no `from` is given, the range begins from the start.
    ///
    /// If more pages are desired, perform another range request using the `next` value from the first request
    /// as the `from` value of the following request, or use `get_all_pages`.
    ///
    /// **NOTE:** This endpoint requires the `RequestTarget` be a `Range`.
    /// # Examples
    /// ```no_run
    /// use seaplane::api::v1::{LocksRequestBuilder, LocksRequest, RangeQueryContext, LockName};
    ///
    /// let root_dir_range: RangeQueryContext<LockName> = RangeQueryContext::new();
    ///
    /// let req = LocksRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .range(root_dir_range)
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.get_page().unwrap();
    ///
    /// if let Some(next_key) = resp.next {
    ///     let mut next_page_range = RangeQueryContext::new();
    ///     next_page_range.set_from(next_key);
    ///     
    ///     let req = LocksRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .range(next_page_range)
    ///     .build()
    ///     .unwrap();
    ///
    ///     let next_page_resp = req.get_page().unwrap();
    ///     dbg!(next_page_resp);
    /// }
    /// ```
    pub fn get_page(&self) -> Result<LockInfoRange> {
        match &self.target {
            RequestTarget::SingleLock(_) | RequestTarget::HeldLock(_) => {
                Err(SeaplaneError::IncorrectLocksRequestTarget)
            }
            RequestTarget::Range(_) => {
                let url = self.range_url()?;

                let resp = self.client.get(url).bearer_auth(&self.token).send()?;
                map_error(resp)?.json::<LockInfoRange>().map_err(Into::into)
            }
        }
    }

    /// Returns all held lock information for the given directory, from the `from` key onwards. May perform multiple requests.
    ///
    /// If no directory is given, the root directory is used.
    /// If no `from` is given, the range begins from the start.
    ///
    /// **NOTE:** This endpoint requires the `RequestTarget` be a `Range`.
    /// # Examples
    /// ```no_run
    /// use seaplane::api::v1::{LocksRequestBuilder, LocksRequest, RangeQueryContext};
    ///
    /// let root_dir_range = RangeQueryContext::new();
    ///
    /// let mut req = LocksRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .range(root_dir_range)
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.get_all_pages().unwrap();
    /// dbg!(resp);
    /// ```
    //TODO: Replace this with a collect on a Pages/Entries iterator
    pub fn get_all_pages(&mut self) -> Result<Vec<LockInfo>> {
        let mut pages = Vec::new();
        loop {
            let mut lir = self.get_page()?;
            pages.append(&mut lir.infos);
            if let Some(next_key) = lir.next {
                // TODO: Regrettable duplication here suggests that there should be a ConfigKeyRequest and a ConfigRangeRequest
                if let RequestTarget::Range(ref mut context) = self.target {
                    context.set_from(next_key);
                } else {
                    return Err(SeaplaneError::IncorrectLocksRequestTarget);
                }
            } else {
                break;
            }
        }
        Ok(pages)
    }
}
