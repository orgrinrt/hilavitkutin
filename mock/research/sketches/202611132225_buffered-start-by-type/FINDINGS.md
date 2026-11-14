# Sketch findings: one constructor name can follow the element type through a fixed sequence of tiers

**Date:** 2026-11-13
**Outcome:** WORKS
**Unblocks:** a buffered column whose registration says what its records start from, chosen by the element type, with the constructor absent where the type gives no start

## Hypothesis

A buffered column is registered with the value every record of it starts from. Op's words give the sequence: the type's null bit where it has one, else its trivial constructor, else its default, and where it has none of them the explicit value is the only constructor there is. A constructor that does not apply to a type should not exist for it.

Two blanket impls of one trait, one over `T: HasTrivialCtor` and one over `T: Default`, overlap, and choosing between them by rank is specialisation, which is forbidden here. What was believed is that the sequence can be had without it: an inherent associated function is looked up before a trait's function of the same name, and an inherent impl whose bound does not hold for the type at hand is passed over. So the first tier is an inherent function under a bound and the second is a trait's function under another. What would settle it is a type with both traits and two different values, and a type with neither.

## What was tried, in order

1. `src/lib.rs` as committed: `Buffered::<T>::start()` inherent under `T: HasTrivialCtor`, and the trait `StartsFromDefault` with the same function under `T: Default`, on `notko`'s own `HasTrivialCtor`. Four element types: `Built` (constructor alone, value 7), `Defaulted` (default alone, value 9), `Both` (constructor 7, default 9) and `Neither`. `cargo test`: five tests pass. `Both` starts from 7, so the inherent tier is the one found.

2. The mutant, `cargo test --features mutant-no-inherent`, which takes the inherent tier away:

```
---- tests::a_type_with_both_starts_from_its_constructor stdout ----
assertion `left == right` failed
  left: Both(9)
 right: Both(7)
test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

So the test of the sequence can fail, and it fails for the reason it is there for.

3. The refusal, `cargo build --features refused-no-start`, a type with neither:

```
error[E0599]: the associated function or constant `start` exists for struct `Buffered<Neither>`, but its trait bounds were not satisfied
   --> src/lib.rs:104:26
    |
 16 | pub struct Buffered<T>(PhantomData<T>);
    | ---------------------- associated function or constant `start` not found for this struct because it doesn't satisfy `Buffered<Neither>: StartsFromDefault`
...
 89 | pub struct Neither(pub u32);
    | ------------------ doesn't satisfy `Neither: Default` or `Neither: HasTrivialCtor`
...
note: trait bound `Neither: HasTrivialCtor` was not satisfied
note: trait bound `Neither: Default` was not satisfied
help: consider annotating `Neither` with `#[derive(Default)]`
```

The compiler names both tiers the type lacks.

## The result

`Buffered::<T>::start()` resolves to the trivial constructor where `T: HasTrivialCtor`, to the default where `T: Default` alone, to the constructor where both hold, and does not compile where neither holds. `start_from(value)` is there for every type. No unstable feature is involved. The trait has to be in scope where the default tier is called.

## What is NOT established

The choice is made where the call is written, from what is known of the type there. A generic function that knows only `T: Default` gets the default, also for a type that has a constructor too (`a_generic_caller_chooses_by_its_own_bounds`). A store is named with a concrete element type where it is registered, so this is the edge and not the rule, and it is a limit all the same.

The null bit is not a tier here. `notko` has `NicheFilled`, which marks a type whose all-zeros pattern is not one of its values, so it cannot give a column of that type a start. The types that carry a null value of their own, `MaybeNull<T>` and `Slot<T>`, and `Maybe<T>`, say so through `Default`, which the second tier already reads. A trait that says a type's all-zeros pattern is a value of it, which would let a column start from zeroed memory with no value written per record, does not exist in `notko`.

A third tier ahead of the two cannot be a second inherent function of the same name, since the two inherent impls would overlap. It would need one more level of lookup, by reference, and that was not tried.
