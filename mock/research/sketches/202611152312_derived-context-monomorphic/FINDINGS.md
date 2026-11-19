# Derived context with no erasure (H-ctx-2)

A work unit in the block form, written in a crate that takes no engine crate and names no context type, is built and run under the engine with every column read and write monomorphised to the engine's own accessor. The first sketch, `202611152150_derived-context-block-form`, reached the same unit shape through an erased pointer and a table of stored function pointers, which the canon rules out. This sketch asks whether the shape survives with that removed.

It was written unattended on 15 and 16 November 2026 by a dispatched seat. The dispatcher reran `cargo +nightly-2026-05-28 test -q -p check` (fixtures 2 passed, run 1 passed), searched `diagnostics/probe.s` and `diagnostics/control-probe.s` itself, and read the engine copy's `block.rs` before writing this file. Nothing here is authority.

## Hypothesis

A block unit can be handed the engine's context through an api trait whose column access is bounded only by witnesses on the unit's own read and write lists, so the body is generic over the runner, the engine implements the trait on its concrete context, and no type is forgotten between the body and the column.

**Outcome:** WORKS

All four conditions hold under the predicate below. Two things are left to the maintainer. The unit registers as `Blk(unit)`, an engine-owned wrapper, and not bare; the attempts at a bare registration are below, and what blocks it is the orphan rule on the type function that names the context. The unit's `run` signature names the api trait `Records` (`records: &impl Records<Self::Read, Self::Write>`), which is a contract and not a context type; whether that meets "names no context type" is his reading to give.

## What the canon rules out

Three ratified rows govern, in `mock/registry/tenet/principles.toml` and `principles-3.toml`: `std_any_is_banned_so_there_is_no_type_id_and_no_type_erasure`, `dyn_is_banned_entirely_with_no_exceptions`, and `the_monomorphised_function_is_the_type_identity_of_a_unit_and_llvm_devirtualises_the_fused_pipeline_into_straight_line_code`. As the seat reads them: nothing between a unit and its data may forget a type and recover it at run time. That rules out `dyn`, and it rules out the hand-built equivalent, an erased pointer beside a table of thunks. The body has to be a monomorphised function over the runner's concrete types. The sketch keeps `WorkUnit::Ctx` a generic associated type over the frame lifetime, as the row `the_context_of_a_unit_is_a_generic_associated_type_over_a_frame_lifetime_the_scheduler_owns` has it; the wrapper assigns the engine's computed `CtxFor`.

## What was built

The sketch carries copies of the api and the engine under `api/` and `engine/`, since a sketch does not edit shipped source. Every edit to a copy is listed in `EDITS.md`, with diffs under `diagnostics/edits-*.diff`; the copies only add.

The api copy gains `block.rs`: `ColList` (a list as a head value and a rest), `ColSlots<L>` (what a runner holds per list member), `ReadsColumn<T, K>` and `WritesAt<T, K>` (the membership witness on the list, indexed `Here` or `There<K>`, with a hook generic over any `B: ColSlots<Self>`), `Records<R, W>` (`len`, `each`, `read`, `write`, bounded by list witnesses alone) and `Block<S = Always>` (`Read`, `Write`, `Hint`, and `fn run(&self, records: &impl Records<Self::Read, Self::Write>)`).

The engine copy gains `dispatch/engine_ctx/block.rs`: `ColSlots` on the bundle nodes `ColPtrNil` and `ColPtrCons`, reading and writing through the node's own pointers, the same read and write the shipped resolve traits make; `Records<R, W>` on `EngineCtx`; and `Blk<U>(pub U)`, a `WorkUnit` whose `Ctx<'frame>` is `CtxFor<'frame, U::Read, U::Write>` and whose `execute` is `self.0.run(ctx)`.

`unit/` holds Stamp, Drift and Warm with the first sketch's bodies. `native/` holds the same three as native `WorkUnit`s naming `CtxFor`.

## The four conditions

1. The unit crate depends on no engine crate. `unit/Cargo.toml` lists `hilavitkutin-api` and `arvo`. `diagnostics/unit-dependency-tree.txt` names no engine, kit or providers crate.
2. The unit source names no context type. `unit/src/lib.rs` imports only `hilavitkutin_api::{access, block, hint, member, store}`. It does name `Records`.
3. A twin touching an undeclared column fails to compile. `check/ui/fail/read_undeclared.stderr`: `error[E0277]: the read list Cons<Column<Pos>, Cons<Column<Vel>, hilavitkutin_api::Empty>> holds no column of Glow`, the trait `ReadsColumn<Glow, There<There<_>>>` not implemented. `check/ui/fail/write_undeclared.stderr`: `error[E0277]: the write list Cons<Written<Column<Glow>>, hilavitkutin_api::Empty> holds no column of Pos`, the trait `WritesAt<Pos, There<_>>` not implemented. The declared twins under `check/ui/pass/` compile.
4. No erasure and no stored function pointer. The route's code has no pointer cast that forgets a type, no thunk table and no `dyn`. The emitted code is below.

## The run, and the case that must fail

`check/tests/run.rs` runs `Blk(Stamp)`, `Blk(Drift)`, `Blk(Warm)` and the three native units under `Scheduler::run` over 4096 records and four frames, and compares the stores every frame and against hand values. It passes.

With the engine copy's `mutant-short-window` feature, the block window one record short, the same test fails, `diagnostics/mutant-short-window.txt`:

```
thread 'block_units_run_under_the_scheduler' panicked at check/tests/run.rs:114:9:
assertion `left == right` failed: position at record 1535 after the first frame
  left: Pos(1535)
 right: Pos(6133)
test result: FAILED. 0 passed; 1 failed
```

## Emitted code for condition 4

`probe/src/lib.rs` puts nine bodies behind `no_mangle`, `inline(never)` symbols over `&CtxFor<..>`: each block unit's `run`, the same through `Blk::execute`, and the native unit. Emitted with `cargo rustc -p probe --release --lib -- --emit asm,llvm-ir` (opt-level 3, fat lto, one codegen unit, x86-64 linux), kept as `diagnostics/probe.s` and `probe.ll`.

The search `^\s*(call|jmp)q?\s+\*` over `probe.s` finds no line, and the wider `^\s*(call|jmp)` finds none either: every body is loads, an add or compare, stores, conditional branches and a return. `probe.ll` calls only `llvm.experimental.noalias.scope.decl`. The block and native bodies are the same loops, differing in register and field order.

The control: `control/control-probe` instantiates the first sketch's bodies over its erased context under the same profile, kept as `diagnostics/control-probe.s`. The same search finds six indirect calls, all in the bodies, at lines 127, 132, 139, 199, 260 and 267. Where the first sketch's table is built on the frame and the body inlined into the wrapper, the optimiser folds them; the body as its own instantiation keeps them. A second showing that the search finds what it looks for is `diagnostics/probe-before-native-inline.s`, the first emission of this probe, with three indirect jumps that were the native `execute`s reached across a crate boundary before they were marked `#[inline]`. The record is `diagnostics/indirect-branch-search.txt`.

This is not a timing and says nothing about cost. It shows the absence of indirect branches in the body instantiated over the engine's context behind a probe symbol, not in the body as inlined in the scheduler's own dispatch.

## Attempts

m1. The witness on the list, the api's `Records` implemented by the engine on `EngineCtx`, registration through `Blk<U>`. Not tried before. Works; it is everything above.

a1. A body generic over the shipped accessor traits. Tried in the first sketch and run again against the api copy. Still fails, `check/ui/attempts/a1_generic_over_shipped_accessors.stderr`: `error[E0277]: the trait bound <C as HasColumnReader<..>>::Provider: ResolveColumnRead<Pos, _> is not satisfied`.

a3. A blanket `impl<U: Block> WorkUnit for U` outside the api. Tried in the first sketch and run again. `error[E0210]: type parameter U must be used as the type parameter for some local type`.

b1. The blanket inside the api, the context read off an api-declared type function `CtxOf`. New. `cargo check -p hilavitkutin-api --features attempt-blanket` compiles, and so do `native` and `probe` with the feature on, so the blanket coheres with native implementations and with `Blk`. Its "no output" has no artifact beyond the command.

b2. The same blanket carrying a witness parameter. New. `error[E0207]: the type parameter I is not constrained by the impl trait, self type, or predicates` at `api/src/attempt_blanket.rs:72:9`, as an earlier reader expected. Moot for m1, which needs no witness on the implementation.

c1. The engine implementing `CtxOf` for every read list. New. `error[E0210]: type parameter R must be used as the type parameter for some local type` at `engine/src/dispatch/engine_ctx/attempt_ctx_of.rs:17:14`.

c2. The same per list shape. New. Two E0210 on `W`, at `attempt_ctx_of.rs:30:14` and `:39:20`.

The outputs of b1 to c2 are in `diagnostics/attempts-blanket-and-type-function.txt`; the error lines of b2, c1 and c2 there are transcribed from the builds.

So a bare registration is expressible in the api (b1), but nothing outside the api can supply its type function (c1, c2): the only free types are the unit and its lists, all foreign to the engine. What would move that wall, none of it built: the context type function living api side; a runner parameter on `WorkUnit` or on the type function, supplied by the scheduler; or `WorkUnit` without `Ctx`, with an `execute` generic over `Records`.

## Predicate

Holds for: toolchain `nightly-2026-05-28`; no feature gate added; arvo at 1b2796e9, notko and notko-hlist on dev at f5d8f563; the api and the engine as copies of this branch's base plus the edits in `EDITS.md`; kit and providers as shipped source with build scripts not run; built without the repository's rustc wrapper flags; dev profile for the run and the twins; release with opt-level 3, fat lto, one codegen unit, x86-64 linux, for the emitted code; schedule `Always`; lists of plain columns only (bare in `Read`; bare, `Written` and `Differs` in `Write`); `Scheduler::run` on one thread; 4096 records, two windows in the first frame, four frames; store `ArenaColumnStorage<StaticBumpMemory>`; registration through the engine copy's `Blk`.

Not checked, and so not held: resources, accumulators, virtuals, `Buffered`, `Reach`, `map`, `reduce` and batch in the block form; a `Differs` write at run time; maintenance contexts, `run_parallel`, `run_fused`, `RecordOp` fusion, `On<V>` schedules, other store kinds; long lists against the recursion limit; a build under the wrapper flags; other targets; any timing; the body inlined in the scheduler's dispatch; the three routes to a bare registration; the repository's lints and `cargo mock test` over the copies.
