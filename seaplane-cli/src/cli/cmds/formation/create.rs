use clap::{ArgMatches, Command};
use const_format::concatcp;
use seaplane::{api::v1::Architecture, rexports::strum::VariantNames};

use crate::{
    cli::{
        cmds::{
            flight::{SeaplaneFlightCommonArgMatches, SeaplaneFlightCreate},
            formation::{build_request, common, SeaplaneFormationFetch},
        },
        request_token_json,
        specs::{FLIGHT_SPEC, REGION_SPEC},
        validator::validate_formation_name,
        CliCommand,
    },
    context::{Ctx, FlightCtx, FormationCtx},
    error::{CliErrorKind, Context, Result},
    ops::formation::{Formation, FormationConfiguration},
    printer::Color,
};

static LONG_ABOUT: &str = "Create a Seaplane Formation

When using the inline-flight-options (--flight-*) all options apply only to a single flight. Other
Flights may be specified using the `--flight` flag, but those are totally independent of the
`--flight-*` specified Flight.";
static LONG_FLIGHT_IMAGE: &str =
    "The container image registry reference that this Flight will use (See IMAGE SPEC below)

All image references using the 'registry.seaplanet.io' registry may omit the domain portions of the
image reference as it is implied. For example, 'registry.seaplanet.io/USER/myimage:latest' can be
supplied simply as 'USER/myimage:latest'

NOTE at this time the only registry supported is registry.seaplanet.io. In the future when other
registries are supported, you must specify the full registry domain and path if using those
alternate registries in order to properly reference your image.";
static LONG_FLIGHT_NAME: &str =
    "A human readable name for the Flight (must be unique within any Formation it

Rules for a valid name are as follows:

  - may only include 0-9, a-z, A-Z, and '-' (hyphen)
  - hyphens ('-') may not be repeated (i.e. '--')
  - no more than three (3) total hyphens
  - the total length must be <= 27

Some of these restrictions may be lifted in the future.";

/// A newtype wrapper to enforce where the ArgMatches came from which reduces errors in checking if
/// values of arguments were used or not. i.e. `seaplane formation create` may not have the same
/// arguments as `seaplane account token` even though both produce an `ArgMatches`.
#[allow(missing_debug_implementations)]
pub struct SeaplaneFormationCreateArgMatches<'a>(pub &'a ArgMatches);

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormationCreate;

impl SeaplaneFormationCreate {
    pub fn command() -> Command<'static> {
        Command::new("create")
            .after_help(concatcp!(FLIGHT_SPEC, "\n\n", REGION_SPEC))
            .visible_alias("add")
            .override_usage("seaplane formation create [OPTIONS]
    seaplane formation create --flight=SPEC... [FORMATION CFG OPTIONS]
    seaplane formation create --flight-image=SPEC [INLINE FLIGHT OPTIONS] [FORMATION CFG OPTIONS]")
            .about("Create a Seaplane Formation")
            .long_about(LONG_ABOUT)
            .args(common::args())
            .arg(arg!(--force).help("Override any existing Formation with the same NAME"))
            .arg(arg!(--fetch - ('F')).help("Fetch remote definitions prior to creating to check for conflicts (by default only local state is considered)"))
            .next_help_heading("INLINE FLIGHT OPTIONS")
            // TODO: allow omitting of USER (TENANT) portion of image spec too...but this requires a an API
            // call to determine the TENANT id (at least until the `seaplane account login` command is done)
            .arg(arg!(--("flight-image")|img =["SPEC"])
                .help("The container image registry reference that this Flight will use (See IMAGE SPEC below)")
                .long_help(LONG_FLIGHT_IMAGE))
            .arg(arg!(--("flight-name") =["STRING"])
                .validator(validate_formation_name)
                .requires("flight-image")
                .help("A human readable name for the Flight (must be unique within any Formation it is a part of) if omitted a pseudo random name will be assigned")
                .long_help(LONG_FLIGHT_NAME))
            .arg(arg!(--("flight-minimum")|("flight-min") =["NUM"=>"1"])
                .requires("flight-image")
                .help("The minimum number of container instances that should ever be running"))
            .arg(arg!(--("flight-maximum")|("flight-max") =["NUM"])
                .overrides_with("flight-no-maximum")
                .requires("flight-image")
                .help("The maximum number of container instances that should ever be running (default: infinite)"))
            .arg(arg!(--("flight-no-maximum")|("flight-no-max"))
                .requires("flight-image")
                .overrides_with("flight-maximum")
                .help("The maximum number of container instances that should ever be running (default: infinite)"))
            .arg(arg!(--("flight-architecture")|("flight-arch")|("flight-arches")|("flight-architectures") =["ARCH"]... ignore_case)
                .requires("flight-image")
                .possible_values(Architecture::VARIANTS)
                .help("The architectures this flight is capable of running on. No value means it will be auto detected from the image definition"))
            .arg(arg!(--("flight-api-permission")|("flight-api-permissions"))
                .requires("flight-image")
                .help("This Flight should be allowed to hit Seaplane API endpoints and will be provided a 'SEAPLANE_API_TOKEN' environment variable at runtime"))
    }
}

impl CliCommand for SeaplaneFormationCreate {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        if ctx.args.fetch {
            let fetch = SeaplaneFormationFetch;
            fetch.run(ctx)?;
        }

        let formation_ctx = ctx.formation_ctx.get_or_init();

        // Check for duplicates and suggest `seaplane formation edit`
        let name = &formation_ctx.name_id;
        if ctx.db.formations.contains_name(name) {
            if !ctx.args.force {
                return Err(CliErrorKind::DuplicateName(name.to_owned())
                    .into_err()
                    .context("(hint: try '")
                    .color_context(Color::Green, format!("seaplane formation edit {}", &name))
                    .context("' instead)\n"));
            }

            // We have duplicates, but the user passed --force. So first we remove the existing
            // formations and "re-add" them

            // TODO: We should check if these ones we remove are referenced remote or not
            // TODO: if more than one formation has the exact same name, we remove them all; that's
            // *probably* what we want? But more thought should go into this...
            ctx.db.formations.remove_name(name);
        }

        // Add the new formation
        let mut new_formation = Formation::new(&formation_ctx.name_id);
        let mut cfg_id = None;

        if let Some(cfg) = formation_ctx.configuration_model(ctx)? {
            let formation_cfg = FormationConfiguration::new(cfg);
            // TODO: if active / deployed add to appropriate in_air / grounded
            new_formation.local.insert(formation_cfg.id);
            cfg_id = Some(formation_cfg.id);
            ctx.db.formations.configurations.push(formation_cfg)
        }

        let id = new_formation.id.to_string();
        ctx.db.formations.formations.push(new_formation);

        ctx.persist_formations()?;

        cli_print!("Successfully created Formation '");
        cli_print!(@Green, "{}", &formation_ctx.name_id);
        cli_print!("' with ID '");
        cli_print!(@Green, "{}", &id[..8]);
        cli_println!("'");

        let api_key = ctx.args.api_key()?;
        if let Some(cfg_id) = cfg_id {
            if formation_ctx.deploy {
                let create_req = build_request(Some(&formation_ctx.name_id), api_key)?;
                let cfg_uuids = create_req.create(
                    formation_ctx.configuration_model(ctx)?.unwrap(),
                    formation_ctx.launch,
                )?;
                if formation_ctx.launch {
                    cli_print!("Successfully launched Formation '");
                    cli_print!(@Green, "{}", &cfg_id.to_string()[..8]);
                    cli_println!("' with Configuration UUIDs:");
                    ctx.db
                        .formations
                        .add_in_air_by_name(&formation_ctx.name_id, cfg_id);
                } else {
                    ctx.db
                        .formations
                        .add_grounded_by_name(&formation_ctx.name_id, cfg_id);
                }
                for uuid in cfg_uuids.into_iter() {
                    cli_println!(@Green, "\t{uuid}");
                    ctx.db.formations.add_uuid(&cfg_id, uuid);
                }
                if formation_ctx.launch {
                    let subdomain = request_token_json(api_key, "")?.subdomain;
                    cli_print!("The Formation URL is ");
                    cli_println!(@Green, "https://{}--{subdomain}.on.seaplanet.io/", &formation_ctx.name_id);
                    cli_println!("(hint: if you have not configured any public endpoints, the Formation will not be reachable from the public internet!)");
                }
            }
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.args.fetch = matches.is_present("fetch");
        ctx.formation_ctx.init(FormationCtx::from_formation_create(
            &SeaplaneFormationCreateArgMatches(matches),
            ctx,
        )?);

        if matches.is_present("flight-image") {
            ctx.flight_ctx.init(FlightCtx::from_flight_common(
                &SeaplaneFlightCommonArgMatches(matches),
                "flight-",
            )?);

            let flight_create: Box<dyn CliCommand> = Box::new(SeaplaneFlightCreate);
            flight_create.run(ctx)?;

            // Store the newly created Flight as if it was passed via `--flight FOO`
            ctx.formation_ctx
                .get_or_init()
                .cfg_ctx
                .flights
                .push(ctx.flight_ctx.get_or_init().name_id.clone());
        }

        // Create any flights required
        let at_flights: Vec<_> = matches
            .values_of("flight")
            .unwrap_or_default()
            .filter(|s| s.starts_with('@'))
            .collect();
        for name in ctx.db.flights.add_from_at_strs(&at_flights)? {
            ctx.formation_ctx.get_or_init().cfg_ctx.flights.push(name);
        }

        ctx.persist_flights()?;

        Ok(())
    }
}
