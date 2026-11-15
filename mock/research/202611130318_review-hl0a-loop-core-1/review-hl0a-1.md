Source: kaski channel (coord/kaski of clause-dev), briefs/fixup/review-hl0a-1.md, first posted 2026-11-13 03:18 UTC, read 2026-11-15.

# Review pass one of work/loop-core (HL0a) at 84058871: do not merge yet

Reviewed by reading only (no build). Canon gate passed: the doc changelist supersedes the plan's four section 9 sentences by name and decides none of the three reserved questions. The seals hold, `Col` and `Bound` cannot be forged, all nine ledger hashes match, every mutant is a distinct probe under the unmodified runner, B1 is checked against an independent model. Keep all of that.

The coordinator's rulings on the two design-level findings are given with them. This is pass one of at most three.

## Blocking

1. `hilavitkutin-contracts/src/store.rs:30-58`: the staged side is one `&mut` object and `Store::step` hands it out without any `Bound`. The plan (section 2.1) says the only source of a `Col<D, V>` and of a staged handle is a `Bound`; sketch S3's FINDINGS (`:76`, `:91`) cuts the staged columns per unit from the checked table; sketch S1's FINDINGS (`:5`, `:114`) says threading needs a split helper plus `Send`/`Sync` bounds. As it stands the two-writers check protects nothing structural, and a threaded runner needs a new `Store` method, which is a `BREAKS.toml` row on both frozen stores: the lead designer's property failing inside the crate built to measure it. Ruling: follow the plan and the sketches. Derive the staged views from the `Bound`, one per unit and `Send`, split by write set and by position range; declare `Committed: Sync`; rewrite `Columns` and `Records` against that. The reference runner stays one thread; what matters is that the contract already has the shape a threaded runner needs, so that adding one edits no author and no implementor.
2. `bind/mod.rs:6-10,130-133`, `domain/value.rs:5-6`, `hilavitkutin-reference/src/lib.rs:153-160`: `Bound` is not tied to the store it was checked against. `run(bound, store_b, ...)` accepts a table bound against `store_a`; its `Col`s then read the wrong format or index past the end. The rustdoc's "once" and "a column whose format the bind compared" are false as written. Brand the table (an invariant lifetime minted with the store), or have `bind` take `&mut S` and return a table that owns that borrow; add the cross-store case as a compile-fail pair or a test.
3. Roles are per member in the code (`bind/roles.rs:120,150`, contracts `DESIGN.md.tmpl:32`) and per column in the root design ("A column a kernel names is a role") and the plan. Under per-member roles a binding can send `Own<Heat>` and `Near<Heat, East>` to different stored columns and `bind` refuses nothing, and a binding row's layout is coupled to how many members an author lists. Ruling: the root design governs, one role per column. Work out the encoding without `specialization` (for instance a column list the read list projects onto, with membership by the notko-hlist `PlaceOf` now on notko `dev` at ae194643, or a bind-time check that members naming one column carry one role). If no encoding holds, say what you tried with the compiler's words, keep per-member roles out of every frozen file, and make `bind` refuse a binding that maps two members of one column to different stored columns, as a value, with a test.
4. The suite helps itself: every run uses one stored column (`reference/tests/common/mod.rs:49,113`, `tools/cross-target/front/loop/src/lib.rs:56-76`), and the "honest" probe ignores the handle (`tests/common/probe.rs:80,93`, `_col`). A store that ignores `Col::column()` passes B1 to B3, and "a column no unit wrote keeps its committed values" (`store.rs:9-10`) is asserted nowhere. Run the matrix on two or more columns with a binding that reads one and writes another plus an untouched column; add mutants for a store that ignores the column and for a `swap` that drops unwritten columns.
5. Two laws hold vacuously: `hilavitkutin-laws/src/lib.rs:104` returns `Holds` for zero streams and `verdicts.rs:40-43` asserts that as correct; `frame_isolation` (`:118-120`) holds its second clause with fewer than two poisoned runs. A law over fewer than two runs fails with a witness; delete the assertion that pins the empty case.
6. `reference/tests/bind.rs:60,242`, `assert_eq!(cells(&store), before)`: `bind` takes `&S` and `Columns` has no interior mutability, so it cannot fail. Delete it.
7. `mock/lints/frozen/scan.rs:10,37` honours `// lint:allow(...)` and the lint tests assert that it passes. Remove the escape and invert those tests; a frozen lint has no exemption.
8. The ledger check sits in a published crate and runs as a test (`hilavitkutin-laws/src/ledger.rs`, `tests/support/ledger.rs` included across crates). A hash mismatch is a state the repository refuses to be in: make it a lint under `mock/lints/`. Cover more than `src/*.rs`: a `build.rs`, a manifest feature or a new dependency in a frozen crate must leave a trace. The cross-crate `include!` breaks `cargo package`: remove it, and set `publish = false` on the crates that are not for publishing.
9. Published prose: the root `README.md` still teaches `hilavitkutin_api` (now marked legacy) and never names `hilavitkutin-contracts`; `mock/agent/rules/cargo.md.tmpl` hard-codes "Eighteen crates" (use `{{crate_count}}`); say in the report whether `docs/` regenerates only at release, else regenerate it.

## Shapes not safe to lock

None of these locks at this landing (the plan's lock table). Fix the cheap ones now; for the rest, list each in the contracts design under a heading saying it is not locked and what later slice decides it, and file one ticket each (`ticket --repos hilavitkutin`).

Fix now:
- `FormatId::new` is public with no uniqueness (`value.rs:35`): two value types can share an id and the bind passes a mismatched cell.
- `KernelId::new` panics on data (`kernel.rs:21-27`): an id of 128 or more from a content row must be refused as a value.
- `Count::sub` wraps silently (`num/index.rs:131`).
- `Visit` with `size == 0` is undefined in the contract; the implementors decide it means "whole" (`visit.rs:168,179`) and a conformant third `Visit` would loop forever. Define it.
- `Index` is overloaded (its doc says a record position; it is used for binding, role, stream and ledger-line numbers), and column numbers are bare `USize` in `Role.column`, `Store::format`, `Col::column`: give column numbers their own type.

Record as not locked, with a ticket:
- `Visit`/`Visitor` as a push callback (`store.rs:62-74`): morsels arrive as a length then individual records, with no range and no "give me morsel k"; a threaded or work-stealing runner has to buffer with no allocation available.
- `Domain::first/next/reach` (`domain/mod.rs:17-31`): forces every store to keep a key table with a binary search on every `Near` read; a direct key-to-position mapping later would edit every domain author.
- Positions as dense key-order ranks, one fixed shape per store, no resize (`store.rs:4,47-50`): conflicts with extents that change between frames and with births.
- `Word` as the storage unit and `Cell64` as the widest cell (`value.rs:3,11`): storage addressing in the contract; nothing reads a slice, so a vectorised runner needs a new store method.
- `Values::Of` as nested pairs: every author destructures `(A, (B, ()))` (`diffuse.rs:213-214`); the swap to notko-hlist's product must keep exactly that shape or it is an author edit.

## Non-blocking, do where cheap

- UI twins: `kernel_asks_position` and `count_for_index` are not one-token twins; `unchecked_table` only tests the signature of `reference::run`; there is no fixture refusing an outside `Col { .. }` literal, a `Col::mint` call or a `Bound` literal, which is the property that matters. Add those three.
- The term bound is tested at `BITS = 8` only (`tests/surface.rs:135-150`); cover 1 to 64 with a macro.
- B3 covers `Records` only under `Forward` and seed 0; B1 runs one input stream.
- Rustdoc missing on the pub fields of `Role`, `Coefficient`, `Binding`, on `ColumnsRead/Write`, `RecordsRead/Write`, and on the `Verdict` variants; README headings and the mention of the internal `BREAKS.toml` process in published prose.

## What the report must add

Every departure from the plan stated as a departure with its reason (finding 1 was neither in the report nor the changelist), and for finding 3 the encoding chosen or what was tried.
