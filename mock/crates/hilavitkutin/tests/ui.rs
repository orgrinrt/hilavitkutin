//! Compile-fail fixtures for the engine builder and the unit context.
//!
//! Every fixture is a pair: the `_fail.rs` file that must be refused, with the
//! compiler's diagnostic committed beside it as `.stderr`, and the `_pass.rs`
//! twin that differs only in the offending token and must compile. The twin is
//! what shows the refusal is about that token and not about the fixture's
//! scaffolding.
//!
//! The `.stderr` files carry the compiler's words verbatim. Regenerate them
//! with `TRYBUILD=overwrite` when the diagnostic is meant to change, and read
//! the diff.

#[test]
fn compile_fail_fixtures() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/unregistered_store_pass.rs");
    t.compile_fail("tests/ui/unregistered_store_fail.rs");
    t.pass("tests/ui/undeclared_read_pass.rs");
    t.compile_fail("tests/ui/undeclared_read_fail.rs");
    t.pass("tests/ui/unmapped_unit_fused_pass.rs");
    t.compile_fail("tests/ui/unmapped_unit_fused_fail.rs");
    t.pass("tests/ui/assert_out_names_no_refusal_pass.rs");
    t.compile_fail("tests/ui/assert_out_names_no_refusal_fail.rs");
}

#[test]
fn a_replaced_value_is_read_by_the_next_run() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/replace_value_installs_pass.rs");
}
