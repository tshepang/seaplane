use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContainerStatus {
    Started,
    Stopped,
    Running,
}

/// The response from `GET /formations/NAME/containers`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct Containers {
    inner: Vec<Container>,
}

/// A single formation name in the response from `GET /formations/NAME/containers/ID`
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Container {
    /// ID of a particular container instance
    uuid: Uuid,
    /// Current Status
    status: ContainerStatus,
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
}

// We don't derive the trait because we only need to check the UUID to determine equivalence
impl PartialEq<Self> for Container {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
