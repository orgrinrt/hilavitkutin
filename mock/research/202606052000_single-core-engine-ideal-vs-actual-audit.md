# Single-core engine: the ideal design vs the actual state, and the perf gate

**Date:** 2026-06-05
**Scope:** hilavitkutin engine single-core execution path. Why the engine benches several times slower than an optimal hand-fused std loop, whether that contradicts the design, and whether the work has drifted from the ideal end-state through repeated deferral.
**Source topics:** the consolidation spec (`mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`), the API-surface T6 bench topic (`.../202603151200_topic.hilavitkutin-api-surface.md`), the dispatch-and-optimisation topic (`.../202603121800_topic.dispatch-and-optimisation.md`), the morsel-pipeline research (`.../202603141800_research.morsel-pipeline-execution.md`), the engine-dispatch build-plan memo (`mock/research/202605282100_engine-dispatch-build-plan.md`), the `#660` macro bench (`mock/benches/engine_vs_std/src/main.rs`), and a verification pass over `dev` source on 2026-06-05.

This memo answers a direct question op raised: the engine benches several times slower than optimal single-threaded std, yet earlier benches confirmed the design should match or beat std even single-core. Did the work lose the thread, or is the engine simply far from complete? The short answer is the second, with a real caveat about emphasis that the rest of this memo documents. The current engine is missing both of the two mechanisms the design names as load-bearing for the single-core win. The benched gap is the expected cost of running without them. The design is not contradicted; it is unfinished, and the specific unfinished piece is the one whose entire job is to close this exact gap.

## 1. The single sentence

The single-core engine is designed to match or beat an optimal hand-written loop, and was bench-confirmed to do so at 0.95x to 1.02x of hand-fused code, but only when two mechanisms are present: dispatch devirtualization (the monomorphised mega-dispatch shape) and stage fusion within a fiber (intermediates living in registers, not materialized columns). The engine on `dev` has neither. `Scheduler::run` dispatches through a runtime-walked array of type-erased function pointers, and every intermediate column is written to memory and read back. A fresh `#660` run on 2026-06-05 measures the engine 2.2x slower at 4K records, 3.4x at 64K, and 4.6x at 1M (the gap widens with record count, the signature of a memory-bandwidth-bound workload). That is what an unfused, un-devirtualized columnar walk costs against an optimal fused loop. The phase that builds both mechanisms (`#340`, dispatch codegen) has not been started.

## 2. What "complete" means: the performance thesis

The design does not aim for "a correct columnar engine that is somewhat slower than hand code." It aims for parity-or-better with optimal hand-written code on a single core, and treats that as a measured, bench-gated target, not an aspiration. The consolidation spec and the API-surface topic state the target in concrete ratios against a hand-fused, NEON-vectorized baseline on aarch64:

- Monomorphised schedule-wide mega-dispatch: 0.97x of hand-fused (3 percent faster).
- Monomorphised per-trunk mega-dispatch: 1.02x.
- The rust-pipe fiber-fusion shape: 0.95x to 0.96x (LLVM schedules the fused body better than the hand-written asm).

So "complete" for the single-core path means: the engine produces, for a realistic multi-stage columnar workload, machine code competitive with what a careful engineer would hand-write, and the bench proves it. Anything materially slower than 1.0x on a workload large enough to amortize dispatch is, by the design's own standard, incomplete.

## 3. The mechanisms that deliver the win

The design names six mechanisms. Two are load-bearing for the single-core case; the other four are foundational layout or secondary multipliers.

The two load-bearing mechanisms:

1. **Dispatch devirtualization via monomorphisation plus fat LTO.** The dispatch must not iterate a function-pointer array stored in a struct field. That pattern fails devirtualization outright and benches 12.6x slower (an indirect `blr` in every inner iteration). The design instead monomorphises each fiber or core body and, where indirect calls remain, reconstructs a local `&[fn]` slice at the call site whose contents LLVM can prove and inline. The mega-dispatch shapes (all fibers in a trunk, or all trunks in a schedule, inlined into one function under fat LTO and codegen-units=1) are what reach 0.97x to 1.02x. Source: `202603151200_topic.hilavitkutin-api-surface.md:1024-1484`; the anti-pattern and the 12.6x figure are restated in the build-plan memo at `202605282100_engine-dispatch-build-plan.md` section 5.3 and risk R2.

2. **Stage fusion within a fiber (the rust-pipe pattern).** A fiber is a chain of WorkUnits where each consumes the previous one's output. The codegen flattener emits one monomorphised function per fiber that: caches resources to a stack-local array, reads the fiber's input columns at morsel start, runs the WU sequence as a pure-locals pipeline with no intermediate column stores, lets fiber-internal columns stay register-to-register (dead-store elimination removes the memory ops), and groups the real output stores at the end of the loop body. The fiber becomes the pure function `d(c(b(a(input))))`. Only the fiber's true outputs reach memory; the intermediates never do. Source: consolidation spec domain 17 at `202603181200_topic.hilavitkutin-design-consolidation.md:1564-1586` and domain 14 at `:1095-1112`; the build-plan memo restates it as Phase D at `202605282100_engine-dispatch-build-plan.md` section 4 Phase D and 5.3.

The four foundational or secondary mechanisms:

3. Cache-resident morsel windowing. Per-fiber morsel size `L1_usable / sum(write_sizes)`, clamped and aligned, so the write working set fits L1 and read-only columns ride the L2 prefetcher. Consolidation `:829-876` and `:1007-1038`.
4. SoA columnar layout, type-native stride, 64-byte-aligned column bases. Consolidation `:377-418`.
5. Bitpacked sub-byte columns (`ColumnValue::BIT_WIDTH`). Density multiplier, not a primary win. Consolidation `:840-870`.
6. Branchless dispatch via compiled per-core programs: morsel boundaries as compile-time constants, phase sync as stack atomics, no runtime scheduler loop. Consolidation `:1596-1627`.

The fusion mechanism (2) is the one that specifically lets a columnar engine beat a hand-fused loop on a multi-stage element-wise workload, because it removes the intermediate-column memory traffic that a naive columnar engine would pay. The devirtualization mechanism (1) is what keeps the dispatch itself from being the bottleneck. The benched workload in `#660` is exactly a multi-stage element-wise chain, so it is exactly the workload where the absence of (1) and (2) hurts most.

## 4. The original "we will win" benches, and why the current result does not contradict them

The benches op remembers are the T6 matrix, sourced in `202603151200_topic.hilavitkutin-api-surface.md:1024-1484`. The workload was a multi-partition columnar pipeline of 3 to 8 WorkUnits over 8 columns, integer arithmetic, 500 to 4M records, measured against a hand-written manually-fused loop (scalar and NEON-vectorized). The matrix isolates dispatch shape:

| Dispatch approach | Ratio vs hand-fused | Note |
|---|---|---|
| struct-field fn pointer | 12.6x FAIL | indirect call every iteration, no devirt |
| const-generic `&[fn; N]` | 5.8x | LLVM cannot prove array contents through a parameter |
| indirect per-fiber | 1.17x | one indirect call per fiber per morsel |
| per-trunk mega-dispatch | 1.02x | trunk fully inlined |
| schedule-wide mega-dispatch | 0.97x | whole schedule inlined; beats hand-fused |

Two things matter for op's question. First, the matrix measures dispatch overhead in isolation; the 0.95x to 0.96x fusion result is a separate measurement on the same ops with the rust-pipe fusion enabled. The "we will win single-core" confidence rests on both being present together. Second, these numbers describe the designed engine, the one with monomorphised mega-dispatch and fusion. They are not contradicted by the `#660` result because `#660` measures a different engine: the current one, which has neither. There is no inconsistency between "the designed engine benches 0.97x" and "the current engine benches several times slower." They are two different engines, and only one of them has been built.

## 5. The actual state on `dev` (verified 2026-06-05)

A read of the runtime path confirms both load-bearing mechanisms are absent, and the dispatch codegen that would supply them is stubbed.

`Scheduler::run` does not call `codegen_fiber` or `codegen_core`. Those functions (`dispatch/mod.rs:86-100`) return empty skeleton records (`FiberDispatch::new()` with `body: Maybe::Isnt`, `CoreDispatch::new()` with zeroed metadata) and are never invoked. `run_fiber` (`dispatch/fiber_dispatch.rs:72`) reads only `S::SHAPE_ID` and has no loop body. The module doc at `dispatch/mod.rs:7-11` says so plainly: the emit functions are a skeleton, the rust-pipe emission lands as a follow-up.

What `run` actually does: it collects a type-erased slot array via `CollectFiber::collect` (`dispatch/fiber_codegen.rs:138`), one `FiberSlot` per registered unit, each slot being an instance pointer plus a monomorphised `fiber_shim<W, ...>` function pointer (`dispatch/fiber_codegen.rs:51`). Then `run` walks the per-fiber descriptors, chooses morsel-outer or unit-outer per fiber, and for each dispatch step indexes the slot array `live[order[k].0]` and calls `shim(ptr, &self.bindings, morsel)` (`scheduler/mod.rs:715` and `:726`). The shim rebuilds an `EngineCtx` and calls `invoke_wu_in_fiber` (`dispatch/wu_fn.rs:34`), which calls `wu.execute(ctx)`.

This is the indirect-call shape, dispatched through a runtime-walked array. It is not the monomorphised mega-dispatch that benches 0.97x; it is closer to the design's indirect or const-generic-array bands (1.17x to 5.8x in the T6 matrix, and worse here because the slot array is collected at runtime and walked by a runtime order, which LLVM cannot devirtualise across the loop). Mechanism (1) is absent.

For intermediates: within one morsel, a WorkUnit's write goes straight to the column buffer in memory. `ResolveColumnWrite::resolve_write` (`dispatch/engine_ctx.rs:958-971`) does `core::ptr::write(ptr.as_ptr().add(idx), v)` where `ptr` is the column base from `ColumnBinding` (`resource/bindings.rs:83-86`). The next WorkUnit's `ResolveColumnRead::resolve_read` (`engine_ctx.rs:916-929`) does `core::ptr::read` from the same buffer. So in the `#660` chain, S1 writes `Av` to memory, S2 reads `Av` back from memory, writes `Bv` to memory, and so on. Nothing stays in a register across the chain. Mechanism (2) is absent.

Supporting placeholders in the same path (each is a real function returning a sound but unoptimized result, not a crash):

- Morsel size in `run` is the flat constant `Cfg::MORSEL_SIZE = 256` for every fiber (`scheduler/mod.rs:693`). The per-fiber `morsel_sizes` array that `size_morsels` computes is stored on the plan but unused by dispatch.
- `size_morsels` (`plan/steps.rs:867`) is an even split, not the L1 formula; self-documented as a placeholder pending C1.
- `group_fibers` (`plan/steps.rs:444`) is a greedy out-degree heuristic. For the linear `#660` chain it happens to produce the single correct fiber; it is not correct for branching DAGs in general.
- `classify_columns` (`plan/steps.rs:960`) marks every store `Internal`. Sound, but it never identifies the Input/Output/Internal split that fusion's dead-store elimination would key off.
- The RCM locality order (`plan/steps.rs:257`) is computed by a real arvo-sparse call and then unused, because the only intended consumer is the dispatch codegen that does not exist.
- `replace_resource` / `replace_value` (`scheduler/mod.rs:614-631`) have empty bodies.

The plan chain's graph analysis is genuinely real and complete: `build_dag`, `topo_sort` (Kahn with real cycle detection), `compute_waists` via `arvo_graph::waist_detect`, `rcm_reorder_via`, `block_diagonal_via`, the CSR fiber/trunk projection, `upward_rank`, `select_phase_configs`. The data plane (arena-backed columns, `EngineCtx` projection, the read/write/append accessors, `CollectFiber`) is real and executes correctly. The single-core `run` loop is correct. What is missing is not correctness; it is the two performance mechanisms, both of which live in the unbuilt Phase D.

## 6. Why the gap is what it is, and exactly what closes it

Measured on 2026-06-05 (release, fat LTO, codegen-units=1; median of N iterations; checksums equal, so both arms compute the same result):

| Records | engine runtime | std runtime | runtime ratio | startup ratio |
|---|---|---|---|---|
| 4,096 | 917 ns | 417 ns | 2.20x | 118.8x |
| 65,536 | 22,125 ns | 6,583 ns | 3.36x | 19.5x |
| 1,048,576 | 480,666 ns | 105,208 ns | 4.57x | 0.48x |

The runtime gap widens with record count (2.2x to 4.6x). That is the signature of a memory-bandwidth-bound workload: the more records, the more the materialized intermediate columns cost, because the engine moves five columns through memory where the fused std loop moves two. The startup axis tells a different and also informative story: the engine's plan build is a roughly fixed cost (around 64 microseconds across all sizes), so it is 119x worse than std's two-buffer allocation at 4K but 0.48x (faster) at 1M, where std must zero two million-element buffers. The startup axis is a fixed-versus-linear crossover, not a steady-state concern, because the design builds the plan once and reuses it across many frames; the runtime axis is the one that governs steady-state throughput.

The `#660` workload is four RAW-chained element-wise stages. The std arm fuses all four into one autovectorized loop, keeps A, B, C in registers, and writes only D. The engine arm runs four separate dispatched passes (or four units interleaved per morsel), each reading its input column from memory and writing its output column to memory, through indirect shim calls the compiler cannot devirtualise.

The gap therefore has two stacked causes, both expected:

1. Memory traffic. The engine moves five columns (In, Av, Bv, Cv, Dv) through memory; the std loop moves two (in, D). On a memory-bound element-wise workload that alone is a large multiple.
2. Dispatch overhead. The engine pays an un-devirtualized indirect call structure where the std loop pays nothing.

Mechanism (2), fiber fusion, removes cause 1: with the rust-pipe shape the chain becomes `d(c(b(a(In[i]))))` with A/B/C in registers and only D stored, matching the std arm's memory profile. Mechanism (1), mega-dispatch, removes cause 2: the monomorphised fiber body inlines the four `execute` calls into one loop. With both present the design measured 0.95x to 0.97x on this class of workload. So the gap is not evidence of a design flaw. It is the measurable absence of Phase D, and Phase D is defined as the thing that closes it.

## 7. Have we strayed? The drift verdict

The honest answer has three parts.

**Architecture: aligned.** Every shape that has shipped matches the ideal. The data plane is the SoA columnar arena the design specifies. The `Context` is the GAT-parameterised provider tuple. The plan chain uses the real arvo graph and spectral primitives. The dispatch nesting (morsel-outer for accumulator-free fibers, unit-outer otherwise) is a faithful piece of the morsel model. There is no reinvention, no wrong abstraction, no parallel substrate. On structure, the thread is held.

**Sequence: mostly defensible.** The build-plan memo orders the work A (snapshot semantics), B (data plane), C (plan chain), D (dispatch codegen), E (concurrency, single-core oracle first). The shipped work is B, parts of C, and a single-core `run` oracle. Building the single-core `run` loop before the codegen is defensible: the memo itself calls the single-core path the correctness oracle against which codegen is validated. You want the oracle first.

**Emphasis: drifted, and this is the part worth flagging.** Op's 2026-05-28 directive that opens the build-plan memo is explicit: "no minimal-correct. Let's not defer any longer. Write real, shippable code on these fundamentals." The two load-bearing performance mechanisms (Phase D, `#340`) are the fundamentals most directly tied to the engine's reason for existing, and they have been deferred behind a series of smaller correctness slices: per-fiber morsel-outer dispatch, `waist_detect` wiring, accumulator-free morsel-outer. Each of those is real and correct work, but none touches mechanism (1) or (2). The project's own durable notes already recorded that morsel-outer dispatch does not close the gap and only Phase D fusion does, yet the next slices kept polishing the correctness band rather than opening Phase D. That is the "deferral after deferral, minimal now expand later" pattern op sensed, recurring after the directive that was meant to end it.

There is a sharper version of the emphasis risk. The current `run` loop is an interim hand-written dispatcher: collect slots, walk an order, call shims. The designed hot path is monomorphised codegen that replaces this loop, not extends it. Continuing to add per-fiber dispatch logic into `run` invests in a scaffold the design says gets thrown away. The scaffold was correct to build as the oracle. The risk is treating it as the destination and refining it further, when the next move is to build the codegen path it validates.

The task tracker already frames this correctly: `#661` (Gate 1, single-core to completion, no placeholders or stubs) is in progress, and `#340` (dispatch codegen) is pending under it. By that framing Gate 1 is not done. The codegen stubs and the missing fusion are precisely the placeholders Gate 1 exists to remove. The recommendation below follows directly.

## 8. The perf gate: a standing red oracle (op's proposal, evaluated)

Op proposed writing a test, or a few, that measure a std implementation against the engine and fail whenever the engine is worse than std, kept red until the engine reaches parity, at which point it turns green and we know the goal is met. This is the right instrument and it aligns with the workspace's strict-by-design discipline (a check written to fail, whose failing state is the specification). It turns the performance thesis from prose into an executable definition of done. Recommended shape:

- **Two axes, matching the `#660` framing.** Assert on STARTUP (build or get-ready) and RUNTIME (process to finish) separately. The runtime axis is the headline; startup matters because the engine builds an arena and a plan where the std arm allocates two buffers.
- **A tolerance band, not exact parity.** The design target is 0.95x to 1.02x. A gate that asserts `engine_runtime <= std_runtime * 1.10` (some agreed slack over the 1.02x target, to absorb measurement noise) is honest and stable. Exact `<= 1.0x` would flake even when the design goal is met.
- **Bench methodology, not a naive `cargo test`.** Reuse the `#660` harness shape: release build with fat LTO and codegen-units=1, single-thread pinned, warmup, median and min of N iterations, FNV checksum equality so the two arms are proven to compute the same result. A perf assertion in plain debug `cargo test` would be meaningless and would flake in CI.
- **A small workload matrix, so the gate is a gradient not a cliff.** At minimum: the pure element-wise RAW chain (where fusion is everything), a branching multi-fiber shape (where mega-dispatch across fibers matters), and an accumulator or reduce shape (where the append surface is exercised). Each becomes its own red gate that turns green as the corresponding mechanism lands, so the gates show progress through Phase D rather than a single all-or-nothing flip.
- **Placement and default state.** Live next to `mock/benches/engine_vs_std/` as an assertion mode (exit nonzero when any arm exceeds its tolerance) or as `#[ignore]`-by-default tests run deliberately, so the red state does not block unrelated CI. The red state is expected and documented as expected until `#340` lands. Keeping it committed and red is the point: it is the oracle that tells us, without re-reading this memo, whether Gate 1 is actually done.

The gate does not replace the existing `#660` macro bench; it builds an assertion layer on the same measurement so "is the engine at parity yet" has a yes-or-no answer in the repo.

## 9. Recommended next work

Gate 1 (single-core to completion) is not finished. The remaining work, in dependency order, is the Phase D / `#340` content plus the supporting plan-chain consumption that fusion needs:

1. Build the perf gate (section 8) first, red. It is cheap, it encodes the target, and it measures every subsequent step. It is also the most downstream-unblocking artifact: it converts every later optimization from "seems faster" into "moved the gate."
2. Open Phase D as a real design round, with the discipline the keystone warrants: a sketch for the trait-solver and GAT risk in the monomorphised fiber-body emission, and a neutral domain-expert read on the codegen surface before building. The build-plan memo's risk register (R2 devirtualization, R3 GAT and aliasing) names the hazards.
3. Implement the rust-pipe fiber fusion (mechanism 2): codegen a per-fiber monomorphised body that reads inputs at morsel start, runs the WU sequence as pure locals, and stores only outputs. This needs `classify_columns` to produce the real Input/Output/Internal split so the flattener knows which columns are fiber-internal (register-only) and which are true outputs.
4. Implement the mega-dispatch shape (mechanism 1): `codegen_core` emitting the inlined per-core program, with the local-slice call structure that devirtualises under fat LTO, never the stored-fn-pointer field.
5. Consume the RCM order and the per-fiber morsel sizes that the plan already computes but `run` currently ignores.

Each step should move the perf gate measurably. When the gate goes green on the workload matrix, Gate 1 is done by the design's own standard, and the parallel work (`#662`, Gate 2) begins on a single-core path that is genuinely complete rather than a scaffold.

## 10. Sources

- Performance thesis and the six mechanisms: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` domains 14 (`:1095-1112`), 17 (`:1564-1586`), 11 (`:829-876`, `:1007-1038`), 05 (`:840-870`), the SoA layout (`:377-418`), per-core programs (`:1596-1627`).
- T6 dispatch matrix: `mock/design_rounds/202604200055/202603151200_topic.hilavitkutin-api-surface.md:1024-1484`.
- Build plan and op no-defer directive: `mock/research/202605282100_engine-dispatch-build-plan.md` (section 0 directive, section 4 phase spine, section 5.3 dispatch, risk register).
- Current-state verification: `src/scheduler/mod.rs` (`:614-631`, `:659`, `:693`, `:708`, `:715`, `:721`, `:726`), `src/dispatch/mod.rs:86-100`, `src/dispatch/fiber_codegen.rs:51,138`, `src/dispatch/wu_fn.rs:34`, `src/dispatch/engine_ctx.rs:916-929,958-971`, `src/resource/bindings.rs:83-86`, `src/plan/steps.rs:257,444,867,960`.
- The macro bench: `mock/benches/engine_vs_std/src/main.rs` (`#660`).
