/// The base URL for our Identity API endpoints.
///
/// Identity contains endpoints for things such as Authentication
pub static IDENTITY_API_URL: &str = "https://flightdeck.cplane.cloud/";

#[cfg(feature = "identity_api_v0")]
pub mod v0;
