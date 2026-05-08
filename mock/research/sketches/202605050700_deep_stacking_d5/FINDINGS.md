# Findings: S1b, deep-stacking typestate-builder at depth 5

**Date:** 2026-05-05.
**Round:** 202605042200.
**Maps to:** Topic 4 sketch S1 extension. Audit C3 / M3 / M5 remediation.
**Outcome:** WORKS. Depth 5 with 25 WUs and 13 stores compiles in 0.53s. Missing-resource error message names the missing marker on the first line of the diagnostic. The depth-5 plan stands; the round-4 design holds.

## Setup

S1 (`../202605050530_deep_stacking/sketch.rs`) covered depth 4 with 9 WUs, 11 stores. The audit's C3 finding was that "depth 5" was Topic 3's stated design target but only depth 4 was tested. S1b extends to the full target.

Tier shape:

| Tier | WUs | Stores written |
|------|-----|----------------|
| Leaf  | 8 | 4 (LeafA, LeafB, LeafC, LeafD; 2 writers per column) |
| Mid   | 6 | 3 (MidA, MidB, MidC; 2 writers per column) |
| Outer | 5 | 2 (OuterA with 2 writers, OuterB with 3 writers) |
| Root  | 3 | 1 (RootR; 3 writers) |
| Meta  | 3 | 1 (MetaR; 3 writers, new tier above RootKit) |
| Total | 25 | 11 owned + 2 shared resources = 13 |

Multi-writer-per-column was deliberate: it puts duplicate entries into `WorkUnitBundle::AccumWrite` and exercises the M2 trait-solver-duplicate concern in the same compile.

## Compile time

```
$ time rustup run nightly rustc --crate-type=lib --edition=2024 sketch.rs --emit=metadata
real 0.53
user 0.04
sys 0.07
EXIT: 0
```

Comparison to S1 baseline (depth 4, 9 WUs):

| Sketch | Depth | WUs | Stores | Wall clock |
|--------|-------|-----|--------|------------|
| S1   | 4 | 9  | 11 | 0.56s |
| S1b  | 5 | 25 | 13 | 0.53s |

Roughly 3x more WUs, similar wall time. The trait solver did not show any visible compile-time pathology at this scale. The synthesis topic projected `O(N^2)` from a single S1 data point; the depth-5 measurement does not contradict O(N^2) (the projected cost at N=25 from a 9-element baseline at 0.56s would be roughly 4.3s under naive O(N^2), so this is well below the projection), nor does it confirm linearity. What it does say: at the substrate's actual round-4 scale, the typestate is cheap.

The S1 synthesis worried about projecting to N=100 / depth 6. S1b does not extend that far. If a real round-5 substrate consumer pushes past N=50, a follow-up sketch should re-measure. For round-4, depth 5 with 25 WUs is the validated ceiling.

## Missing-resource error message

With `Clock` not registered, the build proof fails. The full diagnostic (saved at `/tmp/d5_missing_error.txt` during the sketch run; reproducible by recompiling with `--cfg feature="show_missing_error"`):

```
error[E0599]: the method `build` exists for struct
              `SchedulerBuilder<Cons<MetaWU0, Cons<MetaWU1, Cons<..., ...>>>, ...>`,
              but its trait bounds were not satisfied
   --> sketch.rs:281:10
    |
 35 |   pub struct Empty;
    |   ---------------- doesn't satisfy `Empty: Contains<Clock>`
 36 |   pub struct Cons<H, T>(PhantomData<(H, T)>);
    |   --------------------- doesn't satisfy `_: ContainsAll<Cons<RootR, ...
                                              [36-element chain elided]
                                              ... Cons<Clock, Empty>>>>...`
note: trait bound `Empty: Contains<Clock>` was not satisfied
note: the full name for the type has been written to
      'sketch.long-type-4567595176401932923.txt'
```

Audit M3 decision criterion: "the offending marker's name must appear in the inline error text (not buried in the long-type file note) for the error to count as readable." This passes:

- The very first item in the help block reads `doesn't satisfy 'Empty: Contains<Clock>'`. The marker name `Clock` is right there.
- A second `note: trait bound 'Empty: Contains<Clock>' was not satisfied` appears separately in the diagnostic.
- The verbose `Cons<...>` chain elides into the long-type file but does not bury the marker name; the marker name is in the inline text twice over.

The diagnostic is workable as-is. A future round could ship a richer `#[diagnostic::on_unimplemented]` annotation on `Contains<X>` to surface the marker even more prominently (e.g., "store `Clock` is required by some WorkUnit but not registered on the SchedulerBuilder; add `.resource::<Clock>()` before `.build()`"), but that is a polish concern, not a round-4 blocker.

## AccumWrite / AccumRead duplication (M2 evidence)

The Cons chain in the failing diagnostic contains visible duplication:

- `StringInterner` appears 4 times in the AccumRead segment.
- `Clock` appears 4 times.
- `LeafA` appears 2 times in AccumWrite.
- `LeafB` appears 2 times.
- `LeafC` appears 2 times.
- `LeafD` appears 2 times.
- `MidA` appears 2 times.
- `MidB` appears 3 times in AccumRead (read by Outer and Mid tiers).
- `MidC` appears 3 times.
- `OuterA`, `OuterB` each 3 times.
- `RootR` 3 times.

Total Cons-chain length: roughly 36 nodes for a 25-WU bundle that touches 13 distinct stores. The duplication factor is real.

What this means for round-4: M2's hypothesis (duplicate-driven trait-solver work compounds at depth 5) is partially confirmed in the chain length but does NOT show up as wall-clock pathology at this scale (0.53s remains fast). The cheap fix (type-level dedup-on-Concat, M2 task) is worth pursuing as part of round-4 because:

1. Error-message readability at depth 6+ would benefit from a 13-element chain over a 36-element chain.
2. The trait-solver `O(N^2)` bound on the Cons chain becomes a real cost only above some N threshold the sketch did not cross. Deduplication delays that threshold.

The sketch does not block on M2; the existing duplication path compiles fine. M2 is a polish/scalability concern, not a correctness concern.

## Recommendation

The round-4 design holds at depth 5. Specifically:

- **C3 closes.** Depth 5 with 25 WUs compiles in 0.53s. Topic 3's depth target met empirically.
- **M3 closes.** Error message at depth 5 names the missing marker on the first line of the inline diagnostic. Workable. No `trybuild` fixture required for round-4; future polish.
- **M5 closes.** Same as C3.
- **M2 stays open as a polish task.** The duplicate count is real at depth 5; the wall-clock cost is not. Round-4 design ships without dedup; round-5 may add it.

## Cross-references

- `mock/research/sketches/202605050530_deep_stacking/`. S1: depth-4 baseline, identical typestate substrate.
- `mock/research/sketches/202605050615_kit_taxonomy/`. S5: kit visibility / replaceability axes (audit C1).
- `mock/design_rounds/202605042200_topic_round_4_audit.md`. Audit topic: C3 / M3 / M5 remediation specifications.

## Notes on `feature(marker_trait_attr)`

This sketch, like S1 and S5, depends on `feature(marker_trait_attr)`. The audit's C4 captured this as a substrate keystone risk (see audit topic). Nothing in S1b changes that risk profile; if marker traits' coherence semantics shift in a future rustc, S1b stops compiling for the same reason S1 does.
