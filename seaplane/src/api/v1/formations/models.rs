mod containers;
mod endpoint;
mod formation;
mod image_ref;

pub use containers::*;
pub use endpoint::*;
pub use formation::*;
pub use image_ref::*;

use serde::{Deserialize, Serialize};
use strum::EnumString;

/// The processor architecture a [`Flight`] wants to run on
#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone, strum::Display)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    #[strum(serialize = "amd64")]
    AMD64,
    #[strum(serialize = "arm64")]
    ARM64,
}

/// A backing cloud provider that a [`Flight`] can run on. These are utilized in
/// [`FormationConfiguration`] to tell the scheduler where we can run your [`Flight`]s
#[derive(
    Debug, EnumString, strum::Display, Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone,
)]
#[allow(clippy::upper_case_acronyms)]
pub enum Provider {
    AWS,
    Azure,
    DigitalOcean,
    Equinix,
    GCP,
}

/// A regulatory region used in [`FormationConfiguration`]s that allow fine grained control over
/// where geographically the scheduler will run your [`Flight`]s
#[derive(
    strum::Display, EnumString, Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone,
)]
#[allow(clippy::upper_case_acronyms)]
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
