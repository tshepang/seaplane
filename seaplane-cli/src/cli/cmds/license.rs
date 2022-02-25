use anyhow::Result;
use clap::Parser;

use crate::Ctx;

static THIRD_PARTY_LICENSES: &str = include_str!("../../../../share/third_party_licenses.md");

// @TODO @SIZE this str is ~11.3kb, it can be stored compressed at ~4kb. However that would
// require a code to do the compression/decompression which is larger than the 7.3kb savings. There
// are other locations in the code may benefit as well; if the uncompressed sum of those becomes
// greater than code required to do the compression, we may look at compressing these large strings
// to keep the binary size minimal.
static SELF_LICENSE: &str = include_str!("../../../../LICENSE");

#[derive(Parser)]
pub struct SeaplaneLicenseArgs {
    /// Display a list of third party libraries and their licenses
    #[clap(long)]
    third_party: bool,
}

impl SeaplaneLicenseArgs {
    pub fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        if self.third_party {
            println!("{}", THIRD_PARTY_LICENSES);
        } else {
            println!("{}", SELF_LICENSE);
        }

        Ok(())
    }
}
