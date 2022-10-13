// We have to go through this little bit of indirection because of how integration directory
// structure works.

#[cfg(feature = "compute_api_v1")]
mod formation_requests;
#[cfg(feature = "locks_api_v1")]
mod locks_requests;
#[cfg(feature = "metadata_api_v1")]
mod metadata_requests;
#[cfg(feature = "restrict_api_v1")]
mod restrict_requests;
mod token_requests;
