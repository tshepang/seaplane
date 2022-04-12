#[test]
fn cli_ui_tests() {
    let t = trycmd::TestCases::new();
    t.case("tests/ui/**/*.md");
}
