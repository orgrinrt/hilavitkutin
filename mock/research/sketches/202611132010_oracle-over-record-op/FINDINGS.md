# Sketch findings: a one-thread runner can take value-form units through `RecordOp::apply` alone

**Date:** 2026-11-13
**Outcome:** WORKS
**Unblocks:** the reference runner of the instruments round can be written against `hilavitkutin-api` with no engine dependency and no context type of its own, and the author units it runs are the ones the scheduler runs, unchanged.

## Hypothesis

A unit in the value form gives everything a runner needs through `RecordOp::apply`: the read product in, the write product out. So a runner that keeps the stores itself, gathers the read product at one record, calls `apply` and stores what came back, never has to call `execute` and never has to build a context. If that holds, the oracle the scheduler is held to needs nothing of the engine, and a unit needs no second form to run under it.

What would settle it: five units written as a consumer writes them (each names the engine's `CtxFor` in `Ctx` and writes `execute` as `ctx.each().map(|r| self.apply(r))`) run under such a runner over several frames, and the stores equal a model written by hand in plain integers after every frame. The units cover a plain column written in place, a reach on two axes over a buffered column the same unit writes, an append to an accumulator, a fired virtual, and a unit scheduled `On` that virtual.

## What was tried, in order

The runner is `src/lib.rs`. A store is kept in storage the caller lends (`Lent<S>`), and the world is a nested pair list of those. `Find<'w, S, P>` locates a store in the world by a place the compiler infers, the same way the api's own `PlaceOf` does. `Reads` and `Writes` say per member how to read at a record and how to store a returned value; `Gather` and `Scatter` walk a unit's read and write lists; `Due` says whether a schedule runs in the frame; `Units` runs a nested pair list of units in the sequence given. `U: HasSchedule + RecordOp<U::Sched>` recovers the schedule, so one impl covers `Always` and `On<V>` units.

The first build stopped on the two macros that write the `Writes` impls:

```
src/lib.rs:393:44: error: local ambiguity when calling macro column_writes: multiple parsing options: built-in NTs tt ('bound') or 1 other option.
src/lib.rs:412:49: error: local ambiguity when calling macro buffered_writes: multiple parsing options: built-in NTs tt ('bound') or 1 other option.
```

A matcher's own fault and nothing of the api's: the bounds are bracketed now.

The second build found two things worth keeping:

```
src/digest.rs:15:14: error[E0277]: the trait bound `Picker: Project<arvo_strategy::::container::{impl#5}::{constant#0}, ..., arvo_strategy::::container::{impl#5}::{constant#1}, ...>` is not satisfied: unsatisfied trait bound
src/digest.rs:15:21: error[E0599]: the associated function or constant `new` exists for struct `Fnv1a<_>`, but its trait bounds were not satisfied: associated function or constant cannot be called on `Fnv1a<_>` due to unsatisfied trait bounds
src/lib.rs:490:67: error[E0507]: cannot move out of `vals.head` which is behind a shared reference: move occurs because `vals.head` has type `<H as Writable>::Val`, which does not implement the `Copy` trait
```

`Fnv1a::new()` does not take its width from the field it is stored in; `Fnv1a::<64>::new()` does. And `Writable::Val` is not `Copy` by the api's bounds, so the write product is taken by value and consumed cell by cell, which is the right reading anyway: a value a function returned is stored once.

The third build was clean. `tests/frames.rs` then runs six frames of `(Drift, (Spread, (Cross, (Alarm, (Tally, ())))))` over 48 records, a plane eight wide and six tall, against the hand model:

```
running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The test also requires that the run does what it claims: the virtual fired in some frames and not in others, so `Tally` both ran and rested, and the accumulator took values.

The control is the `reads-live` feature, under which the runner reads the live side of a buffered column where it should read the frozen one. `cargo test --features reads-live`:

```
thread 'the_runner_equals_the_hand_model_after_every_frame' (16246) panicked at tests/frames.rs:172:9:
assertion `left == right` failed: heat after frame 0
  left: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
 right: [8, 18, 24, 19, 25, 20, 26, 20, 11, 29, 20, 22, 24, 26, 28, 28, 15, 34, 25, 16, 29, 20, 22, 21, 19, 28, 30, 21, 34, 25, 16, 25, 12, 22, 24, 26, 28, 30, 21, 29, 10, 14, 20, 15, 21, 16, 22, 22]
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
```

So the comparison sees a runner that reads the wrong side, at the first frame.

The digest is `src/digest.rs`: `arvo-hash`'s streaming `Fnv1a<64>` over 64-bit words, each taken least significant byte first. The second test holds it against FNV-1a written out in the test over the same bytes, against the offset basis for no words, and requires one changed bit to change the digest.

## The result

A runner written against `hilavitkutin-api`, `arvo`, `arvo-hash`, `notko` and `notko-hlist` gathers and scatters for value-form units with every place inferred, and runs them through `apply` alone. The runner itself names nothing of the engine. The crate depends on the engine only because the units in `src/units.rs` name `CtxFor`, as every unit does today.

It covers `Column`, `Buffered` in both modes, `Accum`, `Virtual` and `Resource` as stores; `Column`, `Buffered`, `Reach` over either and `Resource` as read members; `Column`, `Buffered`, `Written` and `Differs` over either, `Accum` and `Virtual` as write members; `Always` and `On<V>` as schedules. `OnMeta` has no `Due` impl, so a maintenance unit in the list is refused by the compiler.

The frame it runs: flags cleared and accumulators emptied, then the units in the sequence listed, each over every record in ascending sequence, an `On<V>` unit only when `V` was fired earlier in the frame, then each buffered column that was written brings its live side in, `Snapshot` by copy and `Exchange` by trading the sides.

The world is a nested pair list and so is the unit list, both built by hand at the call site. No store appears twice in a world; a second `Lent` of the same store leaves the place ambiguous and the build refuses it.

## What is NOT established

Whether the scheduler agrees with this runner on these units, on any of its entries. That is the test the round builds, and the record already holds two places it is expected to part: the incremental skip resting a unit that writes a plain column in place, and `run_parallel` dispatching every unit each pass.

Whether the frame meaning above is the design's in every corner. The sketch models what the engine's design says for the stores it covers; it does not model the change rules (`Written` against `Differs`), which decide who runs next and leave the values alone, nor the write-write sequence by hints, nor a fire by a unit listed after its consumer.

Nothing about cost. The runner is one thread and takes one record at a time on purpose.

Resource reads compile and are not run by the test; no unit here reads one. The `Snapshot` mode compiles and is not run either: the one buffered column is `Exchange`.

Whether the nested pair lists stay pleasant at the size of the frozen author set. Five units and seven stores build in seconds with the default recursion limit.
