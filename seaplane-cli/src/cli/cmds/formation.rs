use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::Ctx;

/// Operate on Seaplane Formations
#[derive(Parser)]
pub struct SeaplaneFormationArgs {
    #[clap(subcommand)]
    cmd: SeaplaneFormationCmds,
}

impl SeaplaneFormationArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        use SeaplaneFormationCmds::*;

        self.update_ctx(ctx)?;

        match &self.cmd {
            _ => unimplemented!(),
        }
    }

    fn update_ctx(&self, _ctx: &mut Ctx) -> Result<()> {
        Ok(())
    }
}

#[derive(Subcommand)]
pub enum SeaplaneFormationCmds {}
