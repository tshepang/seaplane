use seaplane::api::v1::formations::{Architecture, Flight as FlightModel, ImageReference};

use crate::ops::generate_name;

/// Represents the "Source of Truth" i.e. it combines all the CLI options, ENV vars, and config
/// values into a single structure that can be used later to build models for the API or local
/// structs for serializing
pub struct FlightCtx {
    pub image: Option<ImageReference>,
    pub name: String,
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
            name: generate_name(),
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
    /// Creates a new seaplane::api::v1::Flight from the contained values
    pub fn model(&self) -> FlightModel {
        // Create the new Flight model from the CLI inputs
        let mut flight_model = FlightModel::builder()
            .name(self.name.clone())
            .api_permission(self.api_permission)
            .minimum(self.minimum);

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
