//! This module provides types that wrap the API endpoint models and add additional fields/context
//! that is only relevant for the CLI or purposes of consuming the API.

pub mod flight;
pub mod formation;

use std::{fmt, result::Result as StdResult};


use rand::Rng;
use serde::{Deserialize, Serialize};

pub fn validate_name(name: &str) -> StdResult<(), ()> {
    if name.len() > 27
        || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        || name.chars().filter(|c| *c == '-').count() > 3
        || name.contains("--")
    {
        return Err(());
    }

    Ok(())
}

pub fn generate_name() -> String {
    // TODO: Maybe set an upper bound on the number of iterations and don't expect
    names::Generator::default()
        .find(|name| validate_name(name).is_ok())
        .expect("Failed to generate a random name")
}

#[derive(Deserialize, Serialize, Copy, Clone, Hash, PartialEq, Eq)]
#[serde(transparent)]
pub struct Id {
    #[serde(
        serialize_with = "hex::serde::serialize",
        deserialize_with = "hex::serde::deserialize"
    )]
    pub inner: [u8; 32],
}

impl Id {
    pub fn new() -> Self {
        Self {
            inner: rand::thread_rng().gen(),
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.inner))
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id [ {} ]", self)
    }
}
