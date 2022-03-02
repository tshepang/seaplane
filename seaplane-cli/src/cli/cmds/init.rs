use std::fs;

use clap::Parser;

use crate::{context::Ctx, error::Result};

/// Create the Seaplane directory structure at the appropriate locations
#[derive(Parser)]
pub struct SeaplaneInitArgs {
    /// Force create the files and directories (DANGER: will overwrite existing files)
    #[clap(long)]
    pub force: bool,
}

impl SeaplaneInitArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        self.update_ctx(ctx)?;

        // Create the data directory
        cli_debugln!("Creating directory {:?}", ctx.data_dir());
        fs::create_dir_all(ctx.data_dir())?;

        let to_create = &[(ctx.formations_file(), b"{}"), (ctx.flights_file(), b"[]")];
        for (file, empty_bytes) in to_create {
            if file.exists() {
                if ctx.force {
                    cli_warn!(@Yellow, "warn: ");
                    cli_warn!("overwriting existing file ");
                    cli_warn!(@Green, "{:?} ", file);
                    cli_warn!("due to '");
                    cli_warn!(@Green, "--force");
                    cli_warnln!(@noprefix, "'");
                } else {
                    cli_warn!(@Yellow, "warn: ");
                    cli_warn!(@Green, "{:?} ", file);
                    cli_warnln!(@noprefix, "already exists");
                    cli_warn!("(hint: use '");
                    cli_warn!(@Green, "seaplane init --force");
                    cli_warnln!(@noprefix, "to erase and overwrite it)");
                    continue;
                }
            }
            cli_debugln!("creating file {:?}", file);
            fs::write(file, empty_bytes)?;
        }

        cli_println!("Successfully created Seaplane directories");

        Ok(())
    }

    pub fn update_ctx(&self, ctx: &mut Ctx) -> Result<()> {
        ctx.force = self.force;
        Ok(())
    }
}
