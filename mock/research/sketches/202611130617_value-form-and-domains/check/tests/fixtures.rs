//! What a runner refuses, as a pair: the failing fixture under `ui/fail/` with
//! its committed diagnostic, and its twin under `ui/pass/`, built and run.

#[test]
fn a_block_unit_runs_only_under_the_runner_it_names() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
