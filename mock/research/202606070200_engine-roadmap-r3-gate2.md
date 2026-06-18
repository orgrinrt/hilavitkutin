# Engine roadmap r3: GATE-2 corrected (trunk-sectioning then core-pinning)

**Date:** 2026-06-07
**Scope:** GATE-2 only. Supersedes roadmap r1 (`202606061100`) section 5 and r2 (`202606081500`) section 4 wherever they describe *how parallelism is achieved*. r1/r2 remain authoritative for GATE-1 (merged), Phase C plan chain, Phase D dispatch, the plugin/extensibility sections, and Approach 2. This doc re-sequences Phase E around the canonical trunk-per-core model.
**Rationale:** `202606070100_gate2-trunk-sectioning-rechart.md` (the synthesis + drift findings + spec cites). Read it first.
**Oracle:** canonical consolidation spec `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`, the parallelism statements at `:741-746`, `:768-777`, `:1292-1326`, `:1596-1644`, `:1810-1854`.

## The governing correction

Parallelism is **isolated column-disjoint trunks, one per core, with zero sync between trunks** (`:742`, `:769`). The only cross-trunk synchronisation is the **waist** (phase barrier between phases) and the **bridge** (fan-in fiber that runs after parent trunks reach a required record range, `:745-746`). A single fiber's records split across cores only via 2-way head+tail convergence inside a single-trunk phase (`:770`), never an N-way record/morsel partition. Trunks must be **established** in the dispatch path and waists **sectioned** before any core-pinning. r1/r2 framed E1/E2 as record-range / phase-walking distribution; that was drift (see the rechart doc).

The carrier construction changes; the walk machinery is salvaged. `RunFiber` (the per-fiber project+invoke+morsel+devirt walk) is reused verbatim as the innermost level. The plan already computes `PhaseBoundaries` (`compute_waists`) + `BlockPartition` (`block_diagonalise`) + fiber grouping; dispatch must consume them to build a nested `PhaseCons<TrunkCons<FiberCons<WuCons>>>` carrier instead of the flat `WuVals` list it walks today.

## Stage G2-0: dispatch consumes trunk/waist sectioning (single-core, output-equivalent)

The prerequisite op pinned: establish trunks, section waists, before parallelism. Single-threaded throughout; output-equivalent to the merged GATE-1 flat walk, so it validates against the GATE-1 oracle and must not regress `#664`. This is the opening of GATE-2, not a reopening of GATE-1.

### G2-0a. `RunTrunk` over `FiberCons` (= task #670). PROVEN (sketch 202606061400), UNBUILT.
A trunk is a type-level list of fibers; `RunTrunk` walks `FiberCons`/`FiberNil`, delegating each fiber to the unchanged `RunFiber`. Sketch 202606061400 proved the shape devirts (zero blr). This was mis-scoped as post-GATE-1 morsel-locality polish; it is the first structural step here. Build it as the inner nesting level.

### G2-0b. `RunPhase` over a phase list + degenerate waist barrier. PROVEN-IN-PARTS (sketch 202606081600), UNBUILT.
Phases are a type-level list of trunk-bearing sub-carriers walked in waist order; a phase barrier sits between them (degenerate one-arriver at single core). Sketch 202606081600 proved phase sub-carriers + an `AtomicUsize` barrier walk sequentially with zero blr. Build it as the outer nesting level over G2-0a.

### G2-0c. Build the nested carrier from the plan. UNPROVEN as a whole (sketch A gates it).
`build()` emits `PhaseCons<TrunkCons<FiberCons<WuCons>>>` from `PhaseBoundaries` + `BlockPartition` + fiber grouping, replacing the flat `WuVals` walk in `Scheduler::run`. The genuinely-new integration: the full three-level nest built from the *real* plan structures (061400 and 081600 each proved one level in isolation, hand-built). **Sketch A** (rechart doc, Step-9 plan) proves it before the build. Validate output-equivalent to the flat walk + `#664` no-regress + zero blr.

## Stage G2-N: core-pin the trunks (multi-core, the real parallelism)

### G2-Na. Trunk-to-core assignment. PARTIAL (assign_cores shipped, wrong unit in synthesise_core_programs).
Assign each column-disjoint trunk sub-carrier of a phase to a pool thread (core-pinned trunks, `:1829-1837`). `assign_cores` exists; `synthesise_core_programs` currently round-robins fibers with `RecordRange::Full` (the drifted unit) and must be re-pointed at trunks. Leftover threads do convergence, then branches, then bridges in priority order (`:776-777`).

### G2-Nb. Concurrent zero-sync trunk execution + waist barrier. UNPROVEN (sketch B, the keystone).
Trunks of a phase run concurrently on their pinned cores with **no synchronisation between them** (disjoint write columns, `:742`). The only cross-trunk join is the shipped `phase_barrier_arrive` / `phase_barrier_reset` at the waist between phases. **Sketch B** proves two column-disjoint trunks run on two threads zero-sync, waist barrier between phases, output bit-identical to 1-core, zero blr per trunk walk. This replaces the earlier (wrong) record-partition keystone.

### G2-Nc. Bridge fan-in. PARTIAL (branch/bridge plan types partial), UNPROVEN at dispatch (sketch C).
A bridge fiber reads from multiple parent trunks' write columns and runs after they reach the required record range (`:745-746`). **Sketch C** proves the fan-in join composes with the nest.

### G2-Nd. Single-trunk-phase head+tail convergence (= E4b). PROVEN (sketch 202606062200), UNBUILT.
The 2-way record split for a single-trunk commutative phase: two cores from opposite ends, CAS over a packed (low,high) cursor. Degenerate to one range at 1 core. The only place records split across cores.

## How the prior E-steps slot into this spine

- **E2 (spawn-once pool, PROVEN-model 202606062100-adjacent).** The worker substrate each pinned trunk runs on. Underpins G2-Na/G2-Nb.
- **E3 (barrier generation/sense-bit fix, PARTIAL).** Hardens the waist barrier for the multi-episode (many-frame, many-phase) case. Needed once G2-Nb runs more than one barrier episode.
- **E4 (meta-WU pipelining, PROVEN 202606062100).** Overlaps phases via progress counters once trunks run concurrently (`:1847-1854`). Rides on G2-Nb.
- **E4b (head+tail) = G2-Nd.**
- **E5a/E5b (P/E detection + asymmetric morsels, PROVEN 202606062300/400).** The core-pinning *policy* for G2-Na on heterogeneous hardware.
- **E6 (N-vs-1 oracle, PROVEN-shape 202606062500 but RE-GROUND).** The validation discipline, but partitioned by **trunk**, not by the contiguous record range the 062500 sketch used. The sketch's invariant (commutative pipeline, disjoint writes, associative reduction) still holds; the partition unit must be the trunk.
- **E7 (dirty-skip, PROVEN 202606062600) + E8 (adapt, PROVEN 202606062700).** Orthogonal runtime passes, same at any core count. Ride on top.

## Sketch plan (detail in the rechart doc, Step-9/10)

- **Sketch A** gates G2-0c: full `PhaseCons<TrunkCons<FiberCons<WuCons>>>` nest, single-core, output-equivalent + zero blr.
- **Sketch B** gates G2-Nb (the keystone): two column-disjoint trunks, two threads, zero sync, waist barrier, output == 1-core, zero blr per trunk.
- **Sketch C** gates G2-Nc: bridge fan-in join.

A wall in any (uninferrable nest witnesses, trunk-disjointness the types cannot express, a barrier-episode race the shipped barrier cannot carry without E3) is a roadmap-changing finding for op, not a thing to route around.

**Two senses of "compile-time" (canonical-mirror note).** The nested `PhaseCons<TrunkCons<FiberCons<WuCons>>>` carrier is a **build-time type structure** (the static composition R6 fixes at `:2435-2446`). The per-frame record ranges and morsel boundaries the walk consumes are **runtime plan parameters** recomputed between frames (R6's adaptive list). Domain 17's "compiled per-core program... const morsel bounds, baked record ranges" (`:1596-1613`) means constant *relative to a plan execution*, not relative to the binary: the plan stage produces them as values fed into the fixed-type walk. Sketches A and B keep these distinct: the nest shape and devirt are the build-time proof; the record ranges (whole-range at one core, the trunk's own record span at N cores) are passed in as runtime `MorselRange`/`RecordRange` values, not baked into the type. A reader must not expect morsel constants in the binary; they live in the plan-stage output.

## Build order

G2-0a -> G2-0b -> (Sketch A) -> G2-0c [single-core sectioning lands, oracle-validated] -> (Sketch B) -> G2-Na -> G2-Nb [concurrent trunks land] -> G2-Nd -> (Sketch C) -> G2-Nc -> E3/E4 pipelining -> E5 affinity -> E6 oracle at N -> GATE-2 benches (the `#664` branching/accumulator arms turn green here via real trunk parallelism + waisting, never single-core stopgaps).

## Sketch results (Step-10, all PROVEN 2026-06-07)

The corrected model is sketch-proven end to end against the real engine types.

Each sketch is identified by its directory under `mock/research/sketches/` (the stable, ls-findable, greppable identity); commit hashes are secondary audit pointers and a sketch may span more than one commit (its `.rs` plus later addenda). Never identify a sketch by a bare hash.

- **Sketch A** (dir `202606070300_gate2-a-phase-trunk-fiber-nest`; commit `30b9c8c`; G2-0c): the full `PhaseCons<TrunkCons<FiberCons<WuCons>>>` nest over the shipped `RunFiber`, multi-trunk phase 0 + single-trunk phase 1, ran output-equivalent to the flat walk; the 3-deep witness cons-list inferred with no turbofish; `nest_dispatch` objdumped to 590 instructions, zero blr / zero br / zero bl (the whole 4-level walk folds into one straight-line body). Proven working shape: build the nest type from the plan's grouping; `RunPhase -> RunTrunk -> RunFiber` delegating walks; record ranges stay runtime `MorselRange` params, not baked types.
- **Sketch B** (dir `202606070400_gate2-b-concurrent-trunks`; commit `b629fb4`; G2-Nb keystone): two column-disjoint trunks on two real threads, zero sync between them, joined only by the shipped `phase_barrier_arrive` waist; output bit-identical to 1-core; 20/20 stress runs; all `run_one_trunk` monos zero blr. Proven working shape: `SyncBind` shares the immutable bindings (disjoint write columns = no aliasing); one waist episode (no reset, gen-bit is E3); trunk is the parallel unit.
- **Sketch B2** (dir `202606070500_gate2-b2-trunk-parallel-speedup`; commits `9bf41f8` the bench + `c7a3893` the objdump addendum; G2-Nb performance): three column-disjoint compute-bound trunks, sequential 318.97 ms vs parallel 112.51 ms = **2.84x** (ideal 3.00x). objdump (commit `c7a3893`) confirmed the measured path is fully fused (outer record loop + inner kernel inlined, M1 immediate, zero blr / zero bl). Proven: trunk parallelism delivers near-linear speedup on devirt-clean code, not just correct output.
- **Sketch C** (dir `202606070600_gate2-c-bridge-fanin`; commit `d262986`; G2-Nc): a bridge fiber reading two parent trunks' columns (`Cons<Column<AX>, Cons<Column<AY>, Empty>>`) fanned in correctly; all trunk monos zero blr. Proven: the bridge (the only cross-trunk data path) composes with the nest, multi-column read projects devirt-clean.

No wall hit. The build is now mechanical per the build order above, starting at G2-0a (`RunTrunk`/`FiberCons`, = #670, the inner nesting level Sketch A proved).

## See also

`202606070100_gate2-trunk-sectioning-rechart.md` (synthesis + salvage map + drift findings), `canonical-design-outranks-intermediate-rounds.md`, roadmap r1 section 5 / r2 section 4 (the superseded framing).
