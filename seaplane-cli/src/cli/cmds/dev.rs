use anyhow::Result;
use clap::{AppSettings, Parser, Subcommand};

use crate::Ctx;

/// Used for local development as an internal command
#[derive(Parser)]
pub struct SeaplaneDevArgs {
    #[clap(subcommand)]
    cmd: SeaplaneDevCmds,
}

impl SeaplaneDevArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        match &self.cmd {
            SeaplaneDevCmds::Auth(args) => args.run(ctx),
        }
    }
}

#[derive(Subcommand)]
pub enum SeaplaneDevCmds {
    /// Generate a JWT with HS256 signature
    Auth(SeaplaneDevAuthArgs),
}

#[derive(Parser)]
pub struct SeaplaneDevAuthArgs;

impl SeaplaneDevAuthArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let token = crate::dev::generate_auth_token(&ctx.dev.auth_key, &ctx.dev.auth_claims)?;

        println!("Authorization: bearer {}", token);

        Ok(())
    }
}
