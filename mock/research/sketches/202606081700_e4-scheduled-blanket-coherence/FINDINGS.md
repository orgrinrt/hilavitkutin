# FINDINGS: E4 slice-1 blanket-Scheduled coherence

**Round:** 202606081200 (GATE-2 E4 slice 1 — virtual firing + On-gating)
**Hypothesis:** a blanket `impl<W: WorkUnit<Always>> Scheduled for W { type Sched = Always; }` (so every existing Always WU gains `Scheduled` for free, zero churn) can coexist coherently with an explicit `impl Scheduled for <some On<V> WU>`, given an On<V> WU impls `WorkUnit<On<V>>` and not `WorkUnit<Always>`; and the carrier walk recovers each element's `Sched` unambiguously under the mix.

## Outcome: WORKS

`cargo run` prints `producer=1 ontick=1` then the WORKS line. The Always producer (covered by the blanket `Scheduled`) and the explicit-`Scheduled` `On<Tick>` consumer dispatch correctly in one cons walk. The file compiling is the coherence proof: the blanket and the explicit impl do not overlap, because the blanket's `W: WorkUnit<Always>` predicate excludes an On<V> WU (which impls `WorkUnit<On<V>>`).

## What it proved (and the one correction it forced)

1. **The blanket is coherent.** `impl<W: WorkUnit<Always>> Scheduled for W` + explicit `impl Scheduled for OnTick` compile together. So the src CL adds `Scheduled` with this blanket, and ZERO existing Always WUs (or their tests) need a new impl. Only On<V> WUs add the one-line `impl Scheduled { type Sched = On<V>; }` (which they will already be writing as new code).

2. **The carrier-walk bound is `W: Scheduled + WorkUnit<<W as Scheduled>::Sched>`, NOT a free `W: WorkUnit<S>`.** The first attempt used `impl<W, S, Tail> ... where W: WorkUnit<S> + Scheduled` and failed `E0207: the type parameter S is not constrained`. The fix recovers the schedule from `Scheduled` and feeds it back as the `WorkUnit` param: `W: Scheduled + WorkUnit<<W as Scheduled>::Sched>`. This is the exact bound generalisation the real `RunFiber for WuCons<W, Tail>` (and `RunGatedTrunk`) needs: change `W: WorkUnit` to `W: Scheduled + WorkUnit<<W as Scheduled>::Sched>`, and replace every `<W as WorkUnit>::Read/Write/Ctx/execute` with `<W as WorkUnit<<W as Scheduled>::Sched>>::...`.

3. **The gate dispatches at compile time** (carried over from 202606081400): `<<W as Scheduled>::Sched as ScheduleGate>::should_run(fired)`. Always const-folds to `true` (the `if` vanishes, existing Always WUs pay nothing); On<V> is a flag read, ANDed exactly like the existing dirty-mask bit in `RunGatedTrunk::run_trunk`.

## Unblocks the src CL

api additions (hilavitkutin-api):
- `trait ScheduleGate { fn should_run(fired: &FiredSet) -> bool; }` + `impl ScheduleGate for Always` (true) + `impl<V> ScheduleGate for On<V>` (flag read).
- `trait Scheduled { type Sched: ScheduleGate; }` + blanket `impl<W: WorkUnit<Always>> Scheduled for W { type Sched = Always; }`.

engine dispatch generalisation:
- `RunFiber for WuCons<W, Tail>` and `RunGatedTrunk` member walk: bound `W: Scheduled + WorkUnit<<W as Scheduled>::Sched>`; substitute the associated-type projections; gate `execute` on `should_run`.
- thread the fired-set through the dispatch stack like the dirty mask (`run`/`run_trunk`/`dispatch`/`dispatch_core` gain a fired-set arg, or read it from bindings).

## What it deliberately did NOT settle (src-CL design, not mechanism)

- The real `FiredSet` shape (this used a `[Cell<bool>; 8]` stand-in). The src CL builds the domain-10 bit-packed epoch-reset per-virtual fire-flag store (`Bits`, spec `:685-714`); the per-(virtual,consumer) clear-on-dispatch bit layer + affinity bin-packing is the slice-1b refinement (noted for BACKLOG; per-virtual epoch flags are a correct subset for fire/gate/per-pass-reset, not a stopgap).
- Where the fired-set lives (threaded arg vs on bindings) and the exact `Ctx`-side `fire` wiring replacing the `VirtualFirerApi` no-op.
