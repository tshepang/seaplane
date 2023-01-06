pub mod common;
mod delete;
mod get;
mod list;
mod set;

use clap::{ArgMatches, Command};

pub use self::{
    common::SeaplaneRestrictCommonArgMatches,
    delete::SeaplaneRestrictDelete,
    get::SeaplaneRestrictGet,
    list::{SeaplaneRestrictList, SeaplaneRestrictListArgMatches},
    set::{SeaplaneRestrictSet, SeaplaneRestrictSetArgMatches},
};
use crate::cli::CliCommand;

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneRestrict;

impl SeaplaneRestrict {
    pub fn command() -> Command {
        Command::new("restrict")
            .about("Restrict the placement of data for Global Data Coordination API")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(SeaplaneRestrictGet::command())
            .subcommand(SeaplaneRestrictList::command())
            .subcommand(SeaplaneRestrictSet::command())
            .subcommand(SeaplaneRestrictDelete::command())
    }
}

impl CliCommand for SeaplaneRestrict {
    fn next_subcmd<'a>(
        &self,
        matches: &'a ArgMatches,
    ) -> Option<(Box<dyn CliCommand>, &'a ArgMatches)> {
        match &matches.subcommand() {
            Some(("get", m)) => Some((Box::new(SeaplaneRestrictGet), m)),
            Some(("list", m)) => Some((Box::new(SeaplaneRestrictList), m)),
            Some(("set", m)) => Some((Box::new(SeaplaneRestrictSet), m)),
            Some(("delete", m)) => Some((Box::new(SeaplaneRestrictDelete), m)),
            _ => None,
        }
    }
}
