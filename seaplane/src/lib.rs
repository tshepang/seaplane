use jsonwebtoken::{Algorithm, Header, TokenData};
use serde::{Deserialize, Serialize};
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
