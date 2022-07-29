//! Interacting with our REST API endpoints

#[cfg(feature = "api_v1")]
pub mod v1;

// API error handling
pub mod error;
pub use error::*;

// The `/token` endpoint is not versioned
pub mod token;
pub use token::*;

/// The base URL for our Compute API endpoints
///
/// The compute API handles all things compute such as building `FormationConfiguration`s to
/// `Flight`s to the underlying Containers.
pub static COMPUTE_API_URL: &str = "https://compute.cplane.cloud/";

/// The base URL for our Identity API endpoints.
///
/// Identity contains endpoints for things such as Authentication
pub static IDENTITY_API_URL: &str = "https://identity.cplane.cloud/";

/// The base URL for our Metadata endpoints
pub static METADATA_API_URL: &str = "https://metadata.cplane.cloud/";
