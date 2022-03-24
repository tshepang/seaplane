mod common;
mod delete;
mod get;
mod list;
mod set;

use clap::{ArgMatches, Command};
use seaplane::api::v1::config::{ConfigRequest, RangeQueryContext};
use strum::VariantNames;

pub use self::{
    common::SeaplaneKvCommonArgMatches,
    delete::SeaplaneKvDelete,
    get::SeaplaneKvGet,
    list::SeaplaneKvList,
    set::{SeaplaneKvSet, SeaplaneKvSetArgMatches},
};
use crate::{
    cli::{request_token, CliCommand},
    context::Ctx,
    error::{CliError, Context, Result},
    printer::OutputFormat,
};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneKv;

impl SeaplaneKv {
    pub fn command() -> Command<'static> {
        Command::new("key-value")
            .about("Operate on key-value pairs using the Global Data Consensus API")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .visible_alias("kv")
            .arg(
                arg!(--format =["FORMAT"=>"table"] global)
                    .help("Change the output format")
                    .possible_values(OutputFormat::VARIANTS),
            )
            .subcommand(SeaplaneKvGet::command())
            .subcommand(SeaplaneKvSet::command())
            .subcommand(SeaplaneKvDelete::command())
            .subcommand(SeaplaneKvList::command())
    }
}

impl CliCommand for SeaplaneKv {
    fn next_subcmd<'a>(
        &self,
        matches: &'a ArgMatches,
    ) -> Option<(Box<dyn CliCommand>, &'a ArgMatches)> {
        match &matches.subcommand() {
            Some(("get", m)) => Some((Box::new(SeaplaneKvGet), m)),
            Some(("set", m)) => Some((Box::new(SeaplaneKvSet), m)),
            Some(("delete", m)) => Some((Box::new(SeaplaneKvDelete), m)),
            Some(("list", m)) => Some((Box::new(SeaplaneKvList), m)),
            _ => None,
        }
    }
}

/// Requests an Access token and returns a built ConfigRequest.
///
/// The target *must* be URL Safe base64 encoded already
pub fn build_config_request_key<S: Into<String>>(target: S, ctx: &Ctx) -> Result<ConfigRequest> {
    ConfigRequest::builder()
        .token(request_token(ctx, "")?)
        .encoded_key(target)
        .build()
        .map_err(CliError::from)
        .context("Context: failed to build /config endpoint request\n")
}

pub fn build_config_request_dir(range: RangeQueryContext, ctx: &Ctx) -> Result<ConfigRequest> {
    ConfigRequest::builder()
        .token(request_token(ctx, "")?)
        .range(range)
        .build()
        .map_err(CliError::from)
        .context("Context: failed to build /config endpoint request\n")
}
