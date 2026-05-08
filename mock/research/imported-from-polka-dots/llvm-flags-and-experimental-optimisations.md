# LLVM Flags, Passes, and Experimental Optimisations

**Date:** 2026-03-13
**LLVM version:** 22.1.0 (shipped with rustc 1.95.0-nightly 2026-02-23)
**Purpose:** Comprehensive reference for LLVM-level optimisation flags
accessible through rustc, including experimental passes, post-link
tools, and diagnostic capabilities. Companion to the nightly features
and stable optimisation config documents.

**Target architecture:** aarch64-apple-darwin (dev), x86_64-unknown-linux-gnu (deploy)

---

## Table of Contents

1. [How LLVM Flags Reach rustc](#1-how-llvm-flags-reach-rustc)
2. [Vectorisation Tuning](#2-vectorisation-tuning)
3. [Inlining Tuning](#3-inlining-tuning)
4. [Loop Optimisation](#4-loop-optimisation)
5. [Memory and Cache Tuning](#5-memory-and-cache-tuning)
6. [GVN and Value Numbering](#6-gvn-and-value-numbering)
7. [Code Layout and Splitting](#7-code-layout-and-splitting)
8. [AArch64-Specific Flags](#8-aarch64-specific-flags)
9. [x86_64-Specific Flags](#9-x86_64-specific-flags)
10. [Polly (Polyhedral Optimiser)](#10-polly-polyhedral-optimiser)
11. [Post-Link Optimisation (BOLT)](#11-post-link-optimisation-bolt)
12. [AutoFDO](#12-autofdo)
13. [LLVM Diagnostics and Inspection](#13-llvm-diagnostics-and-inspection)
14. [Nightly -Z Flags (LLVM-Related)](#14-nightly--z-flags-llvm-related)
15. [What to Actually Enable](#15-what-to-actually-enable)

---

## 1. How LLVM Flags Reach rustc

Three mechanisms:

| Mechanism | Syntax | Stability |
|-----------|--------|-----------|
| `-C llvm-args=...` | `-C llvm-args=-flag=value` | Stable flag, but LLVM args are NOT stable — they change with LLVM versions |
| `-C passes=...` | `-C passes=pass1 pass2` | Stable flag, unstable pass names |
| `-Z` flags | `-Z flag=value` | Nightly only |

Multiple LLVM args in one flag use space separation:
```
-C llvm-args="-inline-threshold=500 --vectorize-loops"
```

Or in `.cargo/config.toml`:
```toml
[target.aarch64-apple-darwin]
rustflags = [
    "-C", "llvm-args=--inline-threshold=500",
    "-C", "llvm-args=--vectorize-loops",
]
```

**Important:** LLVM has 380+ flags. We list only the ones relevant to
our columnar execution engine — contiguous arrays, batch processing,
cache-aware inner loops, monomorphised trait dispatch.

---

## 2. Vectorisation Tuning

Our inner loops iterate over contiguous `[u8; 16]`-strided column
arrays. LLVM's autovectoriser is the primary mechanism for SIMD
without explicit intrinsics. These flags control its decisions.

### 2.1 Loop vectoriser

| Flag | Default | What it does | Our interest |
|------|---------|-------------|-------------|
| `--vectorize-loops` | on at O2+ | Master switch for loop vectorisation | Already on — leave on |
| `--force-vector-width=N` | 0 (auto) | Force SIMD width in elements | Experiment: `4` for 128-bit NEON, `8` for 256-bit AVX2 |
| `--force-vector-interleave=N` | 0 (auto) | Force loop body duplication for ILP | Experiment: `2` or `4` on out-of-order CPUs |
| `--vectorizer-maximize-bandwidth` | off | Pick VF from smallest type in loop | Try: our column values are mixed sizes |
| `--vectorizer-min-trip-count=N` | varies | Min iterations before vectorising | Lower if our morsels are small |
| `--vectorize-memory-check-threshold=N` | varies | Max runtime alias checks | Raise if vectoriser gives up on our column pointers |
| `--extra-vectorizer-passes` | off | Run cleanup after vectorisation | Try: may catch missed opportunities |
| `--enable-interleaved-mem-accesses` | off | Vectorise interleaved loads/stores | **Potentially high value** — our flag+payload layout is interleaved |
| `--enable-masked-interleaved-mem-accesses` | off | Masked variant | For nullable columns with flag bytes |
| `--prefer-predicate-over-epilogue=...` | varies | Tail-fold vs scalar epilogue | `predicate-else-scalar-epilogue` on AArch64 with SVE |

### 2.2 SLP (Superword-Level Parallelism) vectoriser

SLP finds parallelism in straight-line code (not loops) — e.g.,
packing multiple scalar operations into one SIMD instruction.

| Flag | Default | What it does |
|------|---------|-------------|
| `--vectorize-slp` | on at O2+ | Master switch |
| `--slp-threshold=N` | varies | Only vectorise if gain > N |
| `--slp-max-vf=N` | 0 (unlimited) | Max SLP factor |
| `--slp-max-reg-size=N` | varies | Target register width in bits |

### 2.3 Diagnosing vectorisation failures

The single most useful diagnostic for our inner loops:

```sh
RUSTFLAGS="-Cremark=loop-vectorize -Cdebuginfo=1" cargo build --release 2>&1 | grep "missed"
```

This tells you exactly WHY LLVM didn't vectorise a loop: aliasing,
non-contiguous access, unsupported operation, unknown trip count, etc.
Fix the cause, don't just crank up thresholds.

For interactive HTML visualisation:
```sh
cargo install cargo-remark
cargo remark build
# Opens browser with per-function vectorisation report
```

---

## 3. Inlining Tuning

Our architecture is monomorphisation-heavy — every `WorkUnit`,
`ColumnSlices`, `Column<In<IR>, As<T>>` is a concrete type with
inlineable methods. Aggressive inlining lets LLVM see through trait
dispatch and optimise the whole fused chain as one unit.

| Flag | Default | What it does | Our interest |
|------|---------|-------------|-------------|
| `--inline-threshold=N` | 225 | Main inlining cost limit | **Raise to 400-500** for release — our small methods should always inline |
| `--inlinehint-threshold=N` | ~325 | Threshold for `#[inline]` functions | Raise to 800+ — we use `#[inline]` on hot trait methods |
| `--inlinecold-threshold=N` | ~45 | Threshold for cold functions | Leave low — cold is cold |
| `--inline-enable-cost-benefit-analysis` | off | Smarter inlining based on call frequency | Try with PGO data |
| `--enable-partial-inlining` | off | Inline hot path of a function, leave cold in outline | **Try** — our `process_batch` may have hot+cold paths |
| `--inline-max-stacksize=N` | unlimited | Don't inline if stack > N | Set to prevent stack blowout in deeply nested generics |

**Note:** `-C inline-threshold=N` (the rustc flag) is **deprecated
and does nothing**. Use `-C llvm-args=--inline-threshold=N` instead.

### 3.1 MIR inlining (nightly)

Rust has its own pre-LLVM inlining pass at the MIR level:

| Flag | Default | What it does |
|------|---------|-------------|
| `-Z inline-mir=yes` | yes | Enable MIR inlining |
| `-Z inline-mir-threshold=N` | 50 | MIR inlining cost threshold |
| `-Z inline-mir-hint-threshold=N` | 100 | Threshold for `#[inline]` |
| `-Z cross-crate-inline-threshold=N` | varies | Cross-crate MIR inlining |

MIR inlining happens before LLVM, so it gives LLVM a better starting
point. With our monomorphised trait methods, raising these thresholds
could help LLVM see through more abstraction layers. But it also
increases compile time — profile before cranking.

---

## 4. Loop Optimisation

### 4.1 Unrolling

Our morsel processing loops have known iteration counts (morsel size
is computed at startup, clamped to multiples of 4). Unrolling helps
LLVM schedule instructions across iterations.

| Flag | Default | What it does | Our interest |
|------|---------|-------------|-------------|
| `--unroll-threshold=N` | varies by opt-level | Cost threshold for unrolling | Raise moderately for O3 |
| `--unroll-count=N` | auto | Force specific unroll factor | Experiment: `4` matches our morsel alignment |
| `--unroll-allow-partial` | varies | Allow partial unrolling | Enable — our morsel sizes aren't always powers of 2 |
| `--unroll-allow-remainder` | varies | Generate remainder iterations | Enable |
| `--unroll-allow-peeling` | varies | Peel first iterations when trip count is low | Enable |
| `--unroll-runtime` | varies | Unroll loops with runtime trip count | Enable — morsel size is runtime-determined |

### 4.2 Loop transforms

| Flag | What it does | Our interest |
|------|-------------|-------------|
| `--enable-loop-distribute` | Split loop into independent parts | **Try** — multi-column processing might benefit |
| `--enable-loop-flatten` | Flatten nested loops into one | Try for 2D iteration patterns |
| `--loop-interchange-threshold=N` | Threshold for loop interchange | Relevant if we iterate columns-then-rows vs rows-then-columns |

### 4.3 Alignment

| Flag | What it does | Our interest |
|------|-------------|-------------|
| `--align-loops=N` | Align loop headers to N bytes | `32` or `64` — reduces I-cache misses on hot loops |

---

## 5. Memory and Cache Tuning

| Flag | Default | What it does | Our interest |
|------|---------|-------------|-------------|
| `--cache-line-size=N` | target-dependent | Override cache line size | Set to `64` (or `128` on M1 — Apple Silicon has 128-byte cache lines on P-cores) |
| `--prefetch-distance=N` | varies | Instructions ahead to prefetch | Experiment — columnar scans are sequential, prefetching helps |

Apple Silicon note: M1/M2/M3 have **128-byte cache lines on
performance cores** and 64-byte on efficiency cores. Setting
`--cache-line-size=128` may improve alignment decisions for P-core
hot paths, but could waste space for E-core execution. Default
(64) is safer.

---

## 6. GVN and Value Numbering

GVN (Global Value Numbering) eliminates redundant computations and
loads. Relevant for our trait method dispatch where the same offset
computation may appear multiple times.

| Flag | Default | What it does | Our interest |
|------|---------|-------------|-------------|
| `--enable-gvn-hoist` | off | Hoist common expressions to dominator blocks | **Try** — shared column offset calculations |
| `--enable-gvn-sink` | off | Sink common expressions to successor blocks | Try |
| `--gvn-max-num-deps=N` | 100 | Max dependencies for load PRE | Raise if column accesses create deep dep chains |

---

## 7. Code Layout and Splitting

These affect I-cache behaviour by controlling how code is laid out
in the binary.

| Flag | What it does | Requires | Our interest |
|------|-------------|----------|-------------|
| `--hot-cold-split` | Move cold blocks to separate section | Nothing (heuristic) | **Enable** — our error paths are cold |
| `--split-machine-functions` | Split functions at block level | Profile data | Enable with PGO |
| `--basic-block-sections=all` | Each BB in its own section | Profile data + linker support | Advanced — use with BOLT |
| `--function-sections` | Each function in its own section | Nothing | Enable — allows linker GC |
| `--data-sections` | Each data item in its own section | Nothing | Enable — allows linker GC |

`--hot-cold-split` is the easy win — no profile data needed. LLVM
uses heuristics (cold attributes, unreachable blocks) to identify
cold code and moves it away from hot paths, improving I-cache
utilisation.

---

## 8. AArch64-Specific Flags

### 8.1 Machine outliner

The outliner finds repeated instruction sequences across functions
and outlines them into shared subroutines. Reduces code size at the
cost of an extra call+return per outline. Enabled by default at
`-Os` and `-Oz`.

| Flag | Default | What it does |
|------|---------|-------------|
| `--enable-machine-outliner` | on at Os/Oz | Enable outlining |
| `--machine-outliner-reruns=N` | 0 | Re-run outliner for more savings |
| `--outliner-benefit-threshold=N` | varies | Min byte saving to accept |

For release builds optimising for speed (O3), the outliner is OFF
by default — the extra call/return hurts performance. Leave it off
for our hot paths. Consider enabling for cold code sections only
(via `--hot-cold-split` + separate compilation).

### 8.2 SVE (Scalable Vector Extension)

Apple Silicon does NOT have SVE. These flags are only relevant for
Linux deployment on ARM servers (Neoverse V1/V2, Grace):

| Flag | What it does |
|------|-------------|
| `--aarch64-sve-vector-bits-min=N` | Assume SVE registers are at least N bits |
| `--aarch64-sve-vector-bits-max=N` | Assume SVE registers are at most N bits |
| `--sve-tail-folding=...` | Control SVE tail-folding strategy |

For Neoverse V1 (256-bit SVE): `--aarch64-sve-vector-bits-min=256`

### 8.3 Branch target identification (BTI)

| Flag | What it does |
|------|-------------|
| `-C target-feature=+bti` | Enable Branch Target Identification |

BTI is a security feature (forward-edge CFI). Small performance cost.
Consider for hardened builds.

---

## 9. x86_64-Specific Flags

### 9.1 Branch alignment

| Flag | What it does |
|------|-------------|
| `--x86-align-branch=jcc+fused+jmp` | Align branch instructions |
| `--x86-align-branch-boundary=32` | Alignment boundary |
| `--x86-branches-within-32B-boundaries` | Mitigate Intel SKX102 errata |

The SKX102 errata causes branch prediction degradation when a
branch instruction crosses a 32-byte boundary. This flag inserts
NOP padding to prevent it. Relevant for Intel Skylake through
Ice Lake. No effect on AMD.

### 9.2 AVX/SSE encoding

| Flag | What it does |
|------|-------------|
| `--x86-sse2avx` | Encode SSE instructions with VEX prefix |

VEX encoding avoids the SSE-AVX transition penalty on Intel CPUs.
If targeting AVX2 or higher, this is generally beneficial.

---

## 10. Polly (Polyhedral Optimiser)

### 10.1 What Polly does

Polly uses integer polyhedra mathematics to:
- **Tile loop nests** for cache locality (including 3D hexagonal tiling)
- **Fuse loops** for data locality
- **Interchange loops** for stride-1 access
- **Pack arrays** for contiguous access
- **Detect GEMM** patterns and generate optimised code

### 10.2 Why it matters for us

Our columnar execution model is exactly Polly's sweet spot:
- Contiguous arrays (`[Entry; N]` where `Entry = [u8; 16]`)
- Known iteration counts (morsel size)
- Sequential access patterns (column scans)
- Multiple columns processed in fused chains

Polly could potentially:
- Tile our morsel processing for L1 cache (we do this manually via
  morsel sizing, but Polly could do it automatically)
- Fuse column scans that we currently process separately
- Interchange iteration order if we process columns in the wrong
  direction for cache

### 10.3 Availability — pluggable via `-Z llvm-plugins`

**NOT compiled into standard rustc builds**, but Polly IS loadable
as a shared library plugin. Polly exports `llvmGetPassPluginInfo` —
the standard New Pass Manager plugin entry point — and can be loaded
at runtime.

**Mechanism:**

```sh
# Build Polly as a shared library against LLVM 22.1.0
# (must match the exact LLVM version in our nightly rustc)
cmake -S llvm-project/polly -B build-polly \
    -DLLVM_DIR=/path/to/llvm-22.1.0/lib/cmake/llvm \
    -DBUILD_SHARED_LIBS=ON
cmake --build build-polly

# Load into rustc via -Z llvm-plugins (nightly)
RUSTFLAGS="-Z llvm-plugins=/path/to/LLVMPolly.so -C passes=polly" \
    cargo +nightly build --release
```

This is NOT "build all of rustc from source." It's building one
LLVM subproject (~5 min) against the matching LLVM version. The
result is a `.so`/`.dylib` that rustc loads at compile time.

**Caveats:**
- Must match the exact LLVM major version (22.x for our nightly)
- `-Z llvm-plugins` is nightly-only (we use nightly — fine)
- Polly's `.so` must be rebuilt when updating the nightly toolchain
  if the LLVM version bumps
- rust-lang/rust#39884 (open since 2017) tracks first-class support;
  last activity 2020, unlikely to move soon

**Status of rust-lang/rust#39884:**

The issue has been open since 2017. PR #78566 (merged 2020) enabled
passing `-polly` via `-C llvm-args`, but this only works if Polly
is compiled INTO the LLVM that rustc ships — which standard builds
don't do. The plugin route (`-Z llvm-plugins`) bypasses this
limitation entirely.

### 10.4 Practical alternative (what we do today)

Our manual design achieves Polly's core optimisations:
- **Tiling:** morsel sizing formula (L1-aware, clamped to multiples
  of 4)
- **Loop fusion:** fused chain model (operators in a chain process
  the same morsel in sequence)
- **Contiguous access:** column storage guarantees stride-1 access

Polly would automate and potentially improve on what we do by hand
— particularly loop interchange and array packing that our manual
approach doesn't attempt. Worth trying once the execution engine
exists and we have hot loops to profile.

### 10.5 New Pass Manager status

Polly has **completed migration to NPM** as of LLVM 22. Legacy PM
support is removed. The NPM plugin interface (`llvmGetPassPluginInfo`)
is the only supported loading mechanism — which is exactly what
`-Z llvm-plugins` uses. No compatibility concerns.

### 10.6 Recommendation

**Phase 1 (now):** Don't invest in building Polly. Our manual
morsel/chain design is sound and the execution engine doesn't
exist yet.

**Phase 2 (after hot loops exist):** Build Polly against our
nightly's LLVM, load via `-Z llvm-plugins`, compare inner loop
codegen with and without Polly. If Polly finds optimisations our
manual approach misses, keep it in the release build pipeline.

The build cost is low (~5 min), the integration is clean (one
flag), and the potential upside is real (polyhedral tiling of our
column processing loops). This is worth attempting.

---

## 11. Post-Link Optimisation (BOLT)

BOLT (Binary Optimization and Layout Tool) reorders code layout in
an already-compiled binary based on execution profiles. It operates
on ELF binaries (Linux only, not macOS Mach-O).

### 11.1 What it does

- Reorders basic blocks within functions (ext-TSP algorithm)
- Reorders functions globally (HFSort — call-graph-aware)
- Splits hot/cold code
- Optimises jump targets
- Removes redundant loads

### 11.2 Typical gains

2-5% improvement in CPU cycles. Stacks with PGO and LTO — they
optimise different things:
- **PGO:** branch layout within LLVM (compile-time)
- **LTO:** cross-crate inlining and dead code (link-time)
- **BOLT:** binary-level code placement (post-link)

### 11.3 Workflow

```sh
cargo install cargo-pgo

# Option A: Instrumentation-based
cargo pgo bolt build              # build with relocations
./target/release/binary-bolt-instrumented  # run representative workload
cargo pgo bolt optimize           # apply profile to binary

# Option B: Sampling-based (no runtime overhead)
cargo build --release
perf record -e cycles:u -j any,u -- ./target/release/binary
perf2bolt -p perf.data -o bolt.fdata ./target/release/binary
llvm-bolt ./target/release/binary -o ./target/release/binary.bolt \
    -data bolt.fdata \
    -reorder-blocks=ext-tsp \
    -reorder-functions=hfsort
```

### 11.4 Requirements

- Linux x86_64 or AArch64 ELF (not macOS Mach-O)
- Binary linked with `--emit-relocs` or `-q` (for BOLT to rewrite)
- Unstripped symbols (or separate debug info)
- Representative workload for profiling

### 11.5 Our recommendation

BOLT makes sense for **deployed Linux release binaries** after
the architecture stabilises. Don't invest in it during design phase.
The 2-5% gain is real but the workflow complexity is high.

---

## 12. AutoFDO

AutoFDO (Automatic Feedback-Directed Optimization) uses sampled
profiles from `perf` (no instrumentation overhead) to guide
optimisation. Achieves ~85% of full PGO gains with zero runtime
overhead during profiling.

### 12.1 Workflow (nightly)

```sh
# Profile with perf (sampling, no instrumentation)
perf record -b -e cycles:u -- ./target/release/binary

# Convert to LLVM format
create_llvm_prof --binary=./target/release/binary --out=sample.prof

# Rebuild with profile
RUSTFLAGS="-Zprofile-sample-use=sample.prof" cargo +nightly build --release
```

### 12.2 vs PGO

| Aspect | PGO | AutoFDO |
|--------|-----|---------|
| Profiling overhead | 10-30% runtime | Zero (sampling) |
| Profile accuracy | Exact | ~85% of PGO |
| Build passes | 2 (instrumented + optimised) | 2 (profiled + optimised) |
| Profiling binary | Special instrumented build | Regular release build |
| Nightly required? | No (stable) | Yes (`-Zprofile-sample-use`) |

AutoFDO is simpler for production workloads because you can profile
the actual production binary without deploying an instrumented build.

---

## 13. LLVM Diagnostics and Inspection

### 13.1 Optimisation remarks

The most immediately actionable tool. Tells you exactly why LLVM
made specific decisions.

```sh
# All remarks
RUSTFLAGS="-Cremark=all -Cdebuginfo=1" cargo build --release

# Vectorisation only (most useful for us)
RUSTFLAGS="-Cremark=loop-vectorize -Cdebuginfo=1" cargo build --release

# Inlining only
RUSTFLAGS="-Cremark=inline -Cdebuginfo=1" cargo build --release

# YAML output for tooling (nightly)
RUSTFLAGS="-Cdebuginfo=1 -Cremark=all -Zremark-dir=/tmp/remarks" \
    cargo +nightly build --release
```

Remark types:
- **Passed:** "vectorized loop" — optimisation applied
- **Missed:** "cannot vectorize: unsafe dependence" — WHY it failed
- **Analysis:** metrics and details

### 13.2 cargo-remark

Interactive HTML visualisation of optimisation remarks:

```sh
cargo install cargo-remark
cargo remark build
# Opens browser with per-function report
```

### 13.3 IR inspection

```sh
# Emit LLVM IR (after optimisation)
rustc --emit llvm-ir -C opt-level=3 file.rs

# Emit LLVM IR (before optimisation — raw from rustc)
rustc -O --emit llvm-ir -C no-prepopulate-passes file.rs

# Emit assembly (see what SIMD instructions were generated)
rustc --emit asm -C opt-level=3 file.rs

# Save all intermediates (.bc, .ll, .s, .o)
rustc -C save-temps file.rs
```

### 13.4 Pass timing (nightly)

```sh
# Time each LLVM pass
RUSTFLAGS="-Ztime-llvm-passes" cargo +nightly build --release

# Chrome-compatible trace for analysis
RUSTFLAGS="-Zllvm-time-trace" cargo +nightly build --release
# Open chrome://tracing and load the generated JSON

# Print pass pipeline
RUSTFLAGS="-Zprint-llvm-passes" cargo +nightly build --release
```

### 13.5 Bisecting optimisation failures

If a specific optimisation causes wrong codegen:

```sh
# Binary search for the problematic pass
RUSTFLAGS='-C llvm-args=-opt-bisect-limit=100' cargo build --release
# Decrease limit until the bug disappears → identifies the pass
```

### 13.6 IR verification

```sh
# Verify LLVM IR correctness after each pass (nightly, slow)
RUSTFLAGS="-Zverify-llvm-ir" cargo +nightly build --release
```

---

## 14. MIR Optimisation Passes

MIR (Mid-level Intermediate Representation) passes run BEFORE the
code reaches LLVM. They operate on Rust's own IR, which preserves
type information, borrow semantics, and Rust-specific constructs
that LLVM can't reason about. MIR inlining is particularly important
for us — it gives LLVM a pre-inlined starting point.

### 14.1 The full pass pipeline (44 passes)

These run in order on every function after borrow checking. Most run
at the default `-Z mir-opt-level=2` (optimised builds). Passes
marked "gated 1+" only run at mir-opt-level ≥ 1.

| # | Pass | What it does | Our interest |
|---|------|-------------|-------------|
| 1 | CheckAlignment | Verify alignment constraints | Safety |
| 2 | CheckNull | Verify null pointer checks | Safety |
| 3 | CheckEnums | Verify enum discriminant validity | Safety |
| 4 | LowerSliceLenCalls | Lower `.len()` on slices | Always |
| 5 | InstSimplify::BeforeInline | Simplify instructions pre-inline | Always |
| 6 | ForceInline | Inline `#[inline(always)]` functions | **Critical** — morsel dispatch |
| 7 | **Inline** | MIR-level inlining (threshold-controlled) | **Critical** — SDK trait methods |
| 8 | RemoveStorageMarkers | Drop storage live/dead markers | Codegen |
| 9 | RemoveZsts | Eliminate zero-sized type operations | Our newtypes may hit this |
| 10 | RemoveUnneededDrops | Eliminate drops of Copy types | **Relevant** — Payload: Copy |
| 11 | UnreachableEnumBranching | Remove unreachable match arms | Always |
| 12 | UnreachablePropagation | Propagate unreachable blocks | Always |
| 13 | SimplifyCfg (gated 1+) | Simplify control flow graph | Always |
| 14 | MultipleReturnTerminators | Merge multiple return paths | Always |
| 15 | InstSimplify::AfterSimplifyCfg | Instruction simplification | Always |
| 16 | SimplifyConstCondition (gated 1+) | Evaluate constant conditions | Always |
| 17 | **ReferencePropagation** | Replace ref-to-ref with direct ref | **Relevant** — column slice access |
| 18 | **ScalarReplacementOfAggregates** | Break structs into scalars | **Critical** — our newtypes/Payload types |
| 19 | SimplifyLocals::BeforeConstProp | Remove unused locals | Always |
| 20 | **DeadStoreElimination::Initial** | Remove stores never read | **Relevant** — column writes |
| 21 | **GVN** | Global Value Numbering | **Critical** — column offset dedup |
| 22 | SimplifyLocals::AfterGVN | Clean up after GVN | Always |
| 23 | SsaRangePropagation | Propagate value ranges | Always |
| 24 | MatchBranchSimplification | Simplify match branches | Always |
| 25 | **DataflowConstProp** | Constant propagation via dataflow | **Relevant** — morsel size, stride |
| 26 | SingleUseConsts | Inline single-use constants | Always |
| 27 | SimplifyConstCondition (gated 1+) | Evaluate constant conditions | Always |
| 28 | **JumpThreading** | Optimise branch chains | **Relevant** — chain dispatch |
| 29 | EarlyOtherwiseBranch | Simplify `_` match arms | Always |
| 30 | SimplifyComparisonIntegral | Simplify integer comparisons | Always |
| 31 | SimplifyConstCondition (gated 1+) | Final constant condition pass | Always |
| 32 | RemoveNoopLandingPads (gated 1+) | Remove empty landing pads | Always |
| 33 | SimplifyCfg (gated 1+) | Final CFG simplification | Always |
| 34 | StripDebugInfo | Strip debug info (release) | Always |
| 35 | **CopyProp** | Copy propagation | **Relevant** — value forwarding |
| 36 | **DeadStoreElimination::Final** | Final dead store pass | **Relevant** |
| 37 | **DestinationPropagation** | Reuse destinations, eliminate copies | **Critical** — column value paths |
| 38 | SimplifyLocals::Final | Final local cleanup | Always |
| 39 | MultipleReturnTerminators | Second pass | Always |
| 40 | EnumSizeOpt | Optimise enum size (threshold: 128) | Our repr(C) types may benefit |
| 41 | CriticalCallEdges | Split critical edges at calls | Codegen |
| 42 | ReorderBasicBlocks | Hot/cold block ordering | **Relevant** — I-cache |
| 43 | ReorderLocals | Optimise local variable layout | Always |
| 44 | Marker("PreCodegen") | Testing/debugging marker | Internal |

### 14.2 Passes that matter most for us

**MIR Inlining (#7)** — controlled by `-Z inline-mir-threshold` and
`-Z cross-crate-inline-threshold`. This is the single most important
MIR pass for our architecture. SDK trait methods (Language::parse,
Source::resolve, Payload::to_slot) are defined in polka-sdk but
called from polka-pipeline, polka-cli, and extensions. Without
cross-crate MIR inlining, LLVM sees opaque function calls at every
crate boundary.

Our config: threshold=100 (2× default), hint-threshold=200 (2×
default), cross-crate=200. These ensure small trait methods are
inlined before LLVM, giving LLVM the full picture for vectorisation.

**ScalarReplacementOfAggregates (#18)** — breaks structs into their
component scalars. Our column values are newtypes wrapping
primitives: `struct VersionStr([u8; 15])`. SROA decomposes these
so LLVM sees the raw bytes, enabling register allocation and SIMD.

**GVN (#21)** — eliminates redundant computations at MIR level. When
multiple ops in a fused chain compute the same column offset, MIR
GVN deduplicates them before LLVM even runs. This stacks with
LLVM's own GVN (`--enable-gvn-hoist`).

**DestinationPropagation (#37)** — eliminates copies by writing
results directly to their final destination. In column processing,
this avoids temporary allocations when passing values between ops
in a fused chain.

### 14.3 `-Z mir-opt-level` settings

| Level | What it enables | Our recommendation |
|-------|----------------|-------------------|
| 0 | No MIR optimisations | Debug only |
| 1 | Basic CFG simplification, const conditions | Minimum |
| 2 | All standard passes (the 44 above) | **Default for us** |
| 3 | Experimental passes | Not yet — track stability |
| 4 | Unsound/broken passes | Never |

`-Z unsound-mir-opts` exists as a separate flag (not tied to levels).
It enables passes with known soundness bugs. **Do not use** — it's
for the rustc test suite, not production code.

### 14.4 MIR-related -Z flags (complete list)

| Flag | Default | What it does | Our setting |
|------|---------|-------------|------------|
| `-Z inline-mir=yes/no` | yes | Master switch for MIR inlining | yes |
| `-Z inline-mir-threshold=N` | 50 | Cost limit for MIR inlining | **100** |
| `-Z inline-mir-hint-threshold=N` | 100 | Cost limit for `#[inline]` functions | **200** |
| `-Z cross-crate-inline-threshold=N` | varies | Cross-crate MIR inlining threshold | **200** |
| `-Z mir-opt-level=N` | 2 (opt) / 1 (debug) | MIR optimisation level | 2 (default) |
| `-Z mir-enable-passes=+name,-name` | all enabled | Enable/disable specific passes | Default |
| `-Z dump-mir=pass_name` | off | Dump MIR after specific pass | Debugging |
| `-Z dump-mir-dir=path` | `mir_dump/` | Directory for MIR dumps | Debugging |
| `-Z unsound-mir-opts` | off | Enable unsound passes | **Never** |

---

## 15. Nightly -Z Flags (LLVM-Related)

| Flag | What it does | When to use |
|------|-------------|-------------|
| `-Z print-llvm-passes` | Print pass pipeline | Understand what runs |
| `-Z time-llvm-passes` | Time each pass | Find compile bottlenecks |
| `-Z llvm-time-trace` | Chrome trace | Visualise pass timing |
| `-Z verify-llvm-ir` | Validate IR after passes | Debug miscompilation |
| `-Z fewer-names` | Reduce IR name retention | Reduce memory in large builds |
| `-Z merge-functions=...` | Control MergeFunctions pass | Reduce binary size |
| `-Z remark-dir=path` | YAML remarks directory | Tooling input |
| `-Z llvm-plugins=list` | Load LLVM pass plugins | Polly, custom passes |
| `-Z profile-sample-use=file` | AutoFDO profile | Sampled PGO |
| `-Z instrument-xray` | XRay instrumentation | Low-overhead tracing |

---

## 15. What to Actually Enable

### 15.1 Immediately (release profile)

Low-risk flags that improve optimisation with minimal compile cost:

```toml
# .cargo/config.toml (release builds)
[target.aarch64-apple-darwin]
rustflags = [
    # Parallel compilation
    "-Zthreads=8",
    # Generic sharing
    "-Zshare-generics=y",
    # Target CPU
    "-C", "target-cpu=native",
    # Aggressive inlining (our methods are small)
    "-C", "llvm-args=--inline-threshold=400",
    # Hoist common subexpressions
    "-C", "llvm-args=--enable-gvn-hoist",
    # Move cold code away from hot paths
    "-C", "llvm-args=--hot-cold-split",
    # Each function in its own section (enables linker GC)
    "-C", "llvm-args=--function-sections",
    "-C", "llvm-args=--data-sections",
]
```

### 15.2 Experiment with (profile first)

These need benchmarking to confirm benefit for our specific patterns:

```toml
rustflags = [
    # Interleaved memory access vectorisation
    # Our [flags, payload] layout is interleaved — this might help
    "-C", "llvm-args=--enable-interleaved-mem-accesses",
    # Extra vectoriser cleanup passes
    "-C", "llvm-args=--extra-vectorizer-passes",
    # Partial inlining (inline hot path, outline cold)
    "-C", "llvm-args=--enable-partial-inlining",
    # Loop distribution (split multi-statement loops)
    "-C", "llvm-args=--enable-loop-distribute",
    # Maximise vectorisation bandwidth
    "-C", "llvm-args=--vectorizer-maximize-bandwidth",
    # Runtime unrolling for runtime-determined morsel sizes
    "-C", "llvm-args=--unroll-runtime",
    # Align hot loops to cache line
    "-C", "llvm-args=--align-loops=64",
]
```

### 15.3 Use for diagnostics (not production)

```sh
# WHY didn't LLVM vectorise my inner loop?
RUSTFLAGS="-Cremark=loop-vectorize -Cdebuginfo=1" cargo build --release

# WHAT did LLVM decide about inlining?
RUSTFLAGS="-Cremark=inline -Cdebuginfo=1" cargo build --release

# WHERE is compile time going?
RUSTFLAGS="-Ztime-llvm-passes" cargo +nightly build --release

# Interactive HTML report
cargo remark build
```

### 15.4 Use for deployed Linux binaries (when stable)

```sh
# PGO (stable, 10-20% gain)
RUSTFLAGS="-Cprofile-generate=/tmp/pgo" cargo build --release
./target/release/binary  # run representative workload
llvm-profdata merge -o /tmp/merged.profdata /tmp/pgo
RUSTFLAGS="-Cprofile-use=/tmp/merged.profdata" cargo build --release

# BOLT (post-link, 2-5% on top of PGO)
cargo pgo bolt build && ./target/release/binary-bolt-instrumented
cargo pgo bolt optimize

# AutoFDO (nightly, zero-overhead profiling)
perf record -b -e cycles:u -- ./target/release/binary
create_llvm_prof --binary=./target/release/binary --out=sample.prof
RUSTFLAGS="-Zprofile-sample-use=sample.prof" cargo +nightly build --release
```

### 15.5 The full optimisation stack (updated)

```
Compile-time:
  -Zthreads=8 + -Zshare-generics=y     parallel + dedup
  cranelift (dev) / LLVM (release)      speed vs quality
  lto=thin + codegen-units=1            cross-crate optimisation
  --inline-threshold=400                aggressive inlining
  --enable-gvn-hoist                    eliminate redundant loads
  --hot-cold-split                      I-cache improvement
  --function-sections + --data-sections linker dead code elimination
  -Zbuild-std                           LTO across std

Profile-guided:
  PGO (stable)                          10-20% from branch layout
  AutoFDO (nightly)                     85% of PGO, zero overhead
  BOLT (post-link)                      2-5% from code placement

Together:
  LTO + PGO + BOLT = 15-30% over baseline release
```

### 15.6 What NOT to do

- **Don't raise `--inline-threshold` above ~600** without profiling —
  compilation time and binary size explode
- **Don't use `--force-vector-width` without checking remarks first**
  — if LLVM chose a smaller width, it had a reason (usually aliasing)
- **Don't use `--enable-unsafe-fp-math`** unless you truly don't
  care about IEEE compliance — we probably don't need it
- **Don't use `--unroll-count=N` globally** — some loops shouldn't
  unroll (cold paths, variable-trip-count loops)
- **Don't build with Polly** unless you're willing to maintain a
  custom rustc — the build infra cost isn't worth it when our manual
  morsel/chain design already achieves the same tiling
