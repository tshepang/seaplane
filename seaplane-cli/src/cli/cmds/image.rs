use clap::{ArgMatches, Command};

use crate::{cli::CliCommand, context::Ctx, error::Result};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneImage;

impl SeaplaneImage {
    pub fn command() -> Command { Command::new("image").visible_alias("img") }
}
impl CliCommand for SeaplaneImage {
    fn run(&self, _ctx: &mut Ctx) -> Result<()> { todo!("SeaplaneImage::run") }
    fn update_ctx(&self, _matches: &ArgMatches, _ctx: &mut Ctx) -> Result<()> {
        todo!("SeaplaneImage::update_ctx")
    }
}
