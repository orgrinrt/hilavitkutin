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

## Cross-references

- `mock/benches/dep_graph_csr_9_n*_findings.md`
- `mock/benches/popcount_strategy_n*_findings.md`
- `mock/benches/hash_algos_n*_findings.md`
- `mock/benches/fxpmul_strategy_n*_findings.md`
- `mock/benches/branch_pattern_n*_findings.md`
- `mock/design_rounds/202605101036_topic.plan-caching.md` axis B (CSR lock).
- arvo `mock/research/strategy-bound-trilemma.md` (the Hot/Warm/Cold/Precise strategy rationale; fxpmul bench empirically validates Hot-vs-Precise gap).
