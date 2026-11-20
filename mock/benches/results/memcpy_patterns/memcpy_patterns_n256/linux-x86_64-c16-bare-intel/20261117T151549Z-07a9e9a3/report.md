# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (memcpy_intrinsic) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline memcpy_intrinsic has the worst median (376 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest memcpy_naive at 375 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (282 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader memcpy_naive vs stability leader memcpy_unrolled (+0% speed for 1.0x steadier)

memcpy_naive is fastest (375 ns, CV 75.1%); memcpy_unrolled gives up 0.1% median for 1.0x lower variance (CV 74.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### memcpy_unrolled is inconsistent: worst-20% is 3.3x its best-20%

memcpy_unrolled's best 20% of batches run at 312 ns but its worst 20% at 1.02 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.1% of the fastest

All 3 variants sit between 375 ns and 376 ns - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_naive's edge over baseline is significant but tiny (-0 ns, 0.11%)

memcpy_naive differs from baseline memcpy_intrinsic by -0 ns (0.11%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: memcpy_naive** at 375.3 ns median (-0.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 375.3 ns, slowest 375.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4646464650504040 |
| Counter frequency | 2112829592 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 3099ns | 2265ns | 2028ns | 2331ns | 6476ns | base |
| memcpy_naive | 2867ns | 2261ns | 2042ns | 2263ns | 5506ns | -7.48% |
| memcpy_unrolled | 2974ns | 2262ns | 1877ns | 2272ns | 6178ns | -4.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 514ns | 337ns | 1068ns | base | 0.498 |
| memcpy_naive | 477ns | 339ns | 919ns | -7.22% | 0.537 |
| memcpy_unrolled | 492ns | 312ns | 1016ns | -4.31% | 0.521 |

## Performance model

- Peak throughput: **0.821 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.682 | 83.0% |
| memcpy_naive | 0.682 | 83.1% |
| memcpy_unrolled | 0.682 | 83.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 3099ns | 3099ns | base |
| memcpy_naive | 2867ns | 2867ns | -7.48% |
| memcpy_unrolled | 2974ns | 2974ns | -4.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 376ns | base | --- | [376, 376] | --- | --- | --- | --- |
| memcpy_naive | 375ns | -0.4ns (-0.1%) | [-1, -0]ns | [375, 375] | YES | 0.0000 | 0.0000 | 3 |
| memcpy_unrolled | 376ns | no significant difference | [-0, +0]ns | [376, 376] | no | 0.3627 | 0.3627 | 14 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 945ns | -60.3% | -60.3% |
| 1 | 2 | 386ns | -2.8% | -2.7% |
| 1 | 3 | 307ns | +22.1% | +0.0% |
| 1 | 4 | 375ns | -18.2% | +0.0% |
| 1 | 5 | 376ns | -0.1% | -18.1% |
| 1 | 6 | 376ns | -0.1% | +0.0% |
| 1 | 7 | 376ns | -0.1% | +0.1% |
| 1 | 8 | 376ns | -0.1% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | 0.017 | ok |
| memcpy_naive | -0.160 | ok |
| memcpy_unrolled | -0.374 | moderate- |

**Consistency summary:**

- **memcpy_naive**: won 3/8, lost 1/8
- **memcpy_unrolled**: won 3/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 33.9ns | 514.0ns | 6.6% | HIGH |
| memcpy_naive | 31.1ns | 476.8ns | 6.5% | HIGH |
| memcpy_unrolled | 32.8ns | 491.8ns | 6.7% | HIGH |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 337.0-1067.5 ns)
    337.0 |
    373.5 |########################################
    410.0 |
    446.6 |
    483.1 |
    519.6 |
    556.1 |
    592.7 |
    629.2 |
    665.7 |
    702.3 |
    738.8 |
    775.3 |
    811.8 |
    848.4 |
    884.9 |
    921.4 |
    958.0 |#
    994.5 |
   1031.0 |#
  (18 below, 16 above range)

memcpy_naive (n=160, range 339.3-919.0 ns)
    339.3 |
    368.3 |########################################
    397.3 |
    426.3 |
    455.2 |
    484.2 |
    513.2 |
    542.2 |
    571.2 |
    600.2 |
    629.1 |
    658.1 |
    687.1 |
    716.1 |
    745.1 |
    774.0 |
    803.0 |
    832.0 |
    861.0 |
    890.0 |
  (17 below, 20 above range)

memcpy_unrolled (n=160, range 311.8-1016.2 ns)
    311.8 |
    347.0 |########################################
    382.2 |
    417.5 |
    452.7 |
    487.9 |
    523.1 |
    558.3 |
    593.6 |
    628.8 |
    664.0 |
    699.2 |
    734.4 |
    769.6 |
    804.9 |
    840.1 |#
    875.3 |##
    910.5 |
    945.7 |#
    980.9 |
  (29 below, 13 above range)

```

## Diagnostics

- **memcpy_intrinsic**: CV=57.4% (high variance, measurements may be unstable)
- **memcpy_intrinsic**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **memcpy_naive**: CV=59.1% (high variance, measurements may be unstable)
- **memcpy_unrolled**: CV=57.1% (high variance, measurements may be unstable)
- **memcpy_unrolled**: worst_20/best_20 = 3.3x (possible bimodal distribution)
