use seaplane::api::v1::formations::{Architecture, Flight as FlightModel, ImageReference};

use crate::{
    cli::cmds::flight::{str_to_image_ref, SeaplaneFlightCommonArgMatches, FLIGHT_MINIMUM_DEFAULT},
    error::Result,
    ops::generate_flight_name,
};

/// Represents the "Source of Truth" i.e. it combines all the CLI options, ENV vars, and config
/// values into a single structure that can be used later to build models for the API or local
/// structs for serializing
// TODO: we may not want to derive this we implement circular references
#[derive(Debug, Clone)]
pub struct FlightCtx {
    pub image: Option<ImageReference>,
    pub name_id: String,
    pub minimum: u64,
    pub maximum: Option<u64>,
    pub architecture: Vec<Architecture>,
    pub api_permission: bool,
    pub reset_maximum: bool,
    // True if we randomly generated the name. False if the user provided it
    pub generated_name: bool,
}

impl Default for FlightCtx {
    fn default() -> Self {
        Self {
            name_id: generate_flight_name(),
            image: None,
            minimum: 0,
            maximum: None,
            architecture: Vec::new(),
            api_permission: false,
            reset_maximum: false,
            generated_name: true,
        }
    }
}

impl FlightCtx {
    /// Builds a FlightCtx from ArgMatches using some `prefix` if any to search for args
    pub fn from_flight_common(
        matches: &SeaplaneFlightCommonArgMatches,
        prefix: &str,
    ) -> Result<FlightCtx> {
        let matches = matches.0;
        let mut generated_name = false;
        // We generate a random name if one is not provided
        let name = matches
            .value_of(&format!("{prefix}name"))
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| {
                generated_name = true;
                generate_flight_name()
            });

        // We have to use if let in order to use the ? operator
        let image = if let Some(s) = matches.value_of(&format!("{prefix}image")) {
            Some(str_to_image_ref(s)?)
        } else {
            None
        };

        Ok(FlightCtx {
            image,
            name_id: name,
            minimum: matches
                .value_of_t(&format!("{prefix}minimum"))
                .unwrap_or(FLIGHT_MINIMUM_DEFAULT),
            maximum: matches.value_of_t(&format!("{prefix}maximum")).ok(),
            architecture: values_t_or_exit!(
                matches,
                &format!("{prefix}architecture"),
                Architecture
            ),
            // because of clap overrides we only have to check api_permissions
            api_permission: matches.is_present(&format!("{prefix}api-permission")),
            reset_maximum: matches.is_present(&format!("{prefix}no-maximum")),
            generated_name,
        })
    }

    /// Creates a new seaplane::api::v1::Flight from the contained values
    pub fn model(&self) -> FlightModel {
        // Create the new Flight model from the CLI inputs
        let mut flight_model = FlightModel::builder()
            .name(self.name_id.clone())
            .minimum(self.minimum);

        #[cfg(feature = "unstable")]
        {
            flight_model = flight_model.api_permission(self.api_permission);
        }

        if let Some(image) = self.image.clone() {
            flight_model = flight_model.image_reference(image);
        }

        // We have to conditionally set the `maximum` because the builder takes a `u64` but we have
        // an `Option<u64>` so can't just blindly overwrite it like we do with `minimum` above.
        if let Some(n) = self.maximum {
            flight_model = flight_model.maximum(n);
        }

        // Add all the architectures. In the CLI they're a Vec but in the Model they're a HashSet
        // which is the reason for the slightly awkward loop
        for arch in &self.architecture {
            flight_model = flight_model.add_architecture(*arch);
        }

        // Create a new Flight struct we can add to our local JSON "DB"
        flight_model
            .build()
            .expect("Failed to build Flight from inputs")
    }
}
