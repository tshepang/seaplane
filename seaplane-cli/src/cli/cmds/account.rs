use std::io::{self, BufRead};

use clap::{Parser, Subcommand};
use seaplane::api::{TokenRequest, FLIGHTDECK_API_URL};

use crate::{
    config::RawConfig,
    context::Ctx,
    error::{CliError, CliErrorKind, Context, Result},
    fs::{FromDisk, ToDisk},
    printer::{Color, Printer},
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
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        use SeaplaneAccountCmds::*;

        match &self.cmd {
            Login(args) => args.run(ctx),
            Token(args) => args.run(ctx),
        }
    }
}

#[derive(Subcommand)]
pub enum SeaplaneAccountCmds {
    Login(SeaplaneAccountLoginArgs),
    Token(SeaplaneAccountTokenArgs),
}

#[derive(Parser)]
pub struct SeaplaneAccountTokenArgs;

impl SeaplaneAccountTokenArgs {
    pub fn run(&self, ctx: &Ctx) -> Result<()> {
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

#[derive(Parser)]
pub struct SeaplaneAccountLoginArgs {
    /// Override any existing API key
    #[clap(short, long)]
    force: bool,
}

impl SeaplaneAccountLoginArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let mut cfg = RawConfig::load(ctx.conf_files().first().ok_or_else(|| {
            CliErrorKind::MissingPath
                .into_err()
                .context("Context: no configuration file found\n")
                .context("(hint: try '")
                .color_context(Color::Green, "seaplane init")
                .context("' if the files are missing)\n")
        })?)?;

        if let Some(key) = cfg.account.api_key {
            if self.force {
                cli_warn!(@Yellow, "warn: ");
                cli_warn!("overwriting API key ");
                cli_warn!(@Green, "{} ", key);
                cli_warn!("due to ");
                cli_warnln!(@noprefix, @Green, "--force");
            } else {
                return Err(CliErrorKind::ExistingValue("an API key")
                    .into_err()
                    .context("(hint: add '")
                    .color_context(Color::Green, "--force")
                    .context("' to overwrite it)\n"));
            }
        }
        cli_println!("Enter your API key below.");
        cli_print!("(hint: it can be found by visiting ");
        cli_print!(@Green, "{}", FLIGHTDECK_API_URL);
        cli_println!(")\n");

        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        if let Some(line) = lines.next() {
            ctx.api_key = Some(line?);
        }

        cfg.account.api_key = ctx.api_key.clone();

        cfg.persist()?;

        cli_println!("Successfully saved the API key!");

        Ok(())
    }
}
