use std::fs;

use clap::Parser;

use crate::{config::RawConfig, context::Ctx, error::Result, fs::conf_dirs};

/// Create the Seaplane directory structure at the appropriate locations
#[derive(Parser)]
pub struct SeaplaneInitArgs {
    /// Force create the files and directories (DANGER: will overwrite existing files)
    #[clap(
        long,
        long_help = "Force create the files and directories (DANGER: will overwrite existing files)

Using --force is the same as using --overwrite=all"
    )]
    pub force: bool,

    /// Overwrite select files or directories (DANGER: will overwrite existing data)
    #[clap(long, possible_values = &["all", "formations", "flights", "config"],
        long_help = "Overwrite select files or directories (DANGER: will overwrite existing data)

Using --overwrite=all is the same as using --force")]
    pub overwrite: Option<String>,
}

impl SeaplaneInitArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        self.update_ctx(ctx)?;

        // Create the data directory
        cli_debugln!("Creating directory {:?}", ctx.data_dir());
        fs::create_dir_all(ctx.data_dir())?;

        // We only create the first (most preferred) configuration dir. If the user creates more
        // down our search path, that's fine, but we only create and advertise the first.
        let conf_dir = &conf_dirs()[0];
        cli_debugln!("Creating directory {:?}", conf_dir);
        fs::create_dir_all(conf_dir)?;

        // Tuple below is: (File, "empty" bytes, it's --force=OPTION)
        let to_create = &[
            (
                conf_dir.join("seaplane.toml"),
                toml::to_string_pretty(&RawConfig::default()).unwrap(),
                "config",
            ),
            (ctx.formations_file(), "{}".to_string(), "formations"),
            (ctx.flights_file(), "[]".to_string(), "flights"),
        ];
        // TODO: @security create the file with limited permissions
        for (file, empty_bytes, opt) in to_create {
            if file.exists() {
                match (ctx.force, &self.overwrite) {
                    (true, Some(val)) | (_, Some(val)) if val == opt || val == "all" => {
                        cli_warn!(@Yellow, "warn: ");
                        cli_warn!("overwriting existing file ");
                        cli_warn!("{:?} ", file);
                        cli_warn!("due to '");
                        cli_warn!(@Green, "{}", if ctx.force { "--force".into() } else { format!("--overwrite={}", val)});
                        cli_warnln!(@noprefix, "'\n");
                    }
                    _ => {
                        // We only want to advertise the *least* destructive option, not --force or
                        // --overwrite=all. The user can find those on their own.
                        cli_warn!(@Yellow, "warn: ");
                        cli_warn!("{:?} ", file);
                        cli_warnln!(@noprefix, "already exists");
                        cli_warn!("(hint: use '");
                        cli_warn!(@Green, "seaplane init --overwite={} ", opt);
                        cli_warnln!(@noprefix, "to erase and overwrite it)\n");
                        continue;
                    }
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
