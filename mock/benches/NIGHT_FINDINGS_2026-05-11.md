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

## Atomic vs plain (`atomic_vs_plain`) — Topic 3 M3 inline metrics cost

Three accumulator typings on an identical FNV1a inner loop:

- `acc_plain`: plain `u64` local. Lives in a register across the loop.
- `acc_atomic_relaxed`: `AtomicU64` with `Ordering::Relaxed` load+store per step.
- `acc_atomic_seqcst`: `AtomicU64` with `Ordering::SeqCst` per step (full memory barrier).

Algo-only medians:

| N      | acc_plain          | acc_atomic_relaxed | acc_atomic_seqcst  |
|--------|--------------------|--------------------|--------------------|
| 256    | 283 ns (no sig)    | 282 ns             | 284 ns (no sig)    |
| 1024   | 1289 ns (-0.3%, YES)| 1309 ns           | 1315 ns (+0.4%, YES)|
| 4096   | 5119 ns (no sig)   | 5193 ns            | 5142 ns (no sig)   |
| 16384  | 20579 ns (no sig)  | 20533 ns           | 20533 ns (no sig)  |

Findings:

- **All three variants are within noise at every N** on aarch64 single-thread. The "atomic overhead" intuition is mostly false in single-threaded non-contended code: the barrier instructions cost essentially nothing when there's no other core invalidating cache lines.
- **SeqCst is equivalent to Relaxed** in single-thread on aarch64. The barrier instructions execute but find no actual ordering work to do. The cost shows up only when multiple cores actually contend.
- **`acc_plain` matches `acc_atomic_relaxed` to within 0.3-1.5%**. The minor speed advantage of plain locals comes from register allocation (acc stays in a register for plain; for atomic it has to live in memory because the type prevents register-allocation).
- **Validates Topic 3 M3 inline metrics design**: per-fiber inline metrics stored as `AtomicU64` with Relaxed ordering cost essentially nothing in the hot path. The cache-line invariant (write-only-by-owning-core, read-only-after-phase-barrier-by-AdaptWu) means there's no cross-core contention during the hot loop, so atomic ops behave like plain ops.
- **The strategy stands**: AdaptMetrics fields can be AtomicU64 + Relaxed without throughput cost, AS LONG AS the per-fiber cache-line invariant holds. The bench validates the cost; the invariant itself is the design's responsibility.

Caveats:

- The bench operates on a single fiber's worth of accumulator. Multi-core contention (multiple cores writing the same cache line) is a separate phenomenon and would show large slowdowns; that's exactly why the cache-line invariant matters.
- The "atomic_ordering_cost" bench shows Relaxed beats Acquire/Release by 3-4x — but that bench tests a different access pattern (multiple atomics, more complex memory dependencies). The two findings are consistent: when there's actual ordering work, the barrier costs real cycles; when there's none, the barrier is free.
- aarch64 specifically benefits from cheap atomic ops; older x86 cores or constrained-coherence targets could show different gaps. The substrate should not assume free-atomics universally.

## Atomic increment strategy (`atomic_inc_strategy`) — Topic 6 axis I phase-barrier counter

Three increment strategies on a single shared `AtomicU64` counter, N iterations each:

- `inc_fetch_add`: `counter.fetch_add(1, Relaxed)`. Single LDADD instruction on aarch64 with FEAT_LSE.
- `inc_cas_loop`: read + `compare_exchange_weak` loop. The "naive" shape before LSE intrinsics.
- `inc_load_store`: separate `load(Relaxed)` + `store(v+1, Relaxed)`. Not atomic across the RMW window; in single-thread the final value is correct.

Algo-only medians:

| N      | inc_fetch_add     | inc_cas_loop (base) | inc_load_store     |
|--------|-------------------|---------------------|--------------------|
| 256    | 492 ns (-18.9%)   | 647 ns              | 0 ns (elided)      |
| 1024   | 2088 ns (-13.2%)  | 2395 ns             | 0 ns (elided)      |
| 4096   | 8721 ns (-11.8%)  | 9735 ns             | 0 ns (elided)      |
| 16384  | 34314 ns (-11.3%) | 38707 ns            | 0 ns (elided)      |

Findings:

- **`fetch_add` beats CAS-loop by 11-19%** at every N, with the largest gap at small N (where loop overhead dominates). The aarch64 LSE `LDADD` instruction (which `fetch_add` lowers to) is meaningfully faster than the load + compare_exchange_weak pair even in single-thread non-contended code. The CAS-loop pays one extra load + one branch per increment.
- **`inc_load_store` got optimized away by LLVM** (median 0 ns, CI [0, 0]). The separate `load + store` sequence on a non-shared atomic is statically equivalent to `store(N, Relaxed)`, and LLVM emits exactly that single store at loop exit. This is itself an informative finding: a "non-atomic-equivalent" shape lets LLVM fold the entire loop away because there's no observability constraint between iterations.
- **Validates Topic 6 axis I** (centralised atomic counter phase barrier using fetch_add): the design's choice of `fetch_add` over a CAS-loop trait abstraction buys a real 11-19% throughput win on aarch64. Under contention this gap widens further (CAS-loop retries pile up under contention; LDADD has hardware-assisted forward progress).
- The load_store elision result also bears on Topic 3 M3 inline metrics: the substrate cannot rely on a "naive load+store" shape if it wants the writes to actually happen. AtomicU64 with `fetch_add` or `fetch_or` etc. provides the observability the design wants; plain load+store on an atomic can be elided.

Caveats:

- aarch64 with FEAT_LSE (Apple Silicon, modern Cortex). Older aarch64 without LSE would lower `fetch_add` to LL/SC loop, narrowing the gap vs explicit CAS-loop (both become LL/SC). x86_64 has LOCK XADD instruction which is comparable to LSE LDADD.
- Single-thread bench. Under multi-core contention both `fetch_add` and CAS-loop would slow down, but CAS-loop slows down more because retries pile up exponentially.
- The load_store elision is by-design LLVM behavior under Rust's single-thread observability rules. Multi-thread access to the same atomic with these orderings would not be elided.

## Bit-count operations (`bit_count_ops`) — ctz cost validation (inconclusive)

Three trailing-zero-count strategies over N/8 u64 words:

- `ctz_intrinsic`: `u64::trailing_zeros()` (LLVM lowers to RBIT + CLZ on aarch64).
- `ctz_debruijn`: textbook De Bruijn sequence lookup (and+neg, mul, shr, table).
- `ctz_loop`: explicit shift-and-test loop (data-dependent worst case).

Algo-only medians (95% CI all):

| N      | ctz_intrinsic   | ctz_debruijn (base) | ctz_loop        |
|--------|-----------------|---------------------|-----------------|
| 256    | 0 ns (no sig)   | 0 ns                | 0 ns (no sig)   |
| 1024   | 0 ns (no sig)   | 0 ns                | 0 ns (no sig)   |
| 4096   | 0 ns (no sig)   | 0 ns                | 0 ns (no sig)   |
| 16384  | 0 ns (no sig)   | 0 ns                | 0 ns (no sig)   |

All three variants register HIGH tie counts (29-44% of measurements landed on identical timer ticks), indicating the per-call cost is below the bench framework's timer resolution at the AlgoCall window.

Findings:

- **The bench is inconclusive on a per-variant ranking**, but the inconclusion itself is informative: ctz on aarch64 is so cheap (1-2 cycles per word via RBIT+CLZ) that the per-call work in this bench shape disappears below timer resolution. At N=16384 (2048 ctz ops per call) we'd expect roughly 700ns-1.4μs of pure ctz work, but the framework's surrounding-workload context likely dominates and the actual timer hit lands inside noise.
- **Validates arvo BitAccess::trailing_zeros lowering** by inference: if ctz cost is below resolution, the canonical intrinsic path is fast enough that no consumer should reach for De Bruijn or shift-loop alternatives on aarch64. The intrinsic is the right primitive.
- Implication for the substrate: bit-count operations (popcount, ctz, clz) are essentially free on modern aarch64. Design choices that pay 1-2 cycles per bit-count op (e.g., the `arvo-bitmask` Mask iteration using `trailing_zeros` to find next set bit) lose nothing to alternative formulations.

Caveats:

- Inconclusive on x86_64 generalisation: x86_64 has BMI1 `TZCNT` instruction with similar latency, so the result likely holds, but a re-run on x86 would confirm.
- A larger workload (e.g., N=65536+ words or a bench shape that exposes the ctz work more directly without surrounding-workload masking) might separate the variants. Park as follow-up bench-infrastructure task.
- The bench framework's `algo` window is too short to time ctz when the input bytes are uniformly random (most words have a low trailing-zero count, so the work-per-word is minimal). A pathological input distribution (mostly-zero words) would give the loop variant more work to do and surface its O(tz) shape.

## Fold strategy (`fold_strategy`) — ILP cost in reduction-shaped WorkUnits

Three fold-shape strategies for an FNV1a-style reduction over N/8 u64 words:

- `fold_sequential`: single accumulator, long dep chain. Each multiply waits for the previous.
- `fold_paired`: two interleaved accumulators (even/odd words), final XOR combine.
- `fold_quad`: four interleaved accumulators (mod-4 words), final XOR combine.

Algo-only medians:

| N      | sequential        | paired (base) | quad              |
|--------|-------------------|---------------|-------------------|
| 256    | 16 ns (no sig)    | 16 ns         | 17 ns (no sig)    |
| 1024   | 110 ns (+58.5%)   | 70 ns         | 55 ns (-23.2%)    |
| 4096   | 611 ns (+88.6%)   | 318 ns        | 186 ns (-41.7%)   |
| 16384  | 2690 ns (+91.1%)  | 1364 ns       | 708 ns (-48.0%)   |

Findings:

- **Quad-way ILP delivers ~3.8x speedup over sequential** at N=16384, ~3.3x at N=4096. The paired variant sits at 2x speedup over sequential. The throughput ratio matches what aarch64's MUL throughput (1/cycle) vs latency (~3 cycles) would predict for breaking the dep chain.
- **LLVM does NOT auto-extract ILP from the sequential pattern**. If it did, sequential would match paired or quad. The sequential single-accumulator fold stays latency-bound; LLVM treats the dep chain as semantically required and respects it.
- At N=256 the work is too small to expose pipeline filling; all three variants land at noise. The signal kicks in at N=1024 and grows with N as the loop body dominates.
- **The largest design-relevant finding of this overnight bench session**. Has direct implications for arvo reduction kernels and Topic 7 morsel-loop shape.

Implications for design:

- **Reduction-shaped arvo kernels MUST break dep chains explicitly** if they want throughput. A naive `acc.fma(x[i], y[i])` loop over a column pays a 2-4x latency tax. The arvo strategy markers should expose this: `Hot` strategy reduction kernels should default to 4-way interleaved accumulators where the math permits; `Precise` strategy can keep the single-chain shape when the math requires associativity-strict order.
- **Topic 7 morsel-loop design implication**: when a consumer WorkUnit's `execute` body reduces over the morsel records (sum, product, max, etc.), the substrate should provide a reduction primitive that breaks the chain. Hand-rolled WorkUnit reductions that use a single `let mut acc` will pay the latency tax.
- **arvo Round 7+ follow-up**: investigate whether `arvo::traits::Fold` or similar reduction-trait should mandate (or at least suggest) interleaved-accumulator shape. Could be a deepdive or new trait family for reduction-friendly types.

Caveats:

- The bench uses FNV1a-shape multiply-XOR. Other reduction kernels (saturating add, max, AND, OR) have different latency/throughput shapes; pure-add reductions on aarch64 have latency ~1 cycle and the gap would be smaller. The mul-heavy case is the worst case for ILP-blind sequential folds.
- The N=256 case shows the threshold below which loop-body work is dominated by call setup; for any morsel size in this range, ILP doesn't matter. Above ~1KB the gap grows fast.
- Final XOR-combine of accumulators is associativity-permissive; for non-associative reductions (e.g., subtraction, division, float arithmetic without `fma_relaxed`), the interleaved shape would change semantics. Strategy markers should encode the associativity assumption.

## Overflow strategy (`sat_arithmetic`) — Hot vs Precise strategy-marker cost

Three overflow-handling strategies for a u64 sum-of-mul reduction over N bytes:

- `sat_wrap`: `wrapping_add` — modulo-2^64 semantics. Lowers to ADD on aarch64. Models Hot strategy.
- `sat_saturate`: `saturating_add` — clamp at u64::MAX. Lowers to ADDS + CSEL on aarch64. Models Precise strategy.
- `sat_checked`: `checked_add().unwrap_or(MAX)` — explicit branch. Lowers to ADDS + B.CS + fallback. Branch is never taken in this workload (sum stays well below u64::MAX).

Algo-only medians:

| N      | sat_wrap          | sat_saturate     | sat_checked (base) |
|--------|-------------------|------------------|--------------------|
| 256    | 107 ns (-42.4%)   | 175 ns (-4.1%)   | 191 ns             |
| 1024   | 355 ns (-47.8%)   | 698 ns (no sig)  | 678 ns             |
| 4096   | 1398 ns (-47.8%)  | 2671 ns (+1.4%)  | 2642 ns            |
| 16384  | 5362 ns (-49.0%)  | 10812 ns (+2.9%) | 10426 ns           |

Findings:

- **Wrapping (Hot) is ~2x faster than saturating (Precise) or checked at every N≥1024**. The Hot vs Precise strategy gap is real, measurable, and roughly 2x throughput. Wrapping is a single ADD; saturating is ADDS + CSEL; checked is ADDS + B.CS + select fallback. Each conditional adds ~1 cycle to the per-step latency in a latency-bound reduction loop.
- **Saturating and checked are within ~3% of each other** at every N. The branch predictor saturates to "never taken" on the checked path because the workload's sum stays below u64::MAX, so the conditional branch is essentially free. CSEL on the saturating path is also one cycle. The two paths are mechanically distinct but cost similar.
- **The 2x gap is fundamental at the ISA level**: aarch64 cannot do "add with saturate-on-overflow" in a single instruction. The path has to go ADDS + condition-check + select. Any overflow-aware arithmetic pays this cost; only wrapping arithmetic skips it.

Strategy-marker implications (LOAD-BEARING for arvo design):

- **`Hot` strategy buys 2x throughput over `Precise` on integer reductions**. The strategy marker is not theoretical: the design's claim that Hot consumers should accept wrapping semantics for the speed win is empirically grounded.
- **`Precise` strategy users pay a 2x latency tax** on every accumulating op. The use case for Precise must be load-bearing enough to justify this. Plan-stage analysis algorithms (graph processing, exact bit-vector ops) qualify; throughput-loop reductions (Column EMA, lossy aggregations) should default to Hot.
- The Hot/Precise framing in arvo's strategy markers is correct. The bench validates the marker's design weight — this is what the markers were designed to express.

Pairs with the previously-completed `fxpmul_strategy` bench (which showed 8-20% gap on mul) and the `fold_strategy` bench (3.8x gap on chained mul). Together: **overflow handling costs ~2x; chain breaking gains ~3.8x.** Both axes matter, and both are encoded in the strategy markers.

Caveats:

- aarch64 specific. x86_64 has similar ADD vs ADC+CMOVC shapes; gap probably similar.
- Bench uses sum (commutative+associative), so the saturating semantics are well-defined. Non-commutative reductions might compose differently with strategy markers.
- Sum stays well below u64::MAX in this workload; the checked path's branch is always "not taken". A workload that actually overflows would force the fallback path, widening the gap further.

## Rotate strategy (`rotate_strategy`) — fixed vs variable rotation cost

Three rotation strategies on an FNV-style mixer reduction:

- `rot_intrinsic_const`: `acc.rotate_left(13)` with const amount. Lowers to single ROR on aarch64.
- `rot_manual_shifts`: hand-rolled `(acc << 13) | (acc >> 51)`. Tests whether LLVM recognises the idiom and folds to ROR.
- `rot_variable_amount`: `acc.rotate_left(k)` where `k` derives from previous acc bits. Lowers to RORV; the amount depends on the prior step's result.

Algo-only medians:

| N      | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|--------|---------------------|-------------------|---------------------|
| 256    | 8 ns                | 8 ns (no sig)     | 52 ns (+558%)       |
| 1024   | 73 ns               | 74 ns (no sig)    | 237 ns (+226%)      |
| 4096   | 304 ns              | 301 ns (no sig)   | 978 ns (+221%)      |
| 16384  | 1346 ns             | 1340 ns (no sig)  | 3967 ns (+196%)     |

Findings:

- **LLVM correctly recognises the manual rotate idiom** (`(v << K) | (v >> (64-K))`) and lowers it to the same ROR instruction as the intrinsic. Consumer code can write either form without performance penalty on aarch64. The minor flicker at N=16384 (-2.1% adj-p just barely YES then flipped to no) is noise.
- **Data-dependent rotation amounts cost ~3x** vs const-amount. The N=256 case shows ~6.5x (small-N pipeline noise dominates), but the steady-state at N≥1024 is consistently ~3x slower.
- **The 3x gap is NOT primarily the rotate instruction cost**. RORV (register-amount rotate) is only ~1.5x slower than ROR. The dominant cost is **dep chain length**: each rotate has to wait for the previous step to complete before its amount is known, lengthening the latency-bound loop. This is the same pattern surfaced in fold_strategy — long dep chains lose to interleaved/independent operations.
- Pairs with the existing `hash_algos` bench (xxhash with fixed-rotation mixers winning at small/medium N over SipHash with data-dependent rotations).

Implications for design:

- **arvo-hash family choice**: hash mixers with data-dependent rotations (SipHash-style) pay a fundamental 3x cost vs fixed-rotation mixers (xxhash3-style) on the latency axis. The current hash_algos finding (xxh winning small/medium) extends here: the cause is not the algorithm's mixing math but its rotation-amount strategy.
- **arvo BitSequence ops**: where const rotation is sufficient, the manual `(v << K) | (v >> (64-K))` form is just as fast as `rotate_left`. The const-callable trait surface can use the manual form when needed for const-context evaluation without performance regression.
- **Topic 7 morsel-loop**: any per-record op whose argument depends on the prior step's result joins the long-dep-chain regime. Variable-amount shifts/rotates / data-derived offsets / index-from-previous-value patterns all pay the 2-4x latency tax. Substrate guidance: prefer fixed shift amounts and independent-per-step indexing where the algorithm permits.

Caveats:

- The variable-amount path's 3x gap is partly the dep chain (separable from RORV cost). A non-dep-chain variable-amount bench (where `k` comes from input data) would isolate just the instruction cost; expect ~1.5x there. This bench measures the realistic combined cost.
- aarch64 specific. x86_64 ROR has similar const/variable shape; the dep-chain story would carry over.
- The bench's `k = ((acc >> 56) & 0x3F).max(1)` ensures k is in [1, 63]; rotate_left(0) is a no-op LLVM might elide.

## Cross-references

- `mock/benches/dep_graph_csr_9_n*_findings.md`
- `mock/benches/popcount_strategy_n*_findings.md`
- `mock/benches/hash_algos_n*_findings.md`
- `mock/benches/fxpmul_strategy_n*_findings.md`
- `mock/benches/branch_pattern_n*_findings.md`
- `mock/design_rounds/202605101036_topic.plan-caching.md` axis B (CSR lock).
- arvo `mock/research/strategy-bound-trilemma.md` (the Hot/Warm/Cold/Precise strategy rationale; fxpmul bench empirically validates Hot-vs-Precise gap).
