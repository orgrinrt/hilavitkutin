# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_unchecked is fastest but the noisiest (CV 71.9%)

bc_unchecked wins on median (580 ns) yet has the highest variance (CV 71.9%), while bc_index is the steadiest (CV 64.7%, 580 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (417 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bc_unchecked vs stability leader bc_index (+0% speed for 1.1x steadier)

bc_unchecked is fastest (580 ns, CV 71.9%); bc_index gives up 0.0% median for 1.1x lower variance (CV 64.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bc_unchecked is inconsistent: worst-20% is 3.1x its best-20%

bc_unchecked's best 20% of batches run at 478 ns but its worst 20% at 1.49 us (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.1% of the fastest

All 3 variants sit between 580 ns and 580 ns - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bc_iter's edge over baseline is significant but tiny (0 ns, 0.05%)

bc_iter differs from baseline bc_index by 0 ns (0.05%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bc_unchecked** at 579.8 ns median (-0.0% vs baseline)
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
| Counter frequency | 2112761440 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 2933ns | 2460ns | 2011ns | 2451ns | 5300ns | base |
| bc_iter | 2993ns | 2459ns | 2219ns | 2461ns | 5365ns | +2.06% |
| bc_unchecked | 3155ns | 2462ns | 2028ns | 2463ns | 6359ns | +7.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 689ns | 475ns | 1239ns | base | 5.941 |
| bc_iter | 708ns | 524ns | 1273ns | +2.64% | 5.788 |
| bc_unchecked | 741ns | 478ns | 1487ns | +7.48% | 5.527 |

## Performance model

- Peak throughput: **8.626 Gops/s** (bc_index; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 7.062 | 81.9% |
| bc_iter | 7.058 | 81.8% |
| bc_unchecked | 7.065 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2933ns | 2933ns | base |
| bc_iter | 2993ns | 2993ns | +2.06% |
| bc_unchecked | 3155ns | 3155ns | +7.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 580ns | base | --- | [580, 580] | --- | --- | --- | --- |
| bc_iter | 580ns | +0.3ns (+0.1%) | [+0, +0]ns | [580, 580] | YES | 0.0000 | 0.0000 | 6 |
| bc_unchecked | 580ns | no significant difference | [-0, +0]ns | [580, 580] | no (adj: YES) | 0.0340 | 0.0340 | 9 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 580ns | +0.0% | -0.0% |
| 1 | 2 | 580ns | -18.1% | -18.2% |
| 1 | 3 | 475ns | +22.3% | +22.2% |
| 1 | 4 | 475ns | +22.2% | +22.1% |
| 1 | 5 | 580ns | +0.0% | -18.2% |
| 1 | 6 | 580ns | +0.1% | -0.0% |
| 1 | 7 | 580ns | +0.1% | -0.0% |
| 1 | 8 | 580ns | +0.0% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | 0.292 | moderate+ |
| bc_iter | -0.161 | ok |
| bc_unchecked | -0.376 | moderate- |

**Consistency summary:**

- **bc_iter**: won 1/8, lost 2/8
- **bc_unchecked**: won 2/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 28.1ns | 689.5ns | 4.1% |  |
| bc_iter | 28.9ns | 707.7ns | 4.1% |  |
| bc_unchecked | 30.7ns | 741.0ns | 4.1% |  |

## Distribution (algo ns)

```
bc_index (n=160, range 474.8-1238.7 ns)
    474.8 |###
    513.0 |
    551.2 |########################################
    589.4 |###
    627.6 |
    665.8 |
    704.0 |
    742.2 |
    780.4 |
    818.5 |
    856.7 |
    894.9 |
    933.1 |
    971.3 |
   1009.5 |
   1047.7 |
   1085.9 |
   1124.1 |
   1162.3 |
   1200.5 |
  (25 below, 17 above range)

bc_iter (n=160, range 524.2-1273.3 ns)
    524.2 |
    561.7 |########################################
    599.1 |
    636.6 |
    674.0 |
    711.5 |
    748.9 |
    786.4 |
    823.9 |
    861.3 |
    898.8 |
    936.2 |
    973.7 |
   1011.1 |
   1048.6 |
   1086.0 |
   1123.5 |
   1161.0 |
   1198.4 |
   1235.9 |
  (17 below, 18 above range)

bc_unchecked (n=160, range 477.8-1486.8 ns)
    477.8 |
    528.2 |
    578.7 |########################################
    629.1 |
    679.6 |
    730.0 |
    780.5 |
    830.9 |
    881.4 |
    931.8 |
    982.3 |
   1032.7 |
   1083.2 |
   1133.6 |
   1184.1 |
   1234.5 |
   1285.0 |
   1335.4 |
   1385.9 |
   1436.3 |
  (31 below, 20 above range)

```

## Diagnostics

- **bc_index**: CV=54.4% (high variance, measurements may be unstable)
- **bc_iter**: CV=54.0% (high variance, measurements may be unstable)
- **bc_unchecked**: CV=56.3% (high variance, measurements may be unstable)
- **bc_unchecked**: worst_20/best_20 = 3.1x (possible bimodal distribution)
