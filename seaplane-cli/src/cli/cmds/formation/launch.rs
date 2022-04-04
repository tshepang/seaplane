use clap::{ArgMatches, Command};
use seaplane::{
    api::v1::{ActiveConfiguration, ActiveConfigurations, FormationsErrorKind},
    error::SeaplaneError,
};

use crate::{
    cli::{
        cmds::formation::{build_request, SeaplaneFormationFetch},
        errors, request_token_json,
        validator::{validate_formation_name, validate_name_id},
        CliCommand,
    },
    context::Ctx,
    error::{CliError, Context, Result},
};

static LONG_ABOUT: &str =
    "Start all configurations of a Formation and evenly distribute traffic between them

    In many cases, or at least initially a Formation may only have a single Formation
    Configuration. In these cases this command will set that one configuration to active.

    Things become slightly more complex when there are multiple Formation Configurations. Let's
    look at each possibility in turn.

    \"Local Only\" Configs Exist:

    A \"Local Only\" Config is a configuration that exists in the local database, but has not (yet)
    been uploaded to the Seaplane Cloud.

    In these cases the configurations will be sent to the Seaplane Cloud, and set to active. If the
    Seaplane Cloud already has configurations for the given Formation (either active or inactive),
    these new configurations will be appended, and traffic will be balanced between any *all*
    configurations.

    \"Remote Active\" Configs Exist:

    A \"Remote Active\" Config is a configuration that the Seaplane Cloud is aware of, and actively
    sending traffic to.

    These configurations will remain active and traffic will be balanced between any *all*
    configurations.

    \"Remote Inactive\" Configs Exist:

    A \"Remote Inactive\" Config is a configuration that the Seaplane Cloud is aware of, and but not
    sending traffic to.

    These configurations will be made active. If the Seaplane Cloud already has active
    configurations for the given Formation, these newly activated configurations will be appended,
    and traffic will be balanced between any *all* configurations. ";

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormationLaunch;

impl SeaplaneFormationLaunch {
    pub fn command() -> Command<'static> {
        let validator = |s: &str| validate_name_id(validate_formation_name, s);

        // TODO: make it possible to selectively start only *some* configs
        Command::new("launch")
            .visible_alias("start")
            .about("Start all configurations of a Formation and evenly distribute traffic between them")
            .long_about(LONG_ABOUT)
            .arg(arg!(formation =["NAME|ID"] required)
                .validator(validator)
                .help("The name or ID of the Formation to launch"))
            .arg(arg!(--all -('a')).conflicts_with("exact").help("Stop all matching Formations even when FORMATION is ambiguous"))
            .arg(arg!(--exact -('x')).conflicts_with("all").help("The given FORMATION must be an exact match"))
            .arg(arg!(--fetch -('F')).help("Fetch remote Formation definitions prior to attempting to launch this Formation"))
            .arg(arg!(--grounded).help("Upload the configuration(s) to Seaplane but *DO NOT* set them to active"))
    }
}

impl CliCommand for SeaplaneFormationLaunch {
    // TODO: maybe validate flight names in endpoints? Argument against is it could fail validation
    // if you have not fetched first.
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        if ctx.args.fetch {
            let fetch = SeaplaneFormationFetch;
            fetch.run(ctx)?;
        }
        // Get the indices of any formations that match the given name/ID
        let indices = if ctx.args.exact {
            ctx.db
                .formations
                .formation_indices_of_matches(ctx.args.name_id.as_ref().unwrap())
        } else {
            ctx.db
                .formations
                .formation_indices_of_left_matches(ctx.args.name_id.as_ref().unwrap())
        };

        match indices.len() {
            0 => errors::no_matching_item(ctx.args.name_id.clone().unwrap(), ctx.args.exact)?,
            1 => (),
            _ => {
                // TODO: and --force
                if !ctx.args.all {
                    errors::ambiguous_item(ctx.args.name_id.clone().unwrap(), true)?;
                }
            }
        }

        let api_key = ctx.args.api_key()?;
        let grounded = ctx.formation_ctx.get_or_init().grounded;
        for idx in indices {
            // re unwrap: the indices returned came from Formations so they have to be valid
            let formation = ctx.db.formations.get_formation(idx).unwrap();
            let formation_name = formation.name.as_ref().unwrap().clone();

            // Get the local configs that don't exist remote yet
            let cfgs_ids = formation.local_only_configs();

            // Keep track if we had to make a brand new formation or not
            let mut created_new = false;
            let mut cfg_uuids = Vec::new();

            // Add those configurations to this formation
            'inner: for id in &cfgs_ids {
                if let Some(cfg) = ctx.db.formations.get_configuration(*id) {
                    let add_cfg_req = build_request(Some(&formation_name), api_key)?;
                    // We don't set the configuration to active because we'll be doing that to
                    // *all* formation configs in a minute
                    cli_debug!("Looking for existing Formations...");
                    if let Err(e) = add_cfg_req.add_configuration(cfg.model.clone(), false) {
                        cli_debugln!(@Green, "None");
                        match e {
                            SeaplaneError::FormationsResponse(fr)
                                if fr.kind == FormationsErrorKind::FormationNotFound =>
                            {
                                // If the formation didn't exist, create it
                                let create_req = build_request(Some(&formation_name), api_key)?;
                                cli_debug!("Creating new Formation...");
                                match create_req.create(cfg.model.clone(), !grounded) {
                                    Err(e) => {
                                        cli_debugln!(@Red, "Failed");
                                        return Err(e.into());
                                    }
                                    Ok(cfg_uuid) => {
                                        cli_debugln!(@Green, "Success");
                                        cfg_uuids.extend(cfg_uuid);
                                        ctx.db.formations.add_in_air_by_name(&formation_name, *id);
                                        created_new = true;
                                        break 'inner;
                                    }
                                }
                            }
                            _ => return Err(e.into()),
                        }
                    } else {
                        cli_debugln!(@Green, "Found");
                        ctx.db.formations.add_grounded_by_name(&formation_name, *id);
                    }
                } else {
                    // TODO: Inform the user of possible error? Somehow there is no config by the
                    // ID? This is an internal error that shoulnd't happen. We got the ID from our
                    // own internal state, if that's wrong we have big issues
                    unreachable!()
                }
            }

            // If the user passed `--grounded` they don't want the configuration to be set to
            // active
            if !grounded && !created_new {
                // Get all configurations for this Formation
                let list_cfg_uuids_req = build_request(Some(&formation_name), api_key)?;
                cfg_uuids.extend(
                    list_cfg_uuids_req
                        .list_configuration_ids()
                        .map_err(CliError::from)
                        .context("Context: failed to retrieve Formation Configuration IDs\n")?,
                );
                let mut active_configs = ActiveConfigurations::new();
                for uuid in &cfg_uuids {
                    active_configs.add_configuration_mut(
                        ActiveConfiguration::builder()
                            .uuid(*uuid)
                            .traffic_weight(1)
                            .build()?,
                    );
                }
                let set_cfgs_req = build_request(Some(&formation_name), api_key)?;
                set_cfgs_req
                    .set_active_configurations(active_configs, false)
                    .map_err(CliError::from)
                    .context("Context: failed to start Formation\n")?;
                for id in cfgs_ids {
                    ctx.db.formations.add_in_air_by_name(&formation_name, id);
                }
            }

            cli_print!("Successfully Launched Formation '");
            cli_print!(@Green, "{}", &ctx.args.name_id.as_ref().unwrap());
            if cfg_uuids.is_empty() {
                cli_println!("'");
            } else {
                cli_println!("' with Configuration UUIDs:");
                for uuid in cfg_uuids {
                    cli_println!(@Green, "{uuid}");
                }
            }
        }
        let subdomain = request_token_json(api_key, "")?.subdomain;
        cli_print!("The Formation URL is ");
        cli_println!(@Green, "https://{}--{subdomain}.on.seaplanet.io/", &ctx.args.name_id.as_ref().unwrap());
        // TODO: only show this message when no public endpoints are configured
        cli_println!("(hint: if you have not configured any public endpoints, the Formation will not be reachable from the public internet!)");

        ctx.persist_formations()?;

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.args.name_id = matches.value_of("formation").map(ToOwned::to_owned);
        ctx.args.fetch = matches.is_present("fetch");
        Ok(())
    }
}
