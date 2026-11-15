# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_unchecked dominates: 23% faster than the next best (bc_index)

bc_unchecked (37 ns) leads bc_index (45 ns) by 23%, a clear separation rather than a photo finish. CV 109.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bc_unchecked is fastest but the noisiest (CV 109.2%)

bc_unchecked wins on median (37 ns) yet has the highest variance (CV 109.2%), while bc_iter is the steadiest (CV 86.6%, 46 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (9 ns) is smaller than the fastest variant's own run-to-run std-dev (41 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bc_index is inconsistent: worst-20% is 3.9x its best-20%

bc_index's best 20% of batches run at 37 ns but its worst 20% at 144 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### bc_iter's edge over baseline is significant but tiny (0 ns, 0.44%)

bc_iter differs from baseline bc_index by 0 ns (0.44%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bc_unchecked** at 37.1 ns median (-18.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.23x (fastest 37.1 ns, slowest 45.7 ns)

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
| Counter frequency | 2112761440 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 2810ns | 1922ns | 1572ns | 2139ns | 6061ns | base |
| bc_iter | 2813ns | 1925ns | 1659ns | 2163ns | 5919ns | +0.12% |
| bc_unchecked | 2498ns | 1574ns | 1568ns | 1720ns | 5761ns | -11.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 66ns | 37ns | 144ns | base | 3.850 |
| bc_iter | 66ns | 39ns | 139ns | -0.10% | 3.854 |
| bc_unchecked | 59ns | 37ns | 135ns | -11.83% | 4.366 |

## Performance model

- Peak throughput: **6.928 Gops/s** (bc_unchecked; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 5.633 | 81.3% |
| bc_iter | 5.602 | 80.9% |
| bc_unchecked | 6.900 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2810ns | 2810ns | base |
| bc_iter | 2813ns | 2813ns | +0.12% |
| bc_unchecked | 2498ns | 2498ns | -11.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 45ns | base | --- | [45, 46] | --- | --- | --- | --- |
| bc_iter | 46ns | +0.2ns (+0.4%) | [+0, +0]ns | [46, 46] | YES | 0.0011 | 0.0011 | 0 |
| bc_unchecked | 37ns | -8.3ns (-18.3%) | [-8, -8]ns | [37, 37] | YES | 0.0000 | 0.0000 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 37ns | +22.3% | -0.4% |
| 1 | 2 | 37ns | +22.3% | -0.3% |
| 1 | 3 | 46ns | -18.2% | -17.6% |
| 1 | 4 | 37ns | +23.1% | -0.1% |
| 1 | 5 | 46ns | +0.7% | -18.5% |
| 1 | 6 | 45ns | -17.7% | -18.4% |
| 1 | 7 | 46ns | +0.4% | -18.4% |
| 1 | 8 | 45ns | +0.7% | -18.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | 0.051 | ok |
| bc_iter | -0.377 | moderate- |
| bc_unchecked | -0.208 | moderate- |

**Consistency summary:**

- **bc_iter**: won 2/8, lost 6/8
- **bc_unchecked**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 34.7ns | 66.5ns | 52.2% | HIGH |
| bc_iter | 34.6ns | 66.4ns | 52.1% | HIGH |
| bc_unchecked | 30.7ns | 58.6ns | 52.4% | HIGH |

## Distribution (algo ns)

```
bc_index (n=160, range 37.1-143.5 ns)
     37.1 |###########
     42.5 |########################################
     47.8 |
     53.1 |
     58.4 |
     63.7 |
     69.1 |
     74.4 |
     79.7 |#
     85.0 |
     90.3 |
     95.7 |
    101.0 |
    106.3 |
    111.6 |#
    116.9 |#
    122.3 |###
    127.6 |#
    132.9 |##
    138.2 |
  (19 below, 16 above range)

bc_iter (n=160, range 39.2-139.0 ns)
     39.2 |
     44.2 |########################################
     49.2 |
     54.2 |
     59.2 |
     64.2 |
     69.1 |
     74.1 |
     79.1 |
     84.1 |
     89.1 |
     94.1 |##
     99.1 |
    104.1 |
    109.1 |#
    114.1 |
    119.1 |
    124.1 |#
    129.1 |
    134.0 |#
  (24 below, 15 above range)

bc_unchecked (n=160, range 36.9-134.6 ns)
     36.9 |########################################
     41.8 |
     46.7 |
     51.6 |
     56.5 |
     61.4 |
     66.3 |
     71.1 |
     76.0 |
     80.9 |
     85.8 |
     90.7 |
     95.6 |
    100.4 |
    105.3 |
    110.2 |#
    115.1 |#
    120.0 |#
    124.9 |#
    129.8 |#
  (15 below, 14 above range)

```

## Diagnostics

- **bc_index**: CV=63.0% (high variance, measurements may be unstable)
- **bc_index**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **bc_index**: bridge=52.4% of algo (FFI overhead may distort results)
- **bc_iter**: CV=59.6% (high variance, measurements may be unstable)
- **bc_iter**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **bc_iter**: bridge=52.1% of algo (FFI overhead may distort results)
- **bc_unchecked**: CV=69.1% (high variance, measurements may be unstable)
- **bc_unchecked**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **bc_unchecked**: bridge=52.6% of algo (FFI overhead may distort results)
