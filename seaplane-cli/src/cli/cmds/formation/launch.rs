use clap::{ArgMatches, Command};
use seaplane::api::v1::{ActiveConfiguration, ActiveConfigurations};

use crate::{
    cli::{
        cmds::formation::{build_request, SeaplaneFormationFetch},
        errors,
        validator::validate_name_id,
        CliCommand,
    },
    error::{CliError, Context, Result},
    fs::{FromDisk, ToDisk},
    ops::formation::Formations,
    Ctx,
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
        // TODO: make it possible to selectively start only *some* configs
        Command::new("launch")
            .visible_alias("start")
            .about("Start all configurations of a Formation and evenly distribute traffic between them")
            .long_about(LONG_ABOUT)
            .arg(arg!(formation =["NAME|ID"] required)
                .validator(validate_name_id)
                .help("The name or ID of the Formation to launch"))
            .arg(arg!(--all -('a')).conflicts_with("exact").help("Stop all matching Formations even when FORMATION is ambiguous"))
            .arg(arg!(--exact -('x')).conflicts_with("all").help("The given FORMATION must be an exact match"))
            .arg(arg!(--fetch -('F')).help("Fetch remote Formation definitions prior to attempting to launch this Formation"))
            .arg(arg!(--grounded).help("Upload the configuration(s) to Seaplane but *DO NOT* set them to active"))
    }
}

impl CliCommand for SeaplaneFormationLaunch {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let should_fetch = ctx.formation_ctx().fetch;
        if should_fetch {
            let fetch = SeaplaneFormationFetch;
            fetch.run(ctx)?;
        }
        // Load the known Formations from the local JSON "DB"
        let formations_file = ctx.formations_file();
        let formations: Formations = FromDisk::load(&formations_file)?;

        // Get the indices of any formations that match the given name/ID
        let indices = if ctx.exact {
            formations.formation_indices_of_matches(ctx.name_id.as_ref().unwrap())
        } else {
            formations.formation_indices_of_left_matches(ctx.name_id.as_ref().unwrap())
        };

        match indices.len() {
            0 => errors::no_matching_item(ctx.name_id.clone().unwrap(), ctx.exact)?,
            1 => (),
            _ => {
                // TODO: and --force
                if !ctx.all {
                    errors::ambiguous_item(ctx.name_id.clone().unwrap(), true)?;
                }
            }
        }

        for idx in indices {
            // re unwrap: the indices returned came from Formations so they have to be valid
            let formation = formations.get_formation(idx).unwrap();

            // Get the local configs that don't exist remote yet
            let cfgs_ids = formation.local_only_configs();

            // Add those configurations to this formation
            for id in cfgs_ids {
                if let Some(cfg) = formations.get_configuration(id) {
                    let add_cfg_req = build_request(Some(formation.name.as_ref().unwrap()), ctx)?;
                    // We don't set the configuration to active because we'll be doing that to
                    // *all* formation configs in a minute
                    add_cfg_req.add_configuration(cfg.model.clone(), false)?;
                } else {
                    // TODO: Inform the user of possible error?
                }
            }

            let mut cfg_uuids = Vec::new();
            // If the user pasaed `--grounded` they don't want the configuration to be set to
            // active
            if !ctx.formation_ctx().grounded {
                // Get all configurations for this Formation
                let list_cfg_uuids_req =
                    build_request(Some(formation.name.as_ref().unwrap()), ctx)?;
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
                let set_cfgs_req = build_request(Some(formation.name.as_ref().unwrap()), ctx)?;
                set_cfgs_req
                    .set_active_configurations(active_configs, false)
                    .map_err(CliError::from)
                    .context("Context: failed to retrieve Formation Configuration IDs\n")?;
            }

            cli_print!("Successfully Launched Formation '");
            cli_print!(@Green, "{}", &ctx.name_id.as_ref().unwrap());
            if cfg_uuids.is_empty() {
                cli_println!("'");
            } else {
                cli_println!("' with Configuration UUIDs:");
                for uuid in cfg_uuids {
                    cli_println!(@Green, "{}", uuid);
                }
            }
        }

        // Write out an entirely new JSON file with the new Formation included
        formations
            .persist()
            .with_context(|| format!("Path: {:?}\n", ctx.formations_file()))?;

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.name_id = matches.value_of("formation").map(ToOwned::to_owned);
        Ok(())
    }
}
