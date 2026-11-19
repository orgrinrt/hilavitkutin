# Edits To The Copies

This sketch carries its own copies of three shipped crates, because the form it tests changes all three. `api/` is a copy of `mock/crates/hilavitkutin-api`, `engine/` of `mock/crates/hilavitkutin`, each with its `src/`, its `Cargo.toml` and its `build.rs`, and `providers/src/` is a copy of the source of `mock/crates/hilavitkutin-providers`. All three were taken from `dev` at `07a9e9a3`. Nothing under `mock/crates/` was edited.

Every difference between a copy and the shipped crate is listed here, and the same differences are kept as diffs under `diagnostics/`, made with `git diff --no-index` from `mock/` between the shipped path and the copy. A file not named below is identical to the shipped one. The tests of the three crates were not copied.

`kit/` is not a copy. It is a manifest alone whose `[lib] path` points at the shipped source, with `build = false`, so the workspace holds one api (the copy) and the shipped kit compiles against it unchanged. The build scripts of kit and providers do not run here.

## The api copy

The diff of the source is `diagnostics/edits-api-src.diff`.

`api/src/work_unit.rs` is the change the form needs. `WorkUnit` keeps `Read`, `Write`, `Hint` and `COMMUTATIVE` and loses `Ctx` and `execute`. The two move, unchanged, to a new trait under it, `Positional<Schedule = Always>: WorkUnit<Schedule>`. Nothing else in the file changes: `Plannable`, `HasSchedule` and `WorkUnitBundle` read the declaration and still do.

`api/src/unit.rs` is new and compiles only under the feature `unit-blanket`. It holds `Unit`, the one-impl spelling, and its three blankets. The form runs without it.

`api/src/lib.rs` gains the gated `pub mod unit;` and the re-export of `Positional`.

`api/Cargo.toml` gains the feature `unit-blanket`, empty.

## The engine copy

The diff of the source is `diagnostics/edits-engine-src.diff`: five files, 160 lines added and 32 removed.

`engine/src/dispatch/fiber_run.rs` is the change the form needs. The step that built one unit's context and ran its body was inline in the `WuCons` impl of `RunFiber`. It is now a trait, `RunUnit<A, Wit>`, with two impls told apart by the witness: `ByCtx` for a unit that offers `Positional` (the shipped step, moved) and `ByValue` for a unit that offers `RecordOp` (new: the engine's context over the unit's lists applies the value function). The `WuCons` impl of `RunFiber` calls it, and its witness element is the pair `(U, GI)` where it was a tuple of six.

`engine/src/dispatch/wu_fn.rs` bounds `invoke_wu_in_fiber` on `Positional` and gains `invoke_value_in_fiber`, the same shim for a value function, behind the same boundary an unwind cannot cross.

`engine/src/dispatch/trunk_gate.rs` names the witness element as the pair `(U, GI)`.

`engine/src/dispatch/fusion.rs` splits the one `WorkUnit` impl of the engine's fused unit into a declaration and a `Positional`.

`engine/src/dispatch/engine_ctx/map.rs` gains the mutant: under the feature `mutant-short-window`, `resolve_map` applies the function one record short of the window.

`engine/Cargo.toml` gains the feature `mutant-short-window`.

The `#[cfg(test)]` module of `engine/src/dispatch/order.rs` still holds two `WorkUnit` impls in the shipped shape. No build of this sketch compiles it, and it was left as it is.

## The providers copy

The diff is `diagnostics/edits-providers-src.diff`. `providers/src/adapt_wu.rs` splits the one `WorkUnit` impl of `AdaptWu` into a declaration and a `Positional`, as the engine's fused unit was split.

## What a round would take from this

Two changes carry the form: the split of `WorkUnit` in `api/src/work_unit.rs`, and `RunUnit` with its two impls in `engine/src/dispatch/fiber_run.rs`. Every unit that names a context today then moves its `Ctx` and `execute` into a `Positional` impl, or drops both where its body is its value function. `Unit` and its blankets are an addition on top and can be left out.
