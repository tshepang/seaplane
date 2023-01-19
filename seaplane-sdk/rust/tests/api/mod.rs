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

use httpmock::prelude::*;
use once_cell::sync::Lazy;

// To be used with httpmock standalone server for dev testing
// MockServer::connect("127.0.0.1:5000")
// static MOCK_SERVER: Lazy<MockServer> = Lazy::new(|| MockServer::connect("127.0.0.1:5000"));
static MOCK_SERVER: Lazy<MockServer> = Lazy::new(MockServer::start);
