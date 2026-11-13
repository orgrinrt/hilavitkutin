//! Each failing fixture beside its twin, which differs in the offending token only.

#[test]
fn pairs() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
