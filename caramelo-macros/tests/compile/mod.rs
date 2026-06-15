#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/test-dry-match-no-variable.rs");
    t.compile_fail("tests/ui/test-dry-match-no-type.rs");
    t.compile_fail("tests/ui/test-dry-match-no-block.rs");
    t.compile_fail("tests/ui/test-dry-match-no-acessor.rs");
    t.compile_fail("tests/ui/test-dry-match-no-reference.rs");
    t.compile_fail("tests/ui/test-dry-match-no-condition.rs");
}
