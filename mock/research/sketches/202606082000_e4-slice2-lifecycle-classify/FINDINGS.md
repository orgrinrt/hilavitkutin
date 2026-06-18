# Findings: E4 slice-2 const-time lifecycle classification

**Date:** 2026-06-08
**Round:** 202606081900 (E4 slice-2, task #685), at TOPIC
**Outcome:** WORKS, with a forced surface adaptation.

## Hypothesis

Shape A (the resolved slice-2 shape, see `mock/research/202606081930_…`) needs the
plan/grouping path to order meta WUs vs consumer WUs by lifecycle. That needs a
per-WU const lifecycle rank (plan / consumer / epilogue) computed from the WU's
schedule type at const-eval, the same point the grouping computes masks.

## The wall (confirmed, E0119)

If meta WUs use the SAME `On<V>` marker as consumers (the canonical surface
`WorkUnit<On<meta::PlanStage>>`), a per-WU rule "On<meta::X> -> meta rank,
On<consumerV> -> consumer rank" cannot be written. A blanket
`impl<V> Lifecycle for On<V>` (consumer default) plus a specific
`impl Lifecycle for On<meta::X>` (meta) conflict:

```
error[E0119]: conflicting implementations of trait `Lc2` for type `On<PlanStage>`
```

A negative bound (`V: !MetaVirtual`) is not expressible, and full specialization
is forbidden (`unstable-features.md`). So same-`On` classification is out. This
directly refutes the architect read's "read `<W as HasSchedule>::Sched` and
distinguish On<meta::PlanStage> from On<consumerV> at const time" — that step
needs specialization.

## The escape (WORKS)

Meta WUs declare a DISTINCT schedule marker `OnMeta<V>` (vs consumer `On<V>`).
Then three DISJOINT `Lifecycle` impls classify every schedule with no overlap and
no specialization:

```rust
impl Lifecycle for Always        { const RANK = CONSUMER; }
impl<V> Lifecycle for On<V>      { const RANK = CONSUMER; }
impl<V: MetaVirtual> Lifecycle for OnMeta<V> { const RANK = <V as MetaVirtual>::RANK; }
```

`MetaVirtual` is impl'd only on the four closed-set meta markers
(PlanStage/ScheduleReady/PassStart/ScheduleEnd), each carrying a const rank
(plan=0, consumer=1, epilogue=2). A const fold over a mixed carrier
(`OnMeta<PlanStage>` WU, `Always` WU, `On<Tick>` WU, `OnMeta<ScheduleEnd>` WU)
yields `ranks = [0, 1, 1, 2]` exactly, and the rank is usable as an associated
const in const context (array-length proof). `cargo run` prints WORKS.

## What this proves for the engine

- The slice-1 `HasSchedule { type Sched }` recovery extends to a `Lifecycle`
  const on the schedule with no new trait-solver risk.
- The grouping/plan path can read each WU's lifecycle rank at const time to order
  meta WUs (plan early, epilogue late) around consumers (Shape A), driving the
  phase key, with no specialization and no runtime cost.

## Forced surface adaptation (record it)

The canonical writes meta WUs as `WorkUnit<On<meta::PlanStage>>` (core-design
:720/757/764). The IMPL must instead use a distinct marker (`OnMeta<meta::PlanStage>`,
or equivalently the slice-1 `On<V>` reused for consumers + a separate `OnMeta<V>`
for meta) because same-`On` classification needs forbidden specialization. This
is an unavoidable toolchain-forced deviation (per chart-the-path: record the
divergence + its justification). Behaviour is unchanged: meta WUs are still gated
on lifecycle virtuals and the scheduler self-hosts; only the schedule marker the
meta WU writes in its `WorkUnit<…>` impl differs from the canonical's `On<meta::…>`
spelling. Name TBD in the DOC CL (`OnMeta<V>` vs a `meta`-namespaced gate); the
classification mechanism is the same either way.

## Leeway

SOME-SHAPE: the sketch proves a distinct meta-schedule marker enables const
lifecycle classification; the exact name and whether `MetaVirtual::RANK` is `u8`
or a richer phase enum is a DOC-CL choice. The load-bearing fact (distinct marker
required; classification then const + specialization-free) is settled.

## Next

DOC CL: lock Shape A with the `OnMeta<V>` surface adaptation; add the
`## Self-hosting meta pipeline` section to engine DESIGN.md.tmpl citing this
sketch + the 081930 resolution. Then src CL (meta module + OnMeta + lifecycle
ranks driving the phase key + kernel firing) -> TDD -> lock -> close.
