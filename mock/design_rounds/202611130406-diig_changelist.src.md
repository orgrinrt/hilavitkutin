# Src changelist: the loop core after its first review

**Date:** 2026-11-13
**Phase:** SRC
**Round:** 202611130355-mlvq (loop-core-review-one, branch `work/loop-core`)
**Doc changelist:** `202611130355-yrri_changelist.doc.lock.md`

Builds what the doc changelist designs. No lint severity moves and no exemption is added; the frozen lints lose their escape and one lint is added at error.

## CHANGE: `hilavitkutin-contracts`

Files: `src/bind/mod.rs`, `src/bind/roles.rs`, `src/domain/member.rs`, `src/domain/mod.rs`, `src/domain/value.rs`, `src/kernel.rs`, `src/num/index.rs`, `src/num/int.rs`, `src/num/mod.rs`, `src/store.rs`, `tests/surface.rs`.

`bind` takes the store by `&mut` and `Bound` holds it; `Bound::step` opens a step with an `Opening` and hands a `StepFn` the committed view and `Units`, each unit's part a `Staging` with `split`, `scatter`, `low`, `high` and `unit`. `Store::step` returns the committed view and the whole staged side, `Read + Sync` and `Stage + Send`, and `swap` commits only the columns taken. A kernel declares `Roles`, and members find their column by `PlaceOf` with the places inferred at bind. `FormatId::new` is crate-private and `Value` sealed. `Count::sub` returns a `Maybe`. `Ordinal`, `ColumnNo` and `Morsel` are new. The surface tests cover the new `Count::sub` and the term bound at every width from 1 to 64.

## CHANGE: `hilavitkutin-frozen-implementors`

Files: `src/lanes.rs` (new), `src/columns.rs`, `src/records.rs`, `src/visit.rs`, `src/lib.rs`, `Cargo.toml`, `LEDGER.toml`, `BREAKS.toml`, `tests/ledger.rs` (deleted).

`Lanes` with the orders `Ahead` and `Behind` is both stores' staged side. `Columns` and `Records` step, swap and address cells by `ColumnNo`; `Records` stages column-major. The visits take a `Morsel`. The crate is `publish = false` and loses its ledger test to the lint; each edited file has its line updated and a break recorded, and `lanes.rs` and `Cargo.toml` gain lines.

## CHANGE: `hilavitkutin-frozen-authors`

Files: `src/diffuse.rs`, `Cargo.toml`, `LEDGER.toml`, `BREAKS.toml`, `tests/ledger.rs` (deleted).

`Diffuse` names its `Roles`, one break. The crate is `publish = false`, loses its ledger test, and its manifest gains a ledger line.

## CHANGE: `hilavitkutin-reference`

Files: `src/lib.rs`, `Cargo.toml`, `tests/common/mod.rs`, `tests/common/probe.rs`, `tests/laws.rs`, `tests/mutants.rs`, `tests/bind.rs`, `tests/threads.rs` (new), `tests/ui/fail/*`, `tests/ui/pass/*`.

The runner is `OneThread`, `Stepper`, `Records` and `Digester` over the bound's step; `run` takes a `Morsel` and refuses `Outside` as a value; the public `swap` goes, the bound swaps. The tests bind `Diffuse` and `Mirror` over three columns, add the mutants `IgnoresColumn` and `SwapDropsUnwritten` and a threaded runner, and the compile-fail fixtures gain `bound_literal`, `col_literal`, `col_mint`, `store_touched_while_bound`, `role_named_twice` and `member_outside_roles`; `unchecked_table` goes, since `bind` now takes the store and the case is `store_touched_while_bound`'s.

## CHANGE: `hilavitkutin-laws`

Files: `src/lib.rs`, `Cargo.toml`, `tests/verdicts.rs`, `src/ledger.rs`, `tests/ledger.rs` and `tests/support/ledger.rs` (deleted).

`Witness` is `Differs`, `Length` and `TooFew`, positions as `Ordinal`; the two comparing laws refuse fewer than two runs first. The ledger check leaves the crate.

## CHANGE: the frozen lints

Files: `mock/lints/frozen/scan.rs`, `mock/lints/frozen_no_feature_gate.rs`, `mock/lints/frozen_no_foreign_turbofish.rs`, `mock/lints/frozen_no_recursion_limit.rs` and their `tests.rs`, `mock/lints/frozen_ledger.rs` and `mock/lints/frozen_ledger/tests.rs` (new), `mockspace.toml`.

The `lint:allow` escape is removed from the scan and each lint's tests assert no line escapes. `frozen-ledger` is added at error on commit, build and push.

## CHANGE: the workspace and the cross-target tool

Files: `mock/Cargo.lock`, `mock/tools/cross-target/front/Cargo.lock`, `mock/tools/cross-target/front/loop/src/lib.rs`.

`notko` and `notko-hlist` are locked at `8a74dbcd`. The cross-target crate binds one role per column, lends the store and runs with a `Morsel`.

## Tests

The contracts' surface tests, the laws' verdicts, the reference's laws, mutants, threads, bind and compile-fail suites, the lint unit tests, and the cross-target digests compared native against wasm32.

## Out of scope

The five unlocked shapes keep their current form under their tickets.
