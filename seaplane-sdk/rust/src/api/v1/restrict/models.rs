use std::fmt;

use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize,
};
use strum::{AsRefStr, EnumString, EnumVariantNames};

use crate::{
    api::v1::{impl_deser_from_str, Provider, RangeQueryContext, Region},
    base64::Base64Encoded,
    impl_base64,
};

/// The target of a request. It can be either a single restriction or a list of
/// restrictions
#[non_exhaustive]
#[derive(Debug)]
pub(crate) enum RequestTarget {
    Single {
        api: String,
        directory: RestrictedDirectory,
    },
    ApiRange {
        api: String,
        context: RangeQueryContext<RestrictedDirectory>,
    },
    AllRange {
        from_api: Option<String>,
        context: RangeQueryContext<RestrictedDirectory>,
    },
}

/// Contains information about the restricted API, directory, restriction
/// details and state
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Restriction {
    pub api: Api,
    pub directory: RestrictedDirectory,
    pub details: RestrictionDetails,
    pub state: RestrictionState,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    AsRefStr,
    strum::Display,
    EnumString,
    EnumVariantNames,
)]
#[strum(ascii_case_insensitive)]
#[non_exhaustive]
pub enum Api {
    Config,
    Locks,
}

impl_deser_from_str!(Api);

#[cfg(test)]
mod test_api {
    use super::*;

    #[test]
    fn api_case_insensitive() {
        let api: Api = serde_json::from_str("\"config\"").unwrap();
        assert_eq!(api, Api::Config);
        let api: Api = serde_json::from_str("\"Config\"").unwrap();
        assert_eq!(api, Api::Config);
        let api: Api = serde_json::from_str("\"CONFIG\"").unwrap();
        assert_eq!(api, Api::Config);
    }
}
/// A key pointing to a directory, encoded in url-safe base64.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(transparent)]
pub struct RestrictedDirectory {
    inner: Base64Encoded,
}
impl_base64!(RestrictedDirectory);

impl AsRef<str> for RestrictedDirectory {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}

impl fmt::Display for RestrictedDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Serialize, strum::Display, EnumString, EnumVariantNames,
)]
#[strum(ascii_case_insensitive)]
pub enum RestrictionState {
    Pending,
    Enforced,
}

impl_deser_from_str!(RestrictionState);

/// Defines limits on where data can be stored.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct RestrictionDetails {
    #[serde(default)]
    pub regions_allowed: Vec<Region>,
    #[serde(default)]
    pub regions_denied: Vec<Region>,
    #[serde(default)]
    pub providers_allowed: Vec<Provider>,
    #[serde(default)]
    pub providers_denied: Vec<Provider>,
}

/// The response given from a range query
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RestrictionsRange {
    /// A lower bound of the next page of results
    pub next_key: Option<RestrictedDirectory>,
    /// The range of key value pairs returned
    pub restrictions: Vec<Restriction>,
}
