//! Interacting with our REST API endpoints

#[cfg(feature = "api_v1")]
pub mod v1;

// The `/token` endpoint is not versioned
pub mod token;
pub use token::*;

/// The base URL for our Compute API endpoints
///
/// The compute API handles all things compute such as building `FormationConfiguration`s to
/// `Flight`s to the underlying Containers.
pub static COMPUTE_API_URL: &str = "https://compute.seaplanet.io/";

/// The base URL for our FlightDeck API endpoints.
///
/// FlightDeck contains endpoints for things such as Authentication
pub static FLIGHTDECK_API_URL: &str = "https://flightdeck.seaplanet.io/";

/// The OCI registry URL. The `https://` is omitted because that is not part of an OCI registry
/// reference.
pub static IMAGE_REGISTRY_URL: &str = "registry.seaplanet.io/";
