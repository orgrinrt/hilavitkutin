//! Runtime checks of the shipped default interner, `StringInterner` over
//! `MemoryArena`, against the laws the request names.
//!
//! Each test states the law it asks and asserts it. A red here is the finding.

use hilavitkutin_providers::{InternerApi, MemoryArena, default_interner};
use hilavitkutin_str::{ArenaInterner, Str, StrDomain};
use hilavitkutin_sym::Interner;
use notko::Maybe;

/// Law (hilavitkutin-sym `Interner` doc, src/interner.rs:12): equal values give
/// equal handles. Through the default interner a runtime string interned twice
/// must come back as one handle.
#[test]
fn law_equal_values_give_equal_handles_through_the_default_interner() {
    let interner = default_interner::<256, 16>();
    let a: Str = InternerApi::intern(&interner, "probe-runtime-dup");
    let b: Str = InternerApi::intern(&interner, "probe-runtime-dup");
    assert!(a.is_runtime().0, "the string is not in the const section");
    assert_eq!(a, b, "equal strings must give equal handles");
}

/// The same law, asked through the generic sym-core trait the string domain
/// implements.
#[test]
fn law_equal_values_give_equal_syms_through_the_sym_trait() {
    let interner = default_interner::<256, 16>();
    let a = Interner::<StrDomain>::intern(&interner, "probe-sym-dup");
    let b = Interner::<StrDomain>::intern(&interner, "probe-sym-dup");
    assert_eq!(a, b, "equal strings must give equal syms");
}

/// Observation, not a law: the arena alone. Each repeat takes a new entry and
/// new bytes, so a repeated string drains the capacity.
#[test]
fn observe_the_arena_spends_an_entry_and_bytes_per_repeat() {
    let arena: MemoryArena<64, 4> = MemoryArena::new();
    let ids = [
        arena.arena_intern("same"),
        arena.arena_intern("same"),
        arena.arena_intern("same"),
        arena.arena_intern("same"),
        arena.arena_intern("same"),
    ];
    std::println!("ids for five interns of one string: {ids:?}");
    assert_eq!(
        ids[4],
        u32::MAX,
        "the fifth repeat overflows a four-entry arena"
    );
}

/// The overflow sentinel as the handle carries it. `arena_intern` returns
/// `u32::MAX`; `StringInterner::intern` masks it into the 28-bit id field, so
/// the id the handle carries is `0x0FFF_FFFF`, which is not the sentinel the
/// arena's resolve checks for.
#[test]
fn observe_the_sentinel_does_not_survive_the_28_bit_handle() {
    let interner = default_interner::<4, 2>();
    let first = InternerApi::intern(&interner, "abcd"); // fills the four bytes
    let over = InternerApi::intern(&interner, "efgh"); // overflows
    std::println!(
        "first id {:#x}, overflow id {:#x}, sentinel {:#x}",
        first.id().to_raw(),
        over.id().to_raw(),
        u32::MAX
    );
    assert_eq!(over.id().to_raw(), 0x0FFF_FFFF, "the masked sentinel");
    assert_ne!(
        over.id().to_raw(),
        u32::MAX,
        "the arena never sees its sentinel again"
    );
    // Law (str `resolve` doc, src/interner.rs:101-108): a runtime handle the
    // interner handed out resolves. The id indexes a two-entry table at
    // 268435455, so resolving it is out of bounds. Not called here; see
    // `resolve_of_the_overflow_handle` below, run alone.
}

/// The resolve of an overflow handle, isolated so its outcome is its own. Run
/// with `--ignored`; under Miri it reports the out-of-bounds read.
#[test]
#[ignore = "reads out of bounds by construction; run alone, ideally under Miri"]
fn resolve_of_the_overflow_handle() {
    let interner = default_interner::<4, 2>();
    let _ = InternerApi::intern(&interner, "abcd");
    let over = InternerApi::intern(&interner, "efgh");
    match InternerApi::resolve(&interner, over) {
        Maybe::Is(s) => std::println!("resolved to {} bytes", s.len()),
        Maybe::Isnt => std::println!("resolved to nothing"),
    }
}

/// Two different strings that both overflow come back as one handle, so the
/// overflow folds distinct values onto one identity.
#[test]
fn observe_two_overflowed_strings_share_one_handle() {
    let interner = default_interner::<4, 2>();
    let _ = InternerApi::intern(&interner, "abcd");
    let x = InternerApi::intern(&interner, "xxxx");
    let y = InternerApi::intern(&interner, "yyyy");
    assert_eq!(x, y, "two distinct overflowed strings share a handle");
}
