use std::process::Command;

#[cfg_attr(feature = "ui_tests", test)]
fn cli_ui_tests() {
    let git_hash = String::from_utf8(
        Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    let pkg_ver = format!("v{} ({})", env!("CARGO_PKG_VERSION"), &git_hash[..8]);

    let t = trycmd::TestCases::new();
    t.case("tests/ui/**/*.md");
    t.case("tests/ui/**/*.toml");
    t.insert_var("[PKGVER]", pkg_ver).unwrap();
}

//
// The below tests aren't gated by feature = "ui_tests" because they are simply, "Do the CLI
// arguments we've set up have the semantics we expect" and are not using trycmd
//
// Additionally, we don't care about the output, just whether or not a run failed. These tests
// ensure as we change the CLI it maintains the same semantics
//

fn seaplane()

