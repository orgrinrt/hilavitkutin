# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sat_wrap dominates: 71% faster than the next best (sat_checked)

sat_wrap (295 ns) leads sat_checked (506 ns) by 71%, a clear separation rather than a photo finish. CV 80.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 42% (significant)

sat_wrap is -212 ns (42%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (214 ns) is smaller than the fastest variant's own run-to-run std-dev (236 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### sat_saturate is inconsistent: worst-20% is 5.1x its best-20%

sat_saturate's best 20% of batches run at 413 ns but its worst 20% at 2.11 us (5.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### sat_saturate's edge over baseline is significant but tiny (8 ns, 1.52%)

sat_saturate differs from baseline sat_checked by 8 ns (1.52%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: sat_wrap** at 295.1 ns median (-41.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.72x (fastest 295.1 ns, slowest 508.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Overflow strategy: wrapping_add vs saturating_add vs checked_add |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x52b9de7c318fa064 |
| Counter frequency | 2112805263 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 3732ns | 2390ns | 1958ns | 2354ns | 9638ns | base |
| sat_saturate | 4008ns | 2393ns | 1955ns | 2672ns | 10072ns | +7.42% |
| sat_wrap | 2824ns | 2181ns | 1787ns | 2179ns | 5798ns | -24.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 781ns | 413ns | 2002ns | base | 1.311 |
| sat_saturate | 845ns | 413ns | 2110ns | +8.14% | 1.212 |
| sat_wrap | 381ns | 241ns | 780ns | -51.25% | 2.688 |

## Performance model

- Peak throughput: **4.242 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 2.024 | 47.7% |
| sat_saturate | 2.013 | 47.5% |
| sat_wrap | 3.470 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 3732ns | 3732ns | base |
| sat_saturate | 4008ns | 4008ns | +7.42% |
| sat_wrap | 2824ns | 2824ns | -24.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 506ns | base | --- | [502, 508] | --- | --- | --- | --- |
| sat_saturate | 509ns | +7.7ns (+1.5%) | [+1, +79]ns | [507, 510] | YES | 0.0261 | 0.0261 | 1 |
| sat_wrap | 295ns | -211.7ns (-41.8%) | [-214, -208]ns | [295, 295] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 505ns | +131.9% | -41.6% |
| 1 | 2 | 508ns | +0.1% | -42.0% |
| 1 | 3 | 507ns | -17.8% | -41.8% |
| 1 | 4 | 508ns | +0.1% | -52.5% |
| 1 | 5 | 416ns | +22.4% | -29.0% |
| 1 | 6 | 417ns | +22.0% | -29.3% |
| 1 | 7 | 923ns | -54.2% | -73.9% |
| 1 | 8 | 507ns | -17.9% | -41.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.203 | moderate- |
| sat_saturate | 0.032 | ok |
| sat_wrap | -0.374 | moderate- |

**Consistency summary:**

- **sat_saturate**: won 3/8, lost 4/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 37.2ns | 781.4ns | 4.8% |  |
| sat_saturate | 39.8ns | 845.0ns | 4.7% |  |
| sat_wrap | 31.0ns | 380.9ns | 8.1% | HIGH |

## Distribution (algo ns)

```
sat_checked (n=160, range 413.0-2002.2 ns)
    413.0 |############
    492.4 |########################################
    571.9 |
    651.4 |
    730.8 |
    810.3 |
    889.7 |
    969.2 |
   1048.7 |
   1128.1 |
   1207.6 |
   1287.0 |
   1366.5 |##
   1446.0 |
   1525.4 |#
   1604.9 |#
   1684.3 |
   1763.8 |
   1843.3 |
   1922.7 |
  (15 below, 7 above range)

sat_saturate (n=160, range 413.1-2109.6 ns)
    413.1 |###################
    497.9 |########################################
    582.7 |
    667.5 |#
    752.4 |##
    837.2 |#
    922.0 |
   1006.9 |#
   1091.7 |#
   1176.5 |###
   1261.3 |#
   1346.2 |#
   1431.0 |#
   1515.8 |#
   1600.7 |##
   1685.5 |#
   1770.3 |###
   1855.1 |
   1940.0 |
   2024.8 |
  (15 below, 6 above range)

sat_wrap (n=160, range 241.4-779.6 ns)
    241.4 |####
    268.3 |########################################
    295.2 |
    322.1 |
    349.0 |
    376.0 |
    402.9 |
    429.8 |
    456.7 |
    483.6 |
    510.5 |
    537.4 |
    564.3 |
    591.2 |
    618.1 |
    645.0 |
    672.0 |
    698.9 |
    725.8 |
    752.7 |
  (23 below, 21 above range)

```

## Diagnostics

- **sat_checked**: CV=110.1% (high variance, measurements may be unstable)
- **sat_checked**: worst_20/best_20 = 4.8x (possible bimodal distribution)
- **sat_saturate**: CV=99.9% (high variance, measurements may be unstable)
- **sat_saturate**: worst_20/best_20 = 5.1x (possible bimodal distribution)
- **sat_wrap**: CV=62.0% (high variance, measurements may be unstable)
- **sat_wrap**: worst_20/best_20 = 3.2x (possible bimodal distribution)
