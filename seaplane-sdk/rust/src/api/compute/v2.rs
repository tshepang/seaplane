//! The `/formations` endpoint APIs which allows working with [`Formation`]s,
//! [`Flight`]s, and the underlying containers

mod models;
pub use models::*;

use crate::{
    api::{
        compute::{error::map_api_error, COMPUTE_API_URL},
        ApiRequest, RequestBuilder,
    },
    error::{Result, SeaplaneError},
};

const COMPUTE_API_ROUTE: &str = "v2/formations";

/// A builder struct for creating a [`FormationsRequest`] which will then be used for making a
/// request against the `/formations` APIs
#[derive(Debug)]
pub struct FormationsRequestBuilder {
    builder: RequestBuilder<String>,
}

impl From<RequestBuilder<String>> for FormationsRequestBuilder {
    fn from(builder: RequestBuilder<String>) -> Self { Self { builder } }
}

impl Default for FormationsRequestBuilder {
    fn default() -> Self { Self::new() }
}
impl FormationsRequestBuilder {
    pub fn new() -> Self { RequestBuilder::new(COMPUTE_API_URL, COMPUTE_API_ROUTE).into() }

    /// Builds a FormationsRequest from the given parameters
    pub fn build(self) -> Result<FormationsRequest> { Ok(self.builder.build()?.into()) }

    /// Set the token used in Bearer Authorization
    ///
    /// **NOTE:** This is required for all endpoints
    #[must_use]
    pub fn token<U: Into<String>>(self, token: U) -> Self { self.builder.token(token).into() }

    /// Allow non-HTTPS endpoints for this request (default: `false`)
    #[cfg(any(feature = "allow_insecure_urls", feature = "danger_zone"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "allow_insecure_urls", feature = "danger_zone"))))]
    pub fn allow_http(self, yes: bool) -> Self { self.builder.allow_http(yes).into() }

    /// Allow invalid TLS certificates (default: `false`)
    #[cfg(any(feature = "allow_invalid_certs", feature = "danger_zone"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "allow_invalid_certs", feature = "danger_zone"))))]
    pub fn allow_invalid_certs(self, yes: bool) -> Self {
        self.builder.allow_invalid_certs(yes).into()
    }

    // Used in testing and development to manually set the URL
    #[doc(hidden)]
    pub fn base_url<U: AsRef<str>>(self, url: U) -> Self { self.builder.base_url(url).into() }

    /// The name of the Formation to query as part of the request.
    #[must_use]
    pub fn name<S: Into<String>>(self, name: S) -> Self { self.builder.target(name.into()).into() }
}

/// For making requests against the `/formations` APIs.
#[derive(Debug)]
pub struct FormationsRequest {
    request: ApiRequest<String>,
}

impl From<ApiRequest<String>> for FormationsRequest {
    fn from(request: ApiRequest<String>) -> Self { Self { request } }
}

impl FormationsRequest {
    /// Create a new request builder
    pub fn builder() -> FormationsRequestBuilder { FormationsRequestBuilder::new() }

    /// Create a new Formation and returns the IDs of the created Formation.
    ///
    /// Uses `POST /formations/NAME`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::compute::v2::{FormationsRequest, Formation, Flight};
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let formation = Formation::builder()
    ///     .add_flight(
    ///         Flight::builder()
    ///             .name("myflight")
    ///             .image("my/image:latest")
    ///             .build()
    ///             .unwrap(),
    ///     )
    ///     .build()
    ///     .unwrap();
    /// let resp = req.create(&formation).unwrap();
    /// dbg!(resp);
    /// ```
    pub fn create(&self, formation: &Formation) -> Result<()> {
        if self.request.target.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self
            .request
            .endpoint_url
            // We have to add "formations" because that's how URL's join() method works
            .join(&format!("formations/{}", self.name()))?;
        let req = self
            .request
            .client
            .post(url)
            .bearer_auth(&self.request.token)
            .json(formation);
        let resp = req.send()?;
        map_api_error(resp)?;
        Ok(())
    }

    /// Deletes a formation
    ///
    /// Uses `DELETE /formations/NAME`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::compute::v2::{FormationsRequest};
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(req.delete().is_ok());
    /// ```
    pub fn delete(&self) -> Result<String> {
        if self.request.target.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self
            .request
            .endpoint_url
            .join(&format!("formations/{}", self.name()))?;
        let resp = self
            .request
            .client
            .delete(url)
            .bearer_auth(&self.request.token)
            .send()?;

        map_api_error(resp)?.text().map_err(Into::into)
    }

    /// Query the status of a Formation
    ///
    /// Uses `GET /formations/NAME/status`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::compute::v2::{FormationsRequest};
    /// let req = FormationsRequest::builder()
    ///     .token("abc123")
    ///     .name("foo")
    ///     .build()
    ///     .unwrap();
    ///
    /// dbg!(req.status().unwrap());
    /// ```
    pub fn status(&self) -> Result<FormationStatus> {
        if self.request.target.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self
            .request
            .endpoint_url
            .join(&format!("formations/{}/status", self.name()))?;
        let resp = self
            .request
            .client
            .get(url)
            .bearer_auth(&self.request.token)
            .send()?;

        map_api_error(resp)?
            .json::<FormationStatus>()
            .map_err(Into::into)
    }

    // @TODO: a paging iterator may be more appropriate here in the future
    /// Returns a list of all the Formations you have access to
    ///
    /// Uses `GET /formations`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::compute::v2::{FormationsRequest};
    /// let req = FormationsRequest::builder()
    ///     .token("abc123_token")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.list().unwrap();
    /// dbg!(resp);
    /// ```
    pub fn list(&self) -> Result<Vec<Formation>> {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(self.request.endpoint_url.clone())
            .bearer_auth(&self.request.token)
            .send()?;

        map_api_error(resp)?
            .json::<Vec<Formation>>()
            .map_err(Into::into)
    }

    /// Returns a single Formations
    ///
    /// Uses `GET /formations/NAME`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use seaplane::api::compute::v2::{FormationsRequest};
    /// let req = FormationsRequest::builder()
    ///     .token("abc123_token")
    ///     .name("stubb")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.get().unwrap();
    /// dbg!(resp);
    /// ```
    pub fn get(&self) -> Result<Formation> {
        if self.request.target.is_none() {
            return Err(SeaplaneError::MissingFormationName);
        }
        let url = self
            .request
            .endpoint_url
            .join(&format!("formations/{}", self.name()))?;
        let resp = self
            .request
            .client
            .get(url)
            .bearer_auth(&self.request.token)
            .send()?;

        map_api_error(resp)?.json::<Formation>().map_err(Into::into)
    }

    // Internal, only used when can only be a valid name.
    #[inline]
    fn name(&self) -> &str { self.request.target.as_deref().unwrap() }
}
