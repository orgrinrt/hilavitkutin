//! `trybuild` harness for `#[export_extension]`.
//!
//! Positive fixtures under `tests/fixtures/pass/*.rs` are expected to
//! compile; negative fixtures under `tests/fixtures/fail/*.rs` are
//! expected to fail with a stable error.

#[test]
fn trybuild_fixtures() {
    let t = trybuild::TestCases::new();
    t.pass("tests/fixtures/pass/*.rs");
    t.compile_fail("tests/fixtures/fail/*.rs");
}
