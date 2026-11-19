//! The interning law asked of the shipped arena: equal strings give equal
//! handles, and a resolve never reads outside the arena's table.

#![no_std]

use hilavitkutin_providers::{InternerApi, MemoryArena, default_interner};
use hilavitkutin_str::{ArenaInterner, Str, StrDomain, StringInterner};
use hilavitkutin_sym::Interner;
use notko::Maybe;

fn resolves_to(interner: &impl InternerApi, h: Str, want: &str) {
    match InternerApi::resolve(interner, h) {
        Maybe::Is(s) => assert_eq!(s, want),
        Maybe::Isnt => panic!("expected the handle to resolve to {want:?}"),
    }
}

#[test]
fn the_arena_gives_a_repeated_string_its_first_id() {
    let arena: MemoryArena<64, 4> = MemoryArena::new();
    let a = arena.arena_intern("same");
    let b = arena.arena_intern("same");
    let c = arena.arena_intern("other");
    let d = arena.arena_intern("same");
    assert_eq!(a, b);
    assert_eq!(a, d);
    assert_ne!(a, c);
    assert_eq!(arena.arena_resolve(a), "same");
    assert_eq!(arena.arena_resolve(c), "other");
}

/// A prefix and a string of equal length but other bytes are not the string.
#[test]
fn the_arena_tells_a_prefix_and_an_equal_length_string_apart() {
    let arena: MemoryArena<64, 8> = MemoryArena::new();
    let long = arena.arena_intern("abcd");
    let prefix = arena.arena_intern("abc");
    let same_len = arena.arena_intern("abce");
    assert_ne!(long, prefix);
    assert_ne!(long, same_len);
    assert_ne!(prefix, same_len);
    assert_eq!(arena.arena_intern("abc"), prefix);
    assert_eq!(arena.arena_resolve(same_len), "abce");
}

/// One entry slot holds one string however often it is interned.
#[test]
fn a_one_entry_arena_takes_a_repeat_without_overflowing() {
    let arena: MemoryArena<16, 1> = MemoryArena::new();
    let a = arena.arena_intern("only");
    for _ in 0 .. 4 {
        assert_eq!(arena.arena_intern("only"), a);
    }
    assert_ne!(a, u32::MAX);
}

#[test]
fn equal_strings_give_equal_handles_through_the_provider_trait() {
    let interner = default_interner::<256, 16>();
    let a = InternerApi::intern(&interner, "law-runtime-dup");
    let b = InternerApi::intern(&interner, "law-runtime-dup");
    let other = InternerApi::intern(&interner, "law-runtime-other");
    assert!(a.is_runtime().0);
    assert_eq!(a, b);
    assert_ne!(a, other);
    resolves_to(&interner, a, "law-runtime-dup");
    resolves_to(&interner, other, "law-runtime-other");
}

#[test]
fn equal_strings_give_equal_syms_through_the_sym_trait() {
    let interner = default_interner::<256, 16>();
    let a = Interner::<StrDomain>::intern(&interner, "law-sym-dup");
    let b = Interner::<StrDomain>::intern(&interner, "law-sym-dup");
    let other = Interner::<StrDomain>::intern(&interner, "law-sym-other");
    assert_eq!(a, b);
    assert_ne!(a, other);
}

/// The empty string is a const string where the linker section is read; on
/// any target it interns to one handle and resolves to itself.
#[test]
fn the_empty_string_interns_to_one_handle() {
    let interner = default_interner::<64, 4>();
    let a = InternerApi::intern(&interner, "");
    let b = InternerApi::intern(&interner, "");
    assert_eq!(a, b);
    resolves_to(&interner, a, "");
}

/// The overflow handle reaches the arena masked to the 28-bit id field. It
/// resolves to the empty string, and every handle issued before still resolves.
#[test]
fn the_overflow_handle_resolves_empty_and_the_earlier_handles_hold() {
    let interner = default_interner::<8, 2>();
    let first = InternerApi::intern(&interner, "abcd");
    let second = InternerApi::intern(&interner, "efgh");
    let over = InternerApi::intern(&interner, "ijkl");
    assert_ne!(over, first);
    assert_ne!(over, second);
    resolves_to(&interner, over, "");
    resolves_to(&interner, first, "abcd");
    resolves_to(&interner, second, "efgh");
    assert_eq!(InternerApi::intern(&interner, "abcd"), first);
}

/// A handle issued by one arena is not another's to read: a smaller arena
/// resolves it to the empty string.
#[test]
fn a_handle_from_a_larger_arena_resolves_empty_in_a_smaller_one() {
    let large = default_interner::<256, 16>();
    let mut last = InternerApi::intern(&large, "foreign-0");
    for s in ["foreign-1", "foreign-2", "foreign-3", "foreign-4"] {
        last = InternerApi::intern(&large, s);
    }
    let small: StringInterner<MemoryArena<64, 2>> = StringInterner::new(MemoryArena::new());
    let _ = InternerApi::intern(&small, "local");
    resolves_to(&small, last, "");
}
