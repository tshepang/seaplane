//! The `/config` endpoint APIs which allows working with [`KeyValue`]s

mod models;

use crate::{
    api::COORD_API_URL,
    error::{Result, SeaplaneError},
};

use reqwest::{
    blocking,
    header::{self, CONTENT_TYPE},
    Url,
};

pub use models::*;

/// A builder struct for creating a [`ConfigRequest`] which will then be used for making a
/// request against the `/config` APIs
#[derive(Debug, Default)]
pub struct ConfigRequestBuilder {
    // The target key or range of the request
    target: Option<RequestTarget>,
    // Required for Bearer Auth
    token: Option<String>,
    // Used for testing
    #[doc(hidden)]
    base_url: Option<Url>,
}

/// For making requests against the `/config` APIs.
#[derive(Debug)]
pub struct ConfigRequest {
    target: RequestTarget,
    token: String, // TODO: probably not a string
    #[doc(hidden)]
    client: reqwest::blocking::Client,
    #[doc(hidden)]
    endpoint_url: Url,
}

impl ConfigRequestBuilder {
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

    /// The key with which to query the store, encoded in url-safe base64.
    ///
    /// **NOTE:** This is not required for all endpoints
    #[must_use]
    pub fn encoded_key<S: Into<String>>(mut self, key: S) -> Self {
        self.target = Some(RequestTarget::Key(Key(key.into())));
        self
    }

    /// The context with which to perform a range query
    ///
    /// **NOTE:** This is not required for all endpoints
    #[must_use]
    pub fn range(mut self, context: RangeQueryContext) -> Self {
        self.target = Some(RequestTarget::Range(context));
        self
    }

    /// Build a ConfigRequest from the given parameters
    pub fn build(self) -> Result<ConfigRequest> {
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
            .ok_or(SeaplaneError::IncorrectConfigRequestTarget)?;

        #[cfg_attr(not(feature = "api_tests"), allow(unused_mut))]
        let mut builder = blocking::Client::builder()
            .default_headers(headers)
            .https_only(true);

        #[cfg(feature = "api_tests")]
        {
            builder = builder.https_only(false);
        }

        let url = if let Some(url) = self.base_url {
            url.join("v1/config/")?
        } else {
            let mut url: Url = COORD_API_URL.parse()?;
            url.set_path("v1/config/");
            url
        };
        Ok(ConfigRequest {
            target,
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

/// Adds a path segment to endpoint_url in the form "base64:{key}", assumes the path ends in /
// Needed as Url::join parses the new ending as a URL, and thus treats "base64" as a scheme.
// There might be a good reason it parses it though
fn add_base64_path_segment<S: AsRef<str>>(mut url: Url, key: S) -> Url {
    let new_path = format!("{}base64:{}", url.path(), key.as_ref());
    url.set_path(&new_path);
    url
}

impl ConfigRequest {
    /// Create a new request builder
    pub fn builder() -> ConfigRequestBuilder {
        ConfigRequestBuilder::new()
    }

    // Internal method creating the URL for all single key endpoints
    fn single_key_url(&self) -> Result<Url> {
        match &self.target {
            RequestTarget::Range(_) => Err(SeaplaneError::IncorrectConfigRequestTarget),
            RequestTarget::Key(Key(k)) => Ok(add_base64_path_segment(self.endpoint_url.clone(), k)),
        }
    }

    // Internal method creating the URL for range endpoints
    fn range_url(&self) -> Result<Url> {
        match &self.target {
            RequestTarget::Key(_) => Err(SeaplaneError::IncorrectConfigRequestTarget),
            RequestTarget::Range(RangeQueryContext { dir, after }) => {
                let mut url = self.endpoint_url.clone();

                if let Some(Directory(encoded_dir)) = dir {
                    url = add_base64_path_segment(url, encoded_dir);
                    // A directory is distinguished from a key by the trailing slash
                    url.set_path(&format!("{}/", url.path()));
                }

                if let Some(Key(after)) = after {
                    url.set_query(Some(&format!("after={}", after)));
                }

                Ok(url)
            }
        }
    }

    /// Returns the key value pair associated with the set key.
    ///
    /// **NOTE:** This endpoint requires the `RequestTarget` be a `Key`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use seaplane::api::v1::{ConfigRequestBuilder,ConfigRequest};
    ///
    /// let req = ConfigRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .encoded_key("bW9ieQo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.get_value().unwrap();
    /// dbg!(resp);
    /// ```
    pub fn get_value(&self) -> Result<KeyValue> {
        let url = self.single_key_url()?;
        self.client
            .get(url)
            .bearer_auth(&self.token)
            .send()?
            .json::<KeyValue>()
            .map_err(Into::into)
    }

    /// Adds a base64 encoded value to the store at the given key.
    ///
    /// **NOTE:** This endpoint requires the `RequestTarget` be a `Key`.
    ///
    /// # Examples
    /// ```no_run
    /// use seaplane::api::v1::{ConfigRequestBuilder,ConfigRequest};
    ///
    /// let req = ConfigRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .encoded_key("bW9ieQo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.put_value("YWhhYgo").unwrap();
    /// dbg!(resp);
    /// ```
    pub fn put_value<S: Into<blocking::Body>>(&self, value: S) -> Result<()> {
        let url = self.single_key_url()?;
        self.client
            .put(url)
            .bearer_auth(&self.token)
            .body(value)
            .send()?
            .text()
            .map(|_| ()) // TODO: for now we drop the "success" message to control it ourselves
            .map_err(Into::into)
    }

    /// Deletes the key value pair at from a given base64 encoded key.
    ///
    /// **NOTE:** This endpoint requires the `RequestTarget` be a `Key`.
    ///
    /// # Examples
    /// ```no_run
    /// use seaplane::api::v1::{ConfigRequestBuilder,ConfigRequest};
    ///
    /// let req = ConfigRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .encoded_key("bW9ieQo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.delete_value().unwrap();
    /// dbg!(resp);
    /// ```
    pub fn delete_value(&self) -> Result<()> {
        let url = self.single_key_url()?;
        self.client
            .delete(url)
            .bearer_auth(&self.token)
            .send()?
            .text()
            .map(|_| ()) // TODO: for now we drop the "success" message to control it ourselves
            .map_err(Into::into)
    }

    /// Returns a single page of key value pairs for the given directory, beginning with the `after` key.
    ///
    /// If no directory is given, the root directory is used.
    /// If no `after` is given, the range begins from the start.
    ///
    /// If more pages are desired, perform another range request using the `last` value from the first request.
    /// Or, use `get_all_pages`.
    ///
    /// **NOTE:** This endpoint requires the `RequestTarget` be a `Range`.
    /// # Examples
    /// ```no_run
    /// use seaplane::api::v1::{ConfigRequestBuilder, ConfigRequest, RangeQueryContext};
    ///
    /// let root_dir_range = RangeQueryContext{dir: None, after: None};
    ///
    /// let req = ConfigRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .range(root_dir_range)
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.get_page().unwrap();
    ///
    /// if resp.more {
    ///     let next_page_range = RangeQueryContext{dir: None, after: Some(resp.last)};
    ///     
    ///     let req = ConfigRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .range(next_page_range)
    ///     .build()
    ///     .unwrap();
    ///
    ///     let next_page_resp = req.get_page().unwrap();
    ///     dbg!(next_page_resp);
    /// }
    /// ```
    pub fn get_page(&self) -> Result<KeyValueRange> {
        match &self.target {
            RequestTarget::Key(_) => Err(SeaplaneError::IncorrectConfigRequestTarget),
            RequestTarget::Range(_) => {
                let url = self.range_url()?;

                self.client
                    .get(url)
                    .bearer_auth(&self.token)
                    .send()?
                    .json::<KeyValueRange>()
                    .map_err(Into::into)
            }
        }
    }

    /// Returns all key-value pairs for the given directory, from the `after` key onwards. May perform multiple requests.
    ///
    /// If no directory is given, the root directory is used.
    /// If no `after` is given, the range begins from the start.
    ///
    /// **NOTE:** This endpoint requires the `RequestTarget` be a `Range`.
    /// # Examples
    /// ```no_run
    /// use seaplane::api::v1::{ConfigRequestBuilder, ConfigRequest, RangeQueryContext};
    ///
    /// let root_dir_range = RangeQueryContext{dir: None, after: None};
    ///
    /// let mut req = ConfigRequestBuilder::new()
    ///     .token("abc123_token")
    ///     .range(root_dir_range)
    ///     .build()
    ///     .unwrap();
    ///
    /// let resp = req.get_all_pages().unwrap();
    /// dbg!(resp);
    /// ```
    //TODO: Replace this with a collect on a Pages/Entries iterator
    pub fn get_all_pages(&mut self) -> Result<Vec<KeyValue>> {
        let mut pages = Vec::new();
        loop {
            let mut kvr = self.get_page()?;
            pages.append(&mut kvr.kvs);
            if kvr.more {
                // TODO: Regrettable duplication here suggests that there should be a ConfigKeyRequest and a ConfigRangeRequest
                if let RequestTarget::Range(RangeQueryContext {
                    dir: _,
                    ref mut after,
                }) = self.target
                {
                    *after = Some(kvr.last);
                } else {
                    return Err(SeaplaneError::IncorrectConfigRequestTarget);
                }
            } else {
                break;
            }
        }
        Ok(pages)
    }
}
