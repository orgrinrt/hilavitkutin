# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_unchecked is fastest but the noisiest (CV 92.0%)

bc_unchecked wins on median (45 ns) yet has the highest variance (CV 92.0%), while bc_iter is the steadiest (CV 85.8%, 46 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (42 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bc_unchecked vs stability leader bc_iter (+1% speed for 1.1x steadier)

bc_unchecked is fastest (45 ns, CV 92.0%); bc_iter gives up 0.7% median for 1.1x lower variance (CV 85.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bc_unchecked is inconsistent: worst-20% is 3.9x its best-20%

bc_unchecked's best 20% of batches run at 37 ns but its worst 20% at 143 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.7% of the fastest

All 3 variants sit between 45 ns and 46 ns - a 0.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bc_unchecked's edge over baseline is significant but tiny (-0 ns, 0.44%)

bc_unchecked differs from baseline bc_index by -0 ns (0.44%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bc_unchecked** at 45.3 ns median (-0.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 45.3 ns, slowest 45.6 ns)

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
| bc_index | 2851ns | 1932ns | 1785ns | 2123ns | 6101ns | base |
| bc_iter | 2616ns | 1932ns | 1581ns | 1912ns | 5762ns | -8.24% |
| bc_unchecked | 2769ns | 1931ns | 1578ns | 2058ns | 6093ns | -2.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 67ns | 42ns | 144ns | base | 3.811 |
| bc_iter | 62ns | 37ns | 135ns | -8.17% | 4.150 |
| bc_unchecked | 65ns | 37ns | 143ns | -2.80% | 3.921 |

## Performance model

- Peak throughput: **6.918 Gops/s** (bc_unchecked; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 5.626 | 81.3% |
| bc_iter | 5.614 | 81.1% |
| bc_unchecked | 5.651 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2851ns | 2851ns | base |
| bc_iter | 2616ns | 2616ns | -8.24% |
| bc_unchecked | 2769ns | 2769ns | -2.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 46ns | base | --- | [45, 46] | --- | --- | --- | --- |
| bc_iter | 46ns | no significant difference | [+0, +0]ns | [46, 46] | no | 0.0861 | 0.0861 | 10 |
| bc_unchecked | 45ns | -0.2ns (-0.4%) | [-4, -0]ns | [45, 45] | YES | 0.0001 | 0.0000 | 14 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 46ns | +0.3% | -18.5% |
| 1 | 2 | 46ns | +0.4% | -0.2% |
| 1 | 3 | 46ns | +0.8% | -18.5% |
| 1 | 4 | 45ns | -16.9% | +0.0% |
| 1 | 5 | 46ns | +0.4% | +8.6% |
| 1 | 6 | 37ns | +0.3% | +21.8% |
| 1 | 7 | 45ns | -17.8% | -0.1% |
| 1 | 8 | 45ns | +0.7% | -18.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.163 | ok |
| bc_iter | -0.127 | ok |
| bc_unchecked | -0.105 | ok |

**Consistency summary:**

- **bc_iter**: won 2/8, lost 6/8
- **bc_unchecked**: won 5/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 35.4ns | 67.2ns | 52.6% | HIGH |
| bc_iter | 32.2ns | 61.7ns | 52.2% | HIGH |
| bc_unchecked | 34.0ns | 65.3ns | 52.1% | HIGH |

## Distribution (algo ns)

```
bc_index (n=160, range 42.0-143.7 ns)
     42.0 |########################################
     47.1 |
     52.2 |
     57.3 |
     62.4 |
     67.4 |
     72.5 |
     77.6 |
     82.7 |
     87.8 |
     92.9 |
     97.9 |#
    103.0 |
    108.1 |#
    113.2 |
    118.3 |
    123.4 |
    128.5 |
    133.5 |###
    138.6 |
  (13 below, 15 above range)

bc_iter (n=160, range 37.3-134.8 ns)
     37.3 |###############
     42.2 |########################################
     47.0 |##
     51.9 |
     56.8 |
     61.7 |
     66.6 |
     71.4 |
     76.3 |
     81.2 |
     86.1 |
     90.9 |
     95.8 |
    100.7 |#
    105.6 |
    110.5 |
    115.3 |#
    120.2 |
    125.1 |#
    130.0 |
  (8 below, 14 above range)

bc_unchecked (n=160, range 37.0-143.2 ns)
     37.0 |#############
     42.3 |########################################
     47.6 |####
     52.9 |
     58.2 |
     63.6 |
     68.9 |
     74.2 |
     79.5 |
     84.8 |
     90.1 |
     95.4 |#
    100.7 |
    106.1 |
    111.4 |
    116.7 |#
    122.0 |##
    127.3 |#
    132.6 |####
    137.9 |
  (21 below, 16 above range)

```

## Diagnostics

- **bc_index**: CV=61.0% (high variance, measurements may be unstable)
- **bc_index**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **bc_index**: bridge=52.5% of algo (FFI overhead may distort results)
- **bc_iter**: CV=63.5% (high variance, measurements may be unstable)
- **bc_iter**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **bc_iter**: bridge=52.4% of algo (FFI overhead may distort results)
- **bc_unchecked**: CV=63.8% (high variance, measurements may be unstable)
- **bc_unchecked**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **bc_unchecked**: bridge=52.5% of algo (FFI overhead may distort results)
