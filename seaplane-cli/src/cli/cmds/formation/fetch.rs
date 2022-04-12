use std::collections::HashSet;

use clap::Command;

use crate::{
    cli::{cmds::formation::build_request, CliCommand},
    error::{CliError, Context, Result},
    ops::formation::Formation,
    printer::Color,
    Ctx,
};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormationFetch;

impl SeaplaneFormationFetch {
    pub fn command() -> Command<'static> {
        //TODO: add a --no-overwrite or similar
        Command::new("fetch-remote")
            .visible_aliases(&["fetch", "sync", "synchronize"])
            .about("Fetch remote Formation Instances and create/synchronize local Plan definitions")
            .override_usage(
                "seaplane formation fetch-remote
    seaplane formation fetch-remote [NAME|ID]",
            )
            .arg(
                arg!(formation = ["NAME|ID"])
                    .help("The NAME or ID of the remote Formation Instance to fetch, omit to fetch all Formation Instances"),
            )
    }
}

impl CliCommand for SeaplaneFormationFetch {
    // TODO: async
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let api_key = ctx.args.api_key()?;
        let names = if let Some(name) = &ctx.args.name_id {
            vec![name.to_owned()]
        } else {
            // First download all formation names
            let formations_request = build_request(None, api_key)?;
            formations_request
                .list_names()
                .map_err(CliError::from)
                .context("Context: failed to retrieve Formation Instance names\n")?
                .into_inner()
        };

        // TODO: We't requesting tons of new tokens...maybe we could do multiple per and just
        // retry on error?
        let mut flights_added = HashSet::new();
        let mut formations_added = HashSet::new();
        for name in &names {
            let list_cfg_uuids_req = build_request(Some(name), api_key)?;

            let cfg_uuids = list_cfg_uuids_req
                .list_configuration_ids()
                .map_err(CliError::from)
                .context("Context: failed to retrieve Formation Configuration IDs\n")?;
            let active_cfgs_req = build_request(Some(name), api_key)?;
            let active_cfgs = active_cfgs_req
                .get_active_configurations()
                .map_err(CliError::from)
                .context("Context: failed to retrieve Active Formation Configurations\n")?;

            for uuid in cfg_uuids.into_iter() {
                let get_cfgs_req = build_request(Some(name), api_key)?;
                let cfg_model = get_cfgs_req
                    .get_configuration(uuid)
                    .map_err(CliError::from)
                    .context("Context: failed to retrieve Formation Configuration\n\tUUID: ")
                    .with_color_context(|| (Color::Yellow, format!("{uuid}\n")))?;

                for flight in cfg_model.flights() {
                    let names_ids = ctx.db.flights.update_or_create_flight(flight);
                    flights_added.extend(names_ids);
                }

                let is_active = active_cfgs.iter().any(|ac| ac.uuid() == &uuid);
                let ids = ctx
                    .db
                    .formations
                    .update_or_create_configuration(name, cfg_model, is_active, uuid);
                let mut formation = Formation::new(name);
                formation.local.extend(&ids);
                if is_active {
                    formation.in_air.extend(ids);
                } else {
                    formation.grounded.extend(ids);
                }
                if let Some(id) = ctx.db.formations.update_or_create_formation(formation) {
                    formations_added.insert((name, id));
                }
            }
        }

        let mut count = 0;
        for (name, id) in formations_added {
            count += 1;
            cli_print!("Successfully fetched Formation Instance '");
            cli_print!(@Green, "{name}");
            cli_print!("' with and synchronized with local Plan ID '");
            cli_println!(@Green, "{}", &id.to_string()[..8]);
        }
        for (name, id) in flights_added {
            count += 1;
            cli_print!("Successfully fetched Flight Plan '");
            cli_print!(@Green, "{name}");
            cli_print!("' and synchronized with local ID '");
            cli_print!(@Green, "{}", &id.to_string()[..8]);
            cli_println!("'!");
        }
        if names.is_empty() {
            cli_println!("No remote Formation Instances found");
        } else if count > 0 {
            cli_println!("");
            cli_println!("Successfully fetched {count} items");
        } else {
            cli_println!("All local definitions are up to date!");
        }

        ctx.persist_flights()?;
        ctx.persist_formations()?;

        Ok(())
    }
}
