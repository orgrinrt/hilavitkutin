# Full-engine overview + compile-time-vs-runtime reconciliation

**Date:** 2026-06-08
**Scope:** the complete hilavitkutin engine, every canonical-spec feature, as the chart-the-path Step-2/3/4 deliverable for the dispatch-pivot re-evaluation. Produced because the earlier dispatch-pivot analysis evaluated only the dispatch/devirt/RCM axis and ignored the spec's runtime-adaptability (adaptive plan, EMA replan, dirty-skip) and plugin/hotpluggable-cdylib axes.
**Canonical oracle:** `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` (consolidation spec, 22 domains, 9 resolutions). Citations are `:line` into that file unless another path is given.
**Method:** an independent expert full-engine survey (feature-dev:code-explorer) cross-checked against the spec directly, plus first-party reads of R6, domain 22, the static-schedule resolution, and the self-hosting section. The two agree; divergences are noted.

This document is the short-form index into the design, not a replacement for it. The reasoning lives in the canonical spec; this names every feature, its source line, and its shipped status, then synthesises the one question the re-evaluation exists to answer: does a compile-time-baked devirtualised dispatch coexist with the spec's runtime adaptability and plugin extensibility, or does it forfeit them.

## Part I: Full-engine feature index

### 1. Vocabulary + structural hierarchy (`:69-91`)

`pipeline -> core -> phase <-> waist -> trunk -> fiber <-> branch <-> bridge -> morsel -> micro-morsel -> record`. pipeline = whole scheduled DAG [shipped]; core = pool thread [shipped]; phase = wide DAG section between waists, own strategy, overlaps via progress counters [shipped plan/phase.rs]; waist = local minimum of concurrent path count, defines phase boundaries [shipped plan/steps.rs compute_waists]; trunk = column-disjoint critical path within a phase, zero sync between siblings [shipped plan/trunk.rs]; fiber = column-co-location + morsel-windowing unit, fusion is a codegen choice not a definition [shipped plan/fiber.rs]; branch/bridge = side-path / fan-in peers of fiber [partial]; morsel = L1-sized records×columns×temporal window [shipped dispatch/morsel.rs]; micro-morsel = inner tiling when peak_live > L1 [unshipped]; record = the data identity, never entity/row [shipped, lint-enforced]; column = independent typed store, no joins [shipped api/store.rs]. Dead terms (lint-scanned): chain, chain_group, partition, archetype, entity, row, order.

### 2. Consumer contracts

- WorkUnit (`:638-683`, api/work_unit.rs) [shipped]: `type Read/Write: AccessSet`, `type Hint`, `const COMMUTATIVE`, `fn execute(&self, ctx)`. `execute -> ()`, failure via data flow, abort on panic. `&self` receiver prevents LLVM reordering writes across fused WUs. Schedule conditions `Always`/`On<V>` on the trait impl, not the builder.
- AccessSet (`:514-528`, api/access.rs) [shipped]: compile-time type-level store sets on tuples; converted to runtime `AccessMask` by monomorphised generic fns, no TypeId. `Contains<S>`, `ContainsAll<L>`, `Cons<H,T>`/`Empty`.
- Stores (`:491-528`, api/store.rs) [shipped]: `Resource<T>` singleton, `Column<T>` N-records raw-ptr, `Virtual<T>` edge-only, `Field<T>` ≤16B register-promoted, `Seq<T,N>`/`Map<K,V,N>` const-sized arena, `Replaceable` marker.
- Context (api/context.rs) [shipped]: `Has{ColumnReader,ColumnWriter,ResourceProvider,VirtualFirer,Each,Batch,Reduce}`; stack-local resource caching emitted before the morsel loop.
- ColumnValue (`:425-482`, api/column_value.rs) [shipped]: blanket impl for Copy+'static; `BIT_WIDTH` const for sub-byte bitpacking (the spec-free default is #631).
- Hint (`:638-683`, api/hint.rs) [shipped]: Urgency × Divisibility × Significance; breaks ordering ties, selects morsel dispatch mode.
- Kit (kit/src/lib.rs) [shipped]: `type Units: WorkUnitBundle; type Owned: StoreBundle`; registered via `.with(kit)`.

### 3. Plan analysis chain (`:1236-1517`, plan/steps.rs) [partial: chain runs; some outputs not yet consumed]

build access matrix (Write→Read overlap CSR) [shipped build_dag] → topo sort + reverse-topo renumber [shipped topo_sort] → upward rank / critical path [shipped] → waist detection → phase boundaries [shipped] → **RCM (two outputs, see below)** [partial] → block-diagonal + Dulmage-Mendelsohn [partial: C5a shipped, D-M not yet called] → spectral partition → trunks (>5 fibers; ≤5 single trunk) [partial] → fiber grouping (greedy ≤10 / DP >10 in RCM order) [shipped group_fibers] → column classification Input/Output/Internal/Dead [partial: currently all-Internal] → morsel sizing `(L1/Σwrite).clamp(MIN,8192)&!3` [shipped] → per-phase config MAX_FUSE/BALANCED/MAX_SPLIT [shipped] → core assignment by CoreClass [shipped] → synthesise per-core programs [partial: RecordRange::Full placeholder].

**RCM has TWO outputs (`:1329-1347`, Step 8 `:1328-1348`):** (1) row reordering = WU execution order, fed to fiber grouping, picks the cache-optimal valid topo order for wide fan-out; currently computed but DISCARDED (roadmap C3). (2) column reordering = arena layout (`plan.rcm_order`), co-accessed columns at adjacent offsets; shipped. This is the `canonical-design-outranks-intermediate-rounds.md` worked example; "RCM is arena-only" was the recorded drift.

**Dirty propagation → incremental skip (`:1418-1432`):** runtime, per-pass bitmask OR over generation counters; a WU with an all-clean predecessor set skips. Batch→incremental. [unshipped as integrated step; proven sketch 202606062600]

### 4. Dispatch codegen (`:1523-1661`, dispatch/)

Devirt rules (`:1532-1546`): local `&[fn]` with known values, monomorphised trait dispatch, unrolled params devirtualise; struct-field fn arrays 12.6x, `&[fn;N]` params 5.8x, `static mut` SLOTS indirect-blr do NOT. Approaches (`:1547-1557`, bench T6): A mono-tuple per-fiber-type-trait LOCAL `&[WuFn]` 1.0x; B unrolled 0.9x; C indirect-per-fiber 1.17x; D trunk-mega 1.02x; E schedule-mega 0.97x; struct-field 12.6x FAIL. Selection: <10K → C/D, >10K → E. Flattener rust-pipe (`:1564-1587`): cache resources to stack, read inputs at morsel start, pure-fn pipeline, internal columns register-to-register (DSE), stores grouped at loop end; 0.95-0.96x [proven, codegen_fiber todo!()]. Within-fiber fusion (DSE of internal columns) (`:1609`): 2.09x [proven sketch 202606061600; keyed by classify_columns]. Per-core program (`:1596-1613`): monomorphised fn per core encoding phases, record ranges, per-fiber devirt LOCAL slices, const morsel bounds, stack-AtomicUsize sync [proven shape; codegen_core todo!()]. Inlining (`:1588-1594`): inline(never) on fiber/per-core, inline(always) on WUs within a fiber. Phase sync (`:1619-1633`): AtomicUsize per fiber, Release `stlr` / Acquire `ldar`, stack not Arc [shipped infra]. ASM checklist (`:1636-1644`): zero blr, indexed addressing, no `[sp,...]` in record loop, morsel as immediate, no bl dispatch.

### 5. Runtime adaptability (`:2006-2075`, adapt/)

Two tiers (`:2013-2027`): static (plan-time) cache-pressure/data-flow/lifetime/watermark; runtime (across frames) per-morsel timing, hot/warm/cold change-frequency, cache residency, frame-time EMA, throughput trend. EMA metrics (`:2035-2042`) [config shipped, runtime loop unshipped]: SchedulerMetrics resource, decay `(ema*7+measured)/8`, nine adapt axes shipped as types (adapt/*.rs), vectorised NEON/AVX batch update. **Replan triggers (`:2043-2048`)** [config shipped, loop unshipped, roadmap E8]: BETWEEN FRAMES not during execution; fiber morsel timing change → recompute morsel sizes; phase balance shift → re-select per-phase configs; record count change → full plan recompute; triggers are cheap bitmask comparisons. Plan caching / dirty-detection (scheduler/plan.rs, run_cfg.rs) [shipped]: PlanCache + PlanAffecting; PlanStage fires only when dirty; reuse frames fire only PassStart→ScheduleEnd. Morsel temperature → core assignment (`:2028-2033`): hot→P-core, cold→E-core. Predictive parking (`:2050-2058`, adapt/predictive_parking.rs) [axis shipped]: <100ns spin, 100ns-10µs spin_loop, >10µs park.

### 6. Self-hosting meta pipeline (`:2119-2139`, run_cfg.rs) [types shipped; loop proven sketch 202606062100]

Scheduler is itself a pipeline. Meta virtuals: PlanStage (DAG/plan dirty), ScheduleReady (plan WUs done), PassStart (each pass), ScheduleEnd (after consumer work). Meta resources (MetaAccess-gated): Dag, ExecutionPlan, LaneAssignment, SchedulerMetrics. ~50-line kernel: fire PlanStage → wait → fire ScheduleReady → dispatch consumer WUs → fire ScheduleEnd. Consumer policy via `On<meta::ScheduleEnd>` WU.

### 7. Thread pool + parallelism (`:1793-1895`, thread/)

Pre-allocated pool [infra shipped, spawn-once loop unshipped E2]: physical_core_count threads, spin-128 → park hybrid, raw futex/ulock/WaitOnAddress (no_std), not std::park. Crossover ~2K records vs ~50K spawn/join. Executor trait + work-stealing extension (`:1862-1874`) [api shipped]: default deterministic morsel assignment (no atomic fetch_add), stealing is opt-in consumer Executor. Head+tail convergence (`:1838-1844`) [proven 202606062200]: two cores from opposite ends of a commutative fiber, ~2x, CAS over packed (low,high) cursor; non-commutative skips. Phase sync (`:1619-1633`, thread/barrier.rs) [partial: generation/sense-bit fix open E3]. Pipeline parallelism between phases (`:1847-1854`) [proven 202606062100]. Core-pinned trunks (`:1829-1837`) [assign_cores shipped]. Heterogeneous P/E awareness (`:1810-1827`) [proven E5a/E5b 202606062300/400]: detect topology, critical path→P, leaf→E, proportional morsels, ~1.81x. **No separate single-core path: core count is a plan config parameter (R6); N>1 paths degenerate at N=1** (roadmap :14-15).

### 8. Plugin / hotpluggable extension layer (CLAUDE.md:42-82; linking/extensions crates) [shipped]

**The consolidation spec has NO dedicated plugin domain** (predates the layer). Authoritative source: CLAUDE.md + crate docs. hilavitkutin-linking [shipped]: dlopen/LoadLibrary pull-based explicit-symbol loader, no_std no-alloc, `Library` RAII, lifetime-tied `Symbol`/`StaticRef`, arbitrary-time loading invariant (load/invoke/drop any time, independent of siblings, no global registry). hilavitkutin-extensions [shipped]: `ExtensionDescriptor` (#[repr(C)]), `ProviderId` (FNV-1a stable hash), `ExtensionHost`, required-vs-optional `FailurePolicyFn`, `AbiVersion`, per-extension lifecycle, capability dispatch via stable ProviderId (no string lookup at dispatch). extensions-macros [shipped]: `#[export_extension]` emits descriptor + exported fn + trampolines, no_std output. Invariant (CLAUDE.md:154-170): the engine NEVER loads WUs at runtime; the layer supports arbitrary-time loading of individual extensions and no more; consumer ecosystems build discovery on top.

### 9. Resources, persistence, strategy, constraints, data plane

Resource resolution (`:1676-1779`, resource/, dispatch/engine_ctx.rs) [shipped]: Field/Seq/Map behind pointer indirection to external slab; stack-local resource caching (6 reloads/iter → 0); EngineCtx Project/ColProject/AccumProject GAT-tied; Replaceable opt-in. Strategy markers (`:175-185`) [shipped]: Hot/Precise/Warm; all internal numerics via arvo. Constraints [lint-enforced]: no_std/no_alloc/no_dyn/no_TypeId every crate. MemoryProvider/ColumnStorage (`:288-316`, api/platform.rs, storage.rs) [shipped]: raw pointers not slices (fusion aliasing UB), type-native stride, 64B align, consumer-count release model; R2 evict/dump/inject/import persistence bridge. hilavitkutin-persistence [partial: surface shipped, ColdStore flush/load stubs pending rkyv no_std vetting]. hilavitkutin-ctx/-str/-providers [shipped].

### 10. Build-time layer (`:318-373`, hilavitkutin-build) [partial]

Build-dep only, never linked into runtime. ExpandedLto pragma (`:363-369`) [partial F1]: fat LTO + cgu=1, REQUIRED for cross-fiber/core devirt, release/profiling only. LLVM passes (`:345-355`) [partial F2]: registration at VectorizerStartEP/OptimizerLastEP, Polly, cfg-gated degrade-to-stock. PGO/BOLT (`:351-361`) [design]. Pragma builder (`:335-342`) [framework shipped]. cfg emission (`:329-332`) [shipped]: RUSTC_WORKSPACE_WRAPPER, five profiles dev/dev-opt/release/profiling/ci.

### 11. Two-gate model (roadmap :123-128)

GATE-1: the unified engine at 1-core config (degenerate barriers, one worker), correct + benched at parity; needs Phase C plan chain + Phase D dispatch keystone (D1a/D2/D3/D4 + D-M column classify) + E1 real RecordRanges + E7 dirty-skip; `#664` perf gate green. GATE-2: same engine at N cores; needs E2 spawn-once pool + E3 barrier fix + E4/E4b pipelining/convergence + E5a/b P/E + E6 N-vs-1 oracle. Adapt (E8) + build flags (F1/F2) after both.

## Part II: Reconciliation synthesis (the question the re-evaluation exists to answer)

### The thesis: static composition, adaptive parameters (R6, `:2435-2446`)

R6 is the canonical, top-level resolution and it states the dispatch pivot's thesis as the design: "The WU set, DAG structure, fiber/trunk/phase topology, and monomorphised dispatch functions are all fixed at build time. This is what enables LLVM devirtualisation. No runtime WU registration, no dynamic schedules." And: "Plan-stage parameters are adaptive: morsel sizes, record counts, per-phase strategy selection, morsel-to-core affinity. These adjust at plan time without changing the pipeline composition." The static-schedule resolution (`:2110-2117`) reinforces it and goes further: the static schedule "eliminates the need for fallback dispatch approaches (Approach C) entirely", i.e. the spec explicitly rejects indirect dispatch on the strength of static composition.

So the dispatch pivot (devirtualised type-walk over a statically-composed carrier) is not a compromise of the spec; it is the canonical realisation of R6. Devirt is what R6 says static composition is FOR.

### The recompute boundary: what "the plan" recomputes vs what is locked

Two things both get called "the plan." Distinguish them:

- **Schedule TYPES (locked at compile time):** WU set, dependency DAG, fiber/trunk/phase topology, monomorphised dispatch functions / carrier. R6 fixes all of these. No new types, no new WUs, at runtime, ever.
- **ExecutionPlan as runtime DATA (recomputable between frames):** a `meta::ExecutionPlan` resource holding assignments and parameters: morsel sizes, per-phase configs, core/lane affinity, record ranges. `meta::PlanStage` re-fires to recompute these. The plan recomputes; it recomputes VALUES fed into the fixed-type machine.

So: the plan remains dynamic in the sense the spec means (parameters recompute between frames); the types do not. The plan cannot add a type or a WU.

### Where dispatch ORDER and fiber GROUPING sit on that boundary

This is the load-bearing classification for the pivot. R6's adaptive list is explicit: morsel sizes, record counts, strategy selection, core affinity. Dispatch **order** and fiber **grouping** are NOT on it. They derive from the DAG structure + topology, which R6 places on the FIXED side, and the DAG is a pure function of the WUs' static AccessSets, so it cannot change at runtime in a static-composition engine. Domain 22's heaviest trigger, "record count change → full plan recompute", recomputes morsel sizing and approach selection; it does not reorder dispatch, because record count does not change the dependency structure the order derives from.

The current shipped engine stores `topo_order` as a runtime field and recomputes it on replan (always to the same value, since structure is static). The pivot RECLASSIFIES dispatch order from "runtime plan field" to "compile-time carrier order." That reclassification is sanctioned by R6 (order belongs to the fixed topology) and forced by devirt (only type-order dispatch devirtualises under the morsel loop, proven sketch 202606081200; the runtime-permuted alternatives are the 12.6x / 2-blr / Approach-C-1.17x failures the spec already rejects).

So nothing the spec marks ADAPTIVE moves to the locked side. Order and grouping were never in the adaptive set; they were structure. RCM row-order is a static-structure refinement applied at the static plan/registration boundary, not a runtime-replan thing.

### Extensibility has two surfaces, neither needs runtime-mutable dispatch

1. **New DATA on static WUs (the dominant surface).** Records added to existing columns flow through the same static WUs. `record_count` is an R6 adaptive parameter ("record counts from consumer data"). Unbounded data extensibility is native to static composition with zero new dispatch. A mod adding 500 ships is 500 records, not 500 new code paths.
2. **New CODE (the rarer surface).** A genuinely new transform from a runtime-loaded cdylib does not enter the engine's monomorphised dispatch (R6: no runtime WU registration). It integrates through a statically-registered FACADE WU in the host graph that calls into the extension via the `ProviderId` capability ABI. The plugin keeps its own devirt internally (it was monomorphised when the cdylib was built); the host keeps its own devirt; the only indirect hop is the ABI call at the FFI seam, per-invocation or per-morsel-batch, never per-record. Behind the facade the plugin may run its own statically-composed sub-engine (a sub-graph) or be a per-morsel pure-function capability; that choice is downstream and bench-decidable.

The spec is SILENT on surface 2 (the plugin layer postdates the consolidation spec; expert §12 confirms the reconciliation is implicit, the facade-adapter pattern is architectural inference not a documented decision). CLAUDE.md:154-170 pins only the boundary: the engine never gains runtime WUs; the extension layer does arbitrary-time loading and no more. The facade pattern is the consumer-side integration shape that respects that boundary; it should be made first-class in the roadmap, not left implicit.

### Conclusion: the pivot survives the full picture

The dispatch pivot (static devirt dispatch, parameters adaptive) is the canonical R6 design, not a narrowing of it. Adaptability lives in runtime parameters the static walk consumes; plugins integrate via facade + downstream layer; new data is native. None of the three requires the dispatch STRUCTURE to be runtime-mutable. The registration-order constraint the pivot adds (producer-before-consumer, validated with a BuildError) is the toolchain's price for devirt and lands at the static boundary where the spec already puts structure.

## Part III: Open questions and what the routine still proves

1. **Approach 1 vs Approach 2 (op, 2026-06-08): is the thin-params recompute an arbitrary limit?** Approach 1 = only morsel/config/affinity/range recompute; order + grouping baked static. Approach 2 = order + grouping ALSO runtime-recomputable. Finding from first principles: arbitrary runtime reorder of a type-walk is impossible without indirection (no runtime monomorphisation in Rust); WU-level runtime reorder = 12.6x/2-blr; fiber-level runtime reorder = Approach C 1.17x (spec-rejected). The ONLY devirt-preserving Approach 2 is a BOUNDED set of compile-time-monomorphised order/grouping variants selected at runtime by a per-frame branch (each variant devirt; cost = code size + one predictable off-hot-path branch). **To prove by sketch (next): do N precompiled order/grouping variants each objdump zero-blr, and what is the code-size cost.** If yes, Approach 2 is a viable optional extension we are not arbitrarily foreclosing; if a variant adds indirection, Approach 1 is confirmed as the only sane shape. Spec necessity: Approach 2 has no spec-mandated trigger (no adaptive path reorders dispatch); it is a "could", not a "must".

2. **Canonical-mirror verification (routine Step 6):** confirm NO adaptive trigger anywhere in the spec recomputes dispatch order or fiber grouping from runtime signal (as opposed to morsel/affinity/config). If even one does, that path must keep order runtime-resolved (per-morsel-amortised indirect) and Approach 2 becomes required there, not optional.

3. **Facade/sub-graph plugin pattern:** make it a first-class roadmap item (engine-core boundary spec'd; integration pattern downstream + currently undocumented). Sketch: facade WU → ABI capability → plugin (sub-engine and per-morsel-capability variants), confirm host devirt + plugin devirt intact, indirect hop only at the FFI seam.

4. **Roadmap deltas (Step 5+):** D1a flat schedule-mega = GATE-1 dispatch; builder append + build-time topo-validation (new); run rewrite to type-walk deleting the FiberSlot shim; C3 RCM split (col-layout kept, row-order static via registration); whole-program-flat multi-phase with barriers; adapt-params-into-static-walk; the four new proving sketches.

5. **op decisions pending (do not greenlight implementation until resolved):** (a) loimu order-sensitivity (sets RCM-row-recovery priority + whether Approach 2 matters for a real workload); (b) registration constraint accepted as canonical; (c) greenlight after the revised roadmap is sketch-proven.

## Sources

Consolidation spec `202603181200` (oracle); sibling round topics `202603141800_topic.hilavitkutin-core-design`, `202603121800_topic.dispatch-and-optimisation`, `202603151200_topic.hilavitkutin-api-surface`; roadmap `202606061100_engine-completion-roadmap-draft`; CLAUDE.md; proven sketches `202606061400` (carrier devirt), `202606061500` (D1a), `202606061600` (D4 fusion), `202606071400` (runtime-order devirt no loop), `202606080300` (fn-ptr array under loop FAILS 2 blr), `202606081200` (type-walk under morsel loop devirt + vectorised).
