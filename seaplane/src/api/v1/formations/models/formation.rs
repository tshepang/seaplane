use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[cfg(doc)]
use crate::api::v1::FormationsRequest;
use crate::{
    api::v1::{Architecture, EndpointKey, EndpointValue, Provider, Region},
    error::{Result, SeaplaneError},
};

/// A builder for creating a [`FormationConfiguration`] which is the primary way to describe a
/// valid configuration for a Formation.
#[derive(Debug, Default)]
pub struct FormationConfigurationBuilder {
    affinity: Vec<String>,
    connections: Vec<String>,
    flights: Vec<Flight>,
    public_endpoints: HashMap<EndpointKey, EndpointValue>,
    formation_endpoints: HashMap<EndpointKey, EndpointValue>,
    flight_endpoints: HashMap<EndpointKey, EndpointValue>,
    providers_allowed: HashSet<Provider>,
    providers_denied: HashSet<Provider>,
    regions_allowed: HashSet<Region>,
    regions_denied: HashSet<Region>,
}

impl FormationConfigurationBuilder {
    /// Adds the name of another Formation that this Formation has an affinity for.
    ///
    /// This is a hint to the scheduler to place containers that run in each of these formations
    /// "close" to eachother (for some version of close including but not limited to latency).
    ///
    /// **NOTE:** This method can be called multiple times. All values will be utilized.
    #[must_use]
    pub fn add_affinity<S: Into<String>>(mut self, name: S) -> Self {
        self.affinity.push(name.into());
        self
    }

    /// Removes all `affinity` values
    pub fn clear_affinity(&mut self) {
        self.affinity.clear();
    }

    /// Adds the name of another Formation that this Formation is connected to. Two Formations can
    /// communicate over their `FormationConfigurationBuilder::formation_endpoints` if and only if
    /// both formations opt in to that connection by adding eachother to their connection mapping.
    ///
    /// **NOTE:** This method can be called multiple times. All values will be utilized.
    #[must_use]
    pub fn add_connection<S: Into<String>>(mut self, name: S) -> Self {
        self.connections.push(name.into());
        self
    }

    /// Removes all connection mappings
    pub fn clear_connections(&mut self) {
        self.connections.clear();
    }

    /// Add a [`Flight`] to the makeup of this Formation Configuration.
    ///
    /// **NOTE:** This method can be called multiple times. All values will be utilized.
    #[must_use]
    pub fn add_flight(mut self, flight: Flight) -> Self {
        self.flights.push(flight);
        self
    }

    /// Removes all [`Flight`]s from this Formation Configuration
    pub fn clear_flights(&mut self) {
        self.flights.clear();
    }

    /// Adds an entry to `public_endpoint` map whose keys describe the publically exposed endpoints
    /// of this Formation. The keys take the form `http:{endpoint_route}`.
    ///
    /// The values take the form `{flight_name}:{port}` and describe where traffic hitting this
    /// endpoint should be routed.
    ///
    /// For example, a key of `http:/foo/bar` and value `baz:1234` would mean "Route all HTTP
    /// traffic from the public internet hitting the path `/foo/bar` on this formation's domain to
    /// the [`Flight`] named `baz` on port `1234`.
    ///
    /// For now only `http:path` keys are supported, but in the future `tcp` and `udp` keys may be
    /// supported as well.
    ///
    /// **NOTE:** This method can be called multiple times. All values will be utilized.
    #[must_use]
    pub fn add_public_endpoint(mut self, key: EndpointKey, value: EndpointValue) -> Self {
        self.public_endpoints.insert(key, value);
        self
    }

    /// Adds an entry to the `formation_endpoints` map, which describes the privately exposed
    /// endpoints of this formation. These private endpoints are those that this formation exposes
    /// to other formations listed in is's `connection` mapping
    /// ([`FormationConfigurationBuilder::add_connection`]).
    ///
    /// The keys take the form `http:{endpoint}`, `tcp:{port}` or or `udp:{port}`. The values take
    /// the form `{flight_name}:{port}` and describe where traffic hitting this endpoint should be
    /// routed.
    ///
    /// For example, a key of `tcp:1234` with value `foo:4321` would mean "Route all internal `tcp`
    /// traffic hitting this formation on port `1234` to this Formation's `foo` [`Flight`] on port
    /// `4321`".
    ///
    /// If this conflicts with `flight_endpoints` map
    /// ([`FormationConfigurationBuilder::add_flight_endpoint`]) the configuration will be
    /// rejected.
    ///
    /// **NOTE:** This method can be called multiple times. All values will be utilized.
    #[must_use]
    pub fn add_formation_endpoint(mut self, key: EndpointKey, value: EndpointValue) -> Self {
        self.formation_endpoints.insert(key, value);
        self
    }

    /// Adds an entry to the `flight_endpoints` map, which describes the endpoints which *this
    /// Formation's* containers can hit to communicate with one another.
    ///
    /// Containers within a Formation can always communicate directly to one another but these
    /// endpoints are load-balanced and allow for much more simple usage in many cases.
    ///
    /// The keys take the form `http:{endpoint}`, `tcp:{port}` or `udp:{port}`. The values take the
    /// form `{flight_name}:{port}` and describe where traffic on that endpoint will be routed.
    ///
    /// For example, a key of `udp:1234` with a value `foo:4321` would mean "Route all internal
    /// `udp` traffic hitting this Formation on port `1234` to the Formation's `foo` [`Flight`] on
    /// port `4321`".
    ///
    /// If this conflicts with `formation_endpoints`
    /// ([`FormationConfigurationBuilder::add_formation_endpoint`]) the configuration will be
    /// rejected.
    #[must_use]
    pub fn add_flight_endpoint(mut self, key: EndpointKey, value: EndpointValue) -> Self {
        self.flight_endpoints.insert(key, value);
        self
    }

    /// Add a [`Provider`] which the scheduler is allowed to schedule [`Flight`]s of this formation
    /// to run on. By default all [`Provider`]s are allowed. Adding an entry here effevtively
    /// restricts the [`Flight`]s of this Formation to only the listed [`Provider`]s.
    ///
    /// If this conflicts with `providers_denied`
    /// ([`FormationConfigurationBuilder::add_denied_provider`]) (e.g. [`Provider::GCP`] is both
    /// allowed and denied) the configuration is invalid and will be rejected.
    ///
    /// **NOTE:** This method can be called multiple times. All values will be utilized.
    #[must_use]
    pub fn add_allowed_provider<P: Into<Provider>>(mut self, provider: P) -> Self {
        self.providers_allowed.insert(provider.into());
        self
    }

    /// The inverse of [`FormationConfigurationBuilder::add_allowed_provider`] which specifies
    /// [`Provider`] which the scheduler is prohibited from scheduling [`Flight`]s of this formation to
    /// run on. By default no [`Provider`]s are denied.
    ///
    /// If this conflicts with `providers_allowed`
    /// ([`FormationConfigurationBuilder::add_allowed_provider`]) (e.g. [`Provider::GCP`] is both
    /// allowed and denied) the configuration is invalid and will be rejected.
    ///
    /// **NOTE:** This method can be called multiple times. All values will be utilized.
    #[must_use]
    pub fn add_denied_provider<P: Into<Provider>>(mut self, provider: P) -> Self {
        self.providers_denied.insert(provider.into());
        self
    }

    /// Add a [`Region`] which the scheduler is allowed to schedule [`Flight`]s of this formation
    /// to run within. By default all [`Region`]s are allowed. Adding an entry here effectively
    /// restricts the [`Flight`]s of this Formation to only the listed [`Region`]s.
    ///
    /// If this conflicts with `regions_denied`
    /// ([`FormationConfigurationBuilder::add_denied_region`]) (e.g. [`Region::XN`] is both
    /// allowed and denied) the configuration is invalid and will be rejected.
    ///
    /// **NOTE:** This method can be called multiple times. All values will be utilized.
    #[must_use]
    pub fn add_allowed_region<R: Into<Region>>(mut self, region: R) -> Self {
        self.regions_allowed.insert(region.into());
        self
    }

    /// The inverse of [`FormationConfigurationBuilder::add_allowed_region`] which specifies
    /// [`Region`] which the scheduler is prohibited from scheduling [`Flight`]s of this formation to
    /// run within. By default no [`Region`]s are denied.
    ///
    /// If this conflicts with `regions_allowed`
    /// ([`FormationConfigurationBuilder::add_allowed_region`]) (e.g. [`Region::XN`] is both
    /// allowed and denied) the configuration is invalid and will be rejected.
    ///
    /// **NOTE:** This method can be called multiple times. All values will be utilized.
    #[must_use]
    pub fn add_denied_region<R: Into<Region>>(mut self, region: R) -> Self {
        self.regions_denied.insert(region.into());
        self
    }

    /// Performs validation checks, and builds the instance of [`FormationConfiguration`]
    pub fn build(self) -> Result<FormationConfiguration> {
        if self.flights.is_empty() {
            return Err(SeaplaneError::EmptyFlights);
        }

        if (self
            .providers_allowed
            .intersection(&self.providers_denied)
            .count()
            + self
                .regions_allowed
                .intersection(&self.regions_denied)
                .count())
            > 0
        {
            return Err(SeaplaneError::ConflictingRequirements);
        }

        // TODO: Check the endpoint maps for conflicts
        // TODO: from @Jess3Jane: Check that all endpoints only refer to components that exist
        // (within the confines of what is local to this Formation)

        Ok(FormationConfiguration {
            affinity: self.affinity,
            connections: self.connections,
            flights: self.flights,
            public_endpoints: self.public_endpoints,
            formation_endpoints: self.formation_endpoints,
            flight_endpoints: self.flight_endpoints,
            providers_allowed: self.providers_allowed,
            providers_denied: self.providers_denied,
            regions_allowed: self.regions_allowed,
            regions_denied: self.regions_denied,
        })
    }
}

/// Represents a single configuration of a Formation. A Formation may have many
/// [`ActiveConfiguration`]s at once which will have traffic ballanced between them based on their
/// `traffic_weight` values.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FormationConfiguration {
    #[serde(default)]
    affinity: Vec<String>,
    #[serde(default)]
    connections: Vec<String>,
    flights: Vec<Flight>,
    #[serde(default)]
    public_endpoints: HashMap<EndpointKey, EndpointValue>,
    #[serde(default)]
    formation_endpoints: HashMap<EndpointKey, EndpointValue>,
    #[serde(default)]
    flight_endpoints: HashMap<EndpointKey, EndpointValue>,
    #[serde(default)]
    providers_allowed: HashSet<Provider>,
    #[serde(default)]
    providers_denied: HashSet<Provider>,
    #[serde(default)]
    regions_allowed: HashSet<Region>,
    #[serde(default)]
    regions_denied: HashSet<Region>,
}

impl FormationConfiguration {
    /// Create a [`FormationConfigurationBuilder`] to build a new configuration
    pub fn builder() -> FormationConfigurationBuilder {
        FormationConfigurationBuilder::default()
    }
}

/// A builder to construct [`Flight`]s
#[derive(Debug, Default)]
pub struct FlightBuilder {
    name: Option<String>,
    image: Option<Url>,
    minimum: u64,
    maximum: Option<u64>,
    architecture: Vec<Architecture>,
    api_permission: bool,
}

impl FlightBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// The [`Flight`] name, which must be unique within the Formation
    #[must_use]
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// A container image registry URL which points to the container image this [`Flight`] should uses
    #[must_use]
    pub fn image<U: Into<Url>>(mut self, url: U) -> Self {
        self.image = Some(url.into());
        self
    }

    /// The minimum number of instances this [`Flight`] should ever have running. Default: `0`
    #[must_use]
    pub fn minimum(mut self, num: u64) -> Self {
        self.minimum = num;
        self
    }

    /// The maximum number of instances this [`Flight`] should ever have running. By default if
    /// this is not set Seaplane will run as many instances as are necessary to serve the incoming
    /// traffic.
    #[must_use]
    pub fn maximum(mut self, num: u64) -> Self {
        self.maximum = Some(num);
        self
    }

    /// Reset the maximum number of instances this [`Flight`] should ever have running to infinite.
    pub fn clear_maximum(&mut self) {
        self.maximum = None;
    }

    /// Adds an [`Architecture`]s the [`Flight`] can be run on. If not specified, the default value
    /// is a single entry of [`Architecture::AMD64`].
    ///
    /// **NOTE:** This method can be called multiple times.
    #[must_use]
    pub fn add_architecture<A: Into<Architecture>>(mut self, arch: A) -> Self {
        self.architecture.push(arch.into());
        self
    }

    /// Should this [`Flight`] have access to Seaplane's APIs? If set to `true` an API token will
    /// be generated and provided to the running container instances in the `SEAPLANE_API_TOKEN`
    /// environment variable.
    #[must_use]
    pub fn api_permission(mut self, yes: bool) -> Self {
        self.api_permission = yes;
        self
    }

    /// Perform validation checks and construct a [`Flight`]
    pub fn build(mut self) -> Result<Flight> {
        if self.name.is_none() {
            return Err(SeaplaneError::MissingFlightName);
        } else if self.image.is_none() {
            return Err(SeaplaneError::MissingFlightImageUrl);
        }

        if self.architecture.is_empty() {
            self.architecture.push(Architecture::AMD64);
        }

        Ok(Flight {
            name: self.name.unwrap(),
            image: self.image.unwrap(),
            minimum: self.minimum,
            maximum: self.maximum,
            architecture: self.architecture,
            api_permission: self.api_permission,
        })
    }
}

/// Describes a single [`Flight`] within a Formation.
///
/// Flights are logically a single container. However, Seaplane spins up many actual backing
/// *container instances* around the globe (with your Formation's `regions_allowed` map) and load
/// balances traffic between them.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Flight {
    name: String,
    image: Url,
    #[serde(default)]
    minimum: u64,
    #[serde(default)]
    maximum: Option<u64>,
    #[serde(default)]
    architecture: Vec<Architecture>,
    #[serde(default)]
    api_permission: bool,
}

impl Flight {
    /// Create a new [`FlightBuilder`] in order to construct a new [`Flight`]
    pub fn builder() -> FlightBuilder {
        FlightBuilder::new()
    }

    /// Creates a new [`Flight`] with the two required bits of information, a `name` which must be
    /// unique within the Formation, and a container image registry URL which points to the
    /// container image to use.
    pub fn new<S, U>(name: S, image: U) -> Flight
    where
        S: Into<String>,
        U: Into<Url>,
    {
        FlightBuilder::new()
            .name(name)
            .image(image)
            .build()
            .unwrap()
    }
}

/// The response from the `GET /formations` API call ([`FormationsRequest::list_names`])
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct FormationNames {
    inner: Vec<FormationName>,
}

/// A single Formation name in the response from the `GET /formations` API call
/// ([`FormationsRequest::list_names`])
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FormationName {
    name: String,
}

/// The response from the `GET /formations/<NAME>/activeConfiguration` API call
/// ([`FormationsRequest::get_active_configurations`])
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(transparent)]
pub struct ActiveConfigurations {
    inner: Vec<ActiveConfiguration>,
}

impl ActiveConfigurations {
    /// Creates a new [`ActiveConfigurations`] empty container
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an [`ActiveConfiguration`] via the Builder Pattern style
    #[must_use]
    pub fn add_configuration(mut self, config: ActiveConfiguration) -> Self {
        self.inner.push(config);
        self
    }

    /// Add an [`ActiveConfiguration`] without consuming `self` (as opposed to the Builder Pattern
    /// style in `ActiveConfigurations::add_configuration`)
    pub fn add_configuration_mut(&mut self, config: ActiveConfiguration) {
        self.inner.push(config);
    }

    /// Returns `true` if there are not [`ActiveConfiguration`]s
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

/// A single [`ActiveConfiguration`] from the response from the `GET
/// /formations/<NAME>/activeConfigurations` API call
/// ([`FormationsRequest::get_active_configurations`])
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveConfiguration {
    /// An ID of a configuration within the Formation
    configuration_id: Uuid,
    /// The proportional weight of traffic this configuration should get. For each endpoint we take
    /// the sum of the weights of every configuration with that endpoint exposed and divide traffic
    /// according to the percentage of that sum each configuration's weight has.
    #[serde(default)]
    traffic_weight: Option<u32>,
}

impl ActiveConfiguration {
    /// Create a new builder
    pub fn builder() -> ActiveConfigurationBuilder {
        ActiveConfigurationBuilder::default()
    }
}

// Impl manaully because we only need to check the UUID
impl PartialEq<Self> for ActiveConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.configuration_id == other.configuration_id
    }
}

/// A builder for creating a new [`ActiveConfiguration`]. This can be used to load ballance traffic
/// across different [`FormationConfiguration`]s within a Formation.
pub struct ActiveConfigurationBuilder {
    configuration_id: Option<Uuid>,
    traffic_weight: u32,
}

impl Default for ActiveConfigurationBuilder {
    fn default() -> Self {
        Self {
            configuration_id: None,
            traffic_weight: 1,
        }
    }
}

impl ActiveConfigurationBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// A unique ID of the configuration
    #[must_use]
    pub fn uuid<T: Into<Uuid>>(mut self, uuid: T) -> Self {
        self.configuration_id = Some(uuid.into());
        self
    }

    /// The proportional weight of traffic this configuration should get. For each endpoint we take
    /// the sum of the weights of every configuration with that endpoint exposed and divide traffic
    /// according to the percentage of that sum each configuration's weight has.
    ///
    /// Default: `1`
    #[must_use]
    pub fn traffic_weight(mut self, weight: u32) -> Self {
        self.traffic_weight = weight;
        self
    }

    /// Perform validation checks and build the [`ActiveConfiguration`]
    pub fn build(self) -> Result<ActiveConfiguration> {
        if self.configuration_id.is_none() {
            return Err(SeaplaneError::MissingUuid);
        }

        Ok(ActiveConfiguration {
            configuration_id: self.configuration_id.unwrap(),
            traffic_weight: Some(self.traffic_weight),
        })
    }
}
