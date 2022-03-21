use clap::Command;

use crate::{
    cli::{cmds::formation::build_request, CliCommand},
    error::{CliError, Context, Result},
    fs::{FromDisk, ToDisk},
    ops::{flight::Flights, formation::Formations},
    printer::Color,
    Ctx,
};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormationFetch;

impl SeaplaneFormationFetch {
    pub fn command() -> Command<'static> {
        //TODO: add a --no-overwrite or similar
        Command::new("fetch-remote")
            .visible_alias("fetch")
            .about("Fetch remote Formation definitions")
            .override_usage(
                "seaplane formation fetch-remote
    seaplane formation fetch-remote [NAME|ID]",
            )
            .arg(
                arg!(formation = ["NAME|ID"])
                    .help("The NAME or ID of the formation to fetch, omit to fetch all Formations"),
            )
    }
}

impl CliCommand for SeaplaneFormationFetch {
    // TODO: async
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        // Load the known formations from the local JSON "DB"
        let formations_file = ctx.formations_file();
        let mut formations: Formations = FromDisk::load(&formations_file)?;

        let names = if let Some(name) = &ctx.name_id {
            vec![name.to_owned()]
        } else {
            // First download all formation names
            let formations_request = build_request(None, ctx)?;
            formations_request
                .list_names()
                .map_err(CliError::from)
                .context("Context: failed to retrieve Formation names\n")?
                .into_inner()
        };

        let flights_file = ctx.flights_file();
        let mut flights: Flights = FromDisk::load(&flights_file)?;

        // TODO: We't requesting tons of new tokens...maybe we could do multiple per and just
        // retry on error?
        for name in names {
            let list_cfg_uuids_req = build_request(Some(&name), ctx)?;

            let cfg_uuids = list_cfg_uuids_req
                .list_configuration_ids()
                .map_err(CliError::from)
                .context("Context: failed to retrieve Formation Configuration IDs\n")?;
            let active_cfgs_req = build_request(Some(&name), ctx)?;
            let active_cfgs = active_cfgs_req
                .get_active_configurations()
                .map_err(CliError::from)
                .context("Context: failed to retrieve Active Formation Configurations\n")?;

            for uuid in cfg_uuids.into_iter() {
                let get_cfgs_req = build_request(Some(&name), ctx)?;
                let cfg_model = get_cfgs_req
                    .get_configuration(uuid)
                    .map_err(CliError::from)
                    .context("Context: failed to retrieve Formation Configuration\n\tUUID: ")
                    .with_color_context(|| (Color::Yellow, format!("{}\n", uuid)))?;

                for flight in cfg_model.flights() {
                    let names_ids = flights.update_or_create_flight(flight.clone());
                    for (name, id) in names_ids {
                        cli_print!("Successfully fetched Flight '");
                        cli_print!(@Green, "{}", name);
                        cli_print!("' with ID '");
                        cli_print!(@Green, "{}", &id.to_string()[..8]);
                        cli_println!("'!");
                    }
                }

                let ids = formations.update_or_create_configuration(
                    &name,
                    cfg_model,
                    active_cfgs.iter().any(|ac| ac.uuid() == &uuid),
                    uuid,
                );
                for id in ids {
                    cli_print!("Successfully fetched Formation Configuration '");
                    cli_print!(@Green, "{}", &id.to_string()[..8]);
                    cli_println!("'!");
                }
            }
        }

        // Write out an entirely new JSON file with the new Flights
        flights.persist()?;
        // Write out an entirely new JSON file with the new Formations
        formations.persist()?;

        Ok(())
    }
}
