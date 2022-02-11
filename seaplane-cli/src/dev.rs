use anyhow::Result;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header, TokenData};
use std::collections::HashMap;
use toml::Value;
use serde::{Deserialize, Serialize};

/// Generate an auth token from a secret and claims from toml
/// TODO: don't reuse toml's types, this is a bit of a hack
pub fn generate_auth_token(secret: impl AsRef<str>, claims: &HashMap<String, Value>) -> Result<String> {
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref().as_bytes()))?;
    Ok(token)
}

/// Proxy token type to control the `Header` options and Claims.
pub struct SeaplaneJwt {
    inner: TokenData<SeaplaneClaims>,
}

impl SeaplaneJwt {
    pub fn new() -> Self {
        Self {
            inner: TokenData {
                header: Header {
                    typ: Some("JWT".into()),
                    alg: Algorithm::HS256,
                    ..Default::default()
                },
                claims: SeaplaneClaims::default(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SeaplaneClaims {
    exp: usize,
    nbf: usize,
    iat: usize,
    iss: String,
}

impl Default for SeaplaneClaims {
    fn default() -> Self {
        todo!("impl SeaplaneClaims::default")
    }
}
