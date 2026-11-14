# Sketch findings: a column, plain or buffered, registers through one carrier, and the engine takes it in one arm

**Date:** 2026-11-13
**Outcome:** WORKS, with one part refused: the carrier's `into_inner` is not a `const fn`
**Unblocks:** the plain column registered with the value its records start from, on the carrier and the tiers the buffered column already has, with no second copy of either

## Hypothesis

`202611132225_buffered-start-by-type` showed the sequence of tiers for the buffered marker alone: an inherent `start()` under the trivial constructor, a trait's `start()` under the default, neither for a type with neither. The plain column needs the same. Copying the carrier and the trait for it would be two of each.

What was believed is that one sealed trait over both markers, `Starts` with the element type as `Value`, carries all of it. The carrier is `Started<S: Starts>` holding an `S::Value`. The default tier is one blanket impl over `S: Starts` where `S::Value: Default`. On the engine's side one arm of the store list takes `Started<S>`, and a trait of the engine's own gives the kind the column is kept as, so the arm knows a plain column has one reservation to write and a buffered one two. Three things would settle it: the tiers still resolve in the same sequence through the blanket impl, the constructors stay `const`, and the one arm sits beside the arm of another carrier without overlapping it.

## What was tried, in order

1. `src/lib.rs` with `Started::into_inner` as a `const fn`. Refused:

```
src/lib.rs:50:29: error[E0493]: destructor of `Started<S>` cannot be evaluated at compile-time: the destructor for this type cannot be evaluated in constant functions
```

Moving the one field out of `self` in a `const fn` leaves the compiler unable to say the rest needs no destructor, for a generic `S::Value`. It goes with the `const_destruct` feature and a `[const] Destruct` bound on the value, and that bound would then sit on every caller. Nothing calls `into_inner` at compile time: the engine's drain is the one caller and it runs at build. So `into_inner` is left a plain function, which is what is committed.

2. `src/lib.rs` as committed. `cargo test`: six tests pass. A type with both starts from 7, the constructor, through `Column` and through `Buffered`. A type with a default alone starts from 9 through the blanket impl. The three constants at the foot of the file are made at compile time: `Column::<Built>::start()`, `Buffered::<Both, Exchange>::start()` and `Column::<Neither>::start_from(Neither(3))`.

3. The mutant of the tiers, `cargo test --features mutant-no-inherent`:

```
---- tests::a_type_with_both_starts_from_its_constructor stdout ----
assertion `left == right` failed
  left: Both(9)
 right: Both(7)
test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

4. The mutant of the arm, `cargo test --features mutant-plain-twice`, which keeps a plain column as a buffered one is kept:

```
---- tests::one_arm_writes_a_plain_column_once_and_a_buffered_one_twice stdout ----
assertion `left == right` failed
  left: [2, 2, 2, 0]
 right: [1, 2, 2, 0]
test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

5. The refusal of a type with neither, `cargo build --features refused-no-start`:

```
error[E0599]: the associated function or constant `start` exists for struct `Column<Neither>`, but its trait bounds were not satisfied
   --> src/lib.rs:232:24
    |
 32 | pub struct Column<T>(PhantomData<T>);
    | -------------------- associated function or constant `start` not found for this struct because it doesn't satisfy `Column<Neither>: StartsFromDefault`
...
218 | pub struct Neither(pub u32);
    | ------------------ doesn't satisfy `Neither: Default` or `Neither: HasTrivialCtor`
...
note: if you're trying to build a new `Column<Neither>`, consider using `Column::<T>::new` which returns `Column<_>`
note: trait bound `Neither: HasTrivialCtor` was not satisfied
note: the following trait bounds were not satisfied:
      `&Column<Neither>: Starts`
      `&mut Column<Neither>: Starts`
      `Neither: Default`
help: consider annotating `Neither` with `#[derive(Default)]`
```

6. The refusal of the bare marker, `cargo build --features refused-bare-marker`:

```
error[E0277]: the trait bound `Column<Built>: Input` is not satisfied
   --> src/lib.rs:237:10
    |
237 |     with(Column::<Built>::new());
    |     ---- ^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
help: the following other types implement trait `Input`
180 | impl<S: Starts> Input for Started<S> {}
181 | impl<T> Input for Resource<T> {}
```

## The result

One sealed trait over the two markers carries the carrier, the default tier and the engine's arm. `start()` on either marker resolves to the trivial constructor where the element type has one, to the default where it has that alone, and does not compile where it has neither. `start_from(value)` is there for every type. `start_from` and the constructor tier are `const fn`. The default tier's trait takes no associated type and no impl but the one blanket impl, and being bounded on the sealed trait it can be implemented by nobody else. One arm over `Started<S>` with `S` bounded on the engine's own trait of kinds sits beside the arm of another carrier, and reads how many reservations to write from the kind. The bare marker is no input.

## What is NOT established

The sketch's arm counts writes. It does not reserve or write memory, so that the engine's drain can call its reservation with `S::Value` and `S::Kind` in place of the two it names today is read off the types and not run here.

The refusal in attempt 5 carries one note the buffered sketch did not: the compiler suggests `Column::<T>::new`, the marker's own constructor, to somebody who wrote `start()`. It is a note beside the two that name the missing tiers, and the marker it suggests is then refused by `with` as attempt 6 shows. Whether the note misleads is a matter of the diagnostic text the api can attach and was not tried.

The limit the buffered sketch names stands here unchanged: the choice is made where `start()` is written, from what is known of the type there.
