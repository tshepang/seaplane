//! Interacting with our REST API endpoints

pub mod compute;
pub mod identity;
pub mod locks;
pub mod metadata;
pub mod restrict;
#[cfg(any(
    feature = "compute_api_v1",
    feature = "locks_api_v1",
    feature = "metadata_api_v1",
    feature = "restrict_api_v1"
))]
pub mod shared;

// API error handling
pub mod error;
pub use error::*;

/// Request builder base structs that handle token reuse
#[cfg(any(
    feature = "compute_api_v1",
    feature = "locks_api_v1",
    feature = "metadata_api_v1",
    feature = "restrict_api_v1",
))]
mod request;
#[cfg(any(
    feature = "compute_api_v1",
    feature = "locks_api_v1",
    feature = "metadata_api_v1",
    feature = "restrict_api_v1",
))]
pub(crate) use request::*;
