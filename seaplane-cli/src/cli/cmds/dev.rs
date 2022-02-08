use anyhow::Result;
use clap::{AppSettings, Parser, Subcommand};

use crate::Ctx;

/// Used for local development as an internal command
#[derive(Parser)]
#[clap(setting(AppSettings::Hidden))]
pub struct SeaplaneDevArgs {
    #[clap(subcommand)]
    cmd: SeaplaneDevCmds,
}

impl SeaplaneDevArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneDevArgs::run")
    }
}

#[derive(Subcommand)]
pub enum SeaplaneDevCmds {
    Jwt(SeaplaneDevJwtArgs),
}

#[derive(Parser)]
pub struct SeaplaneDevJwtArgs;

impl SeaplaneDevJwtArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneDevJwtArgs::run")
    }
}
