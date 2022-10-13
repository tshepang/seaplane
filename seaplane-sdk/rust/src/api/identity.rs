/// The base URL for our Identity API endpoints.
///
/// Identity contains endpoints for things such as Authentication
pub static IDENTITY_API_URL: &str = "https://identity.cplane.cloud/";

// The `/token` endpoint is not versioned
pub mod token;
pub use token::*;
