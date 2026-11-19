# Edits To The Copies

This sketch carries its own copies of three shipped crates, because the form it tests changes all three, and a copy of one crate of another repository. `api/` is a copy of `mock/crates/hilavitkutin-api` and `engine/` of `mock/crates/hilavitkutin`, each with its `src/`, its `Cargo.toml` and its `build.rs`, and `providers/src/` is a copy of the source of `mock/crates/hilavitkutin-providers`. All three were taken from `dev` at `07a9e9a3`. `carrier-model/src/lib.rs` is a copy of `research/sketches/202611191400_carrier-as-a-typestate-parameter/model/src/lib.rs` of orgrinrt/notko at `69915c8`. Nothing under `mock/crates/` was edited.

Every difference between a copy and its source is listed here and kept as a diff under `diagnostics/`, made with `git diff --no-index` after the last commit that touched the copies. The source of each crate is in `edits-api.diff`, `edits-engine.diff` and `edits-providers.diff`, each manifest in `edits-api-manifest.diff`, `edits-engine-manifest.diff` and `edits-providers-manifest.diff`, and the model in `edits-carrier-model.diff`. A file not named below is identical to its source. The tests, the examples and the design templates of the three crates were not copied.

The repository's commit hook formats what a commit stages, and it changed the layout of files the sources hold unformatted. Seven carry no edit of this sketch and differ in layout alone: `engine/src/dispatch/sync.rs`, `engine/src/plan/laplacian.rs`, `engine/src/resource/accumulator.rs`, `engine/src/resource/mod.rs`, `engine/src/resource/provenance.rs`, `engine/src/scheduler/plan.rs` and `api/src/work_unit_values.rs`. `carrier-model/src/lib.rs` differs from notko's file in layout alone as well. The diffs show these too.

`kit/` is not a copy. It is a manifest alone whose `[lib] path` points at the shipped source, with `build = false`, so the workspace holds one api (the copy) and the shipped kit compiles against it unchanged. The build scripts of kit and providers do not run here.

## The api copy

`api/src/work_unit.rs` is the first change the form needs. `WorkUnit` keeps `Read`, `Write`, `Hint` and `COMMUTATIVE` and loses `Ctx` and `execute`. The two move, unchanged, to a new trait under it, `Positional<Schedule = Always>: WorkUnit<Schedule>`. Nothing else in the file changes: `Plannable`, `HasSchedule` and `WorkUnitBundle` read the declaration and still do.

`api/src/values.rs` holds the read product on the carrier. `Read<'f>` is new, with its one `Carrier` impl over every `Readable` member, and `ReadProd<'a, R>` names the product. `ReadSet` loses its associated type `Vals` and gains the supertrait `for<'a> model::Project<Read<'a>>`. `ReadVals<'a, R>` is bounded on `R: model::Project<Read<'a>>` and wraps `ReadProd<'a, R>`, and its `get` is bounded on `model::Has<Read<'a>, M, P>`. `RNil`, `RCons` and `ReadAt` are deleted. The write half of the file is as shipped.

`api/src/unit.rs` is new and compiles only under the feature `unit-blanket`. It holds `Unit`, the one-impl spelling, and its three blankets. The form runs without it.

`api/src/attempt_unit_sched.rs` is new and compiles only under the feature `attempt-unit-sched`. It holds attempt s1, which does not compile, and is no part of the form.

`api/src/lib.rs` gains the two gated module declarations, re-exports `Positional`, `Read` and `ReadProd`, and no longer re-exports `ReadAt`.

`api/Cargo.toml` gains the dependency on `carrier-model` and the features `unit-blanket` and `attempt-unit-sched`, both empty.

## The engine copy

`engine/src/dispatch/fiber_run.rs` is the second change the form needs. The step that built one unit's context and ran its body was inline in the `WuCons` impl of `RunFiber`. It is now a trait, `RunUnit<A, Wit>`, with two impls told apart by the witness: `ByCtx` for a unit that offers `Positional` (the shipped step, moved) and `ByValue` for a unit that offers `RecordOp` (new: the engine's context over the unit's lists applies the value function). The `WuCons` impl of `RunFiber` calls it, and its witness element is the pair `(U, GI)` where it was a tuple of six.

`engine/src/dispatch/wu_fn.rs` bounds `invoke_wu_in_fiber` on `Positional` and gains `invoke_value_in_fiber`, the same shim for a value function, behind the same boundary an unwind cannot cross.

`engine/src/dispatch/trunk_gate.rs` names the witness element as the pair `(U, GI)`.

`engine/src/dispatch/fusion.rs` splits the one `WorkUnit` impl of the engine's fused unit into a declaration and a `Positional`, and takes the cells of the read product it builds from the model.

`engine/src/dispatch/engine_ctx/map.rs` returns `ReadProd<'a, Self>` from `gather` and builds the model's cells, and gains the mutant: under the feature `mutant-short-window`, `resolve_map` applies the function one record short of the window.

`engine/Cargo.toml` gains the dependency on `carrier-model` and the feature `mutant-short-window`.

The `#[cfg(test)]` module of `engine/src/dispatch/order.rs` still holds two `WorkUnit` impls in the shipped shape. No build of this sketch compiles it, and it was left as it is.

## The providers copy

`providers/src/adapt_wu.rs` splits the one `WorkUnit` impl of `AdaptWu` into a declaration and a `Positional`, as the engine's fused unit was split.

`providers/Cargo.toml` is the sketch's own and not the shipped one. It names the library by path, sets `build = false` and `publish`, and leaves out the shipped manifest's `description`, `repository`, its dev-dependency on the engine and its build-dependency on `hilavitkutin-build`.

## The model copy

`carrier-model/Cargo.toml` is the sketch's own: the package is named `carrier-model`, the library keeps the name `model`, and `notko-hlist` comes from this workspace where notko's sketch takes it by path.

## What a round would take from this

Two changes carry the form: the split of `WorkUnit` in `api/src/work_unit.rs`, and `RunUnit` with its two impls in `engine/src/dispatch/fiber_run.rs`. Every unit that names a context today then moves its `Ctx` and `execute` into a `Positional` impl, or drops both where its body is its value function. `Unit` and its blankets are an addition on top and can be left out. The read product on the carrier is the third change and stands apart from the two: it waits on `notko-hlist` shipping the shape, and it changes `values.rs` and the engine's `gather` and nothing a unit writes.
