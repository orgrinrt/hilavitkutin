# Hilavitkutin engine completion: strategic synthesis

This memo is Step 4 of a `chart-the-path` planning routine (workspace skill `chart-the-path`). It synthesizes, with cross-source verification, the strategic picture for completing the hilavitkutin engine to its full canonical shape: what the complete thing is, what is built, what is missing, and where the path forward was muddled. It feeds a roadmap (the sibling roadmap draft) that is then reviewed by two further domain experts and proven step by step with sketches before implementation resumes.

It exists because the dispatch-codegen work (#340, Phase D) hit an architectural fork that could not be settled on the fly, and a wrong pick is expensive (work that gets rewritten at the parallel-path stage). Per op: do not ship a caricature of the canonical design; build the real thing in its complete shape; and treat every intermediate artifact (this memo included) as potentially drifted, confirming load-bearing claims against the canonical consolidation spec.

## 0. Source reliability (read this first)

The canonical oracle is the consolidation spec `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` and its sibling topics in that round dir. Everything else is intermediate and can have drifted:

- The build-plan memo `mock/research/202605282100_engine-dispatch-build-plan.md` (2026-05-28) is a four-expert synthesis. High quality, but a synthesis, not the oracle. Cross-checked here.
- The ideal-vs-actual audit `mock/research/202606052000_single-core-engine-ideal-vs-actual-audit.md` (2026-06-05) is a current-state record. Reliable for "what shipped" (truth-of-impl), not for "what is correct."
- The dispatch sketches under `mock/research/sketches/` (2026-05 to 2026-06) prove rustc feasibility of specific shapes. They prove a shape COMPILES and DEVIRTUALISES; they do not establish that shape is the canonical one.
- `mock/crates/hilavitkutin/DESIGN.md.tmpl` is the engine's spec-to-impl doc. It has itself carried drift this very arc (the RCM-arena-only stray, corrected on the current branch). Untrusted until checked.

Method for every load-bearing claim below: quote the canonical spec line, note whether the intermediate sources agree, and flag conflicts. Per `canonical-design-outranks-intermediate-rounds.md`, mutual agreement among intermediate sources is not corroboration; it can be shared drift.

## 1. The complete shape (canonical, verified)

Hilavitkutin is a morsel-driven, statically-composed pipeline execution engine. Consumers declare `WorkUnit`s with typed `Read`/`Write` `AccessSet`s over three store kinds (`Resource<T>` singleton, `Column<T>` N-record, `Virtual<T>` zero-data edge); the engine analyses the WU set into a DAG, decomposes it into a phase/trunk/fiber/morsel hierarchy at plan time, compiles per-core monomorphised dispatch programs, and runs them on a pre-allocated thread pool. `#![no_std]`, no `alloc`, no `dyn`, no `TypeId`, no runtime spawn. "Monomorphisation IS the dispatch" (consolidation `:516-520`).

The governing static/adaptive split is resolution R6 (consolidation `:2435-2446`), verified verbatim:

> "Pipeline composition is static: all WUs registered via the builder at compile time. The WU set, DAG structure, fiber/trunk/phase topology, and monomorphised dispatch functions are all fixed at build time. This is what enables LLVM devirtualisation, the schedule is statically analysable. ... Plan-stage parameters are adaptive: morsel sizes (from runtime hardware detection), record counts (from consumer data), per-phase strategy selection (from EMA metrics), and morsel-to-core affinity (from temperature). These adjust at plan time without changing the pipeline composition."

Single-core and parallel are the same design at two scales. The single-core path is the correctness oracle the parallel path validates against (same `Cfg::Out` for any core count).

## 2. The central architectural question (the muddle), with cross-source verification

The fork that stalled #340: how does the plan topology (which units in which fiber/phase, in what order) become the compile-time-known dispatch that LLVM devirtualises, given that the current engine computes that topology at runtime in `Scheduler::build()`?

### 2.1 What the canonical spec says (verified quotes)

Devirtualisation vehicles, domain 17 (`:1534-1537`):

> What devirtualises: Local `&[fn]` slices with known values; Monomorphised trait dispatch; Unrolled function parameters.

What does NOT (`:1539-1545`): struct-field fn-pointer arrays (12.6x), `&[fn; N]` const-generic params (5.8x), global `static mut` slots, `run_fiber(&[WuFn], ...)` with `#[inline(never)]` (one fn for all callers, LLVM can't prove slice contents).

The compiled per-core dispatch, domain 17 (`:1596-1613`), verified:

> "Each physical core gets a monomorphised function encoding its entire pipeline: 1. Which phases this core processes 2. Which record ranges 3. The WU sequence per fiber (devirtualised LOCAL slices) 4. Morsel boundaries (compile-time constants ...) 5. Phase sync points ..."

Fiber mapping, domain 14 (`:1163-1169`): "Each fiber -> monomorphised dispatch with LOCAL `&[WuFn]` slice." "Local `&[fn]` slices devirtualise; struct-field arrays don't (12.6x)."

So the canonical per-core program uses **per-fiber LOCAL `&[WuFn]` slices**, and lists "monomorphised trait dispatch" as a separate, also-valid devirt vehicle. ExpandedLto (fat LTO + cgu=1) is REQUIRED for cross-fiber/core devirt (`:364-368`, `:1530`).

### 2.2 Where the intermediate sources diverge (the flagged conflict)

- **Build-plan memo Phase D** (`202605282100:50`): single-core/parallel dispatch is `run_fiber<S: FiberShape>`, each distinct `S` monomorphises one function that reconstructs a **LOCAL `&[WuFn]` slice** at the call site. This MATCHES the canonical per-core program (local slices). It explicitly warns (risk R2) never to call through a stored `body` field (the 12.6x pathology).
- **Recent sketches** (202606051601, 202606052130, 202606060500, 202606060730): single-core dispatch is the `RunFiberCol` **cons-list walk** over `WuCons` (the "monomorphised trait dispatch" vehicle). Proven to devirtualise. But this is NOT the local-slice per-core-program shape; it is a different, single-core-oriented realization.
- **DESIGN.md.tmpl** (current, on the feature branch): describes BOTH and labels the cons-list walk "the single-core dispatch core, pure Rust generics, distinct from the per-fiber-shape `run_fiber<S: FiberShape>` monomorphisation ... (the codegen-emitted parallel-path shape)" (`:143-145`). So the current doc treats the cons-list walk as single-core-only and the FiberShape local-slice as parallel-only.

### 2.3 Reasoning about reliability

Both the cons-list walk and the local-`&[WuFn]`-slice are canonical-listed devirt vehicles (`:1534-1537`), so neither sketch nor memo invented a non-canonical mechanism. The conflict is narrower and load-bearing: **the canonical per-core PROGRAM (the compiled, RCM-ordered, per-fiber-sliced dispatch, single AND multi core) uses local slices driven by a per-fiber type** (`run_fiber<S: FiberShape>`), and the single-core path is "one core's program" (`:1596-1599`, the program encodes "which phases this core processes" / "which record ranges", trivially the whole schedule on one core). The recent sketches diverged to a cons-list walk that is single-core-only by construction; on the canonical reading the parallel path then needs the FiberShape/local-slice shape anyway, so a cons-list-walk single-core path is a shape the parallel stage discards.

This is exactly the "rewrite later" outcome to avoid, and it is the drift risk op flagged: the recent sketches are intermediate artifacts that may have strayed toward a simpler single-core-only shape, away from the canonical unified per-core program. The sketches' VALUE stands (they prove the projection machinery, the EngineCtx GAT tie, the accumulator inline, and devirtualisation are all feasible on the pinned nightly). What is in question is whether the single-core dispatch SHAPE should be the cons-list walk or the per-fiber `FiberShape` local-slice that unifies with the parallel per-core program.

### 2.35 DECISIVE op constraint (2026-06-06): the engine is unified; core count is config, not a code fork

**CANON, by op's authority (2026-06-06).** op has ruled this an authoritative addition to the canonical design, equal standing to the consolidation spec; recorded as `mock/research/202606061000_canon-amendment-unified-engine.md`. It is not a finding to be weighed by the expert reviews; it is a fixed constraint they design within. The verification below is what op asked be written down alongside the ruling.

op, verbatim intent: do not treat "single-core" or "single-threaded" as separate from the actual engine. Configure the runner to use one core; it then adapts by its usual rules. No special-casing. The same primitives and code paths run at 1, 2, or 7 cores; the whole plan/algorithm pipeline always computes the single best, most optimal sequence and parallelised per-core programs it can from all statically-available data, within the configured core count. With 1 core that best sequence is serial.

Verified against the canonical spec (cross-check, not taken on trust): the spec NEVER special-cases single-core as a code path. "sequential, 1.00x, single core, no overhead" (`:1956`) is one of the per-phase STRATEGY modes (MAX_FUSE / BALANCED / MAX_SPLIT / sequential, `:1916-1956`) the plan selects, not a separate engine. "single-core = trivially ordered" (`:605`) is about ordering. Pool size = `physical_core_count` (`:1799`); thread count = `min(physical_cores, parallelisable ...)` (`:1826`); morsel-to-core affinity is an adaptive plan-stage parameter (R6, `:2442-2446`). The per-core program (domain 17, `:1596-1613`) is THE dispatch; with 1 core there is one program, its phase sync points degenerate (one arriver), convergence is trivial, the pool has one worker.

Consequence (corrects this memo's own earlier framing and the build-plan memo's): the build-plan memo Phase E "single-core-correct first ... this single-core path is the correctness oracle" (`202605282100:52`) read as a distinct single-core build is INTERMEDIATE DRIFT toward special-casing. The unified model is canonical. There is NO "single-core dispatch shape" to design; the cons-list-walk-as-single-core-path (DESIGN.md.tmpl `:143-145`, the recent sketches' single-core framing) is exactly the special-casing op rejects, and is dead. The two-gate mandate is re-read as VALIDATION/BENCH milestones of the one engine: Gate 1 = the unified engine runs correctly and benches at parity configured to 1 core (exercising the real plan, per-core-program dispatch, pool-with-one-worker, strategy=sequential, adapt; parallel scaffolding present but degenerate); Gate 2 = the same engine benches well at N cores (barriers, convergence, multi-worker distribution now doing real work). No code is special-cased by core count; N>1 paths are written once and degenerate at N=1.

This removes the easy escape hatch (a registration-order cons-list walk that "works for single core") and makes the real unified mechanism unavoidable: the runtime-plan-to-compile-time per-core-program bridge (section 2.4) is THE keystone, for all core counts. The per-fiber `FiberShape` typestate is its likely realization.

### 2.4 The "build time = the type system" finding (load-bearing)

R6 says topology is "fixed at build time." The comprehension pass and sketch 202606051601 both surface that, because fiber grouping and RCM order are functions of each WU's `Read`/`Write` `AccessSet` (associated types resolved by the trait solver), "build time" here means **monomorphisation time, i.e. the type system**, not a pre-compile token pass: a proc-macro or build.rs cannot see resolved associated types. This is consistent with the canonical "monomorphisation IS the dispatch" (`:516-520`) and "the schedule is statically analysable" (R6). It points at the per-fiber partition being a TYPE-LEVEL computation (a `FiberShape` type carrying the fiber's WU sequence, derived from the plan), which is the clever-typestate direction op asked to keep up. Whether the full topology (waist/RCM/fiber grouping) can be lifted to type-level/const on the pinned nightly is the single biggest unproven question and the highest-risk roadmap step; it must be sketched before the roadmap commits to it.

Caveat against my own over-rotation: this does not automatically mean "lift the entire plan to const/type-level now." The runtime plan (computed once at `build()`, amortised) is canonical for the ADAPTIVE parameters (morsel size, record count, strategy, affinity, R6). The open question is only whether the TOPOLOGY (fiber/phase partition + order) that feeds the per-core program's local slices is type-level or whether a hybrid (runtime-computed topology projected into a typed per-fiber shape) is canonical and sufficient. The expert reviews and sketches resolve this; the research stage does not pre-pick it.

## 3. Current state map (truth-of-impl, cross-checked against audit + build-plan memo + direct reads)

Verified against the 2026-06-05 audit and direct source reads this session. The build-plan memo's Phase A/B "does not exist" framing is STALE as of 2026-06: the data plane landed since (tasks #654/#655/#658/#659 completed). Corrected current state:

- **Plan analysis: REAL but partial.** `compute_execution_plan` runs the chain (build_dag, topo_sort with real cycle detection, compute_waists, group_fibers, upward_rank, size_morsels, select_phase_configs, classify_columns). RCM is computed (`steps.rs` rcm step) but its row order is DISCARDED (not fed to group_fibers). `classify_columns` marks every store `Internal` (no real Input/Output/Internal split). `block_diagonalise`/`spectral_partition` are structural-only stubs; `Trunk`/`Branch`/`Bridge` allocated, not populated. `waist_detect` is now the post-#663 2-arg arvo call (fixed in #666). `group_fibers` is a greedy out-degree heuristic (correct for linear chains).
- **Data plane + Context: REAL and correct.** `ResourceArena` over `MemoryProvider`, builder retains staged values, `EngineCtx` projection (`Project`/`ColProject`/`AccumProject`) resolves resources, read/write columns, and accumulator appends; per-WU Context scoped to Read/Write; morsel windowing present. Sketches confirm the projection machinery and the 7-param GAT tie resolve. (Supersedes the build-plan memo's "keystone gap" status.)
- **Dispatch: the live path is the 12.6x anti-pattern.** `Scheduler::run` (`scheduler/mod.rs:659`) collects a type-erased `FiberSlot` array via `CollectFiber::collect` (instance ptr + `fiber_shim` fn ptr), then walks a runtime order `live[order[k].0]` and calls through the stored shim. This is the runtime-walked indirect-call shape the canonical design measures at the wrong end of the matrix. `codegen_fiber`/`codegen_core` are empty skeletons (`body: Maybe::Isnt`), never invoked. `RunFiberCol` (the devirtualising drop-in) is sketch-proven but NOT shipped into `dispatch/`.
- **Stage fusion: absent.** Every intermediate column round-trips through the arena; nothing stays in registers across a fiber chain. `classify_columns` marking all-Internal blocks the scratch-backed-internal-column fusion.
- **Concurrency runtime: structure only.** `thread/` atomics real (barrier, per-OS parking) but no spawn-once mainloop; barrier needs a generation/sense-bit fix (build-plan risk R4). `adapt/` is config-only (PhantomData arena, no observe/tune bodies). `synthesise_core_programs` uses `RecordRange::Full` for every fiber, zeroed trunk mapping.
- **The perf gate is live.** `mock/benches/engine_vs_std/tests/perf_gate.rs` is the standing red oracle (single-core engine vs optimal fused std, three workloads: element-wise chain, branching diamond, accumulator append). Currently red ~2.1x-4.6x, gap widening with N (memory-bandwidth signature = no fusion).

The three gate workloads are all single-phase and per-record (morsel-local); `JoinZ` reads `Xv[i]`/`Yv[i]` per record, not a full column. So none of them exercises a mid-schedule full-column barrier.

## 4. Proven vs unproven (sketches + benches, mapped to claims)

Proven (rustc feasibility, pinned nightly-2026-05-28, fat LTO cgu=1):

- Resource-only monomorphised walk constructs each WU's EngineCtx and runs execute (202605300823).
- Column-capable inline walk devirtualises for a three-deep column chain; objdump shows no surviving dispatch symbol, zero `blr` (202606051601). The risk-R2 devirt premise holds for the trait-dispatch vehicle.
- Order-agnostic walk devirtualises for a branching diamond in any statically-known order (202606052130).
- Multi-phase morsel-outer schedule-mega body, 131072 records, objdump confirms zero `blr`/`bl`, const morsel baked, indexed loads, auto-vectorised body (202606060500). Within-equal-depth RCM order measured ~2% non-neutral.
- Inline walk resolves the full 7-param EngineCtx GAT tie including the lifetime-dependent accumulator projection in the `A`-pinned `run<Witnesses>` context (202606060730). `RunFiberCol` is a faithful drop-in for `CollectFiber + fiber_shim`.
- Store-backed flat-CSR plan reshape: 8.77 MB nested const-array plan to 10.5 KB scratch + 20-byte store-backed handle, round-trips, Send/Sync over a !Send store holds (202605302345).
- Bench matrix (canonical T6): struct-field fn ptr 12.6x FAIL, const-generic `&[fn;N]` 5.8x, indirect-per-fiber 1.17x, trunk-mega 1.02x, schedule-mega 0.97x; rust-pipe fiber fusion 0.95-0.96x; arena addressing no cliff; phased-pool 0.68x vs spawn 15.54x at 200 records.

UNPROVEN (must be sketched before the roadmap commits):

- **The per-fiber `FiberShape` local-slice per-core program shape** (the canonical unified single+multi-core dispatch), as opposed to the cons-list walk. Is it constructible from the plan, and does it devirtualise, on the pinned nightly?
- **Type-level / typed per-fiber partition.** Can the fiber/phase partition be carried as a type (`FiberShape` per fiber) derived from the plan, so the per-core program bakes per-fiber local slices, without GCE-extreme machinery? Highest-risk step.
- **Multi-phase full-column barrier dispatch single-core** (reduction-then-broadcast): the phase-sequencing mechanism over the chosen dispatch shape. No gate workload exercises it; correctness only structurally modeled in 202606060500.
- **The fusion half**: real `classify_columns` Input/Output/Internal split + scratch-backed internal columns that DSE eliminates to registers. This is what moves the #664 gate; the column projection is proven but the register-to-register elimination against the arena is not.
- **RCM-order baking**: realizing the plan's RCM row order as the compile-time dispatch order (the codegen flattener). The "automatic reorder" form is bounded behind the GCE soundness gate (#628); whether a typed projection achieves it without GCE is unproven.
- **Concurrency runtime**: spawn-once parking mainloop, barrier generation-bit fix, head+tail convergence, phase pipelining via progress counters, meta-WU firing. Designed (build-plan Phase E), bench evidence is for the designed shape not current code.

## 5. How the strategic view resolves the muddle

The muddle ("option 1 engine-layer vs option 2 compile-time topology") was the wrong axis, formed by reasoning from the current code and recent sketches rather than the canonical per-core program. Reframed against the oracle:

- The canonical dispatch is ONE shape for single and multi core: the compiled per-core program with per-fiber devirtualised LOCAL slices, morsel constants, phase sync points (`:1596-1613`). Single-core is one core's program.
- The durable, no-rewrite path is to build THAT shape for single-core first (it is the correctness oracle), so the parallel path adds cores/barriers/convergence on top rather than replacing the dispatch.
- The per-fiber slice contents come from a per-fiber type (`FiberShape`), which is the clever-typestate realization of "topology fixed at build time = the type system." Whether that type is derived purely at type-level or projected from the runtime-computed plan is the key unproven question, to be settled by sketch + expert review, not pre-picked here.
- The recent cons-list-walk sketches proved the projection + devirt feasibility but chose a single-core-only shape; the roadmap should treat the cons-list walk as a proven fallback vehicle, not the default target, until a sketch shows the FiberShape local-slice per-core program is or is not constructible.

The hilavitkutin-build LLVM layer is FLAG EMISSION (ExpandedLto mandatory for dispatch-bench profiles; custom passes cfg-gated; PGO/BOLT opt-in), not pass-injection into the runtime crate (build-plan memo §4 Phase D, consistent with the crate charter `:31`). It is sequenced late and is an optimisation layer over a correct monomorphised path, not correctness-gating.

## 6. Inputs to the roadmap

The build-plan memo's A->E spine is the existing roadmap skeleton and is largely sound, but its phase STATUS is stale (A/B landed) and its Phase D under-specifies the dispatch-shape decision (it assumes FiberShape local-slice without reconciling the later cons-list-walk sketches). The roadmap (sibling draft) re-sequences from the true current state: complete Phase C (consume RCM, real classify_columns, real waist/trunk/fiber, the R1 typed-UnitId fix), resolve and prove the Phase D dispatch shape (FiberShape local-slice per-core program vs cons-list walk; the type-level partition feasibility), build Phase D single-core (devirt + fusion) to turn the #664 gate green, then Phase E (single-core-correct concurrency oracle, then parallel, then adapt), with the hilavitkutin-build LLVM layer sequenced for benching. Every unproven item in section 4 gets a TODO-sketch with a pinned success criterion before its step is committed.

## 7. Typestate discipline (carry-forward)

The arc has used the type system well: `AccessSet` cons-lists, `Contains`/`ContainsAll` membership, the 7-param `EngineCtx` GAT tied to per-WU projections, `Capacity`/`PlanDims` typed dimensions, the witness-list inference that needs no caller turbofish, the store-backed flat plan handle. The dispatch design should keep this up: prefer a typed per-fiber `FiberShape`, typed phase/order witnesses, and sealed bridges over runtime indices and `transmute_copy` (risk R1). The decision test from `harness-the-type-system.md` applies to every new dispatch type: associated types / generic params / typed newtypes before concrete runtime-indexed structures. Per op, this is a stated goal of the roadmap, not just a nicety.

## See also

Canonical: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` (oracle) + sibling topics. Intermediate (cross-checked, not authoritative): `202605282100_engine-dispatch-build-plan.md`, `202606052000_single-core-engine-ideal-vs-actual-audit.md`, the dispatch sketches. Rules: `canonical-design-outranks-intermediate-rounds.md`, `design-is-the-oracle.md`, `harness-the-type-system.md`, `chart-the-path` skill.
