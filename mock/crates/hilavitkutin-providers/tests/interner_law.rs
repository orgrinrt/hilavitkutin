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

/// A `Str` names no interner: a handle carried to another arena resolves to
/// what that arena holds at its id.
#[test]
fn a_handle_carried_to_another_arena_resolves_to_what_that_arena_holds() {
    let first = default_interner::<64, 4>();
    let foreign = InternerApi::intern(&first, "foreign");
    let second = default_interner::<64, 4>();
    let _ = InternerApi::intern(&second, "local");
    resolves_to(&second, foreign, "local");
}

/// An id at or past the live count reads nothing and resolves to the empty
/// string, whichever arena issued it.
#[test]
fn a_handle_past_the_live_count_resolves_empty() {
    let first = default_interner::<64, 4>();
    let _ = InternerApi::intern(&first, "foreign-0");
    let past = InternerApi::intern(&first, "foreign-1");
    let second: StringInterner<MemoryArena<64, 2>> = StringInterner::new(MemoryArena::new());
    let _ = InternerApi::intern(&second, "local");
    resolves_to(&second, past, "");
}

/// The law over an overflowing arena: two different strings keep distinct
/// handles even when neither fits.
#[test]
#[ignore = "catalogue: both overflows get the masked sentinel; needs intern to say the arena is full; tracked #an_overflowing_runtime_intern_breaks_the_interning_law_in_me"]
fn two_overflowing_strings_get_distinct_handles() {
    let interner = default_interner::<4, 4>();
    let _ = InternerApi::intern(&interner, "abcd");
    let a = InternerApi::intern(&interner, "efgh");
    let b = InternerApi::intern(&interner, "ijkl");
    assert_ne!(a, b);
}

/// The law over an overflowing arena: a handle resolves back to its string.
#[test]
#[ignore = "catalogue: the overflow handle resolves to the empty string; needs intern to say the arena is full; tracked #an_overflowing_runtime_intern_breaks_the_interning_law_in_me"]
fn an_overflow_handle_resolves_to_its_string() {
    let interner = default_interner::<4, 4>();
    let _ = InternerApi::intern(&interner, "abcd");
    let over = InternerApi::intern(&interner, "efgh");
    resolves_to(&interner, over, "efgh");
}

/// Out of entry slots with bytes to spare overflows the same way as out of
/// bytes: the third distinct string gets the overflow handle.
#[test]
fn running_out_of_entry_slots_overflows_like_running_out_of_bytes() {
    let interner = default_interner::<64, 2>();
    let first = InternerApi::intern(&interner, "ab");
    let second = InternerApi::intern(&interner, "cd");
    let over = InternerApi::intern(&interner, "ef");
    assert_ne!(over, first);
    assert_ne!(over, second);
    resolves_to(&interner, over, "");
    resolves_to(&interner, first, "ab");
    resolves_to(&interner, second, "cd");
    assert_eq!(InternerApi::intern(&interner, "cd"), second);
}

/// `intern_static` and `intern` of one runtime string give one handle.
#[test]
fn intern_static_deduplicates_through_the_default_interner() {
    let interner = default_interner::<64, 4>();
    let a = interner.intern_static("static-runtime");
    let b = InternerApi::intern(&interner, "static-runtime");
    assert!(a.is_runtime().0);
    assert_eq!(a, b);
    resolves_to(&interner, a, "static-runtime");
}

/// An arena with no bytes stores no non-empty string: it overflows, and the
/// handle resolves to the empty string.
#[test]
fn an_arena_with_no_bytes_overflows_every_non_empty_string() {
    let interner: StringInterner<MemoryArena<0, 4>> = StringInterner::new(MemoryArena::new());
    let a = InternerApi::intern(&interner, "x");
    resolves_to(&interner, a, "");
    resolves_to(&interner, InternerApi::intern(&interner, ""), "");
}
