use seaplane_oid::Oid;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[cfg(doc)]
use crate::api::compute::v2::FormationsRequest;
use crate::{
    error::{Result, SeaplaneError},
    rexports::container_image_ref::ImageReference,
};

/// Whether a Flight is Health or Unhealthy as determined by the runtime
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize)]
#[serde(rename_all = "lowercase")]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum FlightHealthStatus {
    Healthy,
    Unhealthy,
}

impl_deser_from_str!(FlightHealthStatus);

#[cfg(test)]
mod flight_health_status_tests {
    use super::*;

    #[test]
    fn deser() {
        assert_eq!(FlightHealthStatus::Healthy, "healthy".parse().unwrap());
        assert_eq!(FlightHealthStatus::Healthy, "Healthy".parse().unwrap());
        assert_eq!(FlightHealthStatus::Healthy, "HEALTHY".parse().unwrap());

        assert_eq!(FlightHealthStatus::Unhealthy, "unhealthy".parse().unwrap());
        assert_eq!(FlightHealthStatus::Unhealthy, "Unhealthy".parse().unwrap());
        assert_eq!(FlightHealthStatus::Unhealthy, "UNHEALTHY".parse().unwrap());
    }

    #[test]
    fn ser() {
        assert_eq!(FlightHealthStatus::Healthy.to_string(), "healthy".to_string());
        assert_eq!(FlightHealthStatus::Unhealthy.to_string(), "unhealthy".to_string());
    }
}

/// The status of a Flight
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub struct FlightStatus {
    /// The human friendly name of the Flight
    pub name: String,

    /// The Object ID of the Flight
    pub oid: Oid,

    /// The health status of the Flight
    pub health: FlightHealthStatus,
}

#[cfg(test)]
mod flight_status_tests {
    use super::*;

    #[test]
    fn deser() {
        let json = r#"{
            "name": "example-flight",
            "oid": "flt-agc6amh7z527vijkv2cutplwaa",
            "health": "healthy"
        }"#;
        let model = FlightStatus {
            name: "example-flight".into(),
            oid: "flt-agc6amh7z527vijkv2cutplwaa".parse().unwrap(),
            health: FlightHealthStatus::Healthy,
        };

        assert_eq!(model, serde_json::from_str(json).unwrap());
    }

    #[test]
    fn ser() {
        let json = r#"{"name":"example-flight","health":"healthy"}"#;
        let model = FlightStatus {
            name: "example-flight".into(),
            oid: "flt-agc6amh7z527vijkv2cutplwaa".parse().unwrap(),
            health: FlightHealthStatus::Healthy,
        };

        assert_eq!(json, serde_json::to_string(&model).unwrap());
    }
}

/// The status of a given Formation and it's associated Flights
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub struct FormationStatus {
    /// The human friendly name of the Formation
    pub name: String,

    /// The Object ID of the Formation
    pub oid: Oid,

    /// The status of each Flight that is part of this Formation
    pub flights: Vec<FlightStatus>,
}

#[cfg(test)]
mod formation_status_tests {
    use super::*;

    #[test]
    fn deser() {
        let json = r#"{
            "name": "example-formation",
            "oid": "frm-agc6amh7z527vijkv2cutplwaa",
            "flights": [{"name":"example-flight","oid":"flt-agc6amh7z527vijkv2cutplwaa","health":"healthy"}]
        }"#;
        let model = FormationStatus {
            name: "example-formation".into(),
            oid: "frm-agc6amh7z527vijkv2cutplwaa".parse().unwrap(),
            flights: vec![FlightStatus {
                name: "example-flight".into(),
                oid: "flt-agc6amh7z527vijkv2cutplwaa".parse().unwrap(),
                health: FlightHealthStatus::Healthy,
            }],
        };

        assert_eq!(model, serde_json::from_str(json).unwrap());
    }

    #[test]
    fn ser() {
        let json = r#"{"name":"example-formation","oid":"frm-agc6amh7z527vijkv2cutplwaa","flights":[{"name":"example-flight","oid":"flt-agc6amh7z527vijkv2cutplwaa","health":"healthy"}]}"#;
        let model = FormationStatus {
            name: "example-formation".into(),
            oid: "frm-agc6amh7z527vijkv2cutplwaa".parse().unwrap(),
            flights: vec![FlightStatus {
                name: "example-flight".into(),
                oid: "flt-agc6amh7z527vijkv2cutplwaa".parse().unwrap(),
                health: FlightHealthStatus::Healthy,
            }],
        };

        assert_eq!(json, serde_json::to_string(&model).unwrap());
    }
}

/// Response from `GET /formations/NAME` which contains metadata about the Formation itself.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub struct FormationMetadata {
    /// The URL where the Formation is exposed at
    pub url: String,

    /// The Object ID of the Formation
    pub oid: Oid,
}

/// A builder for creating a [`Formation`] which is the primary way to describe a
/// valid configuration for a Formation.
#[derive(Debug, Default)]
pub struct FormationBuilder {
    flights: Vec<Flight>,
    name: String,
    gateway_flight: Option<String>,
}

impl FormationBuilder {
    /// Add a [`Flight`] to the makeup of this Formation Configuration.
    ///
    /// **NOTE:** This method can be called multiple times. All values will be utilized.
    #[must_use]
    pub fn add_flight(mut self, flight: Flight) -> Self {
        self.flights.push(flight);
        self
    }

    pub fn gateway_flight(mut self, flight: impl Into<String>) -> Self {
        // @TODO validate flight name
        self.gateway_flight = Some(flight.into());
        self
    }

    /// Removes all [`Flight`]s from this Formation Configuration
    pub fn clear_flights(&mut self) { self.flights.clear(); }

    /// Performs validation checks, and builds the instance of [`Formation`]
    pub fn build(mut self) -> Result<Formation> {
        if self.flights.is_empty() {
            return Err(SeaplaneError::EmptyFlights);
        }

        if self.gateway_flight.is_none() {
            if self.flights.len() == 1 {
                self.gateway_flight = Some(self.flights.get(0).unwrap().name.clone());
            } else {
                return Err(SeaplaneError::NoGatewayFlight);
            }
        }

        // Ensure gateway_flight was defined
        if self
            .gateway_flight
            .as_ref()
            .map(|gw_f| self.flights.iter().any(|f| &f.name == gw_f))
            == Some(false)
        {
            return Err(SeaplaneError::InvalidGatewayFlight);
        }

        Ok(Formation {
            name: self.name,
            oid: None,
            flights: self.flights,
            gateway_flight: self.gateway_flight.unwrap(),
        })
    }
}

/// Represents a single Formation.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Formation {
    /// The human friendly name of the Formation
    name: String,

    /// The Object ID of the Formation that will be assigned by the Compute API upon launch
    #[serde(default, skip_serializing_if = "Option::is_none")]
    oid: Option<Oid>,

    /// The Flights that make up this Formation
    flights: Vec<Flight>,

    /// The Flight who will receive all the public HTTP(s) traffic that arrives on the public
    /// Formation URL
    gateway_flight: String,
}

impl Formation {
    /// Create a [`FormationBuilder`] to build a new configuration
    pub fn builder() -> FormationBuilder { FormationBuilder::default() }

    /// Add a [`Flight`] to the makeup of this Formation Configuration.
    pub fn add_flight(&mut self, flight: Flight) { self.flights.push(flight); }

    /// Remove a [`Flight`] from the makeup of this Formation Configuration.
    pub fn remove_flight(&mut self, name: &str) -> Option<Flight> {
        if let Some(i) =
            self.flights
                .iter()
                .enumerate()
                .find_map(|(i, f)| if f.name == name { Some(i) } else { None })
        {
            Some(self.flights.swap_remove(i))
        } else {
            None
        }
    }

    /// Set the [`Flight`]s that makeup this Formation Configuration.
    pub fn set_flights(&mut self, flights: Vec<Flight>) { self.flights = flights; }

    /// Set the [`Flight`]s that makeup this Formation Configuration.
    pub fn flights(&self) -> &[Flight] { &self.flights }
}

#[cfg(test)]
mod formation_tests {
    use super::*;

    #[test]
    fn deser() {
        let json = r#"{
            "name": "example-formation",
            "oid":"frm-agc6amh7z527vijkv2cutplwaa",
            "flights": [{
                "name":"example-flight",
                "oid":"flt-agc6amh7z527vijkv2cutplwaa",
                "image":"foo.com/bar:latest"
            }],
            "gateway-flight": "example-flight"
        }"#;
        let model = Formation {
            name: "example-formation".into(),
            oid: Some("frm-agc6amh7z527vijkv2cutplwaa".parse().unwrap()),
            flights: vec![Flight {
                name: "example-flight".into(),
                oid: Some("flt-agc6amh7z527vijkv2cutplwaa".parse().unwrap()),
                image: "foo.com/bar:latest".parse::<ImageReference>().unwrap(),
            }],
            gateway_flight: "example-flight".into(),
        };

        assert_eq!(model, serde_json::from_str(json).unwrap());
    }

    #[test]
    fn ser() {
        let json = r#"{"name":"example-formation","oid":"frm-agc6amh7z527vijkv2cutplwaa","flights":[{"name":"example-flight","oid":"flt-agc6amh7z527vijkv2cutplwaa","image":"foo.com/bar:latest"}],"gateway-flight":"example-flight"}"#;
        let model = Formation {
            name: "example-formation".into(),
            oid: Some("frm-agc6amh7z527vijkv2cutplwaa".parse().unwrap()),
            flights: vec![Flight {
                name: "example-flight".into(),
                oid: Some("flt-agc6amh7z527vijkv2cutplwaa".parse().unwrap()),
                image: "foo.com/bar:latest".parse::<ImageReference>().unwrap(),
            }],
            gateway_flight: "example-flight".into(),
        };

        assert_eq!(json.to_string(), serde_json::to_string(&model).unwrap());
    }

    #[test]
    fn ser_no_oid() {
        let json = r#"{"name":"example-formation","flights":[{"name":"example-flight","image":"foo.com/bar:latest"}],"gateway-flight":"example-flight"}"#;
        let model = Formation {
            name: "example-formation".into(),
            oid: None,
            flights: vec![Flight {
                name: "example-flight".into(),
                oid: None,
                image: "foo.com/bar:latest".parse::<ImageReference>().unwrap(),
            }],
            gateway_flight: "example-flight".into(),
        };

        assert_eq!(json.to_string(), serde_json::to_string(&model).unwrap());
    }
}

/// A builder to construct [`Flight`]s
#[derive(Debug, Default)]
pub struct FlightBuilder {
    name: Option<String>,
    image: Option<ImageReference>,
}

impl FlightBuilder {
    /// Create a new builder
    pub fn new() -> Self { Self::default() }

    /// The human readable [`Flight`] name, which must be unique within the Formation
    #[must_use]
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// A container image registry reference which points to the container image this [`Flight`]
    /// should uses
    ///
    /// # Panics
    ///
    /// This method `panic!`s if the `image_ref` provided cannot be parsed into a valid
    /// [`ImageReference`]
    #[must_use]
    pub fn image<R: AsRef<str>>(mut self, image_ref: R) -> Self {
        self.image = Some(
            image_ref
                .as_ref()
                .parse::<ImageReference>()
                .expect("Failed to parse image reference"),
        );
        self
    }

    /// A container image registry reference which points to the container image this [`Flight`]
    /// should uses.
    ///
    /// This method allows providing a pre-parsed [`ImageReference`] instead of a string which can
    /// `panic!` on parsing in [`FlightBuilder::image`].
    #[must_use]
    pub fn image_reference(mut self, image_ref: ImageReference) -> Self {
        self.image = Some(image_ref);
        self
    }

    /// Perform validation checks and construct a [`Flight`]
    pub fn build(self) -> Result<Flight> {
        if self.name.is_none() {
            return Err(SeaplaneError::MissingFlightName);
        } else if self.image.is_none() {
            return Err(SeaplaneError::MissingFlightImageReference);
        }

        Ok(Flight { name: self.name.unwrap(), oid: None, image: self.image.unwrap() })
    }
}

/// Describes a single [`Flight`] within a Formation.
///
/// Flights are logically a single container. However, Seaplane spins up many actual backing
/// *container instances* around the globe (with your Formation's `regions_allowed` map) and load
/// balances traffic between them.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub struct Flight {
    /// Returns the human readable name of the [`Flight`], which is unique with a Formation
    pub name: String,

    /// The container image reference
    pub image: ImageReference,

    /// The Object ID of the Flight
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oid: Option<Oid>,
}

impl Flight {
    /// Create a new [`FlightBuilder`] in order to construct a new [`Flight`]
    pub fn builder() -> FlightBuilder { FlightBuilder::new() }

    /// Creates a new [`Flight`] with the two required bits of information, a `name` which must be
    /// unique within the Formation, and a container image registry URL which points to the
    /// container image to use.
    ///
    /// # Panics
    ///
    /// This method `panic!`s if the `image_ref` provided cannot be parsed into a valid
    /// [`ImageReference`]
    pub fn new<S, R>(name: S, image_ref: R) -> Flight
    where
        S: Into<String>,
        R: AsRef<str>,
    {
        FlightBuilder::new()
            .name(name)
            .image(image_ref)
            .build()
            .unwrap()
    }

    /// Returns the human readable [`Flight`] name, which is unique within the Formation
    #[inline]
    pub fn name(&self) -> &str { &self.name }

    /// Returns the container image reference this [`Flight`] uses, as a [`String`]
    #[inline]
    pub fn image_str(&self) -> String { self.image.to_string() }

    /// Returns the container image reference this [`Flight`] uses, as an [`ImageReference`]
    #[inline]
    pub fn image(&self) -> &ImageReference { &self.image }
}

#[cfg(test)]
mod flight_tests {
    use super::*;

    #[test]
    fn deser() {
        let json = r#"{
            "name":"example-flight",
            "oid":"flt-agc6amh7z527vijkv2cutplwaa",
            "image":"foo.com/bar:latest"
        }"#;
        let model = Flight {
            name: "example-flight".into(),
            oid: Some("flt-agc6amh7z527vijkv2cutplwaa".parse().unwrap()),
            image: "foo.com/bar:latest".parse::<ImageReference>().unwrap(),
        };

        assert_eq!(model, serde_json::from_str(json).unwrap());
    }

    #[test]
    fn ser() {
        let json = r#"{"name":"example-flight","oid":"flt-agc6amh7z527vijkv2cutplwaa","image":"foo.com/bar:latest"}"#;
        let model = Flight {
            name: "example-flight".into(),
            oid: Some("flt-agc6amh7z527vijkv2cutplwaa".parse().unwrap()),
            image: "foo.com/bar:latest".parse::<ImageReference>().unwrap(),
        };

        assert_eq!(json, serde_json::to_string(&model).unwrap());
    }

    #[test]
    fn ser_no_oid() {
        let json = r#"{"name":"example-flight","image":"foo.com/bar:latest"}"#;
        let model = Flight {
            name: "example-flight".into(),
            oid: None,
            image: "foo.com/bar:latest".parse::<ImageReference>().unwrap(),
        };

        assert_eq!(json, serde_json::to_string(&model).unwrap());
    }
}
