//! trybuild test harness.
//!
//! Each fixture in `tests/trybuild/` exercises a compile-error
//! scenario. The expected error message lives in the matching
//! `.stderr` file. Run with `cargo test --test trybuild`.

#[test]
fn compile_error_fixtures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/trybuild/double_derive_runcfg.rs");
    // The members and what a list of them refuses.
    t.compile_fail("tests/trybuild/differs_incomparable.rs");
    t.compile_fail("tests/trybuild/holds_all_missing.rs");
    // The positional accessors reach a bare `Column` and nothing buffered.
    t.compile_fail("tests/trybuild/read_buffered_by_position.rs");
    t.compile_fail("tests/trybuild/write_buffered_by_position.rs");
    // What a value function may read and return.
    t.compile_fail("tests/trybuild/map_bare_incomparable.rs");
    t.compile_fail("tests/trybuild/map_reads_an_accumulator.rs");
    t.compile_fail("tests/trybuild/read_undeclared.rs");
    // What a buffered column is registered with. The refusal of `start` on
    // an element type with neither a constructor nor a default is a
    // `compile_fail` doctest on `Buffered::start`: the compiler's message
    // there points into the checkout of a dependency, which a `.stderr`
    // file cannot hold across machines.
    t.compile_fail("tests/trybuild/buffered_marker_registered.rs");
}
