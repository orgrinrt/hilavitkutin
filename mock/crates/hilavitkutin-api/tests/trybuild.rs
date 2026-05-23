//! trybuild test harness.
//!
//! Each fixture in `tests/trybuild/` exercises a compile-error
//! scenario. The expected error message lives in the matching
//! `.stderr` file. Run with `cargo test --test trybuild`.

#[test]
fn compile_error_fixtures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/trybuild/double_derive_runcfg.rs");
}
