# Sketch findings: a column in two sides takes its swap from a type parameter, and the positional write can follow the member a column is listed under

**Date:** 2026-11-13
**Outcome:** WORKS
**Unblocks:** two forms the doc changelist of round `202611130617-h6mb` names in its fourth writing: the buffered store with its two modes (change 1), and both behaviours of "changed" under the positional write (change 4).

## Hypothesis

Two things were believed and neither had been tried.

The first is that a column held in two sides can take the way its live side becomes the frozen one from a type parameter of its marker, with the two ways the founding research names for loimu's swap buffer (`mock/design_rounds/202604200055/202603141800_research.loimu-storage.md:240-272`): a copy of the live side into the frozen one, and the two sides trading places. What would settle it is one store type, generic over the mode, whose tests pass under both and whose regions never move under either.

The second is that the positional `write::<T, _>` can reach a column a unit lists bare, under `Written` or under `Differs`, with the two generic arguments the call has on the trunk, so that a declaration on the member governs the positional write as it governs a value function's return. The worry was inference: the member and the index would both be unknown at the call.

The sketch `202611130617_value-form-and-domains` already has a store with a front, a back and a staged flag in its plumbing. It has one way of swapping and no positional write through a wrapper.

## What was tried, in order

The crate is `src/lib.rs` with its tests in `src/tests.rs`, its own workspace, `no_std` outside the tests, no dependency. `cargo test` runs ten tests. Three features each break one thing the tests are meant to notice, and two each turn on a function that must not compile.

### 1. The two sides and the mode

`Sides<'s, T, M>` borrows two regions of one length and keeps which of them is frozen and whether a write changed anything. `step` applies a function at each record it is given: the function reads the frozen side through a `Window` (the record and its neighbours) and what it returns is stored on the live side. `close` is the frame boundary. `M: BufferMode` carries one constant, whether the boundary copies, and `close` branches on it, so each mode is its own monomorphisation.

It compiled and passed at the first attempt. What the tests hold it to:

A step over the store it writes gives one result whether the records are visited forward or backward, in both modes, for one to three frames, and that result is the one worked out by hand from the frozen side. With `--features mutant-read-live`, where the window reads the live side, four tests go red, the first with:

```
assertion `left == right` failed
  left: [1, 1, 6, 11, 11, 11, 13, 15]
 right: [14, 13, 12, 12, 7, 2, 2, 2]
```

Both modes give the same values for one to four frames when the writer writes every record.

Under `Exchange` the frozen side's address alternates between the two regions and neither region is copied or moved. With `--features mutant-exchange-copies` that test goes red on the address.

Under `Snapshot` the frozen side is one region in every frame.

A store nobody wrote is left alone by `close`, and so is one whose writer returned what it read under the differs rule; under the written rule the same writer swaps. With `--features mutant-swap-when-unwritten` both tests go red.

Two writers of one store in one frame both read the frozen side, and the later one's values are what the boundary brings in.

### 2. Why `Exchange` asks a total write

A writer that writes every record in frame one and record 0 alone in frame two leaves, after the second boundary:

```
Snapshot: [8, 7, 7, 7]
Exchange: [8, 0, 0, 0]
```

Under `Exchange` the side a frame writes into holds the values of two frames before, so a record the writer left reads back stale a frame later. Nothing in the sketch's raw `step` stops a partial write. In the api a value function is applied at every record of the unit's range, which is what makes it total, and the positional write is not offered for a buffered store (the second refusal below).

### 3. The positional write through three members

`WritesColumn<T, I>` is implemented for a list whose first cell is `Column<T>` (index `AtPlain`), `Written<Column<T>>` (`AtWritten`) or `Differs<Column<T>>` (`AtDiffers`, asking `T: PartialEq`), and for any list whose tail has it (`Next<I>`). The kind of member is folded into the index.

On the trunk the call already has one inferred index, the one a runner's context finds the column's pointer by in a bundle of its own (`ResolveColumnWrite<T, I>`). A second inferred argument would change every call that writes `write::<T, _>`. So the sketch has a bundle too (`PtrCons`, `ResolveSlot<T, I>`), in another sequence than the list, and one trait over the pair of them, `WriteAt<T, (Ik, Ib)>`, implemented once for `(W, P)`. `write<T, I>` is bounded on that, the call has two generic arguments as before, and the second is inferred as the pair.

Both steps compiled at the first attempt. One unit with a bare, a `Written` and a `Differs` column in its list calls `write::<Heat, _>`, `write::<Wet, _>` and `write::<Lit, _>`; an equal value sets the mark for the first two and not for the third, and a value that differs sets it for the third. The inference picks one candidate at every cell because the candidate whose member does not match the head, and the tail candidate past the end of the list, are both dropped by the solver.

### 4. The two refusals

A `Differs` member over a value type with no comparison (`cargo build --features refused-differs-without-eq`):

```
error[E0277]: the trait bound `Cons<Differs<Column<NoEq>>, Empty>: WritesColumn<NoEq, _>` is not satisfied
   --> src/lib.rs:320:40
    |
320 |         unsafe { w.write::<NoEq, _>(0, NoEq(1)) };
    |                    -----               ^^^^^^^ unsatisfied trait bound
...
note: required for `(Cons<Differs<Column<NoEq>>, Empty>, PtrCons<NoEq, PtrNil>)` to implement `WriteAt<NoEq, (_, Here)>`
```

The same value type under `Written` compiles and runs (a test).

A positional write of a buffered store (`cargo build --features refused-positional-write-of-buffered`):

```
error[E0277]: the trait bound `Empty: WritesColumn<u32, _>` is not satisfied
   --> src/lib.rs:333:39
    |
333 |         unsafe { w.write::<u32, _>(0, 1) };
    |                    -----              ^ unsatisfied trait bound
...
note: required for `Cons<Buffered<u32, Exchange>, Empty>` to implement `WritesColumn<u32, Next<_>>`
```

## The result

A two-sided store generic over a sealed mode type works as one type with two monomorphisations, and the regions stay where they are under both modes. A reader and the writer of such a store need no sequence between them: the result is the same in either. `Exchange` is right only under a total write, and `Snapshot` is right under any.

The positional write can follow the member with no change to how it is called. A list may hold all three members at once.

## What is NOT established

Neither refusal reads well. The first does not name `PartialEq`, and the second names the end of the list. The api's own bound would carry a message the way `Holds` does, and that is untried here.

The sketch's bundle and bridge are its own. That the real `ResolveColumnWrite` composes with the list's trait the way `ResolveSlot` does here is expected from the shape and untried against the engine's context.

Nothing here is the planner. That a reader and a writer of a buffered store get no edge between them, that the boundary's swap seeds the next frame's dirty set, and that a dispatch binds the frozen and the live base at the top of a frame are stated in the design and wait for the source window's tests.

No cost is measured. Which mode is cheaper at which size, and whether a compare on the positional write pays, are benches in `mock/benches/`.
