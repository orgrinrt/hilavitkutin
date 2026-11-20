# doc changelist

Round topic `202611192246-jute_topic.a-handle-names-no-interner.md`, the only flat topic, from the pr-reviewer-senior pass over `work/hila-consumer-surface` at `110ecccc`. It corrects a claim written by the closed round `202611192051-ers6`. No canon row changes; the statement rests on `tenet::an_interned_string_handle_is_session_specific_so_it_is_stored_on_disk_as_its_content_hash`.

## Changes proposed

### 1. The arena contract guards the live count, not the issuer

`mock/crates/hilavitkutin-str/DESIGN.md.tmpl`, "`ArenaInterner`, host-implemented trait": `arena_resolve` of an id at or past the arena's live count returns the empty string and reads nothing outside the arena's storage, and the masked overflow id lands there. The claim that a handle from another interner lands there is cut. The contract says instead that a `Str` names no interner, so a handle is meaningful only to the interner that issued it, and one carried to another resolves to whatever that interner holds at its id, undetected.

### 2. `MemoryArena` says the same

`mock/crates/hilavitkutin-providers/DESIGN.md.tmpl`, "`MemoryArena<BYTES, ENTRIES>`": an id at or past the count resolves to the empty string, and a handle another arena issued is not told apart from one of its own. The sentinel paragraph, the `ENTRIES < 2^28` bound and the deduplicating scan are unchanged.

## Not changed

The overflow behaviour stays as designed: the sentinel id from `arena_intern`, the empty string from resolving it. That it breaks the interning law is recorded by the src changelist as tests that fail, under the ticket tracking it, since repairing it needs a signature change already put to op (`where_string_deduplication_lives_in_hilavitkutin_and_what_a`).
