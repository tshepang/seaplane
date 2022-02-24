use trycmd::cargo_bin;

#[cfg_attr(feature = "ui_tests", test)]
fn cli_ui_tests() {
    trycmd::TestCases::new().case("tests/ui/**/*.md");
}
