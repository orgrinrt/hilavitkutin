# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (127 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bc_index)

The baseline bc_index is the fastest (152 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader bc_index vs stability leader bc_unchecked (+0% speed for 1.0x steadier)

bc_index is fastest (152 ns, CV 83.7%); bc_unchecked gives up 0.2% median for 1.0x lower variance (CV 82.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bc_unchecked is inconsistent: worst-20% is 3.6x its best-20%

bc_unchecked's best 20% of batches run at 125 ns but its worst 20% at 446 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.2% of the fastest

All 3 variants sit between 152 ns and 152 ns - a 0.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bc_iter's edge over baseline is significant but tiny (0 ns, 0.20%)

bc_iter differs from baseline bc_index by 0 ns (0.20%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (bc_index) is the fastest** at 152.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 152.2 ns, slowest 152.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Bounds-check elision: iter() vs explicit index vs get_unchecked |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5a3ec07491bf2d68 |
| Counter frequency | 2112876524 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 2840ns | 2037ns | 1664ns | 2209ns | 5910ns | base |
| bc_iter | 2723ns | 2039ns | 1669ns | 2041ns | 5823ns | -4.13% |
| bc_unchecked | 2799ns | 2038ns | 1666ns | 2125ns | 5954ns | -1.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 212ns | 125ns | 439ns | base | 4.837 |
| bc_iter | 203ns | 125ns | 434ns | -3.96% | 5.036 |
| bc_unchecked | 210ns | 125ns | 446ns | -0.99% | 4.885 |

## Performance model

- Peak throughput: **8.221 Gops/s** (bc_index; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 6.728 | 81.8% |
| bc_iter | 6.715 | 81.7% |
| bc_unchecked | 6.715 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2840ns | 2840ns | base |
| bc_iter | 2723ns | 2723ns | -4.13% |
| bc_unchecked | 2799ns | 2799ns | -1.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 152ns | base | --- | [125, 152] | --- | --- | --- | --- |
| bc_iter | 152ns | +0.3ns (+0.2%) | [+0, +1]ns | [152, 152] | YES | 0.0000 | 0.0000 | 12 |
| bc_unchecked | 152ns | no significant difference | [+0, +0]ns | [152, 152] | no (adj: YES) | 0.0352 | 0.0352 | 7 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 152ns | +0.1% | -18.0% |
| 1 | 2 | 125ns | +0.1% | +22.3% |
| 1 | 3 | 125ns | +22.3% | +0.1% |
| 1 | 4 | 125ns | +0.1% | +0.1% |
| 1 | 5 | 325ns | -53.1% | -53.1% |
| 1 | 6 | 125ns | +22.2% | +22.2% |
| 1 | 7 | 152ns | +0.1% | +0.1% |
| 1 | 8 | 125ns | +22.1% | +114.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.248 | moderate- |
| bc_iter | -0.374 | moderate- |
| bc_unchecked | 0.063 | ok |

**Consistency summary:**

- **bc_iter**: won 1/8, lost 3/8
- **bc_unchecked**: won 2/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 33.3ns | 211.7ns | 15.7% | HIGH |
| bc_iter | 32.0ns | 203.3ns | 15.8% | HIGH |
| bc_unchecked | 33.0ns | 209.6ns | 15.8% | HIGH |

## Distribution (algo ns)

```
bc_index (n=160, range 124.6-438.5 ns)
    124.6 |########################################
    140.3 |#####################
    156.0 |
    171.7 |
    187.3 |
    203.0 |##
    218.7 |
    234.4 |#
    250.1 |#
    265.8 |
    281.5 |#
    297.2 |#
    312.9 |##
    328.6 |##
    344.3 |#
    360.0 |#
    375.7 |#
    391.4 |#
    407.1 |
    422.8 |#
  (11 below, 16 above range)

bc_iter (n=160, range 125.2-434.0 ns)
    125.2 |##
    140.7 |########################################
    156.1 |
    171.5 |
    187.0 |
    202.4 |
    217.9 |
    233.3 |
    248.8 |
    264.2 |
    279.6 |
    295.1 |
    310.5 |
    326.0 |
    341.4 |
    356.8 |
    372.3 |
    387.7 |#
    403.2 |
    418.6 |#
  (26 below, 15 above range)

bc_unchecked (n=160, range 124.7-446.0 ns)
    124.7 |################
    140.8 |########################################
    156.8 |#
    172.9 |
    189.0 |
    205.0 |
    221.1 |
    237.2 |#
    253.2 |
    269.3 |#
    285.4 |#
    301.4 |
    317.5 |#
    333.6 |
    349.6 |
    365.7 |
    381.8 |
    397.8 |#
    413.9 |#
    429.9 |#
  (18 below, 18 above range)

```

## Diagnostics

- **bc_index**: CV=60.2% (high variance, measurements may be unstable)
- **bc_index**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **bc_index**: bridge=15.6% of algo (FFI overhead may distort results)
- **bc_iter**: CV=63.3% (high variance, measurements may be unstable)
- **bc_iter**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **bc_iter**: bridge=15.7% of algo (FFI overhead may distort results)
- **bc_unchecked**: CV=60.2% (high variance, measurements may be unstable)
- **bc_unchecked**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **bc_unchecked**: bridge=15.9% of algo (FFI overhead may distort results)
