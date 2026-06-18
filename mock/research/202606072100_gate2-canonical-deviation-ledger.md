# GATE-2 deviation ledger: where the threaded engine diverges from the canonical design

> **CORRECTION (2026-06-08): §1, §4, and §9 are STALE.** This ledger was written at `5b08a83`, before tasks #677-685 landed. Three entries now describe a superseded build state. §1 records the shipped dispatch as a "runtime per-(core,phase) mask, not the compile-time-materialised one"; the const-gated compile-time materialisation (G-a..G-e, the escalation target this entry named) has since shipped (`dispatch/trunk_dispatch.rs` `IsRoot::IS`/`PhaseAt`, `dispatch/trunk_gate.rs` `Member::IS`, all associated consts DCE'd to per-trunk monos; the only runtime element is `rank % ncores` core ownership, which is correct design). §4 records "main thread serialises phases, workers park between phases"; the worker-side sense-reversing `waist_barrier` (one publish/await per frame, workers hot across phases) has since shipped. §9 records the threaded unit-outer accumulator path as "out of scope"; `worker_accum_unit_outer` has since shipped (the §9 round). §2/§3/§5/§6/§7/§8/§10 still hold. The full re-evaluation and the corrected state map live in `202606081600_engine-state-map-and-roadmap-r5.md`. This header preserves the audit trail; the body below is left as the 2026-06-07 record.

**Date:** 2026-06-07
**Scope:** the GATE-2 parallel-engine arc (R4c threaded executor + the const-grouping work that feeds it), as shipped on `feat/hilavitkutin-parallel-engine-gate2` up to `5b08a83`
**Oracle:** the consolidation spec `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` (domain 17 dispatch flattener, domain 11 phase/waist, domain 20 threading), plus roadmap r2

This is a counting-the-chickens audit, not a plan. It exists so the canonical design stays the reference point and every divergence is named, justified, and given a disposition, rather than quietly becoming the new truth. The canonical design is the oracle (`design-is-the-oracle`, `canonical-design-outranks-intermediate-rounds`); the shipped code is the untrusted approximation. Where they disagree, the spec is right and the code is the thing that owes a reconciliation, even when the divergence is a deliberate, blessed call.

The reader who carries the original design in their head should come away knowing exactly which pieces of that mental model the current source no longer matches, and why.

## The one root cause almost everything descends from

Nearly every structural deviation below traces to a single toolchain wall, not to a series of independent design choices. The canonical dispatch path (domain 17) is **compile-time materialisation**: for each fiber the flattener emits a monomorphised function, and each physical core gets a monomorphised function encoding its entire pipeline (which phases, which record ranges, the WU sequence per fiber as devirtualised local slices, morsel boundaries and phase-sync points as compile-time constants), verified to contain zero `blr` (spec `:1564-1633`). Producing those per-core / per-trunk monos requires turning the plan (which trunk owns which units, in which phase) into distinct monomorphic instantiations.

In pure Rust on the pinned nightly, that materialisation walls. Building a type-level N-way partition of the flat work-unit carrier from const data needs either full `specialization` (forbidden) or reflection over const values into the type system, which the language does not offer; the const-gated enumeration over the flat carrier overflowed const-generic recursion and rejected GCE in const-generic position (sketch `202606070906`, options doc `202606071029`). The only ways to get the canonical compile-time monos are a `build.rs`/proc-macro codegen step that reads the plan and emits the monos, or a macro-built fixed function-pointer slice table (runtime-indirect). op's resolution (2026-06-07) was to **not** pay that complexity yet: ship a runtime per-core trunk-ownership mask over the single flat carrier, bench it, and escalate to build.rs/macro codegen only if the mask check proves a real cost (arvo Kind-2 discipline: naive baseline, then bench, then escalate on evidence).

So the headline is: **we shipped the runtime-mask realisation of the canonical dispatch, not the compile-time-materialised one.** Everything else is downstream of accepting a single flat carrier walked under a runtime mask instead of N compile-time-distinct per-core programs.

## Deviations, with implications

Each entry states the canonical position, what shipped, and what it implies. Disposition tags: **[op-blessed]** explicitly approved; **[agent-call]** decided autonomously under the overnight mandate, pending op review; **[right-sized]** correct for the current feature set, widens later; **[not-built]** a canonical feature simply not yet implemented (a gap, not a divergence in mechanism).

### 1. Dispatch: runtime per-(core,phase) mask vs compile-time per-core monomorphised programs [op-blessed]

Canonical (`:1564-1633`): the flattener emits a monomorphised function per fiber, and a monomorphised program per core, with the WU sequence baked in as devirtualised local slices and phase-sync as compile-time constants. "No dynamic dispatch: the entire program is monomorphised." Zero `blr` is a verification gate.

Shipped: one flat type-level carrier (the `WuCons` value list) walked by `run_gated`, gated each frame by a runtime `AdjRow` bitmask that selects the units a given core owns in a given waist phase (`core_phase_mask` + `run_core_phase`). The inner per-record loop still monomorphises and devirtualises (run_gated's walk is a concrete type), but the selection of which units a core runs is a runtime mask test per carrier position, not a compile-time-distinct program.

Implications. Per worker, per phase, the dispatch walks the whole carrier and predicates each unit on a mask bit, rather than executing a tight pre-selected slice. The devirt of the record loop is preserved; the trunk/phase partition is runtime. This is bench-gated: if the mask predicate measurably costs (branch density on wide carriers, or lost DCE of un-owned units), the escalation path is build.rs/macro codegen of the real per-core monos. The full domain-17 flattener (rust-pipe fusion, DSE store-at-end, the per-core compiled program) is therefore **not** the shipped dispatch; it remains the canonical target if the bench demands it. The const-grouping machinery built earlier in the arc (round 1/2a `run_one_trunk`, the `RunPipeline`/`RunPhase`/`RunTrunk` value nests) is superseded for dispatch and kept only as audit trail.

### 2. PoolFrame placement: inline + `Pin` vs scratch arena [agent-call]

Canonical: the runtime data-plane (`PoolFrame`, progress slots) lives in the plan-stage scratch arena; phase sync is a "Stack AtomicUsize, not Arc ... Raw pointer (`as usize`) passed to thread" (`:1620-1624`). The sync state is arena/stack memory the spawned threads reach by raw pointer.

Shipped: `PoolFrame` is an inline field of the `Scheduler` struct, and `run_parallel` takes `self: Pin<&mut Self>`. The runtime Scheduler retains no `MemoryProvider` after `build`, and `PoolFrame` (atomics + `NonNull`, not `Copy`) cannot be a `ColumnValue` column, so the arena route was not directly available; `Pin` supplies the stable address the spawned workers' raw pointers need instead.

Implications. The Scheduler is now `!Unpin`; a consumer must `core::pin::pin!` it before any threaded run, and `run()` (single-threaded) stays `&mut self`, so the two entry points have asymmetric receivers. This is a real surface change versus the canonical "build a scheduler, run it" shape. It is reconcilable: moving `PoolFrame` + worker contexts into the arena (reserving raw bytes via the provider at build) would restore the canonical placement and drop the `Pin` requirement, at the cost of a build-time raw allocation path the Scheduler does not currently have. Worth revisiting if the `Pin` ergonomics bite consumers (viola, vehje).

### 3. PoolFrame sizing: `<1, 1>` vs `<MAX_CORES, MAX_PHASES>` [right-sized]

Canonical: `PoolFrame<MAX_CORES, MAX_PHASES>` carries per-core `idle_accumulator` / `park_count` and per-phase `predicted_wait_ns` arrays that the adapt subsystem (Topic 6 axes J/K, Topic 5 core-idle) reads to drive wake-tier selection.

Shipped: `PoolFrame<'static, 1, 1>`. The scalar sync words (seq, done, exited, shutdown, phase_arrived) are all present and used; the per-core/per-phase adapt arrays are size-1 placeholders.

Implications. The adapt subsystem (#341) cannot use the canonical per-core/per-phase telemetry until `PoolFrame` widens to real `C`/`P`. Widening changes the inline field type and ripples to every `PoolFrame` construction. Until then, parking cannot make the canonical per-phase predicted-wait tier choice (see §5). This is a deliberate right-size for the no-adapt feature set, not a design disagreement.

### 4. Inter-phase waist barrier: main-orchestrated phases vs worker-side per-core sync [agent-call]

Canonical: each core's compiled program sequences its own fibers and hits phase-sync points internally (`:1601-1633`); the workers run their whole program and synchronise at waists via stack-AtomicUsize spin. The barrier is worker-side and lives inside the per-core program.

Shipped: the main thread serialises phases. Per frame it publishes one phase at a time through the round-A frame protocol (`frame_publish` then `frame_await_done`) and only advances to the next phase after every worker reports done, so the main thread itself is the waist barrier. Workers do one phase per wake and derive which phase from the sequence value.

Implications. Each waist becomes a full park/wake round trip (workers park between phases; the main thread wakes them per phase), which is more synchronisation traffic than a worker-side barrier that keeps workers hot across phases. The shipped `phase_barrier_arrive` / `phase_barrier_reset` are consequently unused by `run_parallel`, and the multi-episode barrier reset / sense-reversing generation bit (deferred as E3 in the keystone sketch) remains unbuilt: this design sidesteps the need for it entirely. The choice was made to reuse only already-proven primitives and avoid building a correct sense-reversing barrier under time pressure. It is explicitly bench-deferred: if per-phase parking dominates, the canonical worker-side barrier (with the generation bit) is the escalation.

### 5. Worker parking: futex/ulock atomic-wait vs spin-then-park hybrid WakeStrategy [agent-call / partial]

Canonical: pre-allocated pool with hybrid wake, spin 128 then park (`:778`), per-core-class spin budgets and per-phase predicted-wait thresholds selecting spin vs futex vs park (`WakeStrategy`, Topic 6 axis K). Worker parking via `wfe`/`sev` or `pause`+futex (`:917`).

Shipped: the frame protocol parks directly on the shipped `atomic_wait` / `atomic_wake_all` (futex on Linux, `__ulock` on macOS, WaitOnAddress on Windows) with the lost-wakeup-safe load-check-wait pattern. No spin-budget pre-roll, no WakeStrategy tier selection.

Implications. `WakeStrategy` ships and is correct but is not consulted by `run_parallel`; the engine parks immediately rather than spinning first, so very short waits pay a syscall the canonical spin tier would avoid. This is partly blocked by §3 (no per-phase predicted-wait array to drive the tier). Reconcilable once adapt + the wider PoolFrame land; the parking primitive itself is the canonical one.

### 6. Spawn / join: no-alloc pointer-closure smuggle + exit-counter join [agent-call]

Canonical: `fn spawn(&self, f: impl FnOnce() + Send + 'static)` (`:296`); "spawn N threads, each runs its compiled program, join when complete" (`:1631`).

Shipped: `OsThreadPool::spawn<F>` copies a pointer-sized `F` into the pthread argument via `transmute_copy`, compile-time-guarded that `F` fits a pointer (no alloc to box a fatter closure); the engine's worker closure captures exactly one `*const WorkerCtx`, so it fits. Threads are detached; shutdown ordering is a worker-exit `AtomicU32` counter the Scheduler waits on at `Drop` (`request_shutdown` + `await_exit`), not a thread join handle.

Implications. The os tier's `spawn` silently constrains consumer closures to pointer-size (a fatter closure is a compile error, not a heap box); fine for the engine, a real limit for arbitrary consumer pools. The "join when complete" of the spec is realised as an exit-counter barrier rather than `pthread_join`, which keeps the `ThreadPoolApi` contract fire-and-forget (no join method) but means shutdown correctness rests on the exit-counter discipline. Both are sound; both are specific realisations the spec did not prescribe.

### 7. Carrier access: raw `*const Scheduler` read under discipline vs baked-in raw pointer to a compiled program [agent-call, soundness-relevant]

Canonical: the per-core compiled program receives a raw pointer (`as usize`) to its sync state and runs a self-contained baked program; the data it touches is its own record range, statically known.

Shipped: each worker holds a type-erased `*const ()` back-pointer to the whole `Scheduler`, re-derefs it every frame to reach the bindings/grouping, and `run_parallel` holds a `*mut Self` (from `get_unchecked_mut`) while orchestrating. The `&mut self` (main) vs `*const Self` (workers) aliasing is made sound by discipline, not by the type system: workers are parked whenever the main thread holds the `&mut`, and they touch column-disjoint write regions during a phase.

Implications. Soundness is an invariant maintained by the protocol (parked-between-frames, disjoint columns), not proven by the borrow checker; it is the same unsafe contract the sketches established, now load-bearing in shipped code. This is inherent to running a persistent pool over a single generic `Scheduler` value rather than over N independent baked programs. A reviewer must treat the parked-between-frames invariant as a hard correctness obligation, and any future code that touches scheduler state mid-frame from the main thread breaks it.

### 8. Intra-phase parallelism not yet built: pipeline parallelism, head+tail convergence, progress counters [not-built]

Canonical: within a phase, fibers run with pipeline parallelism; convergence uses head+tail record-range splits; progress counters (one `AtomicUsize` per fiber, plain store/load) drive producer/consumer pipelining across fibers (`:1601-1633`, `:778`, progress-counter section). This is real intra-phase concurrency beyond column-disjoint trunks.

Shipped: parallelism is column-disjoint trunks across cores within a waist phase; each owned trunk walks its full record range morsel-by-morsel. No fiber pipeline parallelism, no head+tail convergence split, no progress counters (the `progress_slots` pointer is dangling/unused).

Implications. This is a capability gap, not a mechanism disagreement: the shipped model is a correct subset (trunk-level parallelism) of the canonical model (trunk + intra-phase pipeline + convergence). The bench may show the subset is enough for the target workloads, or it may show pipeline parallelism is needed for deep-fiber phases. Naming it here so it is not mistaken for "done": GATE-2 as shipped parallelises trunks, not the full canonical intra-phase concurrency.

### 9. Accumulator (unit-outer) carrier path: out of scope in the threaded executor [not-built / residual]

Canonical: accumulator-bearing carriers run unit-outer (cross-record), distinct from the morsel-local pure-RAW path.

Shipped: `run_parallel` resets accumulators then dispatches the morsel-local path; the unit-outer accumulator carrier that `run()` routes specially is not handled threaded. This is the standing red arm that R4d (the #664 bench) is expected to turn green or to localise.

Implications. Accumulator workloads are not yet correct under the threaded path; this is a known residual, not a hidden bug. R4d must either wire the unit-outer path threaded or record it as the next gate.

### 10. Per-Scheduler memory: inline GATE-2 scratch on every scheduler [agent-call, minor]

Shipped: every `Scheduler` (including single-core consumers that only call `run()`) now carries inline `worker_ctxs: [WorkerCtx; MAX_CORES]` (256) and `gate2_phase`/`gate2_trunk: [USize; GATE2_MAX_UNITS]` (256) plus the `PoolFrame`. That is several KB of always-present scratch.

Implications. Consistent with the engine's existing all-inline const-sized layout (topo_order, predecessor_masks, read_masks are already per-Units arrays), but it is dead weight for non-threaded use. Arena-relocating the GATE-2 scratch (tied to §2) would remove it from the single-core footprint.

## Disposition summary

The mechanism deviations that change what the engine *is* relative to the canonical design are §1 (runtime mask vs compile-time monos), §2/§4 (Pin + main-orchestrated barrier vs arena + worker-side baked program), and §7 (discipline-sound raw aliasing). §1 is op-blessed and bench-gated with a defined escalation (build.rs/macro codegen). §2, §4, §5, §6, §10 are agent calls made under the overnight mandate, each reconcilable toward canonical and each with a stated trigger (Pin ergonomics, per-phase parking cost, adapt landing). §3 is a right-size that widens with the adapt subsystem. §8 and §9 are not deviations but unbuilt canonical capabilities (intra-phase pipeline/convergence, accumulator unit-outer) that GATE-2-as-shipped does not yet cover.

The honest one-line summary for the mental-model resync: **we built the trunk-parallel, runtime-masked, main-orchestrated realisation of GATE-2, which is a correct subset of the canonical compile-time-flattened, worker-side-synchronised, pipeline-parallel design, and the gap between them is bench-gated work, not abandoned design.** The canonical design remains the target; nothing here redefines it.

## What the bench (R4d) decides

R4d (the #664 perf gate re-measure) is the oracle for whether the subset suffices. Element-wise should stay green; the branching and accumulator arms were red-by-design awaiting GATE-2. If they reach parity under the runtime-mask + trunk-parallel model, the deviations in §1/§4/§8 are vindicated as sufficient and the canonical compile-time/pipeline machinery becomes a documented future optimisation rather than a requirement. If they do not, the bench result names precisely which canonical piece (compile-time monos, worker-side barrier, pipeline parallelism) has to be built to close the gap. Either way the decision is evidence-led, per the arc's standing rule that algorithm and perf forks are benched, not argued.
