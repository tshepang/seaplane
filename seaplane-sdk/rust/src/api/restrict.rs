/// The base URL for our Restrict API endpoints
pub static RESTRICT_API_URL: &str = "https://metadata.cplane.cloud/";

#[cfg(feature = "restrict_api_v1")]
pub mod v1;
