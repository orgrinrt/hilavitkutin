# Benches still to write for hilavitkutin

This is the catalogue of benches that would decide something about hilavitkutin as part of the stack, written during the stack audit of 13 November 2026. It lists what is missing, not what is on record; the record is judged in the audit itself (`AUDIT.md` beside this file). First pass: it will move as the audit's later passes go through the rest of the record.

The bench record today is 29 families in `mock/benches/bench.toml`, all of them microkernels, and none of them runs the engine. Every number about the engine itself comes from a timing test on one aarch64 machine (`mock/research/sketches/engine_vs_std/`), which has no committed result file and a FINDINGS that disagrees with its own source. So the first group below is the engine against a hand-written loop, because without it no claim about the design's cost can be checked, and the later groups only make sense once the first exists.

Every bench here wants the same four things from the harness, which it does not have yet: an x86_64 arm and a wasm arm beside the aarch64 one, the spread and not only the median (`mock/benches` analysis keeps medians only), the build profile and target features in `meta.json`, and a hand-written reference arm in every family so that a result is a ratio against something a competent person would write. The panel of 13 November names the same gaps (channel `briefs/fixup/panel-consolidation-1.md`, section 8).

Sizes are given as record counts unless said otherwise. The sizes pick three regimes on purpose: inside L1, inside the last-level cache, and well past it, because the design's claims are mostly about the third.

## Group 1, the engine against a hand-written loop

### Frame floor

What a frame costs when it does nothing. One plan with one unit that touches one column, record counts 0, 1, 64 and 4096, and a plan of 1, 8, 64 and 256 units of which one is dirty. Arms are `run`, `run_fused`, `run_parallel` at 1, 2, 4 and 8 cores, and a hand loop with the same work. The engine does a fixed amount per frame before any record is touched: the accumulator reset, the epoch bump, the dirty propagation, a member mask rebuilt per phase and fiber, and two clock reads per phase (`mock/crates/hilavitkutin/src/scheduler/run.rs:105`, `:205`, `:232-242`, `:296`). It decides whether a tick of kaski at 30 ticks a second, with its share of 0.6 of a 16.67 ms frame, has room for the engine's own overhead at the unit counts a game reaches, and where the floor comes from if it does not.

### Map chain

A chain of k map stages over columns of `u32`, `f32` and a 16-byte record, k of 1, 2, 4 and 8, record counts 1024, 262144 and 16777216. Arms are one hand-fused loop, the same stages as hand loops with each intermediate written to memory, an iterator chain in std Rust, the same in no_std over a slice, `run`, `run_fused`, and `run_parallel` across cores. Inside a morsel `run` dispatches the whole carrier with a member mask and so each unit runs its own loop, its intermediates going through memory (`run.rs:265-273`); `run_fused` folds only a whole linear chain of one-in one-out units (`mock/crates/hilavitkutin/src/dispatch/fusion.rs:21-28`, `:157-161`). It decides whether the fusion the design promises closes the gap the old spike measured (2.1x to 4.2x slower than hand on a four-stage chain, uncommitted), and whether the gap is the intermediates, the dispatch or the morsel loop. This is the panel's "engine_vs_std rebuilt", and it replaces the spike.

### Dispatch shape

The cost of the carrier dispatch alone. A carrier of U units for U of 1, 4, 16 and 64, with one active member per morsel, morsel sizes 256, 4096 and 65536. Arms are the engine's masked dispatch, a match over a dense index, a table of function pointers, and a hand-specialised loop that knows the one unit statically. The design's per-core compiled programs were replaced by runtime masks when the compile-time form walled on the toolchain (`mock/research/202606072100_gate2-canonical-deviation-ledger.md:13-19`), and op blessed the mask on the condition of a bench. This is that bench. It decides whether the mask is free at the morsel sizes the engine chooses, or whether the compiled form has to come back.

## Group 2, parallel shape

### Trunk width

Phases holding T trunks for T of 1, 2, 3, 8 and 16 on C cores for C of 1, 2, 4 and 8, with equal and with skewed work per trunk (one trunk 10x the rest). Arms are `run_parallel`, a hand split with scoped threads over equal slices, and a work-stealing pool from the Rust ecosystem as the named comparison. With more than one trunk in a phase the cores take trunks by rank over the full range, so the width is bounded by the trunk count, and only a phase of exactly one trunk is split across cores (`mock/crates/hilavitkutin/src/scheduler/worker.rs:323-348`, `:368-391`). It decides how much of the machine a phase of two or three trunks uses, which is the common shape in a game tick, and whether the head and tail design or the N-way split op asked to see both of (channel `briefs/fixup/op-answers-3.md`) is the one worth building.

### Incremental skip

A plan of 16 units where a fraction f of the columns is dirty each frame, f of 0, 0.01, 0.1 and 1, at 262144 records. Arms are `run` with the skip, `run_parallel` which runs every unit every frame (`worker.rs:320-322`), and a hand loop that knows which columns changed. It decides what the parallel path gives up by having no skip, and whether a one-bit-per-store dirty mark (`mock/crates/hilavitkutin/src/scheduler/dirty.rs:87-94`) is fine enough to pay for itself at the change rates a game has.

### Wake policy at frame scale

Frames whose work per core is 10 us, 100 us, 1 ms and 5 ms, with a gap between frames of 0, 1 ms and 16 ms. Arms are park, spin, spin then park, and the platform's own condition variable. The workers park at once and the wake strategy is not consulted (deviation ledger `:57-63`); the microbench on record measures the wake in isolation, not inside a frame. It decides which wake a game wants and which a server daemon wants, and whether they can be one default.

## Group 3, the shapes kaski needs

### Record sets of different counts

Beings, tiles and items in one frame, at counts like a kaski save (on the order of 10^3 beings, 10^5 to 10^6 tiles, 10^4 items). Arm a is several sets in one schedule, each with its own count as a plan parameter; arm b is one schedule per set, crossing through resources or accumulators; a hand loop over all three is the reference. Op answered "Bench both first" (channel `briefs/fixup/op-answers-13.md:21`), and the api revision takes neither shape until this exists. It decides that call.

### Neighbour reads

A stencil over a tile grid, four and eight neighbours, grids of 256 squared, 1024 squared and 4096 squared. Arms are the api's unsafe read at a bare index (`mock/crates/hilavitkutin-api/src/context.rs:117-137`), a declared offset read if the revision adds one, and a hand loop over row slices. It decides whether neighbour access can be both safe and as fast as hand, which the current form cannot answer because the read index is unconstrained, and under `run_parallel` an N-way slice reading across its own edge would race.

### Frame-buffered store

A store read as last frame and written as next frame, at the three sizes. Arms are a pointer swap, a copy of the written range, and a copy of the dirty range only. Op suspects this was researched and spiked before and may sit unlanded (`op-answers-13.md:11`); the audit's ledger says what the recovery branches hold. It decides the default the revision writes for buffered columns.

### Emission at scale

Records appending to an accumulator at a rate r per record, r of 0, 0.01, 0.1 and 1, at the three sizes and on 1, 4 and 8 cores. Arms are the engine's append, a hand `Vec` push, a hand bump into a preallocated buffer, and per-core buffers merged after. The single-core spike measured the append at 6.3x to 6.5x the hand form (`mock/research/sketches/engine_vs_std/FINDINGS.md:134-164`), and under `run_parallel` the per-core capacity is a ceiling division of the whole, so which records are kept depends on the core count (`context.rs:66-88`, `worker.rs:196-208`). It decides whether the append form is fit for a game that emits events every tick, and whether the capacity split needs to be by demand.

### Fixed-point lane

The kaski lane is `Fixed<FRAC>` over `i64` with an `i128` multiply, scalar (kaski `fixed.rs:54`, `:173-176`, per the panel). A dot product, an integration step and a saturating add over the three sizes. Arms are `i64` with `i128` multiply, `i32` with an `i64` multiply, `f32`, and each of them in a form the compiler can vectorise. Op wants the lane format entirely generic and configurable upstream (`op-answers-1.md`). It decides what a generic lane costs against a fixed one, and which formats deserve a vectorised path.

## Group 4, the planner and the build

### Plan build cost

Building a plan of U units, U of 8, 64, 256 and 1024, each of the nine plan steps timed alone. The arm is the engine as it is; the reference is the same plan built by hand. It decides whether the planner is fit for a game whose unit count grows with its content, and which step owns the cost.

### Compile cost of the carrier

Compile time and peak memory of `cargo build --release` for a carrier of U units, U of 8, 32, 128 and 512, with `generic_const_exprs` on as the engine needs it (`mock/crates/hilavitkutin/src/lib.rs:14-24`). It decides whether the type-level carrier scales to a game's unit count, which is the question that made the per-core programs fall back to masks in the first place. The panel names it "contract-compile-cost".

### Adaptation

A skewed workload, half the records ten times as heavy as the rest, at the large size and on 1 to 8 cores. Arms are fixed morsel sizes of 256, 4096 and 65536 and the engine's adapt. Adapt has no per-core or per-phase telemetry (`PoolFrame<1,1>`, deviation ledger section 3) and its seven contract tests are ignored. It decides whether adaptation reaches the output at all, which nobody has established.

## Group 5, benches on record that do not measure what they claim

These are rewrites, not new benches. Each one is named in the audit with the reason.

The ctz family feeds `v | 1`, which makes the count constant, so its arms tie at a median of 0.0 ns; it wants inputs with a spread of trailing zeros. The bounds-check family claims the check is gone completely and commits no disassembly, and its arms share a multiply chain that hides a one-cycle check; it wants the asm committed and a chain short enough to see the check. Every dispatch arm is `#[inline(never)]`, the "direct" one too, so the family compares calls to calls; the direct arm wants to be inlinable. The hash and atomic-ordering arms do unequal work and want equal work. The dense dependency-graph arm zero-initialises 64 KB inside the timed region and wants the allocation outside it. The rsb families differ by 0.4 to 1.0 percent between arms, which is the size of their run-to-run difference, and want cache miss counters before any arm is called faster. The `results/` families from July (fiber_theory, swap_band, swap_cost, wake_policy, rcm_order, rcm_rivals) were run from a dirty tree whose sources are only in history and want rerunning from committed source. The details and file lines are in the audit's bench section.
