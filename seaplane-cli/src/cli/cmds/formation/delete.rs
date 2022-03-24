use clap::{ArgMatches, Command};

use crate::{
    cli::{cmds::formation::build_request, errors, validator::validate_name_id, CliCommand},
    context::Ctx,
    error::{CliErrorKind, Context, Result},
    fs::{FromDisk, ToDisk},
    ops::formation::Formations,
    printer::Color,
};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormationDelete;

impl SeaplaneFormationDelete {
    pub fn command() -> Command<'static> {
        // TODO: add --recursive to handle configurations too
        Command::new("delete")
            .visible_aliases(&["del", "remove", "rm"])
            .about("Delete a Seaplane Formation")
            .override_usage("seaplane formation delete <NAME|ID> [OPTIONS]")
            .arg(arg!(formation =["NAME|ID"] required)
                .validator(validate_name_id)
                .help("The name or ID of the Formation to remove, must be unambiguous"))
            .arg(arg!(--force)
                .help("Delete this Formation even if there are configurations In Flight (active), which will effectively stop all instances of this Formation"))
            .arg(arg!(--all -('a'))
                .conflicts_with("exact")
                .help("Delete all matching Formations even when FORMATION is ambiguous"))
            .arg(arg!(--local)
                .overrides_with("no-local")
                .help("Delete local Formations (this is set by the default, use --no-local to skip)"))
            .arg(arg!(--("no-local"))
                .overrides_with("local")
                .help("DO NOT delete local Formations"))
            .arg(arg!(--remote)
                .overrides_with("no-remote")
                .help("Delete remote Formations (this is set by default, use --no-remote to skip)"))
            .arg(arg!(--("no-remote"))
                .overrides_with("remote")
                .help("DO NOT delete remote Formations (this is set by the default, use --remote to remove them)"))
            .arg(arg!(--exact -('x'))
                .conflicts_with("all")
                .help("The given FORMATION must be an exact match"))
    }
}

impl CliCommand for SeaplaneFormationDelete {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let formation_ctx = ctx.formation_ctx();

        if !formation_ctx.local && !formation_ctx.remote {
            cli_eprint!(@Red, "error: ");
            cli_eprintln!("nothing to do");
            cli_eprint!("(hint: either remove ");
            cli_eprint!(@Yellow, "--no-local ");
            cli_eprint!("or add ");
            cli_eprint!(@Yellow, "--remote ");
            cli_eprintln!("to the command)");
            std::process::exit(1);
        }

        // Load the known Formations from the local JSON "DB"
        let formations_file = ctx.formations_file();
        let mut formations: Formations = FromDisk::load(&formations_file)?;

        // Get the indices of any formations that match the given name/ID
        let indices = if ctx.exact {
            formations.formation_indices_of_matches(&formation_ctx.name_id)
        } else {
            formations.formation_indices_of_left_matches(&formation_ctx.name_id)
        };

        match indices.len() {
            0 => errors::no_matching_item(formation_ctx.name_id.clone(), ctx.exact)?,
            1 => (),
            _ => {
                // TODO: and --force
                if !ctx.all {
                    errors::ambiguous_item(formation_ctx.name_id.clone(), true)?;
                }
            }
        }

        // Remove the Formations
        //
        // First try to delete the remote formation if required, because we don't want to delete
        // the local one too if this fails
        if formation_ctx.remote {
            for idx in &indices {
                let formation = formations.get_formation(*idx).unwrap();
                if let Some(name) = &formation.name {
                    let delete_req = build_request(Some(name), ctx)?;
                    let cfg_uuids = delete_req.delete(ctx.force)?;
                    cli_print!("Successfully deleted remote Formation '");
                    cli_print!(@Green, "{}", name);
                    if cfg_uuids.is_empty() {
                        cli_println!("'");
                    } else {
                        cli_println!("' with Configuration UUIDs:");
                        for uuid in cfg_uuids.into_iter() {
                            cli_println!(@Green, "\t{uuid}");
                        }
                    }
                } else {
                    return Err(CliErrorKind::NoMatchingItem(formation_ctx.name_id.clone())
                        .into_err()
                        .context("(hint: create the Formation with '")
                        .color_context(Color::Green, "seaplane formation create")
                        .context("')\n")
                        .context("(hint: or try fetching remote references with '")
                        .color_context(Color::Green, "seaplane formation fetch-remote")
                        .context("')\n")
                        .context("(hint: You can also fetch remote references with 'seaplane formation delete ")
                        .color_context(Color::Green, "--fetch")
                        .context("')\n"));
                }
            }
        }
        if formation_ctx.local {
            formations
                .remove_formation_indices(&indices)
                .iter()
                .for_each(|formation| {
                    cli_println!(
                        "Successfully deleted local Formation {}",
                        &formation.id.to_string()
                    );
                });
        }

        // TODO: handle remote Formations

        // Write out an entirely new JSON file with the Formation(s) deleted
        formations.persist()?;

        // TODO: recalculate dichotomy of local v. remote numbers (i.e. --no-local, etc.)
        cli_println!(
            "\nSuccessfully removed {} item{}",
            indices.len(),
            if indices.len() > 1 { "s" } else { "" }
        );

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.force = matches.is_present("force");
        let mut fctx = ctx.formation_ctx();
        fctx.name_id = matches.value_of("formation").unwrap().to_string();
        fctx.remote = !matches.is_present("no-remote");
        fctx.local = !matches.is_present("no-local");

        Ok(())
    }
}
