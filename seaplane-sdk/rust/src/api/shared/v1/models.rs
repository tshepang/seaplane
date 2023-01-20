//! This file contains models that are used across multiple APIs, like `Region`
//! and `Provider`

use serde::Serialize;
use strum::{EnumString, EnumVariantNames};

/// A backing cloud provider used to restrict data placement
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    Serialize,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    strum::Display,
    EnumString,
    EnumVariantNames,
)]
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
#[non_exhaustive]
pub enum Provider {
    AWS,
    Azure,
    DigitalOcean,
    Equinix,
    GCP,
}

impl_deser_from_str!(Provider);

#[cfg(test)]
mod test_provider {
    use super::*;

    #[test]
    fn provider_case_insensitive() {
        let provider: Provider = serde_json::from_str("\"aws\"").unwrap();
        assert_eq!(provider, Provider::AWS);
        let provider: Provider = serde_json::from_str("\"Aws\"").unwrap();
        assert_eq!(provider, Provider::AWS);
        let provider: Provider = serde_json::from_str("\"AWS\"").unwrap();
        assert_eq!(provider, Provider::AWS);
    }
}

/// A regulatory region used to restrict data placement
#[derive(
    strum::Display, EnumString, Debug, Serialize, Hash, Eq, PartialEq, PartialOrd, Ord, Copy, Clone,
)]
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
#[non_exhaustive]
pub enum Region {
    /// Asia
    XA,
    /// People's Republic of China
    XC,
    /// Europe
    XE,
    /// Africa
    XF,
    /// North America
    XN,
    /// Oceania
    XO,
    /// Antarctica
    XQ,
    /// South America
    XS,
    /// The UK
    XU,
}

impl_deser_from_str!(Region);

#[cfg(test)]
mod test_region {
    use super::*;

    #[test]
    fn region_case_insensitive() {
        let region: Region = serde_json::from_str("\"xn\"").unwrap();
        assert_eq!(region, Region::XN);
        let region: Region = serde_json::from_str("\"Xn\"").unwrap();
        assert_eq!(region, Region::XN);
        let region: Region = serde_json::from_str("\"XN\"").unwrap();
        assert_eq!(region, Region::XN);
    }
}
