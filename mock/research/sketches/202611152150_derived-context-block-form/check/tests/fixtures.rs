//! The compile-fail fixtures: the twins under `ui/fail`, each beside a twin
//! under `ui/pass` differing in the offending token, and the attempts that
//! did not compile under `ui/attempts`. `TRYBUILD=overwrite` rewrites the
//! recorded diagnostics.

#[test]
fn twins() {
    let t = trybuild::TestCases::new();
    t.compile_fail("ui/fail/*.rs");
    t.pass("ui/pass/*.rs");
}

#[test]
fn attempts() {
    let t = trybuild::TestCases::new();
    t.compile_fail("ui/attempts/*.rs");
}
