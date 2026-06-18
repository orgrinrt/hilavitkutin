# R4d finding: GATE-2 trunk parallelism does not reach branching parity

**Date:** 2026-06-07
**Scope:** measuring the threaded `run_parallel` (GATE-2 R4c) against the #664 perf gate's `branching` arm
**Verdict:** the threaded executor is correct but does not green `branching`; it regresses it. Branching parity is a later gate, not GATE-2 trunk parallelism. RED stays RED, by design.

## What was measured

The #664 `branching` arm (a diamond: `BranchX` In to Xv, `BranchY` In to Yv, both phase 0 and column-disjoint, then `JoinZ` Xv+Yv to Zv in phase 1 after the waist) was wired to dispatch through the threaded `run_parallel` persistent pool instead of single-core `run()`, then measured release (fat LTO) at N = 4096 / 65536 / 1048576. Checksums passed at every size, so the threaded path produces correct output; this is purely a throughput finding.

Runtime ratio (engine / optimal-std, lower is better):

| N | single-core `run()` (GATE-1 baseline) | threaded `run_parallel` (this measurement) |
|---|---|---|
| 4096 | 1.54x | 30.75x |
| 65536 | 2.21x | 4.12x |
| 1048576 | 2.69x | 2.75x |

The gate tolerance is 1.10x; both dispatches are red, and the threaded one is worse at every size, catastrophically so at small N.

## Why it regresses, not improves

Two independent causes, both predicted by the deviation ledger (`202606072100`, deviations §4 and §8).

First, the per-frame barrier cost. The shipped threaded dispatch is main-orchestrated: per frame the main thread publishes each waist phase and waits all workers done before the next (deviation §4). For the diamond that is two publish/await-done round trips per frame, each waking and re-parking every worker through the futex/ulock primitive. The branching frame does microseconds of actual work at N=4096, so two full wake/park cycles of all workers dominate completely: 30.75x. As N grows the fixed barrier cost amortises (4.12x at 64K, 2.75x at 1M), which is the signature of a fixed per-frame overhead, not a per-record one.

Second, and more fundamental: even at N=1M, where the barrier cost is negligible, the threaded path (2.75x) does not beat single-core (2.69x). Trunk parallelism runs `BranchX` and `BranchY` on two cores in phase 0, but that does not touch the engine's actual disadvantage against std. Optimal std computes `z = join(branch_x(inv), branch_y(inv))` per element, entirely in registers, in one pass. The engine runs `BranchX` over all records materialising the Xv column, `BranchY` over all records materialising Yv, then `JoinZ` reading both: three passes and two column round-trips through the arena. Trunk parallelism parallelises the materialisation; it does not eliminate it. And `JoinZ` is a single trunk in phase 1, so the join pass is serial regardless of core count. The 2.69x single-core gap is the column-materialisation-versus-register-fusion gap, and parallelising two of the three passes cannot close it.

## What this says about the canonical design

The canonical dispatch (domain 17, spec `:1564-1633`) does not have this gap, because its per-core compiled program fuses the fiber's work per element (the rust-pipe pattern: read inputs, pure-function pipeline with locals, stores at end, DSE eliminating the intermediate column writes) and synchronises worker-side at the waist with an inline stack-AtomicUsize spin, not a main-thread round trip. For the diamond that means computing `x`, `y`, `z` per record in registers within the compiled program, never materialising Xv/Yv, and crossing the waist without a park/wake cycle.

So branching parity needs two canonical pieces this GATE-2 work deliberately did not build:

1. Per-element cross-fiber fusion of the diamond (the domain-17 flattener doing more than the linear `OpChain` that greened `element_wise`; the diamond's `JoinZ` reads two columns, so it is not a linear chain, but the whole In to {X,Y} to Z computation is still a single per-record kernel std proves is fusable). This is the compile-time-materialised dispatch, deviation §1, which walled in pure Rust and is the build.rs/macro-codegen escalation op pre-authorised.
2. The worker-side waist barrier (workers stay hot across phases with inline spin sync), deviation §4, replacing the main-orchestrated per-phase park/wake.

Trunk-level parallelism alone (what GATE-2 R4c shipped) is a correct subset that helps embarrassingly-parallel single-phase column work, but it is not the mechanism that brings a fuse-dominated diamond to parity.

## Disposition

`branching` stays RED on the honest single-core baseline. The bench was reverted to dispatch `branching` via `run()` (2.69x), which is the engine's better branching dispatch today; committing the threaded 24-30x regression as the gate would misrepresent progress. The threaded executor remains correct and shipped (R4c); it is simply not the right dispatch for this workload yet.

No stopgap: per `strict-by-design-quality-pressure`, this red arm is the signal that the canonical compile-time flattener + worker-side barrier are required, and it stays red until those land. This is a roadmap-shaping result: it means the compile-time-materialised dispatch (deviation §1's escalation) is not optional for fuse-dominated multi-fiber workloads; it is the parity mechanism. The accumulator arm is a separate later gate (unit-outer carrier, not yet threaded).

The honest GATE-2 status after R4c + R4d: the threaded persistent pool exists, is correct, and parallelises column-disjoint trunks; it does not by itself reach the #664 branching/accumulator parity the canonical compile-time-flattened, worker-side-synchronised, per-element-fused design targets. Those remain the next gates, now bench-justified rather than assumed.

## Fork resolved (probe, same firing): diamond fusion is the branching gap

The finding above left a fork: is the single-core 2.69x gap the Xv/Yv materialisation (=> diamond fusion) or the dispatch quality (=> compiled per-core dispatch, the canonical spec's "multi-fiber needs no flattening")? A throwaway probe (`engine_vs_std/src/bin/fused_probe`, deleted after reading) measured the diamond three ways at N=1M, all checksums equal:

| dispatch | ns | ratio |
|---|---|---|
| engine, 3 WUs (materialises Xv/Yv) | 751750 | 2.94x |
| engine, 1 fused WU (`BranchJoin`, In to Z, x/y in registers) | 340917 | 1.33x |
| optimal std (fused) | 255541 | 1.00x |

Fusing the diamond into one per-element WorkUnit recovers ~75% of the gap (2.94x to 1.33x). So **materialisation is the dominant branching gap, and diamond fusion is the fix** — the R4d bench-read, not the spec's "no flattening needed for multi-fiber". The spec's claim that fiber-boundary materialisation is fine does not hold for a small-arithmetic diamond at scale: the Xv/Yv arena round-trip (write N, read N, twice) dominates the actual work.

A residual ~1.33x remains after fusion: the engine's per-WU dispatch / morsel machinery vs std's tight loop. That is a separate, smaller question (the compiled per-core dispatch, or morsel/loop tuning), and it may or may not need closing to pass the 1.10x gate.

Disposition update: the branching parity mechanism is **diamond fusion** (extend the shipped D4-linear `OpChain` / `FuseCarrier` to the non-linear fork-join diamond: `JoinZ` reads two inputs, so it is not a linear chain, but In to {X,Y} to Z is one fusable per-record kernel). This is a single-core mechanism in the same class as the autonomously-built D4-linear fusion (greened element_wise), NOT the big compile-time-per-core-program flattener the spec reading implied, and NOT op-gated. The fork was a perf question; the bench settled it. Trunk parallelism (R4c) stays correct and orthogonal; it helps embarrassingly-parallel column work, not fuse-dominated diamonds. Next build: diamond fusion, then re-measure branching, then address the residual 1.33x if the gate needs it.
