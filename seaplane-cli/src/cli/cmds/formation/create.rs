use clap::Parser;
use seaplane::{api::v1::Architecture, rexports::strum::VariantNames};

use crate::{
    cli::{
        cmds::{
            flight::{SeaplaneFlightCommonArgs, SeaplaneFlightCreateArgs},
            formation::{build_request, SeaplaneFormationCommonArgs},
        },
        validator::validate_name,
    },
    error::{CliErrorKind, Context, Result},
    fs::{FromDisk, ToDisk},
    ops::formation::{Formation, FormationConfiguration, Formations},
    printer::Color,
    Ctx,
};

/// Create a Seaplane Formation
#[derive(Parser)]
#[clap(visible_aliases = &["add"], override_usage =
    "seaplane formation create [OPTIONS]
    seaplane formation create --flight=SPEC... [FORMATION CFG OPTIONS]
    seaplane formation create --flight-image=SPEC [INLINE FLIGHT OPTIONS] [FORMATION CFG OPTIONS]",
    long_about = "Create a Seaplane Formation

When using the inline-flight-options (--flight-*) all options apply only to a single flight. Other
Flights may be specified using the `--flight` flag, but those are totally independent of the
`--flight-*` specified Flight.")]
pub struct SeaplaneFormationCreateArgs {
    // So we don't have to define the same args over and over with commands that use the same ones
    #[clap(flatten)]
    shared: SeaplaneFormationCommonArgs,

    /// Send this formation to Seaplane immediately (requires a Formation configuration) (implies
    /// --launch, if that is not the desired state use --no-launch)
    #[clap(long, overrides_with = "no-deploy")]
    deploy: bool,

    /// Do *not* send this formation to Seaplane immediately
    #[clap(long, overrides_with = "no-deploy")]
    no_deploy: bool,

    /// Override any existing Formation with the same NAME
    #[clap(long)]
    force: bool,

    // TODO: allow omitting of USER (TENANT) portion of image spec too...but this requires a an API
    // call to determine the TENANT id (at least until the `seaplane account login` command is done)
    /// The container image registry reference that this Flight will use (See IMAGE SPEC below)
    #[clap(
        long,
        help_heading = "INLINE FLIGHT OPTIONS",
        visible_alias = "img",
        value_name = "SPEC",
        long_help = "The container image registry reference that this Flight will use (See IMAGE SPEC below)

All image references using the 'registry.seaplanet.io' registry may omit the domain portions of the
image reference as it is implied. For example, 'registry.seaplanet.io/USER/myimage:latest' can be
supplied simply as 'USER/myimage:latest'

NOTE at this time the only registry supported is registry.seaplanet.io. In the future when other
registries are supported, you must specify the full registry domain and path if using those
alternate registries in order to properly reference your image."
    )]
    pub flight_image: Option<String>, // we use a string because we allow elision of the domain

    /// A human readable name for the Flight (must be unique within any Formation it is a part of)
    /// if omitted a pseudo random name will be assigned
    #[clap(
        long,
        help_heading = "INLINE FLIGHT OPTIONS",
        requires = "flight-image",
        validator = validate_name,
        long_help = "A human readable name for the Flight (must be unique within any Formation it

Rules for a valid name are as follows:

  - may only include 0-9, a-z, A-Z, and '-' (hyphen)
  - hyphens ('-') may not be repeated (i.e. '--')
  - no more than three (3) total hyphens
  - the total length must be <= 27

Some of these restrictions may be lifted in the future."
    )]
    pub flight_name: Option<String>,

    /// The minimum number of container instances that should ever be running
    #[clap(
        long,
        requires = "flight-image",
        help_heading = "INLINE FLIGHT OPTIONS",
        default_value = "1",
        visible_alias = "flight-min"
    )]
    pub flight_minimum: u64,

    /// The maximum number of container instances that should ever be running (default: infinite)
    #[clap(
        long,
        requires = "flight-image",
        help_heading = "INLINE FLIGHT OPTIONS",
        visible_alias = "flight-max"
    )]
    pub flight_maximum: Option<u64>,

    /// The architectures this flight is capable of running on. No value means it will be auto
    /// detected from the image definition.
    #[clap(long,
        requires = "flight-image",
        help_heading = "INLINE FLIGHT OPTIONS",
        visible_aliases = &["flight-arch", "flight-arches"], possible_values = Architecture::VARIANTS, value_delimiter = ',')]
    pub flight_architecture: Vec<Architecture>,

    /// This Flight should be allowed to hit Seaplane API endpoints and will be provided a
    /// 'SEAPLANE_API_TOKEN' environment variable at runtime
    #[clap(
        long,
        requires = "flight-image",
        help_heading = "INLINE FLIGHT OPTIONS",
        alias = "flight-api-permissions"
    )]
    pub flight_api_permission: bool,
}

impl SeaplaneFormationCreateArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        self.update_ctx(ctx)?;

        let formation_ctx = ctx.formation_ctx();

        // Load the known formations from the local JSON "DB"
        let formations_file = ctx.formations_file();
        let mut formations: Formations = FromDisk::load(&formations_file)?;

        // Check for duplicates and suggest `seaplane formation edit`
        let name = &formation_ctx.name;
        if formations.contains_name(name) {
            // TODO: We should check if these ones we remove are referenced remote or not

            if !ctx.force {
                return Err(CliErrorKind::DuplicateName(name.to_owned())
                    .into_err()
                    .context("(hint: try '")
                    .color_context(Color::Green, format!("seaplane formation edit {}", &name))
                    .context("' instead)\n"));
            }

            // We have duplicates, but the user passed --force. So first we remove the existing
            // formations and "re-add" them

            // TODO: if more than one formation has the exact same name, we remove them all; that's
            // *probably* what we want? But more thought should go into this...
            formations.remove_name(name);
        }

        // Add the new formation
        let mut new_formation = Formation::new(&formation_ctx.name);
        let mut cfg_id = None;

        if let Some(cfg) = formation_ctx.configuration_model(ctx)? {
            let formation_cfg = FormationConfiguration::new(cfg);
            // TODO: if active / deployed add to appropriate in_air / grounded
            new_formation.local.insert(formation_cfg.id);
            cfg_id = Some(formation_cfg.id);
            formations.configurations.push(formation_cfg)
        }

        let id = new_formation.id.to_string();
        formations.formations.push(new_formation);

        // Write out an entirely new JSON file with the new Formation included
        formations
            .persist()
            .with_context(|| format!("Path: {:?}\n", ctx.formations_file()))?;

        cli_print!("Successfully created Formation '");
        cli_print!(@Green, "{}", &formation_ctx.name);
        cli_print!("' with ID '");
        cli_print!(@Green, "{}", &id[..8]);
        cli_println!("'");

        if let Some(cfg_id) = cfg_id {
            if formation_ctx.deploy {
                let create_req = build_request(Some(&formation_ctx.name), ctx)?;
                let cfg_uuids = create_req.create(
                    formation_ctx.configuration_model(ctx)?.unwrap(),
                    formation_ctx.launch,
                )?;
                if formation_ctx.launch {
                    cli_print!("Successfully launched Formation '");
                    cli_print!(@Green, "{}", &cfg_id.to_string()[..8]);
                    cli_println!("with Configuration UUIDs:");
                    formations.add_in_air_by_name(&formation_ctx.name, cfg_id);
                } else {
                    formations.add_grounded_by_name(&formation_ctx.name, cfg_id);
                }
                for uuid in cfg_uuids.into_iter() {
                    cli_println!(@Green, "{}", uuid);
                    formations.add_uuid(&cfg_id, uuid);
                }
            }
        }

        Ok(())
    }

    fn update_ctx(&self, ctx: &mut Ctx) -> Result<()> {
        // We only need to check for `--flight-image` because clap ensures it was called if
        // required
        let mut shared = self.shared.clone();
        if let Some(img) = &self.flight_image {
            // Create a Flight args struct so we can create the new flight
            let flight_create_args = SeaplaneFlightCreateArgs {
                shared: SeaplaneFlightCommonArgs {
                    image: Some(img.to_string()), // we use a string because we allow elision of the domain
                    name: self.flight_name.clone(),
                    minimum: self.flight_minimum,
                    maximum: self.flight_maximum,
                    architecture: self.flight_architecture.clone(),
                    api_permission: self.flight_api_permission,
                    no_api_permission: false,
                    no_maximum: false,
                },
                force: ctx.force,
            };
            // Add the newly created inline Flight ID as if the user used --flight=ID
            shared.flight.push(flight_create_args.run(ctx)?.to_string());
        }
        ctx.force = self.force;
        ctx.formation.init(shared.formation_ctx(ctx)?);
        let mut fctx = ctx.formation_ctx();
        fctx.deploy = self.deploy || shared.launch;
        if self.deploy && !self.shared.no_launch {
            fctx.launch = true;
        }

        Ok(())
    }
}
