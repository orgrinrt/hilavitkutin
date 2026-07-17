# Sketch: can a WorkUnit's declaration half be lifted without changing WorkUnit?

**Date:** 2026-07-17
**Round:** `202607171733_topic.plannable-declaration-split`
**Toolchain:** the workspace pin (`rust-toolchain.toml`), plus a bare `rustc --edition 2021` for the
two-crate coherence arm, because the question is a trait-solver question and not a Cargo question.

## Hypothesis

The runner-agnostic half of `WorkUnit` (`Read`, `Write`, `Hint`) can be lifted into a separate trait
that a non-fiber runner implements by hand, with a blanket impl granting it to every existing
`WorkUnit` for free, such that:

1. `WorkUnit` itself is not modified,
2. no existing implementor gains an impl block,
3. a downstream type that is not a `WorkUnit` can still implement the new trait, across a crate
   boundary, without a coherence conflict.

If any of the three fails, the fallback is a supertrait (`WorkUnit: Plannable`), which costs every
implementor a mechanical split of one impl block into two.

## Outcome: WORKS

All three hold. `WorkUnit` is untouched and the fallback is not needed.

### The shape that works (`upstream.rs`, `downstream.rs`)

```rust
pub trait Plannable<Schedule = Always> { type Read: AccessSet; /* Write, Hint */ }
impl<W: WorkUnit<S>, S> Plannable<S> for W { type Read = <W as WorkUnit<S>>::Read; }
```

The load-bearing detail is that `Schedule` is a **parameter of `Plannable`**, not only of the bound.
`S` therefore appears in the impl's trait reference (`Plannable<S>`), which is what constrains it.

Verified in `downstream.rs`, compiled against `upstream.rs` as a separate crate:

- `AdaptWu`, an `impl WorkUnit<On<ScheduleEnd>>` with no `Plannable` impl of its own, is accepted at
  a `T: Plannable<S>` bound. The blanket covers non-`Always` schedules, so nothing needs an explicit
  impl. This is strictly more general than `HasSchedule` (`work_unit.rs:145`), whose blanket pins
  `Always` and requires an explicit impl per `On<V>` unit.
- `DepthPrepass`, a local type that is NOT a `WorkUnit`, hand-implements `Plannable` and coheres with
  the upstream blanket. No E0119.
- One generic function takes both, which is the point: the plan stage reads the declaration and never
  calls `execute`.

### The negative arm (`e0207_negative.rs`)

**FAILS WITH E0207**, as required, and this is why the schedule must sit in the trait's parameters:

```rust
pub trait Plannable { type Read: AccessSet; }              // schedule NOT a parameter
impl<W: WorkUnit<S>, S> Plannable for W { /* ... */ }      // error[E0207]
```

> the type parameter `S` is not constrained by the impl trait, self type, or predicates

A reader who writes `Plannable` without the schedule parameter will hit this and may conclude the
whole approach is impossible. It is not; the parameter is the fix.

### Why the coherence arm passes, stated so it is not mistaken for luck

The blanket impl and `impl Plannable for DepthPrepass` overlap only if `DepthPrepass: WorkUnit<S>`
for some `S`. `DepthPrepass` is local to the downstream crate and `WorkUnit` is foreign to it, so the
orphan rule means only that crate can add such an impl, and it does not. The compiler can therefore
prove disjointness.

The standing obligation this creates: **a downstream type must not be both.** If a consumer later
adds `impl WorkUnit<S> for DepthPrepass`, its hand-written `Plannable` impl becomes a conflict. That
is the correct failure (a type with a fiber execution model should take the blanket, not hand-roll a
second declaration), but it is a real edge and belongs in `Plannable`'s doc comment rather than being
discovered by a downstream author at a red build.

## What this does not establish

- The sketch models `AccessSet`, `SchedulingHint`, and the hint markers as stand-ins. It proves the
  trait-solver shape, not the integration. The src CL carries the real edit against the real
  vocabulary, and the engine suite is the check.
- Whether `T::Read` becomes ambiguous under a combined `T: WorkUnit<S> + Plannable<S>` bound is not
  tested here, because no such bound exists in the crate today. Associated-type resolution reads the
  bounds in scope, so an implemented-but-unbounded `Plannable` cannot introduce ambiguity; a future
  combined bound would disambiguate with `<T as Plannable<S>>::Read`.
- The 4-tuple `HintExt` arm is proven separately in `hint_ext.rs` (additive, non-overlapping), not
  here.

## What it unblocks

The declaration half becomes consumable by a runner that is not the fiber engine, which is the
prerequisite for a downstream GPU consumer to express its passes in this crate's vocabulary instead
of duplicating it. Per `use-the-stack-not-reinvent.md`, that is the fix belonging upstream.
