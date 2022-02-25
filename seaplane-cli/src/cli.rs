pub mod cmds;

use std::env;

use anyhow::Result;
use clap::{crate_authors, Parser, Subcommand};

pub use crate::cli::cmds::*;
use crate::{context::Ctx, printer::ColorChoice};

static VERSION: &str = env!("SEAPLANE_GIT_HASH");
static AUTHORS: &str = crate_authors!();

#[derive(Parser)]
// Unset any term-width detection for UI tests
#[cfg_attr(feature = "ui_tests", clap(term_width = 0))]
#[clap(
    name = "seaplane",
    author = AUTHORS,
    version = VERSION,
    propagate_version = true,
    disable_colored_help = true,
)]
pub struct SeaplaneArgs {
    /// Display more verbose output
    #[clap(
        short,
        long,
        parse(from_occurrences),
        global = true,
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
        global = true,
        long_help = "Suppress output at a specific level and below

More uses suppresses higher levels of output
    -q:   Only display WARN messages and above
    -qq:  Only display ERROR messages
    -qqq: Suppress all output"
    )]
    pub quiet: u8,

    /// Should the output include color?
    #[clap(
        long,
        global = true,
        overrides_with_all = &["color", "no_color"],
        default_value = "auto",
        arg_enum
    )]
    pub color: ColorChoice,
    /// Do not color output (alias for --color=never)
    #[clap(
        long,
        global = true,
        overrides_with_all = &["color", "no_color"],
        overrides_with = "color",
    )]
    pub no_color: bool,

    // Subcommands
    #[clap(subcommand)]
    pub cmd: SeaplaneCmds,
}

impl SeaplaneArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
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
        }
    }

    fn update_ctx(&self, ctx: &mut Ctx) -> Result<()> {
        // There is a "bug" where due to how clap handles nested-subcommands with global flags and
        // overrides (yeah...niche) if two mutually exclusive flags that override each-other are
        // used at different nesting levels, the overrides do not happen.
        //
        // For us this means doing `seaplane --no-color SUBCOMMAND --color=auto` effectively there
        // will be no color output, because clap will evaluate `--no-color` to `true` (i.e. used)
        // even though they override each-other.
        //
        // So we err on the side of not providing color since that is the safer option
        ctx.color = match (self.color, self.no_color) {
            (_, true) => ColorChoice::Never,
            (choice, _) => choice,
        };

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
}
