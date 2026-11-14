//! The refusals, each as a pair: the failing fixture under `ui/fail/` with its
//! committed diagnostic, and its twin under `ui/pass/`, which differs in the
//! offending token only and is built and run.

#[test]
fn every_refusal_holds_and_every_twin_builds() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
