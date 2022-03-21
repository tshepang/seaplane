use clap::{Arg, ArgMatches, Command};

use crate::{context::Ctx, error::Result};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneImage;

impl SeaplaneImage {
    fn common() -> Command<'static> {
        todo!("SeaplaneImage::into_app")
    }
}
impl CliCommand for SeaplaneImage {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        todo!("SeaplaneImage::run")
    }
    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        todo!("SeaplaneImage::update_ctx")
    }
}
