# Findings: the value form routed by its dispatch type, record domains, and neighbour reads over a frame-buffered kind

Outcome: WORKS. A unit written as a function over the values of its declared reads at one record, returning the values of its declared writes, reaches a runner with no engine type named anywhere in the unit's crate, and the same source runs unchanged under two runners that give the same stores. A unit in the form the api has today, the one that names the engine's context, sits in the same list beside it. Record domains with their own extents, neighbour reads at declared offsets on more than one axis, and a frame-buffered store kind all hold as types, and each refusal in the table below is a compile error at the unit's own `impl`. Date 2026-11-13, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, the repository's pinned `nightly-2026-05-28`, `notko-hlist` at `8a74dbcda7fbee2d2e57b47474a3b1e6d08650ed`. No `#![feature]` line and no `unsafe` in any crate of the sketch.

This is the sketch behind round `202611130617-h6mb`, the revision of `hilavitkutin-api` in place (job `hv-api-revision`). The round widens `RecordOp` from one value to the unit's read and write lists and makes it the form an author writes first, gives a column a domain, admits reads of a neighbour, and adds a frame-buffered kind beside the existing ones. Three sketches before this one cover neighbouring ground and are not repeated: `202611122252_runner-generic-workunit` (one unit source under several runners, the kernel handed values), `202611122242_frame-view-typestate` (the committed view, the write-only staged handle, the second writer of a staged lane refused through position masks) and `202611130417_branded-morsel-index` (safe indexed reads and writes in the block form). What none of them settles is how a value-form unit and a block-form unit enter the same builder when only one of them may name the engine, and whether domains and declared neighbour reads fit the same lists. That is the question here.

hilavitkutin declares no canon, so nothing was checked against canon rows. The job's brief and the designs under `mock/` are what it was read against.

## How to run it

From this directory: `CARGO_INCREMENTAL=0 cargo test --workspace`. The runner comparison alone is `cargo test -p check --test run`, and the mutant is `cargo test -p check --test run --features mutant-in-place`, which must fail. The fixtures are `cargo test -p author` and `cargo test -p check --test fixtures`; `TRYBUILD=overwrite` rewrites the diagnostics. The paths inside the recorded diagnostics are the machine they were recorded on.

## Hypothesis

Three claims, each of which could have failed on the pinned toolchain.

The first is that the form of a unit can be read off the type its `BuilderInput::Dispatch` names, so that the api offers one declaration trait implemented once per router type, and a runner implements its own running trait for those router types. Two impls on two distinct types cannot overlap, so neither `specialization` nor a negative bound is needed, and `WorkUnit` and `RecordOp` stay two traits with no blanket impl between them.

The second is that a domain can be a parameter of the column marker with a default, that the one domain a unit iterates can be folded out of its read and write lists by the trait solver, and that two domains in one unit are refused where the unit is declared.

The third is that a neighbour read can be a member of the read list like any other, carrying its offset and its value past the extent in its type, with the offset checked against a reach the domain declares.

## What was tried

Six crates. `api` stands in for the revised parts of `hilavitkutin-api` and is the only crate a unit author depends on. `plumb` holds a toy world (a `notko_hlist` product of lanes over a store list) and the gather, scatter and close folds. `engine` and `oracle` are two runners. `author` holds two value-form units and depends on `api` alone, in its tests too. `check` composes all of them, holds one block-form unit, and carries the comparison tests.

The value form in `api`:

```rust
pub trait RecordOp: 'static {
    type Read: ReadList;
    type Write: Along<Self::Read>;
    fn apply(&self, read: &Vals<Self::Read>) -> Vals<Self::Write>;
}
```

`Vals` is `notko_hlist`'s product of values over a list. A read is `r.get(MEMBER)`, bounded on the list holding that member. The writes come back by value, built in the order the write list names them by the `rec!` macro, so a unit cannot leave a declared write without a value. What a member's value is depends on its kind: a `Column` or `Buffered` member is its `T`, an `Accum<T>` member is `Option<T>` (append or not), a `Virtual<V>` member is `bool` (fire or not). A `Resource<T>` and an `Extent<D>` are readable and not writable in this form; `Accum` and `Virtual` are writable and not readable.

Routing. `BuilderInput` keeps its `Dispatch` associated type. A block-form unit names `UnitDispatch<Self>` as today, a value-form unit names `RecordDispatch<Self>`. The api's `Declares<W>` is implemented for `UnitDispatch<W> where W: WorkUnit` and for `RecordDispatch<U> where U: RecordOp`, and gives the stores read, the stores written and the domain. Each runner has a trait of its own (`engine::Runs`, `oracle::Checks`) implemented for the api's router types, and walks a list of units through `W::Dispatch`. The engine implements its trait for both routers. The oracle implements its trait for `RecordDispatch` only.

Domains. `Domain` has `type Axes: Grid` and `type Reach: List`. An axis is a marker; `Grid` is implemented for lists of axes and gives the record count of a shape and the two conversions between a key and an index, with the first axis outermost. `Column<T, D = Records>` and `Buffered<T, D = Records>` take the domain as a defaulted parameter, and `Records` is the one-axis domain every existing column is in. `Extent<D>` is a member of the store list whose value is the run-time shape of `D`; the runner reads the record count of a unit's domain from it. The world in the tests holds two domains of different counts (a 3 by 4 plane and 7 records).

One domain per unit. Each member has a `Where` that is `Free` or `In<D>`. `Join` is implemented for the four agreeing pairs and for no disagreeing one. `ReadList` and `WriteList` fold it along the list, and `Along<R>` joins the write list's result with the read list's and picks the domain.

Neighbour reads. `Near<C, O, E>` is a read-list member: the committed value of store `C` at offset `O` from the record, or `E::OUTSIDE` past the extent. An offset is a list of `Shift<A, const N: isize>`, so an offset on two axes at once is a two-member list; `author` reads one east, one west, one north, one south and one north-east. The struct carries `where C: Committed, <C::Domain as Domain>::Reach: Contains<O>`, so an offset the domain's reach does not list is refused wherever the type is named. `Committed` is implemented for `Buffered` and for nothing else.

The frame-buffered kind. In `plumb` a lane has a front, a back and a staged flag. A read of a `Buffered` member, its own record or a neighbour, reads the front. A write goes to the back and sets the flag. The close, after every unit of the pass, swaps front and back where the flag is set.

The two runners differ on purpose. The engine walks records from the first and scatters each record's writes before gathering the next. The oracle walks records from the last and takes every record's reads before it writes any. If a unit's result depended on the sequence of records, or a neighbour read saw a value written earlier in the same pass, they would disagree.

## The result

`cargo test --workspace` is green: 3 comparison tests, 11 author fixture pairs, 1 runner fixture pair.

`both_runners_leave_the_same_stores` runs `Diffuse` and `Drift` for five passes under each runner over the two-domain world, with the block-form `Tally` in the engine's list, and compares every store. Accumulator appends are compared sorted, since the sequence of appends is not something the value form promises. The test also asserts that the stores moved, that `Tally` ran once a pass, and one position by hand. `one_pass_of_diffuse_matches_the_hand_worked_plane` checks a 2 by 2 plane against values worked on paper in the test's comments, so that two runners wrong in the same way would not pass.

The case that must fail: the feature `mutant-in-place` makes a write to a `Buffered` store land on the front at once. All three tests go red under it:

```
---- one_pass_of_diffuse_matches_the_hand_worked_plane stdout ----
assertion `left == right` failed
  left: [17, -4, -8, 21]
 right: [17, 16, 12, 5]
---- both_runners_leave_the_same_stores stdout ----
assertion `left == right` failed
  left: [-10686, 37009, -58350, 43025, 18093, -83399, 208648, -225028, -12142, 79754, -265758, 536284]
 right: [-1651, 1231, 1930, 393, 809, 2440, 975, -681, 771, 468, -844, -355]
test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
```

The refusals, each with a twin under `ui/pass/` that differs in the offending token and builds:

| Fixture | Compiler said | Where |
|---|---|---|
| `author` 01, a neighbour read names an offset past the reach | E0277, the reach list `does not hold` the offset | the `type Read` line |
| `author` 02, a unit reads members of two domains | E0277, `a unit iterates one domain, and these members are in two` | the `type Read` line |
| `author` 03, a unit writes a domain it does not read | E0277, the same message | the `type Write` line |
| `author` 04, a unit's crate names the engine | E0432, `unresolved import engine` | the `use` line |
| `author` 05, a value unit declares a context | E0437, `type Ctx is not a member of trait RecordOp` | the `type Ctx` line |
| `author` 06, a unit reads a member absent from its list | E0308, expected the list's member, found the other | the `r.get` call |
| `author` 07, a neighbour read names a column written in place | E0277, `Column<Heat, Plane>: Committed` is not satisfied | the `type Read` line |
| `author` 08, a value unit writes a resource | E0277, `Resource<i64>: Writes` is not satisfied | the `type Write` line |
| `author` 09, a unit returns fewer values than it writes | E0271, the product of the write list against the product built | the `rec!` line |
| `author` 10, a value unit reads an accumulator | E0277, `Accum<Crossed>: Reads` is not satisfied | the `type Read` line |
| `author` 11, a list is asked for a store it does not hold | E0277, `the store Other is not in Cons<..>`, with the api's own note | the call that names the list |
| `check` 01, the oracle is handed a block-form unit | E0277, `UnitDispatch<Tally>: Checks<Tally, _, _>` is not satisfied | the `oracle::pass` call |

Fixture 04 is a fact about the dependency graph and not about a type: it holds because `author/Cargo.toml` lists `api` alone. Fixture 05 is the type-level half: the value form has no associated type in which a context could be named. Fixture 06 reports E0308 because the list has one member and the solver fixes the argument's type to it; with a longer list the same mistake reads as E0277 on `Contains`, as fixture 04b of `202611122242_frame-view-typestate` records.

Fixture 11 was added while the round's design was being written, for one question the adoption of the shared list raises. `Contains` is the shared crate's trait and carries the shared crate's message, and an attribute cannot be put on a foreign trait, so the api's own note on a missing store (how to register it) would be lost. `Holds<S>` is an api trait with that note, implemented by one blanket over every list that `Contains<S>`, and the blanket carries `#[diagnostic::do_not_recommend]`, so the compiler reports the unmet `Holds` bound with the api's message and does not descend to `Contains`. The recorded diagnostic shows the api's message and note and no line of the shared crate's.

Two things that might not have compiled and did. `Grid: Project<Prod<Val> = <Self as Grid>::Shape>`, an equality on a generic associated type in a supertrait position, is accepted and lets a shape be read through `Has::pick` with no cast. The position witnesses of every gather and scatter are inferred through the unit list with no turbofish at either `pass` call site; the one place a caller writes a type argument is `World::fill`, to name the domain.

## What is NOT established

That a second writer of one `Buffered` store is refused. The sketch's engine accepts two (the third comparison test relies on it to read the committed frame twice in one pass). The refusal rests on the position masks of `202611122242_frame-view-typestate`, fixtures 05 and 06 there, and is to be shown again on the real builder with its own compile-fail pair.

Anything about the block form's own reads and writes. `EngineCtx` here carries a tally only. Safe exclusive writes in the block form rest on `202611130417_branded-morsel-index` and on the staged handle of `202611122242_frame-view-typestate`. The domain a block-form unit iterates is given as `Records` in `Declares`; folding it from the unit's access sets the way `Along` does is not shown.

That the real engine's planner, fusion, morsel dispatch or parallel dispatch accept units routed this way. The sketch's runners are single loops with no plan. `plumb` uses `Vec` and `std`; the folds are shown, their no-alloc form over the engine's column pointers is not.

That a value-form unit can declare a schedule other than the default, or a scheduling hint. Where those are named (on the router, or on the unit) is a decision for the design, and nothing here tests it.

Any cost. No timing was taken and none may be read out of this directory. `Grid::key` divides once per axis per record and `Offset::step` rebuilds a key per neighbour read; a runner that walks keys incrementally would do neither, and whether the generated code of a neighbour read is acceptable is a question for a bench under `mock/benches/`.

That neighbour reads over a column written in place could be admitted later. Fixture 07 shows they are refused now. They are unsound under any dispatch that does not fix the sequence of records, which is the reason for the refusal, and the sketch does not explore a kind for which they would be sound.

Bare numbers stand in for arvo's types throughout, as in the sketches before this one.
