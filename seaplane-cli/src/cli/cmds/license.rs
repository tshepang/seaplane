use clap::{ArgMatches, Command};

use crate::{cli::CliCommand, error::Result, Ctx};

static THIRD_PARTY_LICENSES: &str = include_str!("../../../../share/third_party_licenses.md");

// @TODO @SIZE this str is ~11.3kb, it can be stored compressed at ~4kb. However that would
// require a code to do the compression/decompression which is larger than the 7.3kb savings. There
// are other locations in the code may benefit as well; if the uncompressed sum of those becomes
// greater than code required to do the compression, we may look at compressing these large strings
// to keep the binary size minimal.
static SELF_LICENSE: &str = include_str!("../../../../LICENSE");

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneLicense;

impl SeaplaneLicense {
    pub fn command() -> Command<'static> {
        Command::new("license")
            .about("Print license information")
            .arg(
                arg!(--("third-party"))
                    .help("Display a list of third party libraries and their licenses"),
            )
    }
}

impl CliCommand for SeaplaneLicense {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        if ctx.args.third_party {
            println!("{THIRD_PARTY_LICENSES}");
        } else {
            println!("{SELF_LICENSE}");
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.args.third_party = matches.contains_id("third-party");
        Ok(())
    }
}
