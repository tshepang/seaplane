mod common;
mod delete;
mod get;
mod list;
mod set;

use clap::{ArgMatches, Command};
use seaplane::api::v1::config::{ConfigRequest, RangeQueryContext};
use strum::VariantNames;

pub use self::{
    common::SeaplaneMetadataCommonArgMatches,
    delete::SeaplaneMetadataDelete,
    get::SeaplaneMetadataGet,
    list::SeaplaneMetadataList,
    set::{SeaplaneMetadataSet, SeaplaneMetadataSetArgMatches},
};
use crate::{
    cli::{request_token, CliCommand},
    context::Ctx,
    error::{CliError, Context, Result},
    printer::OutputFormat,
};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneMetadata;

impl SeaplaneMetadata {
    pub fn command() -> Command<'static> {
        Command::new("metadata")
            .about("Operate on metadata key-value pairs using the Global Data Coordination API")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .visible_aliases(&["meta", "md"])
            .arg(
                arg!(--format =["FORMAT"=>"table"] global)
                    .help("Change the output format")
                    .possible_values(OutputFormat::VARIANTS),
            )
            .subcommand(SeaplaneMetadataGet::command())
            .subcommand(SeaplaneMetadataSet::command())
            .subcommand(SeaplaneMetadataDelete::command())
            .subcommand(SeaplaneMetadataList::command())
    }
}

impl CliCommand for SeaplaneMetadata {
    fn next_subcmd<'a>(
        &self,
        matches: &'a ArgMatches,
    ) -> Option<(Box<dyn CliCommand>, &'a ArgMatches)> {
        match &matches.subcommand() {
            Some(("get", m)) => Some((Box::new(SeaplaneMetadataGet), m)),
            Some(("set", m)) => Some((Box::new(SeaplaneMetadataSet), m)),
            Some(("delete", m)) => Some((Box::new(SeaplaneMetadataDelete), m)),
            Some(("list", m)) => Some((Box::new(SeaplaneMetadataList), m)),
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
