pub mod cmds;
pub mod errors;
pub mod specs;
pub mod validator;

use std::{
    env,
    io::{self, BufRead},
};

use clap::{crate_authors, Parser, Subcommand};

pub use crate::cli::cmds::*;
use crate::{
    context::Ctx,
    error::Result,
    printer::{ColorChoice, Printer},
};

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

    /// The API key associated with your account used to access Seaplane API endpoints
    #[clap(
        short = 'A',
        long,
        global = true,
        value_name = "STRING",
        env = "SEAPLANE_API_KEY",
        hide_env_values = true,
        long_help = "The API key associated with your account used to access Seaplane API endpoints

The value provided here will override any provided in any configuration files.
A CLI provided value also overrides any environment variables.
One can use a special value of '-' to signal the value should be read from STDIN."
    )]
    pub api_key: Option<String>,

    // Subcommands
    #[clap(subcommand)]
    pub cmd: SeaplaneCmds,
}

impl SeaplaneArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        use SeaplaneCmds::*;

        self.update_ctx(ctx)?;

        // Initilize the printer now that we have all the color choices
        Printer::init(ctx.color);

        match &self.cmd {
            Account(args) => args.run(ctx),
            ShellCompletion(args) => args.run(ctx),
            Config(args) => args.run(ctx),
            Formation(args) => args.run(ctx),
            Image(args) => args.run(ctx),
            Init(args) => args.run(ctx),
            Flight(args) => args.run(ctx),
            License(args) => args.run(ctx),
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

        if let Some(key) = &self.api_key {
            if key == "-" {
                let stdin = io::stdin();
                let mut lines = stdin.lock().lines();
                if let Some(line) = lines.next() {
                    ctx.api_key = Some(line?);
                }
            } else {
                ctx.api_key = Some(key.to_owned());
            }
        }

        Ok(())
    }
}

#[derive(Subcommand)]
pub enum SeaplaneCmds {
    Account(SeaplaneAccountArgs),
    ShellCompletion(SeaplaneShellCompletionArgs),
    Config(SeaplaneConfigArgs),
    Formation(Box<SeaplaneFormationArgs>),
    Image(SeaplaneImageArgs),
    Init(SeaplaneInitArgs),
    License(SeaplaneLicenseArgs),
    Flight(Box<SeaplaneFlightArgs>),
}
