// The build script is used inject compile time info into a --version flag. The information
// gathered is:
//
// - the git short hash of the repository
// - any cargo feature flags used
// - the version of the package as taken from Cargo.toml
//
// In this manner we can tell exactly how a particular binary was built.

use std::process::Command;

use const_format::concatcp;

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
    let commit_id = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|output| !output.stdout.is_empty())
        .map(|output| String::from_utf8(output.stdout).unwrap())
        .unwrap_or_else(|| String::from("UNKNOWN"));

    println!(
        "cargo:rustc-env=SEAPLANE_VER_WITH_HASH=v{} ({})",
        env!("CARGO_PKG_VERSION"),
        commit_id.trim()
    );
    println!("cargo:rustc-env=SEAPLANE_BUILD_FEATURES={}", concatcp!(COLOR, " ", UNSTABLE));
}
