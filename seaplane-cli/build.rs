// The build script is used to get the git short hash of the repository at compile time. This hash
// is then stored in the env var SEAPLANE_GIT_HASH which we use later to insert into the --version
// flag. In this manner we can tell exactly which commit a particular binary was built from.

use const_format::concatcp;
use std::process::Command;

#[cfg(feature = "unstable")]
const UNSTABLE: &str = "+unstable";
#[cfg(not(feature = "unstable"))]
const UNSTABLE: &str = "";

#[cfg(feature = "color")]
const COLOR: &str = "+color";
#[cfg(not(feature = "color"))]
const COLOR: &str = "";

fn main() {
    // If `git` is installed and located in `$PATH` of the build machine, it uses that to determine
    // the latest commit hash. Otherwise uses the string UNKNOWN.
    //
    // This could be changed to a cargo compile-time-feature in the future if there are scenarios
    // where you either know `git` isn't available, or you don't wish to have a hash in the
    // version.
    let git_hash: String =
        if let Ok(output) = Command::new("git").args(&["rev-parse", "HEAD"]).output() {
            String::from_utf8(output.stdout).unwrap_or_else(|_| "UNKNOWN  ".into())
        } else {
            "UNKNOWN  ".into()
        };

    println!(
        "cargo:rustc-env=SEAPLANE_GIT_HASH=v{} ({})",
        env!("CARGO_PKG_VERSION"),
        &git_hash[..8]
    );
    println!(
        "cargo:rustc-env=SEAPLANE_BUILD_FEATURES={}",
        concatcp!(COLOR, " ", UNSTABLE)
    );
}
