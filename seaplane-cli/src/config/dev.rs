//! The configuration of the [dev] table.
use crate::config::ExtendConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml::Value;
use log::warn;

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]

pub struct DevConfig {
    pub auth: DevAuthConfig,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]

pub struct DevAuthConfig {
    pub key: Option<String>,
    pub claims: Option<HashMap<String, Value>>,
}

impl ExtendConfig for DevConfig {
    fn extend(&mut self, other: &Self) {
        self.auth.extend(&other.auth);
    }
}

impl ExtendConfig for DevAuthConfig {
    fn extend(&mut self, other: &Self) {
        // override key if it is set
        if let Some(other_key) = &other.key {
            warn!("overriding auth.key");
            self.key = Some(other_key.clone());
        }

        // override any claims (note it isn't possible to unset a claim)
        if let Some(other_claims) = &other.claims {
            if let Some(claims) = &mut self.claims {
                warn!("extending auth.claims");
                claims.extend(other_claims.clone().into_iter());
            } else {
                warn!("auth.claims set from another source");
                self.claims = Some(other_claims.clone());
            }
        }
    }
}
