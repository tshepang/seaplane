//! The API endpoints related to Tokens and Authentication

use reqwest::{
    blocking,
    header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE},
    Url,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::{identity::IDENTITY_API_URL, map_api_error},
    error::{Result, SeaplaneError},
};

/// An access token with tenant subdomain and ID
#[derive(Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "api_tests", derive(PartialEq))]
pub struct AccessToken {
    /// The JWT token
    pub token: String,
    /// Tenant ID
    pub tenant: u64,
    /// Tenant Subdomain
    pub subdomain: String,
}

#[derive(Default, Debug)]
pub struct TokenRequestBuilder {
    // Required for Bearer Auth
    api_key: Option<String>,
    // Used for testing
    #[doc(hidden)]
    base_url: Option<Url>,
}

impl TokenRequestBuilder {
    /// Create a new `Default` builder
    pub fn new() -> Self { Self::default() }

    /// Set the API Key used in Bearer Authorization
    ///
    /// **NOTE:** This is required
    #[must_use]
    pub fn api_key<S: Into<String>>(mut self, key: S) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Build a TokenRequest from the given parameters
    pub fn build(self) -> Result<TokenRequest> {
        if self.api_key.is_none() {
            return Err(SeaplaneError::MissingRequestApiKey);
        }

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        #[cfg_attr(not(feature = "api_tests"), allow(unused_mut))]
        let mut builder = blocking::Client::builder()
            .default_headers(headers)
            .https_only(true);

        #[cfg(feature = "api_tests")]
        {
            builder = builder.https_only(false);
        }

        let url = if let Some(url) = self.base_url {
            url.join("token")?
        } else {
            let mut url: Url = IDENTITY_API_URL.parse()?;
            url.set_path("token");
            url
        };

        Ok(TokenRequest {
            api_key: self.api_key.unwrap(),
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

/// For making requests against the `/token` APIs.
#[derive(Debug)]
pub struct TokenRequest {
    api_key: String,
    #[doc(hidden)]
    client: reqwest::blocking::Client,
    #[doc(hidden)]
    endpoint_url: Url,
}

impl TokenRequest {
    /// Create a new request builder
    pub fn builder() -> TokenRequestBuilder { TokenRequestBuilder::new() }

    /// Returns a short lived JWT that can be used to authenticate to other API endpoints
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::identity::TokenRequest;
    /// let req = TokenRequest::builder().api_key("abc123").build().unwrap();
    ///
    /// let resp = req.access_token().unwrap();
    /// dbg!(resp);
    /// ```
    pub fn access_token(&self) -> Result<String> {
        let resp = self
            .client
            .post(self.endpoint_url.clone())
            .bearer_auth(&self.api_key)
            .send()?;
        map_api_error(resp)?.text().map_err(Into::into)
    }

    /// Returns a JSON response of an `AccessToken` which contains the short lived JWT used to
    /// authenticate to other public API endpoints, along with addition fields for tenant ID and
    /// subdomain
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::identity::TokenRequest;
    /// let req = TokenRequest::builder().api_key("abc123").build().unwrap();
    ///
    /// let resp = req.access_token_json().unwrap();
    /// dbg!(resp);
    /// ```
    pub fn access_token_json(&self) -> Result<AccessToken> {
        let resp = self
            .client
            .post(self.endpoint_url.clone())
            .bearer_auth(&self.api_key)
            .header(ACCEPT, HeaderValue::from_static("application/json"))
            .send()?;
        map_api_error(resp)?
            .json::<AccessToken>()
            .map_err(Into::into)
    }
}
