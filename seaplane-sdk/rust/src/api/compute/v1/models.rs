mod containers;
mod endpoint;
mod formation;
use serde::Serialize;
use strum::{Display, EnumString, EnumVariantNames};

pub use crate::api::compute::v1::models::{containers::*, endpoint::*, formation::*};

/// The processor architecture a [`Flight`] wants to run on
#[derive(
    Debug, Serialize, Hash, Eq, PartialEq, Copy, Clone, Display, EnumString, EnumVariantNames,
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
