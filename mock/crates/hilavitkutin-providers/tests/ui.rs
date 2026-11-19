//! Compile-time refusals of the providers crate.

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*_fail.rs");
}
