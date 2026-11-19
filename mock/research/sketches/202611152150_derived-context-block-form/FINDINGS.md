# Derived context for a block-form unit

Hypothesis: a work unit in the block form can be built and run under the engine when its crate takes no engine crate as a dependency and its source names no context type. The context is derived from the unit's declared `Read` and `Write` lists, and a twin that reads or writes an undeclared column does not compile.

**Outcome:** INCONCLUSIVE. The three conditions are met, but only by a vehicle the canon rules out, so the step is not proven in a shape that may ship.

## The canon and this shape

The first version of this file said WORKS and did not ask the canon about the vehicle. A later reader did. The shape below erases the runner's context to a pointer and reaches every column through a table of function pointers, one indirect call per access per record. Two ratified rows of hilavitkutin's registry forbid that: `std_any_is_banned_so_there_is_no_type_id_and_no_type_erasure`, and `the_monomorphised_function_is_the_type_identity_of_a_unit_and_llvm_devirtualises_the_fused_pipeline_into_straight_line_code`, which links to it and to `dyn_is_banned_entirely_with_no_exceptions`. A table of thunks is dynamic dispatch built by hand. The sketch bought an unedited engine with it, and that is not a trade the canon allows.

What still carries, read for what it proved: the unit crate's dependency set, the two refusals by list position, attempt a1's wall (the keyed witness sits on the provider and not on the list) and attempt a3's E0210 (a blanket over a block form must live in the crate that defines `WorkUnit`). The sentence below that an api-owned blanket "could be written there" was not built, and a reader has since argued it fails as written, since the bridge names the engine's `CtxFor` and carries witnesses no part of the implementing type constrains. The route still owed a sketch is the monomorphic one: an api trait bounded by list membership, implemented by the engine on its own context, a unit body generic over it, with the emitted code read for indirect branches.

## What was built

The sketch was written unattended on 15 November 2026 by a dispatched seat, and the dispatcher reran `cargo +nightly-2026-05-28 test -q -p check` before writing this file (fixtures 2 passed, run 1 passed). Nothing here is authority. It is one instance of proof for the three conditions below, inside the predicate at the end.

The unit crate depends on `derived`, `hilavitkutin-api` and arvo, and on nothing else (`unit/Cargo.toml`). It declares `Read`, `Write` and `Hint` and writes a body `fn run(&self, records: &impl Records<Self::Read, Self::Write>)`. The body is generic over a sealed trait, `Records`, which has exactly one implementation, `Cx<'c, R, W>`, built per call by `derived::enter` from the runner's context. `Cx` holds the runner's context as an erased pointer, plus two `repr(C)` tables of erased fn-pointer thunks, one per list member in list order. The tables are built from the api's own traits: `ResolveColumnRead`, `ResolveColumnWrite` with `WritesColumn`, and `BatchApi`. A read or a write finds its thunk by the member's position in the list, `notko_hlist::PlaceOf` for reads and the sketch's `WritesAt` for writes. A column the list does not hold has no position, so the call does not compile.

Registration with the engine goes through a wrapper, `bridge::Blk<U, RI, WI>`, which implements `WorkUnit` with `Ctx<'f> = CtxFor<'f, U::Read, U::Write>`. Its witness lists `RI` and `WI` are inferred at `Scheduler::run`, and no turbofish is written. The wrapper is there because a blanket `impl<U: Block> WorkUnit for U` outside the api is E0210 (attempt a3). Were the block form the api's own, the blanket could be written there and the wrapper would go. That was not built here.

## What is left to the maintainer

Condition 2 says the unit names no context type. The unit names none, but it does name the sealed trait `Records` as the bound of what it is handed (`unit/src/lib.rs:42`, `:59`, `:82`). Whether a trait bound that stands for the context counts as naming one is not this sketch's call.

The thunk tables cost one indirect call per column access per record. No timing was done, since a sketch times nothing. Whether the indirection survives inlining under the engine's build flags is a bench, not a finding of this sketch.

## The three conditions

1. The unit crate takes no engine crate. `diagnostics/unit-dependency-tree.txt` holds the output of `cargo tree -p unit -e normal,build,dev`: unit, derived, hilavitkutin-api, and under the api hilavitkutin-ctx and hilavitkutin-build, plus arvo and notko crates. No `hilavitkutin` and no `hilavitkutin-providers`.
2. The unit names no context type. `unit/src/lib.rs` imports `derived::{Block, Records}` and api types only. See the point above on `Records`.
3. A twin naming an undeclared column fails to compile. `check/ui/fail/read_undeclared.rs` is Drift reading `Glow`, which its read list does not hold:

```
error[E0277]: `Cons<Column<Pos>, Cons<Column<Vel>, hilavitkutin_api::Empty>>` has no place for `Column<Glow>`
  --> ui/fail/read_undeclared.rs:23:35
   |
23 |                 let vel = records.read::<Glow, _>(i);
   |                                   ^^^^ the trait `PlaceOf<Column<Glow>, There<There<_>>>` is not implemented for `Cons<Column<Pos>, Cons<Column<Vel>, hilavitkutin_api::Empty>>`
note: required by a bound in `derived::Records::read`
```

`check/ui/fail/write_undeclared.rs` is Warm writing `Pos`, which it only reads:

```
error[E0277]: the write list `hilavitkutin_api::Empty` holds no column of `Pos`
  --> ui/fail/write_undeclared.rs:21:40
   |
21 |             records.write::<Pos, _>(i, Pos(pos.0 + pos.0));
   |                     -----              ^^^^^^^^^^^^^^^^^^ the trait `WritesAt<Pos, _>` is not implemented for `hilavitkutin_api::Empty`
   = note: required for `Cons<Written<Column<Glow>>, hilavitkutin_api::Empty>` to implement `WritesAt<Pos, There<_>>`
note: required by a bound in `derived::Records::write`
```

Each has a pass twin under `check/ui/pass/` that differs only in the column named. The write twin carries the turbofish `write::<Pos, _>`; without it a one-member list fixes the type from the list and the error is E0308, a mismatch, not the refusal above.

## Attempts

a1, a body generic over the shipped accessor traits, `fn run<C: HasColumnReader<R> + HasColumnWriter<W> + HasEach<R, W>>`. FAILS: the keyed witness of `ResolveColumnRead<T, I>` does not resolve through an opaque `C::Provider`. From `check/ui/attempts/a1_generic_over_shipped_accessors.stderr`:

```
error[E0277]: the trait bound `<C as HasColumnReader<Cons<Column<Pos>, Cons<Column<Vel>, hilavitkutin_api::Empty>>>>::Provider: ResolveColumnRead<Pos, _>` is not satisfied
  --> ui/attempts/a1_generic_over_shipped_accessors.rs:49:36
   |
49 |             let pos = ctx.reader().read::<Pos, _>(i);
   |                                    ^^^^ unsatisfied trait bound
```

a2, a body generic over the sealed `Records`, whose one implementation is the erased-table context. WORKS, and is the shape described above.

a3, every block unit a `WorkUnit` by one blanket written outside the api. FAILS, from `check/ui/attempts/a3_blanket_workunit_outside_the_api.stderr`:

```
error[E0210]: type parameter `U` must be used as the type parameter for some local type (e.g., `MyStruct<U>`)
 --> ui/attempts/a3_blanket_workunit_outside_the_api.rs:9:6
  |
9 | impl<U> WorkUnit for U
  |      ^ type parameter `U` must be used as the type parameter for some local type
```

A defaulted associated `Ctx` was not built: it needs `associated_type_defaults`, which the workspace's unstable-feature rule does not allow.

## The run and its control

`check/tests/run.rs` runs the block units Stamp, Drift and Warm, wrapped in `Blk`, under `Scheduler::run`, beside native `WorkUnit` controls on `CtxFor` over the same columns, seeds and record count. It checks frame 1 against hand values, that the window index restarts at least once, that the block and native stores are equal after every frame, and `pos = i + 4*vel` and `glow = 2*pos` after four frames. Both schedulers are marked dirty before frames 2 to 4, since the engine otherwise skips units whose inputs did not change.

The case that must fail: the feature `mutant-short-window` makes `enter` hand the body a window one record short, and the run then panics at `check/tests/run.rs:185:9`, position at record 1535 after the first frame, `Pos(1535)` against `Pos(6133)`. The output is in `diagnostics/mutant-short-window.txt`.

## Predicate

Holds for: toolchain `nightly-2026-05-28`, dev profile, no `#![feature]` in any sketch crate; arvo at rev 1b2796e9, notko and notko-hlist on dev at 8a74dbcd, hilavitkutin crates by path at this branch's base; built without hilavitkutin-build's rustc wrapper flags; schedule `Always`; reads of bare `Column<T>`; writes of `Column<T>`, `Written<Column<T>>` and `Differs<Column<T>>`; `Scheduler::run`, single thread; 4096 records, two windows in frame 1, four frames; registration through `bridge::Blk`.

Not checked, and so not held: resources, accumulators, virtuals, `Buffered`, `Reach`; `run_parallel` and `run_fused`, and whether `RecordOp` fusion and the block form can coexist; other store kinds than `ArenaColumnStorage<StaticBumpMemory>`; other schedules; long lists against the recursion limit; a build under the wrapper flags; an api-owned blanket in place of `Blk`; any timing.
