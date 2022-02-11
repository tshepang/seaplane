//! Context for the dev command
use crate::config::dev::DevConfig;
use std::collections::HashMap;
use toml::Value;

/// The dev command context
#[derive(Debug, Default)]
pub struct DevCtx {
    pub auth_key: String,
    pub auth_claims: HashMap<String, Value>,
}

impl From<&Option<DevConfig>> for DevCtx {
    fn from(cfg: &Option<DevConfig>) -> Self {
        if let Some(cfg) = cfg {
            DevCtx {
                auth_key: cfg.auth.key.clone().unwrap_or_default(),
                auth_claims: cfg.auth.claims.clone().unwrap_or_default(),
            }
        } else {
            DevCtx::default()
        }
    }
}
