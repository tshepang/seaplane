use clap::{Parser, Subcommand};

use crate::{context::Ctx, error::Result};

#[derive(Parser)]
pub struct SeaplaneAccountArgs {
    // subcommands
    #[clap(subcommand)]
    cmd: SeaplaneAccountCmds,
}

impl SeaplaneAccountArgs {
    pub fn run(&self, _ctx: &Ctx) -> Result<()> {
        todo!("impl SeaplaneAccountArgs")
    }
}

#[derive(Subcommand)]
pub enum SeaplaneAccountCmds {
    /// Authenticate a Seaplane account
    Login(SeaplaneAccountLoginArgs),
    /// Logout of a Seaplane account
    Logout(SeaplaneAccountLoginArgs),
}

#[derive(Parser)]
pub struct SeaplaneAccountLoginArgs;

#[derive(Parser)]
pub struct SeaplaneAccountLogoutArgs;
