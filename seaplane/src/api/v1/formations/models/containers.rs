use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::v1::formations::models::{Provider, Region};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContainerStatus {
    Started,
    Stopped,
    Running,
}

/// Information about a particular Container Host
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContainerHostInfo {
    /// The approximate decimal latitude of the container host (in the range of `-90.0..90.0`)
    host_latitude: Option<f32>,
    /// The approximate decimal longitude of the container host (in the range of `-180.0..180.0`)
    host_longitude: Option<f32>,
    /// An IATA airport code that the container host is near
    host_iata: Option<String>,
    /// The ISO 3166-1 alpha-2 country code the container host is operating in
    host_country: Option<String>,
    /// The regulatory region the container host is within
    host_region: Option<Region>,
    /// The provider the container host is backed by
    host_provider: Option<Provider>,
}

/// The response from `GET /formations/NAME/containers`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct Containers {
    inner: Vec<Container>,
}

/// A single formation name in the response from `GET /formations/NAME/containers/ID`
///
/// **NOTE:** All `usage` and the fields are currently unimplemented in the backend and
/// will always return `None`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Container {
    /// ID of a particular container instance
    uuid: Uuid,
    /// Current Status
    status: ContainerStatus,
    /// The name of the Flight that this container instance is running for
    flight_name: String,
    // TODO: The flight could be a member of multiple configurations within the Formation which
    // would mean this could be multiple UUIDs, no?
    /// The Formation Configuration's UUID that this container instance is a part of
    configuration_id: Uuid,
    /// Exit status if the container has stopped
    #[serde(default)]
    exit_status: Option<i32>,
    /// Number of bytes received from the public internet
    #[serde(default)]
    public_ingress_usage: Option<u64>,
    /// Number of bytes sent to the public internet
    #[serde(default)]
    public_egress_usage: Option<u64>,
    /// Number of bytes received from other container instances
    #[serde(default)]
    private_ingress_usage: Option<u64>,
    /// Number of bytes sent to other container instances
    #[serde(default)]
    private_egress_usage: Option<u64>,
    /// Number of bytes used by this container's disk
    #[serde(default)]
    disk_usage: Option<u64>,
    /// Number of bytes of RAM this container has used
    #[serde(default)]
    ram_usage: Option<u64>,
    /// Total number of CPU seconds this container has used
    #[serde(default)]
    cpu_usage: Option<u64>,
    /// Information about the host the container is running on
    #[serde(flatten, default)]
    host_info: Option<ContainerHostInfo>,
}

// We don't derive the trait because we only need to check the UUID to determine equivalence
impl PartialEq<Self> for Container {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
