use clap::Parser;

use crate::{error::Result, Ctx};

/// Generate a template skeleton of a Formation
#[derive(Parser)]
pub struct SeaplaneFormationTemplateArgs;

impl SeaplaneFormationTemplateArgs {
    pub fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFormationTemplateArgs::run")
    }
}
