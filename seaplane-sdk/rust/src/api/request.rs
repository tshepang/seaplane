//! Purpose of those structs is basically just to allow token re-use/retry so
//! that we don't have to request a new API token on each and every call

use reqwest::{
    blocking,
    header::{self, CONTENT_TYPE},
    Url,
};

use crate::error::{Result, SeaplaneError};

/// A builder struct for creating a ApiRequest which will then be used for
/// making a request against the APIs
#[derive(Debug, Default)]
pub(crate) struct RequestBuilder<T> {
    // Target resource of this request
    pub target: Option<T>,
    // Required for Bearer Auth
    pub token: Option<String>,
    // API URL
    pub api_url: String,
    // Base path for the api
    pub base_path: String,
    // Used for testing
    #[doc(hidden)]
    pub base_url: Option<Url>,
}

impl<T> RequestBuilder<T> {
    /// Create a new builder
    pub(crate) fn new<S: Into<String>>(api_url: S, base_path: S) -> Self {
        Self {
            target: None,
            token: None,
            api_url: api_url.into(),
            base_path: base_path.into(),
            base_url: None,
        }
    }

    /// Set the token used in Bearer Authorization
    ///
    /// **NOTE:** This is required for all endpoints
    pub(crate) fn token<U: Into<String>>(mut self, token: U) -> Self {
        self.token = Some(token.into());
        self
    }

    /// The target resource to query as part of the request.
    ///
    /// **NOTE:** This is not required for all endpoints
    pub(crate) fn target(mut self, target: T) -> Self {
        self.target = Some(target);
        self
    }

    /// Build an APIRequest from the given parameters
    pub(crate) fn build(self) -> Result<ApiRequest<T>> {
        if self.token.is_none() {
            return Err(SeaplaneError::MissingRequestAuthToken);
        }

        let mut headers = header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, header::HeaderValue::from_static("application/json"));

        #[cfg_attr(not(feature = "api_tests"), allow(unused_mut))]
        let mut builder = blocking::Client::builder()
            .default_headers(headers)
            .https_only(true);

        // We allow HTTP traffic when testing or debugging
        // TODO: We should consider replacing debug_assertions with a scary
        // sounding feature flag like "insecure_urls" when making this public
        #[cfg(any(feature = "api_tests", debug_assertions))]
        {
            builder = builder.https_only(false);
        }

        let url = if let Some(url) = &self.base_url {
            url.join(&self.base_path)?
        } else {
            let mut url: Url = self.api_url.parse()?;
            url.set_path(&self.base_path);
            url
        };

        Ok(ApiRequest::<T> {
            target: self.target,
            token: self.token.unwrap(),
            client: builder.build()?,
            endpoint_url: url,
        })
    }

    // Used in testing and development to manually set the URL
    #[doc(hidden)]
    pub(crate) fn base_url<U: AsRef<str>>(mut self, url: U) -> Self {
        self.base_url = Some(url.as_ref().parse().unwrap());
        self
    }
}

#[derive(Debug)]
pub(crate) struct ApiRequest<T> {
    /// The target resource
    pub(crate) target: Option<T>,
    pub(crate) token: String,
    #[doc(hidden)]
    pub(crate) client: reqwest::blocking::Client,
    #[doc(hidden)]
    pub(crate) endpoint_url: Url,
}
