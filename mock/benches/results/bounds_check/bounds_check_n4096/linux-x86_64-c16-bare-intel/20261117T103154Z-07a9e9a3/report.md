# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_unchecked is fastest but the noisiest (CV 68.2%)

bc_unchecked wins on median (580 ns) yet has the highest variance (CV 68.2%), while bc_index is the steadiest (CV 62.3%, 580 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (395 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bc_unchecked vs stability leader bc_index (+0% speed for 1.1x steadier)

bc_unchecked is fastest (580 ns, CV 68.2%); bc_index gives up 0.0% median for 1.1x lower variance (CV 62.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bc_unchecked is inconsistent: worst-20% is 2.7x its best-20%

bc_unchecked's best 20% of batches run at 474 ns but its worst 20% at 1.27 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.1% of the fastest

All 3 variants sit between 580 ns and 580 ns - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bc_iter's edge over baseline is significant but tiny (0 ns, 0.05%)

bc_iter differs from baseline bc_index by 0 ns (0.05%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bc_unchecked** at 579.8 ns median (-0.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 579.8 ns, slowest 580.3 ns)

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
| bc_index | 2904ns | 2467ns | 2023ns | 2468ns | 5094ns | base |
| bc_iter | 3027ns | 2468ns | 2241ns | 2469ns | 5487ns | +4.23% |
| bc_unchecked | 2966ns | 2465ns | 2018ns | 2448ns | 5465ns | +2.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 682ns | 476ns | 1192ns | base | 6.009 |
| bc_iter | 710ns | 527ns | 1283ns | +4.21% | 5.766 |
| bc_unchecked | 695ns | 474ns | 1272ns | +1.90% | 5.897 |

## Performance model

- Peak throughput: **8.635 Gops/s** (bc_unchecked; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 7.062 | 81.8% |
| bc_iter | 7.058 | 81.7% |
| bc_unchecked | 7.065 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2904ns | 2904ns | base |
| bc_iter | 3027ns | 3027ns | +4.23% |
| bc_unchecked | 2966ns | 2966ns | +2.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 580ns | base | --- | [580, 580] | --- | --- | --- | --- |
| bc_iter | 580ns | +0.3ns (+0.1%) | [+0, +0]ns | [580, 580] | YES | 0.0000 | 0.0000 | 7 |
| bc_unchecked | 580ns | -0.2ns (-0.0%) | [-0, -0]ns | [580, 580] | YES | 0.0000 | 0.0000 | 9 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 580ns | +0.1% | -0.0% |
| 1 | 2 | 580ns | +0.0% | -0.1% |
| 1 | 3 | 475ns | +22.3% | -0.0% |
| 1 | 4 | 580ns | +0.0% | -18.2% |
| 1 | 5 | 580ns | +0.1% | -0.0% |
| 1 | 6 | 475ns | +0.0% | +22.2% |
| 1 | 7 | 580ns | +0.0% | -0.0% |
| 1 | 8 | 580ns | +0.1% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.375 | moderate- |
| bc_iter | -0.160 | ok |
| bc_unchecked | 0.292 | moderate+ |

**Consistency summary:**

- **bc_iter**: won 0/8, lost 1/8
- **bc_unchecked**: won 1/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 27.9ns | 681.7ns | 4.1% |  |
| bc_iter | 29.4ns | 710.4ns | 4.1% |  |
| bc_unchecked | 29.4ns | 694.6ns | 4.2% |  |

## Distribution (algo ns)

```
bc_index (n=160, range 476.1-1192.2 ns)
    476.1 |#
    511.9 |
    547.7 |########################################
    583.5 |###
    619.3 |
    655.2 |
    691.0 |
    726.8 |
    762.6 |
    798.4 |
    834.2 |
    870.0 |
    905.8 |
    941.6 |
    977.4 |
   1013.2 |
   1049.0 |
   1084.8 |
   1120.6 |
   1156.4 |
  (29 below, 17 above range)

bc_iter (n=160, range 527.5-1283.4 ns)
    527.5 |
    565.3 |########################################
    603.1 |
    640.9 |
    678.7 |
    716.5 |
    754.3 |
    792.1 |
    829.9 |
    867.7 |
    905.5 |
    943.2 |
    981.0 |
   1018.8 |
   1056.6 |
   1094.4 |
   1132.2 |
   1170.0 |
   1207.8 |
   1245.6 |
  (16 below, 17 above range)

bc_unchecked (n=160, range 474.4-1271.8 ns)
    474.4 |########
    514.2 |
    554.1 |########################################
    594.0 |
    633.9 |
    673.7 |
    713.6 |
    753.5 |
    793.3 |
    833.2 |
    873.1 |
    913.0 |
    952.8 |
    992.7 |
   1032.6 |
   1072.4 |
   1112.3 |
   1152.2 |
   1192.1 |
   1231.9 |
  (13 below, 17 above range)

```

## Diagnostics

- **bc_index**: CV=53.0% (high variance, measurements may be unstable)
- **bc_iter**: CV=54.7% (high variance, measurements may be unstable)
- **bc_unchecked**: CV=56.9% (high variance, measurements may be unstable)
