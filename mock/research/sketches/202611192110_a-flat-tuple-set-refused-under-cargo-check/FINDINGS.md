# Sketch findings: a flat tuple of pragma types is refused under a plain check

**Date:** 2026-11-19
**Outcome:** WORKS
**Unblocks:** the pragma round states a set as the flat tuple the canon's builder spells, keeps the requirement as an associated type of the pragma, and has a broken set refused by `cargo check` at the line that names it

## Hypothesis

The sketch `202611191545_pragma-requirements-as-bounds` left four things open: the list was a nested pair, the constant route refused only at a build, a pragma with a value was not in it, and one refusal text served a pragma's group and its requirement both. This sketch tries all four at once. A set is a flat tuple, `(A, B, C)`, with `Set` implemented per arity by one macro. A pragma names its requirement as a type, `type Requires = All<(FastMath,)>` or `Any<(Pgo, Bolt)>`, and its group as a type that carries the group's own refusal. The set folds its members into one constant, `VERDICT`, the first thing wrong or the empty string. A script asserts over it in a `const _` item, which is no generic and so is evaluated by a check. A second assertion refuses a set that holds a pragma of workspace reach in a script that is not the workspace's. `ParallelCodegen<const N: u8>` stands for a pragma with a value.

`no_std`, no allocation, no feature gate. Built under `nightly-2026-05-28`, from this directory.

## What was tried

### 1. Two coherent sets: run

`cargo run --quiet --offline --example ok`

```
math-peephole fast-math profiling pgo bolt mold -Zthreads=4
```

The first set holds Profiling with both Pgo and Bolt, the case the bounds route of the earlier sketch refused.

### 2. Every broken set: refused by a check, in the pragma's or the group's own words

`cargo check --offline --examples --keep-going`

```
error[E0080]: evaluation panicked: Profiling stands on Pgo or Bolt, and the set holds neither
   --> examples/breaks_any.rs:3:15
    |
  3 | const _: () = checked::<(Profiling, FastMath)>();
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ evaluation of `_` failed inside this call

error[E0080]: evaluation panicked: the set holds two linkers
   --> examples/breaks_group.rs:3:15
    |
  3 | const _: () = checked::<(Mold, Pgo, Wild)>();
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ evaluation of `_` failed inside this call

error[E0080]: evaluation panicked: the set holds a pragma that reaches the whole workspace, and this script is not the workspace's
   --> examples/breaks_reach.rs:4:15
    |
  4 | const _: () = crate_reach::<(FastMath, ThinLto)>();
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ evaluation of `_` failed inside this call

error[E0080]: evaluation panicked: MathPeephole stands on FastMath, which the set does not hold
   --> examples/breaks_all.rs:3:15
    |
  3 | const _: () = checked::<(MathPeephole,)>();
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ evaluation of `_` failed inside this call
```

The example `ok` checked clean in the same run. Each error points at the line that names the set.

## The result

A flat tuple is a set, with one macro arm per arity and no nested list. The requirement stays an associated type of the pragma, as the canon's row has it, and is read as two masks. The group is a type with its own refusal, so a pragma that is in a group and has a requirement says the right one of two sentences. A pragma with a value is a type with a const parameter and takes one bit whatever the value. A `const _` item makes a check report what the earlier sketch saw only at a build.

The four broken sets are the cases that must fail, and each was seen failing.

## What is NOT established

Two pragmas given the same bit by mistake are not caught here; the crate assigns the bits of its own pragmas in one place, and a test over that list is what holds them apart.

The fold reports the first thing wrong, not every one.

Arity stops where the macro's arms stop, five here.

Nothing here touches what a target admits or what the machine has; those are known only when the script runs.
