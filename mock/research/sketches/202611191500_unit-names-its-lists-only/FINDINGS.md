# Sketch findings: a unit can name its read and write lists and nothing else

**Date:** 2026-11-19
**Outcome:** WORKS
**Unblocks:** a design round on the unit contract, once the maintainer has answered the two questions named below. Nothing here changes a shipped crate.

This is the third sketch of the derived unit context. The first, `202611152150_derived-context-block-form`, reached a unit that names no context type through an erased pointer and a table of function pointers, which the canon rules out. The second, `202611152312_derived-context-monomorphic`, removed the erasure, and left two things: the body's signature named an api trait (`records: &impl Records<Self::Read, Self::Write>`), and the unit registered through an engine wrapper (`Blk(unit)`). The maintainer was shown that shape on 16 November and chose "Neither, sketch further": the unit names its read and write lists and nothing else, with no trait in the body's signature and no wrapper at registration.

The form below meets that for a unit whose body is a function from the values it reads to the values it writes. It was written unattended on 19 November 2026 by one session, which built and ran everything quoted here itself. Nothing in this file is authority. It is one instance of proof, inside the predicate at the end.

The form is not new machinery. The api already ships the value function under a name (`RecordOp::apply`), its two products (`ReadVals`, `WriteVals`) and the runner's side of it (`ResolveMap`), and the engine already applies it (`resolve_map`). What the sketch changes is who asks for a context. Today every unit must name one, since `WorkUnit` carries `Ctx` and `execute`, and a value-form unit writes `execute` as the one line `ctx.each().map(|r| self.apply(r))`. Here that line is the engine's, so the unit has nothing left to name a context for.

## Hypothesis

A unit whose body is its value function needs no context type and no `execute`, if `WorkUnit` is the declaration alone and the engine, which knows its own context type, applies the function. The orphan rule that stopped the second sketch does not arise, because nothing is implemented for the unit's lists: the engine states a bound on a type of its own.

## The form

A unit, as its author writes it (`unit/src/lib.rs`):

```rust
impl BuilderInput for Drift {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Drift {
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
    type Write = Cons<Column<Pos>, Empty>;
}

impl RecordOp for Drift {
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
        let pos = read.get(Column::<Pos>::new());
        let vel = read.get(Column::<Vel>::new());
        rec![Pos(pos.0 + vel.0)]
    }
}
```

It is registered as `.with(Drift)`. The same unit as one impl, under the api copy's feature `unit-blanket`:

```rust
impl Unit for Drift {
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
    type Write = Cons<Column<Pos>, Empty>;

    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
        let pos = read.get(Column::<Pos>::new());
        let vel = read.get(Column::<Vel>::new());
        rec![Pos(pos.0 + vel.0)]
    }
}
```

Two changes to the copies carry it, and `EDITS.md` lists every edit. In the api copy, `WorkUnit` keeps `Read`, `Write`, `Hint` and `COMMUTATIVE`, and `Ctx` and `execute` move unchanged to `Positional<S>: WorkUnit<S>`, which only a unit with a positional body implements. In the engine copy, the step that builds one unit's context and runs its body is a trait, `RunUnit<A, Wit>`, with one impl for a unit that offers `Positional` and one for a unit that offers `RecordOp`. The second builds `EngineCtx` over the unit's lists by the shipped `project` and calls `resolve_map` with the unit's `apply`. The two impls differ in their witness type (`ByCtx`, `ByValue`), which is inferred at `Scheduler::run` the way the projection indices inside it already are.

## What the canon says of this vehicle

Asked of `cargo mock canon subject units` and `cargo mock canon find context` before this file said WORKS. The rows are in `mock/registry/tenet/units.toml` and `units-2.toml`.

Two ratified rows ask for what the form gives. `a_unit_declares_only_its_reads_writes_and_hints_and_everything_else_is_derived` (`units.toml:278`): the unit above declares those three and a body. `the_consumer_spelling_of_a_unit_reduces_to_read_write_hint_and_schedule_with_the_engine_context_computed_if_feasible` (`units-2.toml:103`): the spelling reduces to the lists, the hint and the schedule. One provisional row describes the body: `inside_a_fiber_a_unit_is_a_pure_function_from_column_values_to_column_values` (`units-2.toml:77`).

Four rows, read as written, say a unit has an `execute` that takes a context, and the form takes that away from a unit whose body is its value function. One is ratified: `a_unit_declares_its_name_read_set_write_set_hint_commutativity_and_an_execute_function` (`units.toml:4`). Three are provisional: `execute_receives_a_context_and_nothing_else` (`units-2.toml:48`), `the_context_of_a_unit_is_a_generic_associated_type_over_a_frame_lifetime_the_scheduler_owns` (`units-2.toml:127`), and `a_pure_per_record_map_may_be_declared_as_an_opt_in_fusion_contract_beside_execute` (`units-2.toml:4`), which has the value function beside `execute` and not in its place. For a unit with a positional body the four still hold in the form, on `Positional`.

The three bans the first sketch broke are kept: `std_any_is_banned_so_there_is_no_type_id_and_no_type_erasure`, `dyn_is_banned_entirely_with_no_exceptions`, and `the_monomorphised_function_is_the_type_identity_of_a_unit_and_llvm_devirtualises_the_fused_pipeline_into_straight_line_code`. No type is forgotten between the unit and its columns, and the emitted code is below.

The maintainer's call of 16 November is later than every row above. Whether it amends the four is his to say, and it is asked of him as Q1 below. The second ratified row also carries the sentence "`BuilderInput` and `HasSchedule` impls stay explicit either way", which the one-impl spelling goes against for `BuilderInput`. That is Q2.

The hint stays declared in both spellings, since the first ratified row says a unit must declare it.

## The facts, each with its predicate

Every line holds under the predicate at the end, and under nothing wider. A line that names a narrower condition holds under that as well.

1. A unit that implements `BuilderInput`, `WorkUnit` and `RecordOp`, and no `Positional`, is accepted by `.with(unit)` and run by `Scheduler::run`. Its source names no context type, and its crate takes no engine crate (`diagnostics/unit-dependency-tree.txt`).
2. The body's signature names `ReadVals<'_, Self::Read>` and `WriteVals<Self::Write>`, both structs of the api, and no trait.
3. Two such units, `Drift` and `Warm`, registered beside a positional unit in one carrier, leave the stores that the same two units with positional bodies leave, after each of four frames, and the stores match values worked by hand after the first and the last frame. `cargo test -q -p check --test run`: 1 passed. The first frame was cut into two windows.
4. The case that must fail: with the engine copy's feature `mutant-short-window`, where `resolve_map` stops one record short of the window, the same test fails at record 1535, `Pos(1535)` against `Pos(6133)` (`diagnostics/mutant-short-window.txt`).
5. A twin that reads a column its read list does not hold does not compile: `error[E0277]: the read list Cons<Column<Pos>, Cons<Column<Vel>, hilavitkutin_api::Empty>> has no value for Column<Glow>` (`check/ui/fail/read_undeclared.stderr`). The twin that reads `Vel` compiles.
6. A twin that returns a value its write list does not hold does not compile: `error[E0271]: type mismatch resolving <Val as Carrier>::Of<Written<Column<Glow>>> == Pos` (`check/ui/fail/write_undeclared.stderr`). The twin that returns `Glow` compiles. `cargo test -q -p check --test fixtures`: 2 passed.
7. The emitted code of each value body over the engine's context holds no call and no jump, direct or indirect: the value function is inlined into the loop. The same search finds the control's `jmpq *%rax` in the same file (`diagnostics/indirect-branch-search.txt`, `diagnostics/probe.s`). Holds for the profile `probe` (opt-level 3, fat lto, one codegen unit), x86-64 linux, the body behind a probe symbol.
8. The one-impl spelling runs the same test to the same result: `cargo test -q -p check --test run --features one-impl`, 1 passed. The three blankets cohere with every hand-written `BuilderInput`, `WorkUnit` and `RecordOp` impl in the graph: the api's own stores, the engine's fused unit, kit, providers, and the positional units of `native`.
9. A unit that offers both bodies does not run: `error[E0283]: type annotations needed`, `multiple impls satisfying Both: RunUnit<ColumnBinding<Pos, BindingNil>, _> found` (`check/ui/attempts/d1_both_bodies.stderr`). Every value-form unit shipped today offers both, since its `execute` is the one line over `apply`. In the form that `execute` and its `Ctx` are deleted.
10. A unit that is a declaration alone, with no body, is accepted by `.with` and refused at `run`: `error[E0277]: the trait bound Bodiless: RunUnit<ColumnBinding<Pos, BindingNil>, _> is not satisfied` (`check/ui/attempts/d2_no_body.stderr`). `WorkUnit` alone no longer says a unit can run.

## What was tried, in order

m1. `WorkUnit` as the declaration, `Positional` for the context and `execute`, the engine's `RunUnit` in two impls. Not tried before. Works, and is everything above. It compiled on the first build.

u1. The one-impl trait `Unit` with three blankets in the api. Not tried before. Works (fact 8).

s1. The one-impl trait over a schedule parameter, so a unit on a virtual firing is one impl too. FAILS, `diagnostics/attempt-s1-unit-over-a-schedule.txt`: `error[E0119]: conflicting implementations of trait BuilderInput for type ExtensionSurface<_>`, with the note `downstream crates may implement trait attempt_unit_sched::UnitOn<_> for type builder_input::ExtensionSurface<_>`, and the same for every `BuilderInput` impl the api holds. A unit on another schedule takes the three impls.

f1. A unit that is a plain function, `fn drift(read: &ReadVals<'_, R>) -> WriteVals<W>`, with no impl at all. FAILS at the impl that would let the builder take it, `check/ui/attempts/f1_fn_item_unit.stderr`: `error[E0207]: the type parameter Rd is not constrained by the impl trait, self type, or predicates`, and the same for `Wr`. The lists cannot be read off a function's signature in an impl header.

d1 and d2 are facts 9 and 10.

Two routes were weighed and not built, so nothing is claimed of them. One gives `WorkUnit::Ctx` a runner parameter that the scheduler supplies, with an api blanket whose `execute` calls the runner's map. The session's reading is that the blanket's `execute` cannot ask the runner for the map without a bound the trait's own method does not carry, and that a bound on the trait's method would reach every positional unit as well. The other is the positional body, next.

## What is NOT established

A lists-only form for a positional body. A unit that walks the records itself still names the engine's `CtxFor` in its `Positional` impl here. The ratified consumer-spelling row words the context as computed by "an api-side type function". With the context's struct and its bundle types api side, `execute` could take `&Ctx<'_, Self::Read, Self::Write>` and name the lists alone. That means moving the context's layout into the api, and it was not built.

Lists beyond plain columns. `resolve_map` is shipped code, and the frozen authors' units declare accumulators, virtuals and a `Reach` in their lists, but this sketch ran columns only, bare in `Read` and bare or `Written` in `Write`.

A unit on `On<V>` or `OnMeta<V>` in the form. The three impls are written the same way over a schedule, and none was run.

`run_parallel` and `run_fused`. Neither was run here, and how each reaches a unit's body was not read. Whether the `RecordOp` fold of `run_fused` and the `ByValue` step pick the same units was not looked at.

The engine's own tests, the conformance crate and the frozen authors against the edited copies. The `#[cfg(test)]` module of the engine copy's `dispatch/order.rs` still holds units in the shipped shape and does not build under the split.

The diagnostics a consumer meets. Fact 9's message names the engine's two impls in full and is long.

Any timing. A sketch times nothing, and fact 7 is about the absence of branches and not about cost.

## The two questions for the maintainer

Neither could be filed from the session that wrote this: the ticket tool builds in the container's one build directory, and a stuck build held its lock for the whole session. Both are written out in full, each with its options, in `mock/research/202611191600_unit-names-its-lists-only-report/REPORT.md`, for the coordinator to file with the maintainer as holder.

Q1, whether a unit whose body is its value function may have no `execute` and no context, so that the four rows are amended through the canon flow.

Q2, whether such a unit is three impls of shipped traits or one impl of a new `Unit` with blankets.

## Predicate

Holds for: toolchain `nightly-2026-05-28`; no feature gate added to any crate; arvo at `1b2796e9`, notko and notko-hlist on dev at `1b2a5723`; the api, the engine and the providers source as copies of `dev` at `07a9e9a3` plus the edits in `EDITS.md`; kit as shipped source with its build script not run; built without the repository's rustc wrapper flags; dev profile for the run, the twins and the attempts; profile `probe` (opt-level 3, fat lto, one codegen unit), x86-64 linux, for the emitted code; schedule `Always`; lists of plain columns (bare in `Read`; bare and `Written` in `Write`); `Scheduler::run` on one thread; 4096 records, two windows in the first frame, four frames; store `ArenaColumnStorage<StaticBumpMemory>`; a carrier of one positional unit and two value units.

Not checked, and so not held: everything under "What is NOT established"; long lists against the recursion limit; a build under the wrapper flags; other targets; the repository's lints and `cargo mock test` over the copies.
