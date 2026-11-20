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

/// Interned through the provider trait, where the test's arena has room.
fn interned(interner: &impl InternerApi, s: &str) -> Str {
    InternerApi::intern(interner, s).expect("the arena has room for this string")
}

#[test]
fn equal_strings_give_equal_handles_through_the_provider_trait() {
    let interner = default_interner::<256, 16>();
    let a = interned(&interner, "law-runtime-dup");
    let b = interned(&interner, "law-runtime-dup");
    let other = interned(&interner, "law-runtime-other");
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
    assert!(a.is());
    assert_eq!(a, b);
    assert_ne!(a, other);
}

/// The empty string is a const string where the linker section is read; on
/// any target it interns to one handle and resolves to itself.
#[test]
fn the_empty_string_interns_to_one_handle() {
    let interner = default_interner::<64, 4>();
    let a = interned(&interner, "");
    let b = interned(&interner, "");
    assert_eq!(a, b);
    resolves_to(&interner, a, "");
}

/// A handle the arena did not issue resolves to none of the given held
/// strings. Callers pass non-empty ones: the arena answers such a handle
/// with `""`, which aliases a held `""` (the red below).
fn aliases_none_of(interner: &impl InternerApi, h: Str, held: &[&str]) {
    if let Maybe::Is(s) = InternerApi::resolve(interner, h) {
        for want in held {
            assert_ne!(s, *want, "an unissued handle resolved to a held string");
        }
    }
}

/// A string past the arena's bytes is refused, and every handle issued
/// before still resolves and still interns to itself.
#[test]
fn an_overflowing_intern_is_refused_and_the_earlier_handles_hold() {
    let interner = default_interner::<8, 2>();
    let first = interned(&interner, "abcd");
    let second = interned(&interner, "efgh");
    assert!(InternerApi::intern(&interner, "ijkl").isnt());
    resolves_to(&interner, first, "abcd");
    resolves_to(&interner, second, "efgh");
    assert_eq!(interned(&interner, "abcd"), first);
}

/// An id at or past the live count reads nothing outside the arena and
/// aliases none of its non-empty strings, whichever arena issued it.
#[test]
fn a_handle_past_the_live_count_aliases_no_held_non_empty_string() {
    let first = default_interner::<64, 4>();
    let _ = interned(&first, "foreign-0");
    let past = interned(&first, "foreign-1");
    let second: StringInterner<MemoryArena<64, 2>> = StringInterner::new(MemoryArena::new());
    let _ = interned(&second, "local");
    aliases_none_of(&second, past, &["local"]);
}

/// The law over an overflowing arena: two different strings that do not fit
/// are both refused, so no handle is shared between them.
#[test]
fn two_overflowing_strings_are_both_refused() {
    let interner = default_interner::<4, 4>();
    let _ = interned(&interner, "abcd");
    assert!(InternerApi::intern(&interner, "efgh").isnt());
    assert!(InternerApi::intern(&interner, "ijkl").isnt());
    assert!(Interner::<StrDomain>::intern(&interner, "efgh").isnt());
}

/// Out of entry slots with bytes to spare refuses the same way as out of
/// bytes: the third distinct string is refused, the first two hold.
#[test]
fn running_out_of_entry_slots_refuses_like_running_out_of_bytes() {
    let interner = default_interner::<64, 2>();
    let first = interned(&interner, "ab");
    let second = interned(&interner, "cd");
    assert!(InternerApi::intern(&interner, "ef").isnt());
    resolves_to(&interner, first, "ab");
    resolves_to(&interner, second, "cd");
    assert_eq!(interned(&interner, "cd"), second);
}

/// `intern_static` and `intern` of one runtime string give one handle.
#[test]
fn intern_static_deduplicates_through_the_default_interner() {
    let interner = default_interner::<64, 4>();
    let a = interner.intern_static("static-runtime").expect("interned");
    let b = interned(&interner, "static-runtime");
    assert!(a.is_runtime().0);
    assert_eq!(a, b);
    resolves_to(&interner, a, "static-runtime");
}

/// An arena with no bytes stores no non-empty string: it refuses one, and
/// the empty string, which takes no bytes, still gets a handle of its own.
#[test]
fn an_arena_with_no_bytes_refuses_every_non_empty_string() {
    let interner: StringInterner<MemoryArena<0, 4>> = StringInterner::new(MemoryArena::new());
    assert!(InternerApi::intern(&interner, "x").isnt());
    let empty = interned(&interner, "");
    resolves_to(&interner, empty, "");
}

/// An id the arena did not issue, the one past the live count, does not
/// alias a held empty string.
#[test]
#[ignore = "catalogue: an unissued id resolves to the empty string, which aliases a held one; needs a resolve that can say it holds nothing; tracked #evict_str_turns_an_unresolvable_runtime_str_into_the_hash_of"]
fn an_unissued_id_does_not_alias_a_held_empty_string() {
    let arena: MemoryArena<16, 4> = MemoryArena::new();
    let empty = arena.arena_intern("");
    let ab = arena.arena_intern("ab");
    assert_ne!(arena.arena_resolve(ab + 1), arena.arena_resolve(empty));
}
