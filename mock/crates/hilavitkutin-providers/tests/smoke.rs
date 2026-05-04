//! Smoke test for hilavitkutin-providers v0.

#![no_std]

use hilavitkutin_providers::{InternerApi, MemoryArena, default_interner};
use hilavitkutin_str::ArenaInterner;
use notko::Maybe;

#[test]
fn memory_arena_intern_and_resolve_round_trip() {
    let arena: MemoryArena<1024, 32> = MemoryArena::new();
    let id_a = arena.arena_intern("alpha");
    let id_b = arena.arena_intern("beta-gamma");
    assert_ne!(id_a, id_b);
    assert_eq!(arena.arena_resolve(id_a), "alpha");
    assert_eq!(arena.arena_resolve(id_b), "beta-gamma");
}

#[test]
fn memory_arena_byte_overflow_returns_sentinel() {
    let arena: MemoryArena<8, 32> = MemoryArena::new();
    let id_a = arena.arena_intern("12345"); // 5 bytes, fits.
    let id_b = arena.arena_intern("67890"); // 5 bytes, would overflow 8 cap.
    assert_ne!(id_a, u32::MAX);
    assert_eq!(id_b, u32::MAX);
}

#[test]
fn memory_arena_entry_overflow_returns_sentinel() {
    let arena: MemoryArena<1024, 2> = MemoryArena::new();
    let _ = arena.arena_intern("a");
    let _ = arena.arena_intern("b");
    let id_overflow = arena.arena_intern("c");
    assert_eq!(id_overflow, u32::MAX);
}

#[test]
fn default_interner_intern_and_resolve_round_trip() {
    let interner = default_interner::<2048, 64>();
    let h_a = InternerApi::intern(&interner, "alpha-string");
    let h_b = InternerApi::intern(&interner, "beta-string");
    match InternerApi::resolve(&interner, h_a) {
        Maybe::Is(s) => assert_eq!(s, "alpha-string"),
        Maybe::Isnt => panic!("alpha resolution failed"),
    }
    match InternerApi::resolve(&interner, h_b) {
        Maybe::Is(s) => assert_eq!(s, "beta-string"),
        Maybe::Isnt => panic!("beta resolution failed"),
    }
}
