use anyhow::Result;
use clap::{AppSettings, Parser, Subcommand};

use crate::Ctx;

/// Used for local development as an internal command
#[derive(Parser)]
#[clap(setting(AppSettings::Hidden))]
pub struct SeaplaneDevArgs {
    // Unlike normal subcommands we have to wrap this one with an Option so that clap doesn't
    // preempt us and display a "error: no subcommand found" when in fact the dev subcommand
    // should be disabled. This allows the parsing to continue, and we handle the "error: the 'dev'
    // command is disabled" error manually in the SeaplaneArgs::run method.
    #[clap(subcommand)]
    cmd: Option<SeaplaneDevCmds>,
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
