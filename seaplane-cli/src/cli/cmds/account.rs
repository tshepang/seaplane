use clap::{Parser, Subcommand};
use seaplane::api::TokenRequest;

use crate::{
    context::Ctx,
    error::{CliError, CliErrorKind, Result},
    printer::Printer,
};

/// Operate on your Seaplane account, including access tokens
#[derive(Parser)]
#[clap(visible_alias = "acct")]
pub struct SeaplaneAccountArgs {
    // subcommands
    #[clap(subcommand)]
    cmd: SeaplaneAccountCmds,
}

impl SeaplaneAccountArgs {
    pub fn run(&self, ctx: &Ctx) -> Result<()> {
        use SeaplaneAccountCmds::*;

        match &self.cmd {
            Token(args) => args.run(ctx),
        }
    }
}

#[derive(Subcommand)]
pub enum SeaplaneAccountCmds {
    Token(SeaplaneAccountTokenArgs),
}

#[derive(Parser)]
pub struct SeaplaneAccountTokenArgs;

impl SeaplaneAccountTokenArgs {
    pub fn run(&self, ctx: &Ctx) -> Result<()> {
        Printer::init(ctx.color);

        let t = TokenRequest::builder()
            .api_key(
                ctx.api_key
                    .as_ref()
                    .ok_or_else(|| CliErrorKind::MissingApiKey.into_err())?,
            )
            .build()
            .map_err(CliError::from)?;

        cli_println!("{}", t.access_token().map_err(CliError::from)?);

        Ok(())
    }
}
