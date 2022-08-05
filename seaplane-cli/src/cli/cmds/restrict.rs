mod common;
mod get;

use clap::{value_parser, ArgMatches, Command};

pub use self::{common::SeaplaneRestrictCommonArgMatches, get::SeaplaneRestrictGet};
use crate::{cli::CliCommand, printer::OutputFormat};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneRestrict;

impl SeaplaneRestrict {
    pub fn command() -> Command<'static> {
        Command::new("restrict")
            .about("Restrict the placement of data for Global Data Coordination API")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .arg(
                arg!(--format =["FORMAT"=>"table"] global)
                    .help("Change the output format")
                    .value_parser(value_parser!(OutputFormat)),
            )
            .subcommand(SeaplaneRestrictGet::command())
    }
}

impl CliCommand for SeaplaneRestrict {
    fn next_subcmd<'a>(
        &self,
        matches: &'a ArgMatches,
    ) -> Option<(Box<dyn CliCommand>, &'a ArgMatches)> {
        match &matches.subcommand() {
            Some(("get", m)) => Some((Box::new(SeaplaneRestrictGet), m)),
            _ => None,
        }
    }
}
