use clap::Command;

use crate::{
    cli::{
        cmds::formation::build_request,
        errors,
        validator::{validate_formation_name, validate_name_id},
        CliCommand,
    },
    error::Result,
    Ctx,
};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormationLand;

impl SeaplaneFormationLand {
    pub fn command() -> Command<'static> {
        let validator = |s: &str| validate_name_id(validate_formation_name, s);
        Command::new("land")
            .visible_alias("stop")
            .about("Land (Stop) all configurations of a Formation")
            .arg(
                arg!(formation =["NAME|ID"] required)
                    .help("The name or ID of the Formation to land")
                    .validator(validator),
            )
            .arg(
                arg!(--all - ('a'))
                    .conflicts_with("exact")
                    .help("Stop all matching Formations even when FORMATION is ambiguous"),
            )
            .arg(
                arg!(--exact - ('x'))
                    .conflicts_with("all")
                    .help("The given FORMATION must be an exact match"),
            )
    }
}

impl CliCommand for SeaplaneFormationLand {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
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
        for idx in indices {
            // re unwrap: the indices returned came from Formations so they have to be valid
            let formation = ctx.db.formations.get_formation_mut(idx).unwrap();

            // re unwrap: We got the formation from the local DB so it has to have a name
            let stop_req = build_request(Some(formation.name.as_ref().unwrap()), api_key)?;
            stop_req.stop()?;

            // Move all configurations from in air to grounded
            let ids: Vec<_> = formation.in_air.drain().collect();
            for id in ids {
                formation.grounded.insert(id);
            }

            ctx.persist_formations()?;

            cli_print!("Successfully Landed Formation '");
            cli_print!(@Green, "{}", &ctx.args.name_id.as_ref().unwrap());
            cli_println!("'");
        }

        Ok(())
    }
}
