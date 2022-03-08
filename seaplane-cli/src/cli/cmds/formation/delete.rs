use clap::Parser;

use crate::{
    cli::errors,
    error::Result,
    fs::{FromDisk, ToDisk},
    ops::formation::Formations,
    Ctx,
};

/// Delete a Seaplane Formation
#[derive(Parser)]
#[clap(visible_aliases = &["del", "remove", "rm"],
    override_usage = "seaplane formation delete <NAME|ID> [OPTIONS]")]
pub struct SeaplaneFormationDeleteArgs {
    /// The name or ID of the Formation to remove, must be unambiguous
    #[clap(value_name = "NAME|ID")]
    formation: String,

    /// Delete this Formation even if there are configurations In Flight (active), which will
    /// effectively stop all instances of this Formation
    #[clap(long)]
    force: bool,

    /// Delete all matching Formations even when FORMATION is ambiguous
    #[clap(short, long)]
    all: bool,

    /// Delete local Formations (this is set by the default, use --no-local to skip)
    #[clap(long, overrides_with = "no-local")]
    local: bool,

    /// DO NOT delete local Formations
    #[clap(long, overrides_with = "local")]
    no_local: bool,

    /// Delete remote Formations
    #[clap(long, overrides_with = "no-remote")]
    remote: bool,

    /// DO NOT delete remote Formations (this is set by the default, use --remote to remove them)
    #[clap(long, overrides_with = "remote")]
    no_remote: bool,

    /// the given FORMATION must be an exact match
    #[clap(short = 'x', long)]
    exact: bool,
    // TODO: add --recursive to handle configurations too
}

impl SeaplaneFormationDeleteArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        self.update_ctx(ctx)?;

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

        // TODO: find remote Formations too to check references

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

        // Remove the Formations
        if formation_ctx.local {
            formations
                .remove_formation_indices(&indices)
                .iter()
                .for_each(|formation| {
                    cli_println!("Deleted Formation {}", &formation.id.to_string());
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

    fn update_ctx(&self, ctx: &mut Ctx) -> Result<()> {
        ctx.force = self.force;
        let mut fctx = ctx.formation_ctx();
        fctx.remote = self.remote;
        fctx.local = self.local || !self.no_local;

        Ok(())
    }
}
