# Doc changelist: the loop core after its first review

**Date:** 2026-11-13
**Phase:** DOC
**Round:** 202611130355-mlvq (loop-core-review-one, branch `work/loop-core`)
**Plan:** engine roadmap r7, step HL0a, with sketches S1 (`mock/research/sketches/202611122252_runner-generic-workunit/`) and S3 (`mock/research/sketches/202611122242_frame-view-typestate/`)
**Answers:** the first review of round `202611130153-yegv`, closed at `8405887`

This changelist rewrites the designs of the five loop core crates for the shapes the review asked for, and records the five shapes it named as not safe to lock. Nothing outside the five crates, the frozen lints and the agent template changes. The repository declares no canon, so the designs govern; every departure from the plan or from the first round's designs is said under its own heading below rather than folded into a change.

## CHANGE: the staged side is cut from the checked table

**Files:** `mock/crates/hilavitkutin-contracts/DESIGN.md.tmpl`, `mock/crates/hilavitkutin-reference/DESIGN.md.tmpl`

A store's `step` takes an `Opening`, which only the checked table makes, and returns the committed view, `Read` and `Sync`, with the whole staged side, `Stage` and `Send`. The checked table cuts the staged side with `Stage::take` into one part per unit holding exactly the stored columns the unit writes, and a runner cuts a part further by position with `Stage::split`. `Bound::step` hands the parts out through a `StepFn`, as `Units`, each unit's part arriving at a `StagedFn` as a `Staging`. A threaded runner is then a `StepFn` and a `StagedFn` of its own, and needs no change in an author or an implementor; the reference crate's tests prove it with a runner on scoped threads. The swap commits only the columns taken. This replaces the first round's `Write` on the whole staged side, which every unit shared.

## CHANGE: the checked table holds its store

**Files:** `mock/crates/hilavitkutin-contracts/DESIGN.md.tmpl`

`bind` takes the store by `&mut` and `Bound` keeps it for its life, so a table is never run over a store it was not checked against, and the store cannot change under it. While the table lives the store is reached only through it: read by `Bound::store` and `Bound::committed`, stepped by `Bound::step`, swapped by `Bound::swap`, and walked unit by unit by `Bound::each`.

## CHANGE: one role per column

**Files:** `mock/crates/hilavitkutin-contracts/DESIGN.md.tmpl`, `mock/crates/hilavitkutin-frozen-authors/DESIGN.md.tmpl`

A kernel declares `Roles`, its columns each once, and a binding gives one `Role` for each, in that order. Every `Own` and `Near` member finds its column's place in the list by `notko-hlist`'s `PlaceOf`, with the places inferred where the list is bound, so two members of one column read one stored column and no specialization is needed. A list naming a column twice leaves the place ambiguous and the bind does not build (E0283); a member whose column is not in the list has no place and does not build (E0277). The fall-back, had `PlaceOf` not carried it, was a role per member with a bind-time check that the members of one column name one stored column; it is not needed and not built.

## CHANGE: the five shapes fixed now

**Files:** `mock/crates/hilavitkutin-contracts/DESIGN.md.tmpl`

`FormatId` is made inside the crate only, one per sealed `Value`, so two values cannot share one by accident. A `KernelId` is checked to fit only where a kernel declares it, at build time, where its list is bound; a binding naming an id that does not fit is data and is refused as `UnknownKernel`, never a panic. `Count::sub` returns nothing below zero rather than wrapping. A morsel size is a `Morsel`, never zero. `Index` is a position only: `Ordinal` counts through a stream or a list, and `ColumnNo` names a stored column.

## CHANGE: the laws refuse too few runs, and the ledger check is a lint

**Files:** `mock/crates/hilavitkutin-laws/DESIGN.md.tmpl`, `mock/crates/hilavitkutin-frozen-implementors/DESIGN.md.tmpl`, `mock/crates/hilavitkutin-frozen-authors/DESIGN.md.tmpl`, their `README.md.tmpl`s, `mock/DESIGN.md.tmpl`, `mock/agent/rules/cargo.md.tmpl`, `mockspace.toml`

`morsel_split_and_order` and `frame_isolation` fail with `Witness::TooFew` when handed fewer than two runs, before comparing anything. The ledger check leaves the laws crate and becomes the repo lint `frozen-ledger`, at error on commit, build and push, which reads every file of a frozen crate, its manifest and any build script included, refuses a crate with no ledger, and refuses a dependency on anything but `hilavitkutin-contracts`. The frozen crates lose their ledger tests and are `publish = false`. The frozen lints lose their `lint:allow` escape.

## CHANGE: the law matrix runs over three columns

**Files:** `mock/crates/hilavitkutin-reference/DESIGN.md.tmpl`, `mock/crates/hilavitkutin-laws/DESIGN.md.tmpl`

The reference tests bind two units, `Diffuse` and the test kernel `Mirror`, over three stored columns, one of which no unit writes, and run every law over the three stores, both visiting orders, three morsel sizes and three inputs. Two mutants join: a store that serves every column from the first, and a swap that commits the columns no step took.

## CHANGE: the agent template counts its crates

**Files:** `mock/agent/rules/cargo.md.tmpl`

The count is `{{crate_count}}`, not a number in words.

## Shapes not locked

**Files:** `mock/crates/hilavitkutin-contracts/DESIGN.md.tmpl`

The contracts design gains a section naming five shapes as unlocked, each with its ticket: the push-callback visit (`lock_or_replace_the_push_callback_visit_shape_in_hilavitkuti`), the domain's `first`, `next` and `reach` (`lock_or_replace_the_domain_first_next_reach_walk_in_hilavitk`), positions as dense key-order ranks with no resize (`decide_positions_beyond_dense_key_order_ranks_with_no_resize`), `Word` as the storage unit with `Cell64` the widest cell (`decide_the_storage_word_and_the_widest_cell_in_hilavitkutin`), and a kernel's values as nested pairs (`decide_whether_a_kernel_s_values_stay_nested_pairs_in_hilavi`).

## Departures

1. `Records` stages column-major. Its committed cells keep a record's cells side by side, last key first, but its staged cells are one run per column, so that a unit's part is its columns alone and a part cut by position is one slice per column. The swap copies the written columns into the records. The first round's design staged it record-major.
2. `Value` is sealed, and `Int` is its one impl. A consumer's own value type is not possible until the seal is lifted with a uniqueness rule for `FormatId`; the plan left `Value` open.
3. A morsel size is a `Morsel` and a size of zero is not the whole; the plan and the first round took zero as the whole.
4. `Ordinal` and `ColumnNo` are new beside `Index` and `Count`.
5. `notko` and `notko-hlist` move to `ae194643`'s line for `PlaceOf`, locked at `8a74dbcd`.
6. Only the two frozen crates are `publish = false`. The laws and the reference stay publishable, as the oracle a third party's store or runner is checked against.
7. The reference's `Refusal` gains `Outside`, a write a visiting order hands outside the store, refused as a value rather than ignored.
8. `Mirror` is a test kernel in the reference's tests, not a frozen author, so the frozen crates carry one author still.
9. The ledger lint refuses any dependency of a frozen crate but `hilavitkutin-contracts`, dev and build dependencies included; the plan named the rule for normal dependencies only.

## Flat topics this round inherits

1. `202607200100_topic.b1-cap-lifting-completion.md` moves the engine scheduler's scratch onto `PlanDims` capacities. This round touches neither the engine nor `hilavitkutin-api`, so it needs nothing here and is re-filed for the engine's own next round.
2. `202607281547_topic.adopt-the-shared-hlist.md` switches `hilavitkutin-api`'s list to `notko-hlist`. The five new crates are on `notko-hlist`; the legacy crate's switch is moot if the port deletes it first, and it is re-filed until the port decides which.
3. `origin/recover/unlanded-rounds-and-benches` parks older rounds as a record. It is no flat topic of this branch and is left to whoever recovers it.

## Out of scope

The root `README.md` and `docs/` are no round's documents: the README is rewritten on this branch beside the round, and `docs/` is regenerated by `cargo mock` from these designs at the release, as the release flow does. The engine, `hilavitkutin-api` and every other existing crate keep their designs and their code.
