use std::fs;

use clap::{ArgMatches, Command};

use crate::{cli::CliCommand, config::RawConfig, context::Ctx, error::Result, fs::conf_dirs};

static LONG_FORCE: &str =
    "Force create the files and directories (DANGER: will overwrite existing files)

Using --force is the same as using --overwrite=all";
static LONG_OVERWRITE: &str =
    "Overwrite select files or directories (DANGER: will overwrite existing data)

Using --overwrite=all is the same as using --force";

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneInit;

impl SeaplaneInit {
    pub fn command() -> Command<'static> {
        Command::new("init")
            .about("Create the Seaplane directory structure at the appropriate locations")
            .arg(arg!(--force)
                .help("Force create the files and directories (DANGER: will overwrite existing files)")
                .long_help(LONG_FORCE))
            .arg(arg!(--overwrite =["ITEM"])
                .help("Overwrite select files or directories (DANGER: will overwrite existing data)")
                .long_help(LONG_OVERWRITE)
                .possible_values(&["all", "formations", "flights", "config"]))
    }
}

impl CliCommand for SeaplaneInit {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
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
                // Due to how match guards work, we can't use them, we have to use if-else
                if ctx.force
                    || ctx.overwrite.as_deref() == Some(opt)
                    || ctx.overwrite.as_deref() == Some("all")
                {
                    cli_warn!(@Yellow, "warn: ");
                    cli_warn!("overwriting existing file ");
                    cli_warn!("{:?} ", file);
                    cli_warn!("due to '");
                    cli_warn!(@Green, "{}", if ctx.force { "--force".into() } else { format!("--overwrite={opt}")});
                    cli_warnln!(@noprefix, "'\n");
                } else {
                    // We only want to advertise the *least* destructive option, not --force or
                    // --overwrite=all. The user can find those on their own.
                    cli_warn!(@Yellow, "warn: ");
                    cli_warn!("{:?} ", file);
                    cli_warnln!(@noprefix, "already exists");
                    cli_warn!("(hint: use '");
                    cli_warn!(@Green, "seaplane init --overwrite={} ", opt);
                    cli_warnln!(@noprefix, "to erase and overwrite it)\n");
                    continue;
                }
            }
            cli_debugln!("creating file {:?}", file);
            fs::write(file, empty_bytes)?;
        }

        cli_println!("Successfully created Seaplane directories");

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.force = matches.is_present("force");
        ctx.overwrite = matches.value_of("overwrite").map(ToOwned::to_owned);
        Ok(())
    }
}
