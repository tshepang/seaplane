use seaplane::{
    api::compute::v1::{Architecture as ArchitectureModel, Flight as FlightModel},
    rexports::container_image_ref::ImageReference,
};

use crate::{
    cli::{
        cmds::flight::{
            common::Architecture, str_to_image_ref, SeaplaneFlightCommonArgMatches,
            FLIGHT_MINIMUM_DEFAULT,
        },
        validator::validate_flight_name,
    },
    context::Ctx,
    error::{CliErrorKind, Result},
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
    pub architecture: Vec<ArchitectureModel>,
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
            minimum: FLIGHT_MINIMUM_DEFAULT,
            maximum: None,
            architecture: Vec::new(),
            api_permission: false,
            reset_maximum: false,
            generated_name: true,
        }
    }
}

impl FlightCtx {
    /// Builds a FlightCtx from a string value using the inline flight spec syntax:
    ///
    /// name=FOO,image=nginx:latest,api-permission,architecture=amd64,minimum=1,maximum,2
    ///
    /// Where only image=... is required
    pub fn from_inline_flight(inline_flight: &str, registry: &str) -> Result<FlightCtx> {
        if inline_flight.contains(' ') {
            return Err(CliErrorKind::InlineFlightHasSpace.into_err());
        }

        let mut fctx = FlightCtx::default();

        let parts = inline_flight.split(',');

        macro_rules! parse_item {
            ($item:expr, $f:expr) => {{
                let mut item = $item.split('=');
                item.next();
                if let Some(value) = item.next() {
                    if value.is_empty() {
                        return Err(
                            CliErrorKind::InlineFlightMissingValue($item.to_string()).into_err()
                        );
                    }
                    $f(value)
                } else {
                    Err(CliErrorKind::InlineFlightMissingValue($item.to_string()).into_err())
                }
            }};
            ($item:expr) => {{
                parse_item!($item, |n| { Ok(n) })
            }};
        }

        for part in parts {
            match part.trim() {
                // @TODO technically nameFOOBAR=.. is valid... oh well
                name if part.starts_with("name") => {
                    fctx.name_id = parse_item!(name, |n: &str| {
                        if validate_flight_name(n).is_err() {
                            Err(CliErrorKind::InlineFlightInvalidName(n.to_string()).into_err())
                        } else {
                            Ok(n.to_string())
                        }
                    })?;
                    fctx.generated_name = false;
                }
                // @TODO technically imageFOOBAR=.. is valid... oh well
                img if part.starts_with("image") => {
                    fctx.image = Some(str_to_image_ref(registry, parse_item!(img)?)?);
                }
                // @TODO technically maxFOOBAR=.. is valid... oh well
                max if part.starts_with("max") => {
                    fctx.maximum = Some(parse_item!(max)?.parse()?);
                }
                // @TODO technically minFOOBAR=.. is valid... oh well
                min if part.starts_with("min") => {
                    fctx.minimum = parse_item!(min)?.parse()?;
                }
                // @TODO technically archFOOBAR=.. is valid... oh well
                arch if part.starts_with("arch") => {
                    fctx.architecture.push(parse_item!(arch)?.parse()?);
                }
                "api-permission" | "api-permissions" => {
                    fctx.api_permission = true;
                }
                // @TODO technically api-permissionFOOBAR=.. is valid... oh well
                perm if part.starts_with("api-permission") => {
                    let _ = parse_item!(perm, |perm: &str| {
                        fctx.api_permission = match perm {
                            t if t.eq_ignore_ascii_case("true") => true,
                            f if f.eq_ignore_ascii_case("false") => true,
                            _ => {
                                return Err(CliErrorKind::InlineFlightUnknownItem(
                                    perm.to_string(),
                                )
                                .into_err());
                            }
                        };
                        Ok(())
                    });
                }
                _ => {
                    return Err(CliErrorKind::InlineFlightUnknownItem(part.to_string()).into_err());
                }
            }
        }

        if fctx.image.is_none() {
            return Err(CliErrorKind::InlineFlightMissingImage.into_err());
        }

        Ok(fctx)
    }

    /// Builds a FlightCtx from ArgMatches using some `prefix` if any to search for args
    pub fn from_flight_common(
        matches: &SeaplaneFlightCommonArgMatches,
        ctx: &Ctx,
    ) -> Result<FlightCtx> {
        let matches = matches.0;
        let mut generated_name = false;
        // We generate a random name if one is not provided
        let name = matches
            .get_one::<String>("name")
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| {
                generated_name = true;
                generate_flight_name()
            });

        // We have to use if let in order to use the ? operator
        let image = if let Some(s) = matches.get_one::<String>("image") {
            Some(str_to_image_ref(&ctx.registry, s)?)
        } else {
            None
        };

        Ok(FlightCtx {
            image,
            name_id: name,
            minimum: *matches
                .get_one::<u64>("minimum")
                // clap validates valid u64 prior to this
                .unwrap_or(&FLIGHT_MINIMUM_DEFAULT),
            maximum: matches.get_one::<u64>("maximum").copied(),
            // clap validates valid u64 prior to this
            //.expect("failed to parse valid u64"),
            architecture: matches
                .get_many::<Architecture>("architecture")
                .unwrap_or_default()
                .map(|a| a.into())
                .collect(),
            // because of clap overrides we only have to check api_permissions
            api_permission: matches.get_flag("api-permission"),
            reset_maximum: matches.get_flag("no-maximum"),
            generated_name,
        })
    }

    /// Creates a new seaplane::api::compute::v1::Flight from the contained values
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::DEFAULT_IMAGE_REGISTRY_URL as IR;

    #[test]
    fn from_inline_flight_valid() {
        assert!(FlightCtx::from_inline_flight(
            "image=demos/nginx:latest,name=foo,maximum=2,minimum=2,api-permission,architecture=amd64", IR
        )
        .is_ok());
        assert!(FlightCtx::from_inline_flight(
            "image=demos/nginx:latest,name=foo,maximum=2,minimum=2,api-permission",
            IR
        )
        .is_ok());
        assert!(FlightCtx::from_inline_flight(
            "image=demos/nginx:latest,name=foo,maximum=2,minimum=2",
            IR
        )
        .is_ok());
        assert!(FlightCtx::from_inline_flight("image=demos/nginx:latest,name=foo", IR).is_ok());
        assert!(FlightCtx::from_inline_flight("image=demos/nginx:latest", IR).is_ok());
        assert!(FlightCtx::from_inline_flight(
            "image=demos/nginx:latest,name=foo,max=2,minimum=2,api-permission,architecture=amd64",
            IR
        )
        .is_ok());
        assert!(FlightCtx::from_inline_flight(
            "image=demos/nginx:latest,name=foo,maximum=2,min=2,api-permission",
            IR
        )
        .is_ok());
        assert!(
            FlightCtx::from_inline_flight("image=demos/nginx:latest,api-permissions", IR).is_ok()
        );
        assert!(FlightCtx::from_inline_flight("image=demos/nginx:latest,arch=amd64", IR).is_ok());
        assert!(FlightCtx::from_inline_flight("image=demos/nginx:latest,arch=arm64", IR).is_ok());
        assert!(FlightCtx::from_inline_flight("image=demos/nginx:latest,api-permission=true", IR)
            .is_ok(),);
        assert!(FlightCtx::from_inline_flight("image=demos/nginx:latest,api-permission=false", IR)
            .is_ok());
    }

    #[test]
    fn from_inline_flight_invalid() {
        assert_eq!(FlightCtx::from_inline_flight(
            "image= demos/nginx:latest,name=foo,maximum=2,minimum=2,api-permission,architecture=amd64", IR
        )
        .unwrap_err().kind(), &CliErrorKind::InlineFlightHasSpace);
        assert_eq!(
            FlightCtx::from_inline_flight(
                "image=demos/nginx:latest, name=foo,maximum=2,minimum=2,api-permission",
                IR
            )
            .unwrap_err()
            .kind(),
            &CliErrorKind::InlineFlightHasSpace
        );
        assert_eq!(
            FlightCtx::from_inline_flight("name=foo,maximum=2,minimum=2", IR)
                .unwrap_err()
                .kind(),
            &CliErrorKind::InlineFlightMissingImage
        );
        assert_eq!(
            FlightCtx::from_inline_flight(",image=demos/nginx:latest,name=foo", IR)
                .unwrap_err()
                .kind(),
            &CliErrorKind::InlineFlightUnknownItem("".into())
        );
        assert_eq!(
            FlightCtx::from_inline_flight("image=demos/nginx:latest,", IR)
                .unwrap_err()
                .kind(),
            &CliErrorKind::InlineFlightUnknownItem("".into())
        );
        assert_eq!(
            FlightCtx::from_inline_flight("image=demos/nginx:latest,foo", IR)
                .unwrap_err()
                .kind(),
            &CliErrorKind::InlineFlightUnknownItem("foo".into())
        );
        assert_eq!(
            FlightCtx::from_inline_flight("image=demos/nginx:latest,name=invalid_name", IR)
                .unwrap_err()
                .kind(),
            &CliErrorKind::InlineFlightInvalidName("invalid_name".into())
        );
        assert!(FlightCtx::from_inline_flight("image=demos/nginx:latest,max=2.3", IR)
            .unwrap_err()
            .kind()
            .is_parse_int(),);
        assert!(FlightCtx::from_inline_flight("image=demos/nginx:latest,max=foo", IR)
            .unwrap_err()
            .kind()
            .is_parse_int());
        assert!(FlightCtx::from_inline_flight("image=demos/nginx:latest,arch=foo", IR)
            .unwrap_err()
            .kind()
            .is_strum_parse(),);
        assert_eq!(
            FlightCtx::from_inline_flight("image=demos/nginx:latest,name", IR)
                .unwrap_err()
                .kind(),
            &CliErrorKind::InlineFlightMissingValue("name".into())
        );
        assert_eq!(
            FlightCtx::from_inline_flight("image=demos/nginx:latest,name=foo,arch", IR)
                .unwrap_err()
                .kind(),
            &CliErrorKind::InlineFlightMissingValue("arch".into())
        );
        assert_eq!(
            FlightCtx::from_inline_flight("image,name=foo", IR)
                .unwrap_err()
                .kind(),
            &CliErrorKind::InlineFlightMissingValue("image".into())
        );
        assert_eq!(
            FlightCtx::from_inline_flight("image=demos/nginx:latest,name=foo,min=", IR)
                .unwrap_err()
                .kind(),
            &CliErrorKind::InlineFlightMissingValue("min".into())
        );
        assert_eq!(
            FlightCtx::from_inline_flight("image=demos/nginx:latest,name=foo,max=", IR)
                .unwrap_err()
                .kind(),
            &CliErrorKind::InlineFlightMissingValue("max".into())
        );
    }
}
