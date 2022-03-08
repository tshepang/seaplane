use clap::Parser;
use seaplane::api::v1::{ActiveConfiguration, ActiveConfigurations};

use crate::{
    cli::{
        cmds::formation::{build_request, SeaplaneFormationFetchArgs},
        errors,
        validator::validate_name_id,
    },
    error::{CliError, Context, Result},
    fs::{FromDisk, ToDisk},
    ops::formation::Formations,
    Ctx,
};

// TODO: make it possible to selectively start only *some* configs
/// Start all configurations of a Formation and evenly distribute traffic between them
#[derive(Parser)]
#[clap(
    visible_alias = "start",
    long_about = "Start all configurations of a Formation and evenly distribute traffic between them

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
    and traffic will be balanced between any *all* configurations. "
)]
pub struct SeaplaneFormationLaunchArgs {
    #[clap(value_name = "NAME|ID", validator = validate_name_id)]
    formation: String,

    /// Stop all matching Formations even when FORMATION is ambiguous
    #[clap(short, long)]
    all: bool,

    /// the given FORMATION must be an exact match
    #[clap(short = 'x', long)]
    exact: bool,

    /// Fetch remote Formation definitions prior to attempting to launch this Formation
    #[clap(short = 'F', long)]
    fetch: bool,

    /// Upload the configuration(s) to Seaplane but *DO NOT* set them to active
    #[clap(long)]
    grounded: bool,
}

impl SeaplaneFormationLaunchArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        if self.fetch {
            let fetch = SeaplaneFormationFetchArgs {
                formation: Some(self.formation.clone()),
            };
            fetch.run(ctx)?;
        }
        // Load the known Formations from the local JSON "DB"
        let formations_file = ctx.formations_file();
        let formations: Formations = FromDisk::load(&formations_file)?;

        // Get the indices of any formations that match the given name/ID
        let indices = if self.exact {
            formations.formation_indices_of_matches(&self.formation)
        } else {
            formations.formation_indices_of_left_matches(&self.formation)
        };

        match indices.len() {
            0 => errors::no_matching_item(self.formation.clone(), self.exact)?,
            1 => (),
            _ => {
                // TODO: and --force
                if !self.all {
                    errors::ambiguous_item(self.formation.clone(), true)?;
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
                    let add_cfg_req = build_request(Some(&formation.name.as_ref().unwrap()), ctx)?;
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
            if !self.grounded {
                // Get all configurations for this Formation
                let list_cfg_uuids_req =
                    build_request(Some(&formation.name.as_ref().unwrap()), ctx)?;
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
                let set_cfgs_req = build_request(Some(&formation.name.as_ref().unwrap()), ctx)?;
                set_cfgs_req
                    .set_active_configurations(active_configs, false)
                    .map_err(CliError::from)
                    .context("Context: failed to retrieve Formation Configuration IDs\n")?;
            }

            cli_print!("Successfully Launched Formation '");
            cli_print!(@Green, "{}", &self.formation);
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
}
