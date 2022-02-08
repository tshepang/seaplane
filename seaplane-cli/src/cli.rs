pub mod cmds;

use std::env;

use anyhow::Result;
use clap::{crate_authors, AppSettings, Parser, Subcommand};

pub use crate::cli::cmds::*;
use crate::context::Ctx;

static VERSION: &str = env!("SEAPLANE_GIT_HASH");
static AUTHORS: &str = crate_authors!();

#[derive(Parser)]
#[clap(
    author = AUTHORS,
    version = VERSION,
    global_setting(AppSettings::PropagateVersion),
    global_setting(AppSettings::DisableColoredHelp),
    global_setting(AppSettings::UseLongFormatForHelpSubcommand),
)]
pub struct SeaplaneArgs {
    /// Display more verbose output
    #[clap(
        short,
        long,
        parse(from_occurrences),
        long_help = "Display more verbose output

More uses displays more verbose output
    -v:  Display debug info
    -vv: Display trace info"
    )]
    pub verbose: u8,

    /// Suppress output at a specific level and below
    #[clap(
        short,
        long,
        parse(from_occurrences),
        long_help = "Suppress output at a specific level and below

More uses suppresses higher levels of output
    -q:   Only display WARN messages and above
    -qq:  Only display ERROR messages
    -qqq: Suppress all output"
    )]
    pub quiet: u8,

    /// Use ANSI color codes in output
    #[clap(long, global = true, overrides_with = "no_color")]
    pub color: bool,

    /// Do not use ANSI color codes in output
    #[clap(long, global = true, overrides_with = "color")]
    pub no_color: bool,

    // Subcommands
    #[clap(subcommand)]
    pub cmd: SeaplaneCmds,
}

impl SeaplaneArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        match self.verbose {
            0 => match self.quiet {
                0 => env::set_var("RUST_LOG", "seaplane=info"),
                1 => env::set_var("RUST_LOG", "seaplane=warn"),
                2 => env::set_var("RUST_LOG", "seaplane=error"),
                _ => env::set_var("RUST_LOG", "seaplane=off"),
            },
            1 => env::set_var("RUST_LOG", "seaplane=debug"),
            _ => env::set_var("RUST_LOG", "seaplane=trace"),
        }

        self.update_ctx(ctx)?;

        match &self.cmd {
            SeaplaneCmds::Account(args) => {
                todo!("SeaplaneAccountArgs::run")
            }
            SeaplaneCmds::ShellCompletion(args) => args.run(ctx),
            SeaplaneCmds::Config(args) => {
                todo!("SeaplaneConfigArgs::run")
            }
            SeaplaneCmds::Formation(args) => {
                todo!("SeaplaneFormationArgs::run")
            }
            SeaplaneCmds::Image(args) => {
                todo!("SeaplaneImageArgs::run")
            }
            SeaplaneCmds::License(args) => args.run(ctx),
          
            // Internal for now...used for local development
            SeaplaneCmds::Dev(args) => args.run(ctx),
        }
    }

    fn update_ctx(&self, ctx: &mut Ctx) -> Result<()> {
        // There is a "bug" where due to how clap handles nested-subcommands with global flags and
        // overrides (yeah...niche) if two mutually exclusive flags that override each-other are
        // used at different nesting levels, the overrides do not happen.
        //
        // For us this means doing `seaplane --no-color SUBCOMMAND --color` effectively there will
        // be no color output, because clap will evaluate both `--color` and `--no-color` to `true`
        // (i.e. used) even though they override each-other.
        //
        // So we err on the side of not providing color by only checking the --no-color flag (since
        // showing color is *on* by default).
        ctx.color = !self.no_color;

        Ok(())
    }
}

#[derive(Subcommand)]
pub enum SeaplaneCmds {
    Account(SeaplaneAccountArgs),
    ShellCompletion(SeaplaneShellCompletionArgs),
    Config(SeaplaneConfigArgs),
    Formation(SeaplaneFormationArgs),
    Image(SeaplaneImageArgs),
    License(SeaplaneLicenseArgs),
    // Local Development/Internal...will potentially separate
    Dev(SeaplaneDevArgs),
}
