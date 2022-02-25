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

    let pkg_ver = format!("v{} ({})", env!("CARGO_PKG_VERSION"), &git_hash[..10]);

    let t = trycmd::TestCases::new();
    t.case("tests/ui/**/*.md");
    t.case("tests/ui/**/*.toml");
    t.insert_var("[PKGVER]", pkg_ver).unwrap();
}
