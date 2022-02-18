//! Interacting with our REST API endpoints

#[cfg(feature = "api_v1")]
pub mod v1;

/// The base URL for our Compute API endpoints
///
/// The compute API handles all things compute such as building `FormationConfiguration`s to
/// `Flight`s to the underlying Containers.
pub static COMPUTE_API_URL: &str = "https://compute.seaplane.io/";
