mod containers;
mod endpoint;
mod formation;
mod image_ref;

pub use containers::*;
pub use endpoint::*;
pub use formation::*;
pub use image_ref::*;

use serde::{
    de::{self, Deserialize, Deserializer},
    Serialize,
};
use strum::{EnumString, EnumVariantNames};

/// Implements Deserialize using FromStr
macro_rules! impl_deser_from_str {
    ($t:ty) => {
        impl<'de> Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                s.parse().map_err(de::Error::custom)
            }
        }
    };
}

/// The processor architecture a [`Flight`] wants to run on
#[derive(
    Debug, Serialize, Hash, Eq, PartialEq, Copy, Clone, strum::Display, EnumString, EnumVariantNames,
)]
#[strum(ascii_case_insensitive)]
pub enum Architecture {
    AMD64,
    ARM64,
}

impl_deser_from_str!(Architecture);

#[cfg(test)]
mod test_arch {
    use super::*;

    #[test]
    fn architecture_case_insensitive() {
        let arch: Architecture = serde_json::from_str("\"amd64\"").unwrap();
        assert_eq!(arch, Architecture::AMD64);
        let arch: Architecture = serde_json::from_str("\"Amd64\"").unwrap();
        assert_eq!(arch, Architecture::AMD64);
        let arch: Architecture = serde_json::from_str("\"AMD64\"").unwrap();
        assert_eq!(arch, Architecture::AMD64);
    }
}

/// A backing cloud provider that a [`Flight`] can run on. These are utilized in
/// [`FormationConfiguration`] to tell the scheduler where we can run your [`Flight`]s
#[derive(
    Debug, Serialize, Hash, Eq, PartialEq, Copy, Clone, strum::Display, EnumString, EnumVariantNames,
)]
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
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

/// A regulatory region used in [`FormationConfiguration`]s that allow fine grained control over
/// where geographically the scheduler will run your [`Flight`]s
#[derive(strum::Display, EnumString, Debug, Serialize, Hash, Eq, PartialEq, Copy, Clone)]
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
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
