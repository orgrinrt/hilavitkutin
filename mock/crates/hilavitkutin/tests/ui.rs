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
    t.pass("tests/ui/unregistered_wrapped_store_pass.rs");
    t.compile_fail("tests/ui/unregistered_wrapped_store_fail.rs");
    t.pass("tests/ui/undeclared_read_pass.rs");
    t.compile_fail("tests/ui/undeclared_read_fail.rs");
    t.pass("tests/ui/unmapped_unit_fused_pass.rs");
    t.compile_fail("tests/ui/unmapped_unit_fused_fail.rs");
    t.pass("tests/ui/appending_unit_fused_pass.rs");
    t.compile_fail("tests/ui/appending_unit_fused_fail.rs");
    t.compile_fail("tests/ui/buffered_unit_fused_fail.rs");
    t.pass("tests/ui/assert_out_names_no_refusal_pass.rs");
    t.compile_fail("tests/ui/assert_out_names_no_refusal_fail.rs");
    t.compile_fail("tests/ui/memory_provider_safe_impl_fail.rs");
    // The engine's seal: each codegen trait it closes has no twin, since
    // the refusal is of the impl itself, and the seal's own module is
    // private, which is what keeps the two refusals standing.
    t.compile_fail("tests/ui/dispatch_codegen_outside_the_engine_fail.rs");
    t.compile_fail("tests/ui/fiber_shape_outside_the_engine_fail.rs");
    t.compile_fail("tests/ui/engine_seal_is_private_fail.rs");
    // The executor is open: a consumer's own builds.
    t.pass("tests/ui/executor_outside_the_engine_pass.rs");
    // A zero divisor in the ceiling division fails the build.
    t.compile_fail("tests/ui/ceiling_div_by_zero_fail.rs");
}

#[test]
fn a_replaced_value_is_read_by_the_next_run() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/replace_value_installs_pass.rs");
}

// The host path takes a column store only: an `Accum`, a `Virtual`, a
// `Resource`, a wrapper such as `Written` or a column the builder never
// registered is refused in `import` and `dump`.
#[test]
fn the_host_path_takes_only_a_registered_column() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/host_import_pass.rs");
    for fixture in [
        "accum",
        "accum_dump",
        "virtual",
        "virtual_dump",
        "resource",
        "resource_dump",
        "unregistered",
        "unregistered_dump",
        "written",
        "written_dump",
    ] {
        t.compile_fail(format!("tests/ui/host_import_{fixture}_fail.rs"));
    }
}
