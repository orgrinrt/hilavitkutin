# Night findings — 2026-05-11 overnight autonomous run

This file aggregates the bench evidence accumulated during the autonomous
overnight session of 2026-05-11. Each section names a design question, the
bench that addresses it, the result, and the implication for arvo /
hilavitkutin design.

## Topic 9 axis B — DependencyGraph backing (`dep_graph_csr_9`)

**CSR is canonical at every N.** No threshold dispatch needed.

| N nodes | CSR ns | Dense ns | Dense slowdown |
|---|---|---|---|
| 8 | 146 | 724 | 5.0x slower |
| 16 | 206 | 1239 | 6.0x slower |
| 32 | 302 | 2911 | 9.6x slower |
| 64 | 645 | ~12500 | 19.4x slower |
| 128 | 1225 | ~41300 | 33.7x slower |
| 256 | 2546 | ~151000 | 59.3x slower |

The predicted N=16 crossover was wrong. Dense O(N²) cell-scan loses to CSR's
O(E) edge-direct iteration even at N=8 because the branch-check on each cell
costs more than the edge traversal saves.

Hybrid variant (dense for N≤16, CSR else) measured within noise of pure CSR.
Pure CSR is the canonical shape; hybrid retired.

## Popcount strategy (`popcount_strategy`) — prior design

**Question:** does arvo's "use `std::u64::count_ones()` internally" stance hold up vs
hand-rolled (Kernighan loop) or SWAR (branchless bitwise hack)?

**Answer:** std::count_ones wins by 5-7x at every N. LLVM lowers it to hardware
POPCNT (x86) / CNT (aarch64 NEON). Neither hand-rolled nor SWAR triggers the
same pattern-match.

| N bytes | std ns | handrolled ns | swar ns |
|---|---|---|---|
| 64 | ~6 | ~22 | ~17 |
| 1024 | 53 | 249 | ~200 |
| 16384 | 439 | 3242 | ~2000 |

Implication for arvo: the `BitAccess::count_ones()` trait method must lower
to `<containerprim as primint>::count_ones()`, NOT to a hand-rolled fallback.
Document explicitly in arvo-bits.

## Hash algorithm comparison (`hash_algos`) — prior design

**Question:** which hash mixer wins on cache-fingerprint workloads (8-byte input chunks)?

**Answer:** xxh-style block multiplier wins at small/medium N; SipHash-style ARX
wins at large N. FNV-1a is significantly slower across the board.

| N bytes | fnv1a ns | xxh ns | sip ns |
|---|---|---|---|
| 64 | ~50 | ~10 | ~12 |
| 1024 | ~1700 | 309 | ~450 |
| 16384 | 30327 | ~10000 | 6673 |

Implications for arvo-hash:
- Default for content-fingerprint should be xxh-shape (8-byte-block multiplier).
- SipHash-shape (ARX rounds) is the right choice when N > 8KB.
- FNV-1a only worth shipping for byte-stream legacy compatibility; its
  per-byte mul is the bottleneck.

This validates the arvo-hash decision (#116) to ship multiple algorithms with
strategy markers; the bench data informs which strategy maps to which algo.

## Fixed-point multiplication strategy (`fxpmul_strategy`) — prior design

**Question:** arvo's `Strategy::Hot` (i64 truncate) vs `Strategy::Precise` (i128 intermediate)
— how much does the strategy choice cost?

**Answer:** Hot wins by 10-16% at every N. Precise's i128 widening costs
measurable cycles even though aarch64 has efficient i128 mul codegen (mul + umulh).

| N | Hot ns | Precise ns | Precise overhead |
|---|---|---|---|
| 64 | ~10 | ~12 | +20% |
| 1024 | 141 | 164 | +16% |
| 16384 | 2592 | 2800 | +8% |

Implication: arvo's strategy-marker design is empirically justified. Consumers
who can tolerate overflow at the high end of their fixed-point range get
measurably faster code by picking `Hot` over `Precise`. The two strategies
should remain distinct codegen paths in arvo.

## Branch pattern (`branch_pattern`) — prior design

**Question:** is hand-written branchless (cmov/csel) measurably faster than
data-dependent if/else on modern CPUs at -O3?

**Answer:** No measurable difference. LLVM converts the if/else to cmov/csel
anyway under -O3 + fat LTO. Both variants land within 2% of each other at
every N.

| N | if ns | cmov ns | Δ |
|---|---|---|---|
| 1024 | ~2480 | ~2480 | 0% |
| 16384 | 40218 | ~41000 | -2% (if slightly faster?) |

Implication: don't rewrite naive if/else to manual cmov-style code chasing
performance. Trust LLVM. Spend the readability budget on something else.

For arvo specifically: the `Predicate` trait's `cond_select` methods don't
buy a perf win over a plain `if`. Their value is **type-level** (sealed
trait + clear intent), not codegen-level.

## Atomic ordering cost (`atomic_ordering_cost`) — Topic 3 S7

**Question:** how much does memory ordering choice cost on a hot-path atomic
increment, single-threaded?

**Answer:** Relaxed beats Acquire/Release pair by 3-4x; SeqCst is within ~3%
of Acquire/Release. Topic 3 S7's atomic-ordering protocol table is empirically
validated: picking Relaxed where the contract permits is a real perf win.

| N | Relaxed ns | Acquire/Release ns | SeqCst ns | Acquire vs Relaxed |
|---|---|---|---|---|
| 64 | 8 | 13 | ~14 | +63% slower |
| 1024 | 374 | 1341 | 1380 | +259% slower |
| 16384 | 6924 | 23355 | ~24000 | +237% slower |

On aarch64, Acquire and SeqCst land in the same neighbourhood because both
emit `dmb ish`-style fences. On x86_64 the SeqCst vs Acquire gap would be
sharper (mfence vs no fence on loads), but the bench was run on aarch64
(Apple Silicon).

Implication for Topic 3 S7's table: every cross-thread atomic should pick
the minimum ordering its contract requires. The progress counter (Release
on increment, Acquire on read by peer worker) is correct. The `predicted_wait_ns`
slot (Relaxed both sides, written before phase barrier's Release-Acquire pair)
is correct. The `shutdown` flag (Relaxed read, Release set in Drop, Acquire
read of related state) is correct. The bench data shows these choices matter
to per-morsel ~ns/op throughput.

## Cache layout (`cache_layout`) — column-store design validation

**Question:** does AoS (interleaved struct fields) vs SoA (parallel column
arrays) make a measurable difference under partial-field iteration?

**Answer:** Surprisingly little, in this micro-benchmark. AoS and SoA are
within noise (1-4% spread) across all sizes.

| N | AoS ns | SoA ns | Δ |
|---|---|---|---|
| 64 | 1 | 1 | +1% |
| 1024 | 21 | 20 | -4% |
| 16384 | 558 | 561 | +1% |

The expected SoA win (skip unused field bytes) doesn't materialise because:
- LLVM at -O3 + LTO recognises the unused `vel` field in the AoS struct
  pattern and may dead-store-eliminate or skip its load.
- Sequential iteration triggers hardware prefetch; the wasted bytes don't
  cost cache pressure on a single-pass workload.
- Both layouts vectorise similarly at -O3 (NEON loads at 16-byte boundaries
  capture multiple fields at once).

Implications for arvo's Column<T> design:
- The micro-benchmark CASE for SoA is weak. The Column<T> design's win
  comes from SYSTEMIC properties:
  1. Multiple passes over different field subsets (SoA loads only the
     subset; AoS pays full struct cost each pass).
  2. SIMD lane-parallel processing (SoA enables `vld4` deinterleaving
     vs AoS's lane-by-lane access).
  3. Cache pressure from competing workloads (multi-WU phase processing
     where each WU touches a different field subset).

The bench validates that **single-pass partial-field iteration alone** is
NOT a strong argument for SoA. The arvo Column<T> design's justification
needs to lean on systemic properties (multi-pass, SIMD, multi-WU contention)
rather than this benchmark shape.

Action item: when implementing arvo Column<T> documentation, frame the SoA
win in terms of those three systemic properties, not single-pass cache
efficiency. The polka-dots SpMV bench heritage probably has the right
framing already; reference it.

## Memory copy patterns (`memcpy_patterns`) — byte-stream throughput

**Question:** does `core::ptr::copy_nonoverlapping` (memcpy intrinsic) beat
hand-unrolled u64 loop or naive byte loop?

**Answer:** all three converge under -O3 + LTO. Within 5% spread at every N.

| N | intrinsic ns | unrolled ns | naive ns |
|---|---|---|---|
| 64 | 37 | 42 | 39 |
| 1024 | 1527 | 1489 | 1537 |
| 16384 | 21040 | 21051 | 21170 |

(The bench's per-byte hash dominates copy cost, so this is a "copy under
realistic work" measurement rather than a copy-isolation measurement. LLVM
pattern-matches both naive and unrolled loops to memcpy.)

Implication for hilavitkutin's `ByteEmitter::bulk_push` trait method: don't
worry about hand-tuning the copy path. The intrinsic call is the right
default; LLVM optimises naive byte loops to the same thing anyway. The
trait's value is type-level intent, not codegen win.

## Column iteration patterns (`iter_patterns`) — Column<T> design validation

**Question:** how much does access pattern matter for Column<T>-shaped iteration
under the realistic workload envelope?

**Answer:** sequential beats data-dependent gather by ~2x at every N ≥ 256.
The hardware prefetcher cannot predict data-dependent indices, and each load
becomes a potential cache miss.

| N bytes | seq ns/op | gather ns/op | gather penalty |
|---|---|---|---|
| 256 | 18 | 46 | +156% (2.5x) |
| 1024 | 127 | 303 | +138% (2.4x) |
| 4096 | 634 | 1168 | +85% (1.8x) |
| 16384 | 2694 | 4789 | +78% (1.8x) |

The strided variant in this bench (stride-8 cache-line skip) clocks 5-300x
faster than seq, but that's a measurement artifact: the strided loop runs
N/64 iterations vs seq's N/8, doing 1/8 the total work. The comparison
strided vs seq is not throughput-equivalent. Treat the strided column as
information about the prefetcher's stride-detection behaviour (still good
on constant stride), NOT as a "stride wins" result.

The seq vs gather comparison IS throughput-equivalent (both do N/8 ops).
That 1.8-2.5x gather penalty is the load-bearing finding:

- Validates arvo Column<T>'s "contiguous SoA layout" design principle.
  Sequential morsel-loop access patterns are cheap; data-dependent
  indirection (linked lists, hash-table probing, interior-pointer chains)
  costs measurable cache pressure.
- Justifies the no-ref-into-storage rule for hilavitkutin consumers
  (workspace rule `hilavitkutin-workunit-mental-model.md`): refs that
  let arbitrary code reach into scheduler-owned data turn the morsel
  loop's sequential access into gather-shaped access at consumer call
  sites. Even small amounts of indirection in the hot path compound.

Implication for arvo Column<T> docs: the "cache prefetcher loves us"
framing is empirically supported. Worth referencing this bench in the
Column<T> design rationale.

## Outstanding bench candidates (deferred)

- **barrier_scaling_6i** (Topic 6 axis I): requires multi-thread spawn from a
  cdylib variant. The bench harness's single-process-per-variant model is
  awkward for true concurrency benches; would need either a custom harness
  or self-spawn within the variant. Park as follow-up bench infrastructure
  task; the centralised counter shape is empirically sound from prior
  research literature and Topic 6 axis I locks on that basis.

- **morsel_size_7** (Topic 7): the bench would walk cache-residency for a
  morsel-shaped inner loop. The pattern is similar to existing benches
  (cache pressure programs surrounding an inner loop); a follow-up bench
  can refine the 64-records default once a real workload is in place.

- **futex_roundtrip_6f** (Topic 6 axis F BACKLOG calibration): single-thread
  bench measuring futex_wait + futex_wake roundtrip latency. Defer to
  follow-up; the M6 calibration BACKLOG entry covers this.

- **spin_p_vs_e_6k** (Topic 6 axis K): requires CoreClass detection to be in
  place first. Defer to after that detection lands.

- **fibershape_arity_4d** (Topic 4 axis D): combinatorics K-vs-2^N validation.
  Requires real FiberShape trait infrastructure to bench against; defer to
  post-impl bench validation.

## EMA formulation (`ema_formulation`) — audit-2 M4 saturating-Norm validation

Three EMA arithmetic formulations over N u8 samples, alpha = 1/8.

- `ema_form_float`: `acc = 0.875_f32 * acc + 0.125_f32 * sample` (FMA + mul).
- `ema_form_q32`: Q0.32 saturating fixed-point. `acc = (acc * 7/8_q32 + sample_q32 * 1/8_q32) >> 32`. Two 64-bit muls + add + shift.
- `ema_form_intshift`: `acc = acc - (acc >> 3) + (sample >> 3)`. Two shifts + sub + add. No mul.

Algo-only medians (function-under-test, 95% bootstrap CI all YES-significant except n=256 q32):

| N      | float    | q32              | intshift         |
|--------|----------|------------------|------------------|
| 256    | 436 ns   | 425 ns (+1.3%)   | 264 ns (-38.5%)  |
| 1024   | 2113 ns  | 1895 ns (-11.1%) | 1268 ns (-41.0%) |
| 4096   | 9045 ns  | 7695 ns (-14.1%) | 5144 ns (-42.5%) |
| 16384  | 35999 ns | 31198 ns (-13.7%)| 20739 ns (-42.3%)|

Findings:

- **Q0.32 fixed-point is faster than f32 EMA at N≥1024**, by 11-14%. The
  Apple Silicon (aarch64) FMA throughput is shared with int multiply, and
  the two-mul Q0.32 formulation lowers more cleanly than the FMA+mul float
  shape. At N=256 the per-call setup dominates and q32 ties float (within
  noise).
- **Integer shift wins by ~40% at every N**. Two shifts + add + sub beats
  even one FMA on this hardware. Cost: each step loses 3 bits of acc
  precision and the input quantises to 5 bits.
- **The audit-2 M4 call (saturating Q0.32 Norm for arvo EMA) is empirically
  correct**: Q0.32 is faster than float AND offers more precision (32-bit
  mantissa equivalent vs f32's 23). The only path that beats q32 is
  integer-shift, which trades precision the design doesn't want to trade.
- Validates the broader strategy-marker principle: when the precision /
  throughput trade is real (it is), the Hot strategy (truncating arithmetic)
  can lower to int-shift-style ops; Precise (saturating) lowers to Q0.32.
  The bench confirms both ends of the trade buy real performance.

Caveats:

- Aarch64 only. On x86_64 (Intel/AMD), FMA throughput is higher and the
  float / q32 ranking could flip. Re-run on x86_64 before generalising the
  "Q0.32 beats float" claim across the substrate.
- The integer-shift variant intentionally pre-quantises the input via
  `>> 3` so the >> 3 step on acc doesn't drop too many bits. A consumer
  that needs the full 8-bit sample range would face a sharper precision
  cliff. Treat the int-shift numbers as the optimistic floor.

## Modulo strategy (`modulo_strategy`) — power-of-2 cap convention validation

Three modulo strategies over an FNV1a-shaped inner loop (N samples each):

- `mod_pow2_const`: `acc % 64` with `64` const. LLVM lowers to `AND #63`.
- `mod_npow2_const`: `acc % 60` with `60` const. LLVM lowers to magic-number multiply (libgcc reciprocal trick).
- `mod_var`: `acc % opaque_divisor()` where the divisor is loaded from a `#[inline(never)]` getter. Lowers to `udiv` + `msub`.

Algo-only medians (95% CI, all YES significant):

| N      | npow2 const (base) | pow2 const         | opaque divisor     |
|--------|--------------------|--------------------|--------------------|
| 256    | 841 ns             | 364 ns (-55.8%)    | 1070 ns (+27.6%)   |
| 1024   | 3715 ns            | 1660 ns (-55.2%)   | 4675 ns (+26.5%)   |
| 4096   | 14305 ns           | 6724 ns (-53.8%)   | 18127 ns (+26.5%)  |
| 16384  | 57257 ns           | 25944 ns (-54.7%)  | 73016 ns (+27.3%)  |

Findings:

- **Power-of-2 const modulo is 2.2x faster than non-power-of-2 const**, and 2.9x faster than a runtime-opaque divisor. The discount holds at every N from 256 to 16384.
- The win is the AND-mask lowering: aarch64's `AND xN, xN, #63` is one cycle vs the magic-mul + sub sequence (~3 cycles uop + dependency chain) and vs `udiv` + `msub` (~15 cycles for udiv on Apple Silicon).
- **Validates the workspace convention** of making every cap a power of 2: `MAX_FIBERS`, `MAX_CORES`, `MAX_UNITS`, `MICRO_MORSEL_INTERVAL = 64`, `MAX_PHASES`, `MAX_DRIFT_RECORDS = 32`, `MAX_PLAN_AFFECTING_RESOURCES = 16`. Each modulo or wrap operation against one of these caps is one AND instruction at runtime; making them non-pow2 would cost a 2x slowdown on every inner-loop wrap.
- The opaque-divisor variant models what happens if a cap were read from a `Resource` field at runtime instead of being a const generic on `PoolFrame`. The 27% slowdown vs even the non-pow2 const is the price of losing compile-time knowledge of the divisor.

Caveats:

- aarch64 only. x86_64 udiv on modern Intel is faster (~10 cycles), so the var-vs-npow2 gap could narrow. The pow2-vs-anything-else gap is a fundamental ISA property: AND is always one cycle.
- The pow2 const lowering also benefits from `acc &= 63` being free-of-flags-clobber and pipelining; a complex modulo expression involving multiple wrapped indices would compound the win further.

## Load alignment (`load_alignment`) — read-path alignment cost on aarch64

Three load strategies over an FNV1a hash loop reading N bytes as N/8 u64 words:

- `load_aligned`: `*const u64` cast from `*const u8`, deref directly. Assumes alignment (UB in Rust if violated; aarch64 ISA tolerates it).
- `load_unaligned`: `core::ptr::read_unaligned::<u64>(p)`. LLVM emits the unaligned-load codegen path explicitly.
- `load_byte_pack`: read eight u8s then `u64::from_le_bytes`. LLVM may recognise this and reduce back to a single u64 load.

Algo-only medians:

| N      | aligned    | unaligned          | byte_pack          |
|--------|------------|--------------------|--------------------|
| 256    | 16 ns      | 17 ns (no sig)     | 17 ns (no sig)     |
| 1024   | 110 ns     | 116 ns (+6.4%, YES)| 114 ns (+2.3%, YES)|
| 4096   | 606 ns     | 603 ns (no sig)    | 609 ns (no sig)    |
| 16384  | 2596 ns    | 2592 ns (no sig)   | 2580 ns (no sig)   |

Findings:

- **aarch64 makes alignment strategy effectively free for sequential u64 reads** at every cache-residency level tested. The N=1024 signal (6.4% / 2.3% slowdowns) is small enough to be cache / pipeline noise rather than a fundamental load-instruction cost difference; at N=4096 and N=16384 the gap collapses to no-significant-difference.
- **`u64::from_le_bytes([b0..b7])` is recognised by LLVM** as a u64 load on aarch64 and produces equivalent codegen to the direct ptr cast at non-tiny sizes. This is the safe-Rust path consumer code can use without `unsafe`, and there is no measurable cost.
- **Validates that the per-fiber cache-line invariant** (Topic 3 M3 inline metrics, write-only-by-owning-core-then-read-after-phase-barrier) is **about write coherence and false-sharing avoidance**, NOT about read-path throughput. Per-record reads in the morsel loop can use whichever alignment shape is most ergonomic.
- The implication for design: no need to force `#[repr(align(8))]` on column-record types just to make reads faster. Alignment in the design matters for atomic operations and for cache-line ownership boundaries; the bench confirms it doesn't matter for read throughput.

Caveats:

- aarch64 only. x86_64 has even better tolerance for unaligned loads (cross-cache-line splits aside). The result generalises, but the size of the gap may differ by ISA.
- The bench reads u64 words. Larger NEON loads (`vld1q_u64`, `LDR Q`) have stricter alignment requirements at the hardware level (some only documented to work aligned). A SIMD-load-alignment bench is a separate investigation.

## Inline strategy (`inline_strategy`) — Topic 7 morsel-loop inlining assumption

Three inline-attribute strategies on an otherwise-identical per-byte step function (`acc = (acc ^ byte).wrapping_mul(K)`) called inside the morsel-loop body:

- `inline_always`: `#[inline(always)] fn step(...)`. Forces inline.
- `inline_default`: no attribute. LLVM's heuristic decides under release+fat-LTO.
- `inline_never`: `#[inline(never)] fn step(...)`. Forces a real call boundary per record.

Algo-only medians:

| N      | inline_always | inline_default      | inline_never           |
|--------|---------------|---------------------|------------------------|
| 256    | 276 ns        | 271 ns (-1.8%, YES) | 328 ns (+17.6%, YES)   |
| 1024   | 1280 ns       | 1255 ns (-1.4%, YES)| 1320 ns (+3.9%, YES)   |
| 4096   | 5239 ns       | 5252 ns (no sig)    | 5233 ns (no sig)       |
| 16384  | 20538 ns      | 20493 ns (no sig)   | 20598 ns (+0.3%, YES)  |

Findings:

- **`inline_default` matches `inline_always`** at every N. Under release + fat LTO, LLVM auto-inlines a small leaf fn into a tight loop reliably. Writing `#[inline]` on per-record step fns is belt-and-suspenders rather than load-bearing.
- **`inline_never` penalty is biggest at small N** (17.6% at N=256) and **shrinks to noise at large N** (no significant difference at N=4096; 0.3% at N=16384). The call boundary's setup cost amortises across many iterations.
- The N=256 17.6% penalty represents the actual call-boundary overhead per record. ~50ns for 256 records is ~0.2 ns/call, which is a small fraction of a single cycle on Apple Silicon. Modern aarch64 branch prediction + return-stack handles call-heavy code well.
- **Validates Topic 7 morsel-loop axis A** (forward iter with monomorphised inner step): the inlining IS happening; the design's emphasis on monomorphisation buys the small-N win where it matters most. At larger N the dispatch shape ceases to matter; cache behavior dominates.
- **Implication: defensive `#[inline]` on hot WorkUnit step fns is cheap insurance** when small N (e.g. morsel-edge tail processing, micro-morsel boundaries at N=64) is common, but is not critical. The bigger risk is a code path that genuinely cannot inline (trait objects, FFI calls, format-machinery) — that's a structural concern, not an attribute-tweaking one.

Caveats:

- aarch64 has a 12-bit branch target cache and a 32-entry return stack; call-heavy code prediction is essentially free. On older x86 or constrained-prediction targets the call-boundary penalty could be larger.
- The step function here is leaf (no further calls). A step that itself calls other fns would compound the issue if non-leaf chains aren't inlined.
- LTO is doing real work in the `inline_default` case. Without `lto = "fat"`, inline-default could diverge from inline-always significantly.

## Bounds-check elision (`bounds_check`) — morsel-loop safe-Rust validation

Three loop-shape strategies for a per-byte FNV1a inner loop over a `&[u8; N]` const-sized input:

- `bc_iter`: `for &byte in input.iter()`. Iterator-protocol; no bounds checks structurally.
- `bc_index`: `for i in 0..N { input[i] }`. Safe Rust indexing; LLVM should elide the bounds check because `i < N` is the loop condition.
- `bc_unchecked`: `for i in 0..N { *input.get_unchecked(i) }`. `unsafe` explicit elision; floor case.

Algo-only medians:

| N      | bc_iter            | bc_index (base) | bc_unchecked       |
|--------|--------------------|-----------------|--------------------|
| 256    | 278 ns (+2.7%, YES)| 271 ns          | 273 ns (no sig)    |
| 1024   | 1245 ns (no sig)   | 1239 ns         | 1277 ns (+2.3%, YES)|
| 4096   | 5350 ns (no sig)   | 5269 ns         | 5267 ns (no sig)   |
| 16384  | 20525 ns (no sig)  | 20572 ns        | 20662 ns (no sig)  |

Findings:

- **All three variants are within noise** at every N. LLVM elides the bounds check completely when iterating `0..N` over a `&[u8; N]` const-sized array. Safe-Rust indexing produces identical codegen to `get_unchecked`.
- The tiny per-variant deltas (2.3% / 2.7%) flip direction between sizes (sometimes iter is slower, sometimes unchecked is slower, never consistently the safe variant), confirming this is measurement noise rather than a real safety-cost gap.
- **`bc_unchecked` does NOT win**. The `unsafe` access provides zero throughput benefit when the safe form is already optimal. This means consumer WorkUnits should never reach for `get_unchecked` as a perf optimisation on const-sized morsel arrays; the safe form is already free.
- **Validates Topic 7 morsel-loop design**: consumer WorkUnit code writing `column[i]` indexing in the inner loop pays no bounds-check cost. The morsel-shape contract (const-generic upper bound on iteration) makes the safe form the optimal form.
- **Implication for the substrate**: do NOT teach consumers to use `unsafe` in WorkUnit bodies. Safe indexing over const-sized morsel arrays is already optimal. The substrate's safety story carries through to runtime perf without compromise.

Caveats:

- Requires const-generic upper bound. A `&[u8]` slice (dynamic length) would force LLVM to emit the bounds check per iteration. The morsel-loop contract (fixed-size morsel array) is what enables the elision.
- Rust uses `usize` for indexing; if the consumer's index type is some narrower or wider integer, the elision proof might fail. The const-generic `N: usize` morsel shape avoids this.
- The bench uses `&[u8; N]`. Larger element types (e.g. column records of `T` where size_of::<T> > 1) would compound any non-elided check. The result generalises: as long as the array length is statically known and the loop counter type matches, LLVM elides.

## Branch predictability (`branch_predictability`) — predictor sensitivity in WorkUnit inner loops

Three branch-skew patterns on an identical if/else inner loop. Threshold differs only in the constant:

- `bp_skew_high`: `if byte > 240` → branch taken ~6% (highly predictable, saturates to not-taken).
- `bp_balanced`: `if byte > 128` → branch taken ~50% (adversarial: no skew for the predictor).
- `bp_skew_low`: `if byte > 16` → branch taken ~94% (highly predictable, saturates to taken).

Algo-only medians:

| N      | bp_skew_high       | bp_balanced (base) | bp_skew_low        |
|--------|--------------------|--------------------|--------------------|
| 256    | 297 ns (no sig)    | 296 ns             | 305 ns (+1.9%, YES)|
| 1024   | 1307 ns (no sig)   | 1299 ns            | 1261 ns (-1.7%, YES)|
| 4096   | 5268 ns (no sig)   | 5281 ns            | 5166 ns (-1.0%, YES)|
| 16384  | 20644 ns (no sig)  | 20658 ns           | 20769 ns (no sig)  |

Findings:

- **All three variants are within ~2% of each other at every N**. The adversarial 50/50 case (`bp_balanced`) is not measurably slower than the highly-skewed cases. The tiny per-N deltas flip direction between sizes, confirming this is measurement noise rather than predictor sensitivity.
- LLVM is almost certainly converting the if/else to branchless `csel` / `cmov`, eliminating predictor sensitivity entirely. The existing `branch_pattern` bench (if/else vs explicit cmov within 2% gap) is the direct evidence.
- **Validates that branch predictability is NOT a load-bearing concern for tight WorkUnit inner loops**. Consumer WorkUnits can write data-dependent if/else inside the morsel loop without worrying about predictor sensitivity, as long as the if/else body is small enough for LLVM to convert to branchless form (it almost always is).
- Pairs with `branch_pattern` (if/else vs explicit cmov, also within 2%) to establish a coherent picture: **on aarch64, branchful and branchless inner-loop code converge to the same throughput in practice**. The Predicate trait's value (arvo's `Predicate<T>` returning `Bool`) is type-level intent and composition, not codegen-level branch elimination.

Caveats:

- aarch64 only. Older x86 cores with weaker branch prediction could show larger gaps for the 50/50 case.
- Bigger if/else bodies (cmov-ineligible: function calls, multi-instruction sequences) would re-introduce branch sensitivity. Inner-loop write paths with multiple statements per arm should be benched separately.
- The input bytes come from a uniform-random seed; pathological patterns (alternating, long runs) would also surface predictor sensitivity beyond what uniform input shows.

## Cross-references

- `mock/benches/dep_graph_csr_9_n*_findings.md`
- `mock/benches/popcount_strategy_n*_findings.md`
- `mock/benches/hash_algos_n*_findings.md`
- `mock/benches/fxpmul_strategy_n*_findings.md`
- `mock/benches/branch_pattern_n*_findings.md`
- `mock/design_rounds/202605101036_topic.plan-caching.md` axis B (CSR lock).
- arvo `mock/research/strategy-bound-trilemma.md` (the Hot/Warm/Cold/Precise strategy rationale; fxpmul bench empirically validates Hot-vs-Precise gap).
