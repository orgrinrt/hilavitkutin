# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 24% faster than the next best (dispatch_volatile)

dispatch_direct (577 ns) leads dispatch_volatile (717 ns) by 24%, a clear separation rather than a photo finish. CV 69.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_direct is fastest but the noisiest (CV 69.1%)

dispatch_direct wins on median (577 ns) yet has the highest variance (CV 69.1%), while dispatch_runtime is the steadiest (CV 66.0%, 738 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### dispatch_direct shows alternating (throttle bounce) (autocorr -0.63)

dispatch_direct's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (161 ns) is smaller than the fastest variant's own run-to-run std-dev (399 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (577 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_runtime is inconsistent: worst-20% is 3.0x its best-20%

dispatch_runtime's best 20% of batches run at 605 ns but its worst 20% at 1.82 us (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 577.4 ns median
- 2 variants significantly slower than baseline
- Spread: 1.28x (fastest 577.4 ns, slowest 738.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x40ffee15d34db33f |
| Counter frequency | 2112860326 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 2883ns | 2454ns | 2007ns | 2300ns | 5507ns | base |
| dispatch_runtime | 3316ns | 2620ns | 2142ns | 2656ns | 6471ns | +15.03% |
| dispatch_volatile | 3028ns | 2602ns | 2129ns | 2437ns | 5702ns | +5.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 681ns | 472ns | 1310ns | base | 6.019 |
| dispatch_runtime | 937ns | 605ns | 1822ns | +37.62% | 4.373 |
| dispatch_volatile | 832ns | 586ns | 1562ns | +22.27% | 4.923 |

## Performance model

- Peak throughput: **8.670 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 7.094 | 81.8% |
| dispatch_runtime | 5.548 | 64.0% |
| dispatch_volatile | 5.713 | 65.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2883ns | 2883ns | base |
| dispatch_runtime | 3316ns | 3316ns | +15.03% |
| dispatch_volatile | 3028ns | 3028ns | +5.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 577ns | base | --- | [577, 578] | --- | --- | --- | --- |
| dispatch_runtime | 738ns | +160.1ns (+27.7%) | [+158, +164]ns | [736, 741] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 717ns | +135.3ns (+23.4%) | [+115, +140]ns | [715, 717] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 578ns | +116.7% | +24.1% |
| 1 | 2 | 473ns | +29.7% | +24.3% |
| 1 | 3 | 578ns | +27.4% | +24.3% |
| 1 | 4 | 473ns | +157.2% | +51.9% |
| 1 | 5 | 473ns | +28.3% | +24.0% |
| 1 | 6 | 578ns | +28.9% | +1.7% |
| 1 | 7 | 473ns | +27.6% | +24.3% |
| 1 | 8 | 578ns | +27.8% | +24.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.625 | HIGH- (thermal bounce) |
| dispatch_runtime | -0.286 | moderate- |
| dispatch_volatile | -0.124 | ok |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 28.0ns | 680.5ns | 4.1% |  |
| dispatch_runtime | 31.1ns | 936.6ns | 3.3% |  |
| dispatch_volatile | 28.5ns | 832.1ns | 3.4% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 472.4-1309.6 ns)
    472.4 |###############################
    514.3 |
    556.2 |########################################
    598.0 |###
    639.9 |
    681.7 |
    723.6 |
    765.5 |
    807.3 |
    849.2 |
    891.0 |
    932.9 |
    974.7 |
   1016.6 |#
   1058.5 |
   1100.3 |
   1142.2 |
   1184.0 |
   1225.9 |
   1267.8 |
  (16 below, 19 above range)

dispatch_runtime (n=160, range 604.6-1822.0 ns)
    604.6 |#########################
    665.4 |##
    726.3 |########################################
    787.2 |###
    848.0 |#
    908.9 |##
    969.8 |
   1030.7 |#
   1091.5 |#
   1152.4 |#
   1213.3 |#
   1274.2 |
   1335.0 |
   1395.9 |##
   1456.8 |
   1517.7 |#
   1578.5 |
   1639.4 |#
   1700.3 |#
   1761.2 |#
  (16 below, 15 above range)

dispatch_volatile (n=160, range 586.5-1562.2 ns)
    586.5 |###########################
    635.3 |
    684.0 |########################################
    732.8 |
    781.6 |
    830.4 |
    879.2 |
    928.0 |
    976.8 |
   1025.6 |
   1074.4 |
   1123.1 |
   1171.9 |
   1220.7 |
   1269.5 |
   1318.3 |
   1367.1 |
   1415.9 |
   1464.7 |
   1513.5 |
  (17 below, 19 above range)

```

## Diagnostics

- **dispatch_direct**: CV=58.6% (high variance, measurements may be unstable)
- **dispatch_runtime**: CV=52.1% (high variance, measurements may be unstable)
- **dispatch_runtime**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **dispatch_volatile**: CV=57.1% (high variance, measurements may be unstable)
