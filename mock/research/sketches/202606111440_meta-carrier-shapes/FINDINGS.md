# Meta-carrier shapes: shared consumer carrier vs dedicated meta carrier

Hypothesis: the two candidate homes for meta work units (`OnMeta<V>` lifecycle
schedules) can both be carried far enough against the real engine machinery to
compare complexity and codegen. Shape A keeps the shipped single shared carrier
(rank-renumbered phase bands in one WuCons list) and fixes the known warts by
hoisting the meta bands out of the morsel loop on record-bearing paths, using
the band const fns the grouping already ships. Shape B registers a second
cons-list for meta units and walks it once per frame before and after consumer
dispatch, instantiating the same `RunTrunkDispatch` + grouping machinery once
per carrier. Both probes drive the real `RunTrunkDispatch` / `RunGatedTrunk` /
`RunFiber` walk, the real const grouping (`phase_count`, `plan_phase_count`,
`pre_consumer_phase_count`, `consumer_phase_end`), and real builder-produced
bindings; the engine is not edited.

Fixture (both bins): three columns (`In -> Mid -> Out`), two RAW-chained
consumer units (C1, C2), three meta units (`OnMeta<PlanStage>` rank 0,
`OnMeta<PassStart>` rank 2, `OnMeta<ScheduleEnd>` rank 4) with no store access,
1024 records at morsel size 256 (4 morsels per frame), every `execute` appending
to a static event log so per-frame counts and dispatch order are assertable.
Toolchain `nightly-2026-05-28`, release, fat LTO, codegen-units 1.

## Baseline (shipped `Scheduler::run`, the wart measured)

Frame 1 (plan-dirty, all units dirty, no accumulator, so the morsel-outer path):
`plan=4 pass=4 end=4 c1=4 c2=4`. Every meta unit fired once per MORSEL, because
`run`'s record-bearing arm loops morsels outer and `dispatch_trunks` walks ALL
phases (meta bands included) per morsel. Frame 2 (clean): `plan=0 pass=0 end=0
c1=0 c2=0`. The incremental dirty mask is empty and the morsel-outer walk gates
every member on its dirty bit, so on a clean frame the meta units do not run at
all. Both faces of the wart reproduce in shipped code. (The shipped unit-outer
and `run_parallel` paths already hoist the bands; the morsel-outer path is the
one that does not.)

## Shape A: hoisted-band dispatch over the shared carrier

Probe: a `Rig` struct pinning carrier + bindings + an all-ones mask, with two
`#[inline(never)]` drivers generic only over the two witness lists.
`current_shape_frame` replicates the shipped loop shape (morsels outer, all
phases inner). `hoisted_frame` is the restructure: leading meta bands once per
frame over an empty morsel (the shipped `run_parallel` designated-thread shape),
with the plan band skipped when not plan-dirty; consumer bands (`pre..cend`) per
morsel; trailing meta bands once per frame. Same carrier, same grouping const
fns, same dispatch walk; only the loop nesting moves.

### Outcome: WORKS

The replica reproduces the wart exactly (`plan=4 pass=4 end=4`). The hoisted
driver produces `plan=1 pass=1 end=1 c1=4 c2=4`, dispatch order
`[plan, pass, (c1, c2) x4, end]`, correct `Out` values across the morsel loop,
plan band skipped on the not-plan-dirty frame while pass-start and schedule-end
still ran once (the per-band all-ones mask also cures the clean-frame meta
skip). New machinery: none. No new types, no new traits, no builder change; the
restructure is roughly 50 lines of loop reshuffle inside `run`'s record-bearing
arm plus the same treatment wherever the morsel loop wraps `dispatch_trunks`
(`run_fused` untested here).

One solver finding that applies to both shapes: a free fn generic over carrier,
bindings, Adj, AND the witness lists stalls the old trait solver with E0271 on
the higher-ranked `AccumProject` / `VirtualProject` GAT normalization in
`RunFiber`'s Ctx-equality bound (the projections stay unnormalized). Pinning
everything except `Witnesses, GW` on a struct (the inference shape of the
shipped `scheduler.run::<_, _>()` call) resolves it. The real engine change
keeps `run`'s inherent-method shape, which already has this property.

## Shape B: dedicated meta carrier

Probe 1, registration routing (the builder typestate cost): a `MiniBuilder`
whose `with` routes each unit into one of two retained WuCons lists by
lifecycle rank at the type level. The router is a const-bool-keyed pair of
`Route` impls on `ByRank<const META: bool>` (no specialization; the const
argument is the discriminant), selected by the generic const expression
`ByRank<{ is_meta::<W::Sched>() }>` where `is_meta` is a const fn reading
`Lifecycle::RANK` (the `USize` field access lives inside the const fn body,
which sidesteps the documented field-access-in-generic-constants limitation).
Appends route through the engine's order-preserving `WuAppend`.

Probe 2, two-carrier dispatch: a `Rig2` driver carrying both carriers, generic
over FOUR witness lists, with the doubled bound block (two `RunTrunkDispatch`,
two `BundleMasks`). The meta carrier walks its own rank bands once per frame
(leading bands before the consumer loop, plan band gated on plan-dirty;
trailing bands after); the consumer carrier walks all its phases per morsel
with no band arithmetic at all, since it cannot hold a meta unit.

### Outcome: WORKS

The GCE routing compiles and runs on `nightly-2026-05-28`: registering
`Plan, Pass, C1, C2, End` in one mixed sequence produces
`meta = WuCons<PlanWu, WuCons<PassWu, WuCons<EndWu, WuNil>>>` and
`cons = WuCons<C1, WuCons<C2, WuNil>>` (type-ascription checked, order
preserved). The two-carrier frame produces the same observable behaviour as
Shape A hoisted: `plan=1 pass=1 end=1 c1=4 c2=4`, order
`[plan, pass, (c1, c2) x4, end]`, correct `Out`, plan band skipped on the clean
frame. New machinery measured in the sketch: the router is roughly 45 lines
(one const fn, one marker, one trait, two impls); the dispatch driver itself is
the same loop code as A's hoisted driver split across two carriers. Projected
engine cost (not built here): the builder retains a second WuVals list and
gains the rank-routing axis on every registration (composed into the existing
`Place` / `RouterKind` machinery), `Scheduler` gains a second carrier type
parameter and field, `run` / `run_parallel` signatures double their witness
parameters (two to four) and bound blocks (the `Rig2::frame` where clause is
the measured shape), and plan/build paths must either group the union for
RAW-edge analysis or accept that the meta carrier's grouping cannot see
consumer units.

## Codegen observations (objdump, aarch64 release, fat LTO)

All three drivers contain ZERO `blr`: no indirect calls anywhere; the
type-level walks devirtualise in both shapes, meta bands included. Counts per
`#[inline(never)]` driver mono:

| Driver | insns | direct `bl` | of which |
|---|---|---|---|
| A `current_shape_frame` (shipped shape) | 1765 | 4 | 1 grouping (`final_phases_of`), 2 panic bounds checks (log fixture), 1 stub |
| A `hoisted_frame` | 2222 | 18 | 4 grouping (`final_phases_of`, one per band const fn), 2 outlined direct-call `dispatch` monos (empty-morsel band walks), 5 panic bounds checks, stubs |
| B `Rig2::frame` | 1712 | 17 | 5 grouping (`compute_phases_waist`), 1 panic bounds check, 11 stubs |

Two factual notes. First, the band const fns did not const-fold in these
runtime-generic drivers: each band bound lowers to an outlined runtime call
that recomputes the full grouping (`GATE2_MAX_UNITS`-sized scratch) per frame,
once per band fn used (1 call in the shipped shape, 4 in A hoisted, 5 in B).
Any real adoption of either shape has the same incentive to lift the band
bounds to build-time state (as `run_parallel` already does for `gate2_phase` /
`gate2_trunk`) or to positions where they fold. Second, B's per-loop walks are
shorter (the consumer morsel loop folds a 2-unit carrier instead of DCE-folding
meta positions out of a 5-unit carrier per phase pass), and B's frame mono came
out smallest despite driving two carriers; A's hoisted mono is largest, with
the two empty-morsel band walks outlined as separate direct-call monos. DCE
folds the off-phase positions away in both shapes; the difference is walk
length per instantiation, not dispatch quality.

## Comparison (factual)

Both shapes cure the per-morsel meta wart and the clean-frame meta skip on the
record-bearing path, with identical observable dispatch order and identical
zero-indirect-call codegen. Shape A reaches the cure with zero new machinery
(a loop restructure inside `run`, reusing the four shipped band const fns) and
keeps one grouping DAG over all units, so meta and consumer units stay in one
rank-renumbered phase space; its consumer morsel loop still walks the full
carrier with meta positions folded out per phase, and the per-core slice walk
keeps needing the `consumer_mask` gate. Shape B needs the routing layer and the
doubled builder/scheduler surface (second carrier parameter, field, witness
pair, bound block), but removes band arithmetic and the `consumer_mask` need
from every consumer walk by construction, and its routing resolved at the type
level without specialization on the pinned nightly. Shape B's meta grouping
cannot represent a cross-carrier data edge (meta-unit ordering against consumer
stores is positional only: before all or after all), while Shape A's single DAG
carries those edges through the same rank-outer renumber; with rank as the
outer key the bands are positional in both today, so this differs only for
hypothetical meta units ordered WITHIN consumer phases. Neither shape, by
itself, changes where leading-band accumulator appends land relative to the
parallel unit-outer rebase/merge window (a frame-protocol question, single
threaded here, not exercised), and neither resolves #687's per-(virtual,
consumer) clear-on-dispatch semantics, though both give the same natural
once-per-frame dispatch point to hang it on.

Not settled here: `run_fused` and the worker mainloop restructure for either
shape, build-time lifting of the band bounds, builder integration of the
routing into the real `Place` / `RouterKind` typestate, and any parallel-path
behaviour.
