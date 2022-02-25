mod common;
mod copy;
mod create;
mod delete;
mod edit;
mod list;
mod template;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{
    cli::cmds::flight::{
        common::{SeaplaneFlightCommonArgs, IMAGE_SPEC},
        copy::SeaplaneFlightCopyArgs,
        create::SeaplaneFlightCreateArgs,
        delete::SeaplaneFlightDeleteArgs,
        edit::SeaplaneFlightEditArgs,
        list::SeaplaneFlightListArgs,
        template::SeaplaneFlightTemplateArgs,
    },
    context::Ctx,
};

/// Operate on Seaplane Flights (logical containers), which are the core component of Formations
#[derive(Parser)]
pub struct SeaplaneFlightArgs {
    #[clap(subcommand)]
    cmd: SeaplaneFlightCmds,
}

impl SeaplaneFlightArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        use SeaplaneFlightCmds::*;

        match &self.cmd {
            Create(args) => args.run(ctx),
            Copy(args) => args.run(ctx),
            Edit(args) => args.run(ctx),
            Delete(args) => args.run(ctx),
            List(args) => args.run(ctx),
            Template(args) => args.run(ctx),
        }
    }
}

#[derive(Subcommand)]
pub enum SeaplaneFlightCmds {
    Create(SeaplaneFlightCreateArgs),
    Copy(SeaplaneFlightCopyArgs),
    Edit(SeaplaneFlightEditArgs),
    Delete(SeaplaneFlightDeleteArgs),
    List(SeaplaneFlightListArgs),
    Template(SeaplaneFlightTemplateArgs),
}
