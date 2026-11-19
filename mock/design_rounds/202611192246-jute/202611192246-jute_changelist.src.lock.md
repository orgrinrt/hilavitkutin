# src changelist

Round topic `202611192246-jute_topic.a-handle-names-no-interner.md`, against the locked doc changelist of this round. No signature changes; rustdoc and tests only.

## Changes proposed

### 1. The `ArenaInterner` rustdoc says what resolve guards

`mock/crates/hilavitkutin-str/src/interner.rs`, the trait doc and `arena_resolve`'s doc: an id at or past the arena's live count resolves to the empty string and reads nothing outside the arena, the masked overflow id among them; a `Str` names no interner, so a handle carried to another resolves to what that one holds at its id. The phrase "an id this arena did not issue" and the claim about a handle from another interner are cut.

### 2. The `MemoryArena` rustdoc and resolve comment say the same

`mock/crates/hilavitkutin-providers/src/interner.rs`: the type doc drops "an id another arena issued" from what resolves empty and says a handle from another arena is not told apart from one of its own; the comment in `arena_resolve` says ids past the live count are refused, not ids it did not issue.

### 3. The tests ask what holds, and the law's breaks are red

`mock/crates/hilavitkutin-providers/tests/interner_law.rs`:

- `a_handle_from_a_larger_arena_resolves_empty_in_a_smaller_one` is deleted; its input was the one case where the cut claim held.
- `a_handle_carried_to_another_arena_resolves_to_what_that_arena_holds` asserts what the design now says: a first arena's id-0 handle resolves, in a second arena holding one string, to that string.
- `a_handle_past_the_live_count_resolves_empty` keeps the bound check asked: an id past the count of the second arena resolves to the empty string.
- `two_overflowing_strings_get_distinct_handles` and `an_overflow_handle_resolves_to_its_string` state the interning law over an overflowing arena and fail today; each is `#[ignore]` naming ticket `an_overflowing_runtime_intern_breaks_the_interning_law_in_me`.
- `mock/catalogue.md` gains the ticket's row, the ledger the `catalogue-ignore` lint reads.
- `running_out_of_entry_slots_overflows_like_running_out_of_bytes` overflows a `default_interner::<64, 2>` on its third distinct string.
- `intern_static_deduplicates_through_the_default_interner` asks `intern_static` and `intern` of one runtime string give one handle.
- `an_arena_with_no_bytes_overflows_every_non_empty_string` asks a `MemoryArena<0, 4>`: a non-empty string gets the overflow handle and resolves to the empty string.

## Not changed

The overflow behaviour and the `ENTRIES < 2^28` bound. That `2^28 - 1` entries still builds is not asked: a const arena of that size is gigabytes of entry table for the compiler to evaluate. `evict_str` turning an unresolvable runtime handle into the hash of `""` is ticket `evict_str_turns_an_unresolvable_runtime_str_into_the_hash_of`, for the round that lets `arena_resolve` say it holds nothing.
