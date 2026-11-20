//! Compile-time refusals of the providers crate.
//!
//! Each fixture runs on the builds where its refusal holds: the arena's
//! entry bound where `portable` carries the arena, and the unnamed
//! implementations where their feature is off.

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    #[cfg(feature = "portable")]
    t.compile_fail("tests/ui/*_fail.rs");
    #[cfg(not(feature = "portable"))]
    t.compile_fail("tests/ui/default/memory_arena_unnamed_without_portable_fail.rs");
    #[cfg(not(feature = "static"))]
    t.compile_fail("tests/ui/default/static_bump_memory_unnamed_without_static_fail.rs");
}
