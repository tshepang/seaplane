mod acquire;
mod common;
mod list;
mod release;
mod renew;

use clap::{ArgMatches, Command};
use strum::VariantNames;

pub use self::{
    acquire::SeaplaneLocksAcquire, common::SeaplaneLocksCommonArgMatches, list::SeaplaneLocksList,
    release::SeaplaneLocksRelease, renew::SeaplaneLocksRenew,
};
use crate::{cli::CliCommand, printer::OutputFormat};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneLocks;

impl SeaplaneLocks {
    pub fn command() -> Command<'static> {
        Command::new("locks")
            .about("Operate on the Locks API")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .visible_aliases(&["l"])
            .arg(
                arg!(--format =["FORMAT"=>"table"] global)
                    .help("Change the output format")
                    .possible_values(OutputFormat::VARIANTS),
            )
            .subcommand(SeaplaneLocksList::command())
            .subcommand(SeaplaneLocksAcquire::command())
            .subcommand(SeaplaneLocksRelease::command())
            .subcommand(SeaplaneLocksRenew::command())
    }
}

impl CliCommand for SeaplaneLocks {
    fn next_subcmd<'a>(
        &self,
        matches: &'a ArgMatches,
    ) -> Option<(Box<dyn CliCommand>, &'a ArgMatches)> {
        match &matches.subcommand() {
            Some(("list", m)) => Some((Box::new(SeaplaneLocksList), m)),
            Some(("acquire", m)) => Some((Box::new(SeaplaneLocksAcquire), m)),
            Some(("release", m)) => Some((Box::new(SeaplaneLocksRelease), m)),
            Some(("renew", m)) => Some((Box::new(SeaplaneLocksRenew), m)),
            _ => None,
        }
    }
}
