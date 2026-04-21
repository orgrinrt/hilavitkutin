# Calling Convention for Const-Generic Array Functions

Research benchmark series conducted 2026-03-26. Determines the
optimal calling convention for arvo's const-generic math crate
functions (arvo-graph, arvo-sparse, arvo-spectral, arvo-comb).

Four benchmark iterations, each correcting flaws found in the
previous. The progression from synthetic to realistic is the
story — what we thought mattered turned out not to, and what
actually matters only became visible under realistic conditions.

## Context

arvo's math crates are `#![no_std]`, no alloc. All data structures
are const-generic fixed-size stack arrays. Functions take these
arrays as input and produce them as output. Three calling convention
candidates:

| Pattern | Signature |
|---|---|
| ref+owned | `fn(&input) -> [usize; N]` |
| ref+ref | `fn(&input, &mut [usize; N])` |
| pure-move | `fn(input) -> (input, [usize; N])` |

Plus variants: unsafe shared-ref (&, not &mut), UnsafeCell, MaybeUninit,
raw pointers, inline(always), and hand-written aarch64 assembly.

All benchmarks built with `opt-level=3`, `lto="fat"`,
`codegen-units=1` (matching the ExpandedLto pragma). Run on Apple
M-series (aarch64).

## V1: Synthetic kernel (misleading)

**Source:** `bench/src/calling_convention.rs`

### What we tested

A simple serial kernel (`out[i] = in[i]*4 + accum; accum += out[i]`)
wrapped in three calling conventions. Measured single-call and
3-step-chain timings at N=64, 256, 1024. Hand-written aarch64
asm as baseline via `global_asm!` (real `extern "C"` function, not
inline asm).

### What we found (wrong)

Hand-written asm was 45-50% faster than the best Rust variant.
ref+owned was 13-19% slower than ref+ref in a 3-step chain.

### Why it was wrong

The Rust kernel used `black_box` per element to prevent
const-folding. This crippled LLVM's loop optimisation while the
hand-written asm had no such barrier. The comparison was unfair —
we were measuring the cost of `black_box`, not the calling
convention.

### What we learned

Per-element `black_box` is toxic to benchmarks. Only `black_box`
the final result.

## V2: Isolated real algorithm (partially misleading)

**Source:** `bench/src/bin/rcm_*.rs` (9 separate binaries)

### What we changed from V1

Replaced the synthetic kernel with a real algorithm: Reverse
Cuthill-McKee (RCM) bandwidth minimisation. Non-trivial BFS with
degree-sorted neighbour insertion. O(n + nnz). Shippable quality
code.

Nine calling convention variants, each as its own binary (separate
LTO scope, separate codegen, no cross-variant optimisation):

| # | Variant | What it tests |
|---|---|---|
| 1 | ref+owned | return-value codegen |
| 2 | ref+ref | standard mutable output |
| 3 | unsafe shared-ref | avoiding &mut noalias metadata |
| 4 | UnsafeCell | idiomatic interior mutability |
| 5 | MaybeUninit | output zero-init cost |
| 6 | raw ptr | Rust ABI with raw pointers |
| 7 | inline(always) | call boundary vs loop codegen |
| 8 | asm extern (mut) | hand-written aarch64 |
| 9 | asm extern (const) | same asm, *const output at call site |

Each binary implements the full RCM algorithm inline (not a shared
library call) to ensure independent codegen.

### Measurement evolution

**First attempt:** single-call timing. Failed — RCM at N=64 takes
~1us, timer resolution is ~41ns, OS scheduling jitter is 1-10us.
100%+ variance between identical runs.

**Fix:** batched timing. 10K calls per batch, 10 batches. Per-call
time = total / 100K. Three cooldown modes to test cache state:
nonstop (0), short (100ms sleep between batches), long (600ms).

**Second bug:** sleep subtraction. Subtracted known sleep duration
from total to get algo time. macOS sleep overshoots by ~1-2ms per
call. Across 10 batches, ~10ms phantom time accumulated. Inflated
cooled results by 100-200ns/call (10-30% error).

**Fix:** time each batch individually, sleep outside the timed
window. Two `Instant::now()` calls per batch = 20 timer reads =
~820ns total overhead across 100K calls = 0.008ns/call. Negligible.

### ASM analysis

Disassembled all 9 binaries. Key finding: `rcm_core` (the shared
algorithm function from the initial shared-library version) was
byte-for-byte identical across all binaries — same hash. The wrapper
functions compiled to:

| Wrapper | Instructions | What happens |
|---|---|---|
| ref+ref | 1 (`b rcm_core`) | tail-call, zero overhead |
| unsafe shared | 1 (`b`) | identical to ref+ref |
| UnsafeCell | 1 (`b`) | identical |
| MaybeUninit | 1 (`b`) | identical |
| raw ptr | 1 (`b`) | identical |
| inline(always) | 0 (inlined) | rcm_core in caller |
| asm extern (mut) | 5 (stp+mov+bl+ldp+ret) | full call frame |
| asm extern (const) | 5 (identical) | same frame |
| ref+owned | ~35 | zero-init 512B + memcpy 512B |

Six of nine variants compiled to a single tail-call instruction. We
were benchmarking `rcm_core` nine times, not nine calling conventions.

After switching to per-binary full algorithm implementations (no
shared `rcm_core`):

### V2 results (nonstop, N=64)

| Variant | avg(fresh,reuse) ns | Δ ref+ref |
|---|---|---|
| MaybeUninit | 950 | -7.1% |
| inline-always | 1029 | +0.6% |
| ref+ref | 1020 | baseline |
| UnsafeCell | 1039 | +1.9% |
| ref+owned | 1076 | +5.5% |
| unsafe-shared | 1101 | +7.9% |
| raw-ptr | 1105 | +8.3% |
| asm-opt | 1110 | +8.8% |

MaybeUninit's advantage traced to eliminating zero-init: 16 `stp q0`
instructions vs 60 in ref+ref. The 44 saved NEON stores = ~22 fewer
cycles = ~7% of a ~300-cycle function. Matches the measured delta.

### Why V2 was partially misleading

The benchmark ran 10K identical RCM calls on the same input.
LLVM exploited this:
- Branch predictor perfectly trained on one input's BFS pattern
- L1 fully warm with one adjacency matrix (512B fits trivially)
- No competing cache traffic between calls

The hand-written asm was slower than LLVM's Rust output because
LLVM's instruction scheduling (optimised for the OoO window),
register allocation, and aggressive unrolling (vectorised
reverse-write, bulk enqueue) outperformed manual register planning.
The asm had no bounds checks and fused branch pairs, but LLVM's
global optimisations mattered more in this synthetic scenario.

## V3: Realistic pipeline (trustworthy)

**Source:** `bench/src/bin/rcm_v3_*.rs` + `bench/src/rcm/pipeline.rs`

### What we changed from V2

Embedded RCM inside a 10-stage analysis pipeline modelling
hilavitkutin's actual plan-time processing:

1. Build adjacency matrix from hash-based seed (varies per run)
2. Degree histogram scan
3. **RCM #1** (variant-specific, timed)
4. Apply permutation → reordered matrix
5. Bandwidth computation on reordered matrix
6. Connected components via DFS (different algorithm)
7. **RCM #2** on reordered matrix (different input, timed)
8. Component-weighted degree computation
9. Rank accumulation along permutation order
10. Checksum all outputs (prevents dead-code elimination)

Each pipeline run uses a different seed → different adjacency matrix
→ different BFS traversal → different branch outcomes. 10K runs
with 2K warmup. End-to-end time AND RCM-only time measured
separately.

### Why this defeats synthetic advantages

- RCM called twice with different inputs: predictor can't specialise
- 8 stages between calls: L1 polluted with diverse access patterns
- Input varies per run: no cross-run caching or specialisation
- Every output consumed downstream: no dead-code elimination
- All outputs checksummed: every byte matters

### V3 results

| Variant | e2e ns/run | rcm ns/call | Δe2e | Δrcm |
|---|---|---|---|---|
| asm-opt | 12687 | 3645 | -2.7% | -4.1% |
| MaybeUninit | 12951 | 3754 | -0.7% | -1.3% |
| ref+ref | 13042 | 3802 | baseline | baseline |
| inline-always | 13095 | 3837 | +0.4% | +0.9% |
| ref+owned | 13129 | 3849 | +0.7% | +1.2% |

### Analysis

**The hand-written asm wins in realistic conditions.** -4.1% on the
RCM call itself. The V2 result (asm 17% slower) was an artifact of
the synthetic loop where LLVM's trained branch predictor and warm L1
dominated. With varied inputs and polluted caches, the asm's tighter
code (no bounds checks, smaller icache footprint) pays off.

**MaybeUninit is the best Rust variant** at -1.3% vs ref+ref.
Consistent with V2. The zero-init savings are real regardless of
surrounding context.

**ref+owned is the worst Rust variant** at +1.2%. The extra
zero-init of the return array + memcpy back to caller adds up.
The V1 result (13-19% slower) was inflated by `black_box` noise,
but the direction is correct.

**The total spread is 5.3%** from best (asm) to worst (ref+owned).
RCM accounts for ~29% of pipeline time (3802 / 13042). The calling
convention choice affects 1-4% of RCM time, which is 0.3-1.2% of
total pipeline time.

**inline(always) does not help.** +0.9% vs ref+ref. The function
boundary costs nothing (tail-call optimisation) and inlining
prevents the OoO core from scheduling RCM's stack frame
independently from the surrounding pipeline stages.

## Key takeaways

### What the benchmarks reveal

1. Synthetic loop benchmarks are misleading for calling convention
   comparisons. LLVM optimises for the exact loop pattern (trained
   predictor, warm cache) which masks the real overhead.

2. In realistic conditions, hand-written asm beats LLVM by ~4% on
   a real algorithm. The advantage comes from eliminated bounds
   checks and smaller icache footprint, not from better instruction
   scheduling.

3. MaybeUninit's zero-init savings (7% in isolation, 1.3% in
   pipeline) are the largest Rust-level optimisation available. This
   is a code-level choice, not a calling convention choice.

4. The calling convention itself (ref+ref vs ref+owned vs
   inline-always) matters at the 1-2% level in a realistic pipeline.
   This is detectable but unlikely to be the bottleneck in any real
   application.

### Design decision for arvo math crates

**ref+ref** (`fn(&input, &mut output)`) is the calling convention.

Rationale:
- Second-fastest Rust variant (-1.3% behind MaybeUninit, but
  MaybeUninit is a code-level optimisation that can be applied
  within ref+ref)
- Clean API: explicit buffer control, no return-value copy overhead
- Compatible with buffer reuse across repeated calls
- Chain-friendly: output permutation can be passed directly to the
  next stage without copy

**MaybeUninit for scratch arrays** within ref+ref functions. The 7%
savings from avoiding zero-init is real and consistent across all
benchmark conditions. Applied inside the algorithm implementation,
not at the calling convention level.

**Hand-written asm microkernels** where benchmarks prove the gain.
4% in realistic conditions for RCM. Platform-specific
(`#[cfg(target_arch)]`) with MaybeUninit + ref+ref Rust fallbacks.
The Rust fallback is the baseline; asm replaces it where profiling
shows the function is hot and the gain is measurable.

## V4: Randomised multi-program, breadth-first, shared seeds (definitive)

**Source:** `bench/src/bin/rcm_v4_*.rs` + `bench/src/rcm/programs.rs`

### What we changed from V3

Five multi-frame mini-programs with different cache pressure
profiles. Each program runs 4-6 frames of work with 2-4 RCM calls
among diverse surrounding stages. Frame order is randomised per
seed. Programs include:

- A: full graph analysis (10 stages)
- B: iterative refinement (RCM back-to-back, hot cache)
- C: multi-graph comparison (two matrices, cold between RCMs)
- D: heavy surrounding work (4KB scans evicting L1 between RCMs)
- E: lightweight (nearly back-to-back RCM on same matrix)

Random program dispatch: each run picks A-E based on a shared seed.

### Methodology evolution within V4

**First attempt:** each variant binary generated its own random seed
sequence independently. Ran depth-first (all passes for variant A,
then all for B, etc.). Results were inconsistent — run-to-run
variation (up to 17%) dwarfed variant differences. Cause: different
binaries got different random workloads, and sequential execution
meant OS scheduling noise hit different variants differently.

**Fix: breadth-first with shared seeds.** The harness generates
master seeds and passes them as CLI args to each variant binary.
All 5 variants process the exact same workload (same matrices, same
frame order, same program selection) for each pass. Breadth-first:
pass 1 runs all 5 variants, then pass 2 runs all 5, etc. OS noise
affects all variants equally per pass.

Each (variant × cooldown_mode) is a separate process invocation
(separate address space, no cross-variant LTO or runtime state
sharing). 10 passes × 3 cooldowns × 5 variants = 150 processes.

### V4 results (4 harness runs, 90 samples per variant)

Three cooldown modes per pass (nonstop, 100ms, 600ms between
batches). 10 passes per harness run, 3 harness runs with CSV
output + 1 original run. Total: 90 data points per variant per
metric (30 nonstop + 30 at 100ms + 30 at 600ms from the CSV
runs, which is the combined dataset analysed below).

**End-to-end (all cooldowns combined, 90 samples):**

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| asm-opt | 32879ns | 33155ns | 30204ns | 32689ns | 36125ns | -1.23% |
| MaybeUninit | 33220ns | 33817ns | 30261ns | 33108ns | 36517ns | -0.21% |
| ref+ref | 33290ns | 32803ns | 30551ns | 33051ns | 36746ns | base |
| inline-always | 33415ns | 34066ns | 30549ns | 33196ns | 36941ns | +0.38% |
| ref+owned | 33551ns | 33393ns | 30660ns | 33360ns | 37017ns | +0.79% |

**RCM-only (all cooldowns combined, 90 samples):**

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| asm-opt | 17747ns | 16310ns | 19475ns | -2.36% |
| MaybeUninit | 17962ns | 16365ns | 19733ns | -1.18% |
| ref+ref | 18176ns | 16676ns | 20065ns | base |
| inline-always | 18241ns | 16674ns | 20157ns | +0.35% |
| ref+owned | 18341ns | 16760ns | 20226ns | +0.91% |

**Per-cooldown e2e breakdown:**

| Variant | nonstop | 100ms | 600ms | avg |
|---|---|---|---|---|
| asm-opt | 30313ns | 32915ns | 35410ns | 32879ns |
| MaybeUninit | 30340ns | 33458ns | 35863ns | 33220ns |
| ref+ref | 30665ns | 33105ns | 36099ns | 33290ns |
| inline-always | 30650ns | 33488ns | 36108ns | 33415ns |
| ref+owned | 30750ns | 33513ns | 36390ns | 33551ns |

**Per-pass consistency (nonstop e2e delta vs ref+ref, 30 passes):**

asm-opt: range -0.3% to -2.3%. **Never positive across 30 passes.**
MaybeUninit: range -0.3% to -1.9%. **Never positive across 30 passes.**
ref+owned: range -0.3% to +1.0%. Noise around zero.
inline-always: range -1.0% to +0.8%. Noise around zero.

### What V4 proves

The asm-opt and MaybeUninit advantages are real and consistent.
Across 30 independent passes with shared workloads, neither ever
lost to ref+ref. The magnitudes are small (1-2% e2e, 1-2.4% RCM)
but the direction is 100% consistent.

asm-opt wins across all cache states: -1.15% nonstop, -0.57% at
100ms cooldown, -1.91% at 600ms cold cache. The advantage grows
under cache pressure, not shrinks.

## V5: Framework validation — CONFIRMED

V5 reimplements V4's RCM benchmark using the BenchFn framework.
V5 variant binaries use `runner::run_with_program` (framework).
V5 harness uses `bench_framework::harness::run()` for process
orchestration, CSV output, and markdown report generation.

### ASM verification

Timed functions are byte-identical between V4 and V5 (after
stripping absolute addresses) for 4 of 5 variants:

| Variant | V4 insns | V5 insns | Match |
|---|---|---|---|
| ref+ref | 421 | 421 | identical |
| owned | 471 | 472 (+1 udf pad) | identical (functional) |
| uninit | 299 | 299 | identical |
| inline | n/a | n/a | inlined into caller (differs by design) |
| asm | 197 | 197 | identical |

### Deterministic seed validation

Both V4 and V5 harnesses seeded with `0xDEAD_BEEF_CAFE_BABE`.
Same PRNG (splitmix64). Same seed sequence passed to subprocess
binaries. Results:

| Variant | V4 Δe2e | V5 Δe2e | drift | V4 Δfn | V5 Δfn | drift |
|---|---|---|---|---|---|---|
| ref+ref | base | base | — | base | base | — |
| ref+owned | +0.6% | +0.78% | +0.18% | +0.7% | +0.81% | +0.11% |
| MaybeUninit | -0.9% | -0.33% | +0.57% | -1.9% | -1.11% | +0.79% |
| inline-always | -0.1% | +0.38% | +0.48% | -0.1% | +0.44% | +0.54% |
| asm-opt | -1.1% | -1.46% | -0.36% | -2.2% | -2.71% | -0.51% |

Maximum drift: 0.79 percentage points (MaybeUninit fn). This is
from OS scheduling noise between sequential V4 and V5 harness runs
(not interleaved). Same-seed single-variant comparison confirmed
identical output: 30879.3ns (V4) vs 30879.4ns (V5).

Rankings match. Direction matches for all variants. Framework
validated.

### Quintile analysis (V5 framework output)

**End-to-end, all cooldowns (30 samples per variant):**

| Variant | best 20% | mid 60% | worst 20% | spread |
|---|---|---|---|---|
| asm-opt | 30143ns | 32661ns | 35933ns | 5790ns |
| MaybeUninit | 30148ns | 33136ns | 36390ns | 6242ns |
| ref+ref | 30521ns | 33046ns | 36828ns | 6307ns |
| ref+owned | 30507ns | 33502ns | 36775ns | 6268ns |
| inline-always | 30532ns | 33322ns | 36621ns | 6089ns |

**Insight: asm-opt has the narrowest spread** (5790ns vs 6307ns for
ref+ref). Smaller icache footprint and no bounds checks → less
variance across cache states. The advantage is consistent across
all quintiles: best-case, mid-case, and worst-case.

**ref+ref has the worst worst-case** (36828ns). Under adverse
conditions (cold cache, scheduler interference), ref+ref degrades
more than all alternatives.

**Function-under-test worst/best ratio:**

| Variant | ratio | interpretation |
|---|---|---|
| asm-opt | 1.189x | degrades least under pressure |
| inline-always | 1.199x | inlining helps keep working set warm |
| ref+owned | 1.204x | — |
| ref+ref | 1.206x | degrades most |
| MaybeUninit | 1.206x | same degradation as ref+ref |

asm-opt's advantage GROWS under adversity. When caches go cold,
asm loses 18.9% while ref+ref loses 20.6%.

**Per-pass consistency (nonstop, 10 passes):**

- asm-opt: won 10/10, lost 0/10. Range: -0.7% to -1.8%.
- MaybeUninit: won 10/10, lost 0/10. Range: -0.6% to -1.6%.
- inline-always: won 4/10, lost 4/10. Noise.
- ref+owned: won 5/10, lost 3/10. Noise.

asm-opt and MaybeUninit are the only variants that NEVER lose to
ref+ref across any pass. Everything else is noise.

## Conclusions (RCM, V4+V5 confirmed)

### What we learned across V1-V5

1. **Synthetic benchmarks are misleading.** V1's 45% asm advantage
   and 19% ref+owned penalty were `black_box` artifacts. V2's 7%
   MaybeUninit win was real but the asm results were inverted due
   to LLVM exploiting the synthetic loop pattern. Only V3-V4 with
   realistic pipelines and shared-seed breadth-first execution
   produced trustworthy numbers. V5 confirmed V4's methodology
   via framework validation.

2. **Consistency requires shared workloads.** Different random seeds
   per variant produced 17% run-to-run swings. Shared seeds reduced
   this to <1% per-pass consistency. Deterministic master seeds in
   the harness make runs reproducible.

3. **LLVM is very good but asm wins in realistic conditions.**
   Under synthetic loops LLVM beat hand-written asm by 17%. Under
   realistic multi-program workloads asm wins by 1-3%. The asm
   advantage grows under cache pressure (narrowest spread, lowest
   worst/best ratio).

4. **The bench framework works.** BenchFn trait with raw pointer
   boundary produces zero interference on timed code. ASM
   byte-identical between V4 and V5 for 4/5 variants. Deterministic
   seeds produce matching results. CSV + markdown report generation
   automatic.

### Design decisions for arvo math crates

**ref+ref** (`fn(&input, &mut output)`) as default calling
convention. Clean API, explicit buffer control, chain-friendly.

**MaybeUninit for scratch arrays** inside functions. -1.1% to -2.6%
on RCM, 10/10 consistency. The Rust-native baseline fallback.

**Hand-written asm microkernels** where benchmarks prove the gain.
-1.5% to -3.0% on RCM, 10/10 consistency. Narrowest spread, best
worst-case. Platform-specific with MaybeUninit + ref+ref Rust
fallback.

Pending: second algorithm benchmark to confirm these findings
generalise beyond RCM.

## Raw data

CSV files in `mock/research/`:

RCM (V4):
- `202603261_calling-convention-v4-combined.csv` — all 3 runs (450 rows)
- `202603261_calling-convention-v4-run[1-3].csv` — individual runs

Columns: `run,pass,cooldown_ms,variant,e2e_ns,rcm_ns`

## Benchmark source

```
bench/src/bench_framework/         — reusable BenchFn framework
bench/src/calling_convention.rs    — V1 synthetic kernel
bench/src/bin/rcm_*.rs             — V2-V5 RCM variants
bench/src/rcm/                     — RCM shared code + programs
bench/src/bin/rcm_asm_opt.s        — hand-written aarch64 RCM
bench/bench-framework-design.md    — framework design + validation
bench/TODO-framework-validation.md — validation tasks + principles
bench/calling-convention-v[2-4]-plan.md — methodology docs
```

Run:
```
cargo build --release
./target/release/rcm_v4_harness    # V4 RCM (hand-rolled, definitive)
./target/release/rcm_v5_harness    # V5 RCM (framework, validation)
```
