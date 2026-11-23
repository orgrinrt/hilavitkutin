# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (41 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bc_unchecked vs stability leader bc_iter (+1% speed for 1.0x steadier)

bc_unchecked is fastest (45 ns, CV 90.7%); bc_iter gives up 0.7% median for 1.0x lower variance (CV 90.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bc_index is inconsistent: worst-20% is 3.9x its best-20%

bc_index's best 20% of batches run at 37 ns but its worst 20% at 145 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.7% of the fastest

All 3 variants sit between 45 ns and 46 ns - a 0.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bc_unchecked's edge over baseline is significant but tiny (-0 ns, 0.44%)

bc_unchecked differs from baseline bc_index by -0 ns (0.44%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bc_unchecked** at 45.2 ns median (-0.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 45.2 ns, slowest 45.5 ns)

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
| Counter frequency | 2112744983 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 2837ns | 1923ns | 1571ns | 2149ns | 6168ns | base |
| bc_iter | 2611ns | 1922ns | 1571ns | 1887ns | 5823ns | -7.97% |
| bc_unchecked | 2893ns | 1923ns | 1571ns | 2270ns | 6083ns | +1.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 67ns | 37ns | 145ns | base | 3.822 |
| bc_iter | 62ns | 37ns | 141ns | -6.80% | 4.101 |
| bc_unchecked | 67ns | 37ns | 141ns | +0.67% | 3.797 |

## Performance model

- Peak throughput: **6.929 Gops/s** (bc_unchecked; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 5.639 | 81.4% |
| bc_iter | 5.626 | 81.2% |
| bc_unchecked | 5.664 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2837ns | 2837ns | base |
| bc_iter | 2611ns | 2611ns | -7.97% |
| bc_unchecked | 2893ns | 2893ns | +1.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 45ns | base | --- | [45, 45] | --- | --- | --- | --- |
| bc_iter | 46ns | no significant difference | [+0, +0]ns | [45, 46] | no | 0.4064 | 0.4064 | 15 |
| bc_unchecked | 45ns | -0.2ns (-0.4%) | [-0, -0]ns | [45, 45] | YES | 0.0000 | 0.0000 | 12 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 37ns | +0.3% | -0.4% |
| 1 | 2 | 37ns | +0.5% | -0.4% |
| 1 | 3 | 45ns | -17.8% | +112.0% |
| 1 | 4 | 46ns | +0.0% | -0.7% |
| 1 | 5 | 37ns | +22.6% | +21.5% |
| 1 | 6 | 37ns | +4.4% | -0.5% |
| 1 | 7 | 45ns | +0.6% | -0.2% |
| 1 | 8 | 105ns | -56.4% | -56.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | 0.055 | ok |
| bc_iter | 0.230 | moderate+ |
| bc_unchecked | -0.176 | ok |

**Consistency summary:**

- **bc_iter**: won 2/8, lost 5/8
- **bc_unchecked**: won 6/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 35.2ns | 67.0ns | 52.5% | HIGH |
| bc_iter | 32.2ns | 62.4ns | 51.6% | HIGH |
| bc_unchecked | 35.6ns | 67.4ns | 52.7% | HIGH |

## Distribution (algo ns)

```
bc_index (n=160, range 37.1-145.4 ns)
     37.1 |########################################
     42.5 |#######################################
     47.9 |
     53.3 |
     58.7 |#
     64.2 |##
     69.6 |#
     75.0 |
     80.4 |
     85.8 |#
     91.2 |
     96.7 |
    102.1 |#
    107.5 |##
    112.9 |##
    118.3 |##
    123.8 |##
    129.2 |#
    134.6 |#
    140.0 |
  (9 below, 18 above range)

bc_iter (n=160, range 37.2-140.7 ns)
     37.2 |#############################
     42.4 |########################################
     47.5 |
     52.7 |
     57.9 |
     63.1 |#
     68.2 |
     73.4 |
     78.6 |
     83.8 |
     89.0 |
     94.1 |
     99.3 |
    104.5 |#
    109.7 |
    114.8 |##
    120.0 |#
    125.2 |##
    130.4 |
    135.5 |#
  (8 below, 14 above range)

bc_unchecked (n=160, range 36.9-140.6 ns)
     36.9 |####################
     42.1 |########################################
     47.3 |
     52.5 |
     57.7 |
     62.9 |
     68.0 |
     73.2 |
     78.4 |
     83.6 |
     88.8 |#
     93.9 |####
     99.1 |#
    104.3 |
    109.5 |
    114.7 |###
    119.9 |
    125.0 |###
    130.2 |#
    135.4 |###
  (17 below, 15 above range)

```

## Diagnostics

- **bc_index**: CV=64.2% (high variance, measurements may be unstable)
- **bc_index**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **bc_index**: bridge=52.4% of algo (FFI overhead may distort results)
- **bc_iter**: CV=66.0% (high variance, measurements may be unstable)
- **bc_iter**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **bc_iter**: bridge=52.3% of algo (FFI overhead may distort results)
- **bc_unchecked**: CV=60.8% (high variance, measurements may be unstable)
- **bc_unchecked**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **bc_unchecked**: bridge=52.7% of algo (FFI overhead may distort results)
