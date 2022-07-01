//! Wrapping seaplane SDK calls with things like CLI specific contexts, errors, etc.

mod formations;
mod locks;
mod metadata;

pub use formations::FormationsReq;
pub use locks::LocksReq;
pub use metadata::ConfigReq;

use reqwest::Url;
use seaplane::api::{AccessToken, TokenRequest};

use crate::error::{CliError, Context, Result};

/// Follows the same process as `request_token` but only returns the raw JWT string part of the
/// token
pub fn request_token_jwt(api_key: &str, identity_url: Option<&Url>) -> Result<String> {
    Ok(request_token(api_key, identity_url)?.token)
}

/// Makes a request against the `/token` endpoint of FlightDeck using the discovered API key and
/// returns the short lived Access token response.
///
/// Subject to change, but the access token is only good for 60 seconds (the raw JWT under the
/// `token` field contains `iat`, `nbf` and `exp` fields to determine the exact length of time the
/// token is valid for. However we don't want to introspect the token if possible as it's not
/// stable)
pub fn request_token(api_key: &str, identity_url: Option<&Url>) -> Result<AccessToken> {
    let mut builder = TokenRequest::builder().api_key(api_key);

    if let Some(url) = identity_url {
        builder = builder.base_url(url);
    }

    builder
        .build()
        .map_err(CliError::from)
        .context("Context: failed to build Access Token request\n")?
        .access_token_json()
        .map_err(CliError::from)
        .context("Context: failed to retrieve an Access Token\n")
}
