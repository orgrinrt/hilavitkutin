# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 24% faster than the next best (dispatch_volatile)

dispatch_direct (578 ns) leads dispatch_volatile (717 ns) by 24%, a clear separation rather than a photo finish. CV 56.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (166 ns) is smaller than the fastest variant's own run-to-run std-dev (327 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (578 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_runtime is inconsistent: worst-20% is 2.6x its best-20%

dispatch_runtime's best 20% of batches run at 605 ns but its worst 20% at 1.60 us (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 577.6 ns median
- 2 variants significantly slower than baseline
- Spread: 1.29x (fastest 577.6 ns, slowest 743.1 ns)

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
| Counter frequency | 2112761094 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 2903ns | 2465ns | 2251ns | 2466ns | 4864ns | base |
| dispatch_runtime | 3059ns | 2636ns | 2149ns | 2540ns | 5528ns | +5.40% |
| dispatch_volatile | 3043ns | 2601ns | 2127ns | 2513ns | 5548ns | +4.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 678ns | 529ns | 1130ns | base | 6.038 |
| dispatch_runtime | 873ns | 605ns | 1595ns | +28.74% | 4.690 |
| dispatch_volatile | 837ns | 587ns | 1520ns | +23.35% | 4.895 |

## Performance model

- Peak throughput: **7.750 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 7.091 | 91.5% |
| dispatch_runtime | 5.512 | 71.1% |
| dispatch_volatile | 5.711 | 73.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2903ns | 2903ns | base |
| dispatch_runtime | 3059ns | 3059ns | +5.40% |
| dispatch_volatile | 3043ns | 3043ns | +4.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 578ns | base | --- | [578, 578] | --- | --- | --- | --- |
| dispatch_runtime | 743ns | +168.6ns (+29.2%) | [+165, +179]ns | [742, 755] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 717ns | +139.3ns (+24.1%) | [+136, +140]ns | [717, 718] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 578ns | +5.2% | +24.3% |
| 1 | 2 | 578ns | +28.6% | +1.7% |
| 1 | 3 | 578ns | +31.0% | +1.7% |
| 1 | 4 | 578ns | +32.4% | +24.4% |
| 1 | 5 | 578ns | +4.5% | +24.1% |
| 1 | 6 | 578ns | +31.3% | +24.2% |
| 1 | 7 | 473ns | +56.7% | +59.4% |
| 1 | 8 | 578ns | +5.2% | +1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.160 | ok |
| dispatch_runtime | -0.265 | moderate- |
| dispatch_volatile | 0.006 | ok |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 28.0ns | 678.3ns | 4.1% |  |
| dispatch_runtime | 28.4ns | 873.3ns | 3.3% |  |
| dispatch_volatile | 28.9ns | 836.7ns | 3.5% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 528.5-1130.3 ns)
    528.5 |
    558.6 |########################################
    588.7 |###
    618.8 |
    648.9 |
    679.0 |
    709.1 |
    739.1 |
    769.2 |
    799.3 |
    829.4 |
    859.5 |
    889.6 |
    919.7 |
    949.7 |
    979.8 |
   1009.9 |
   1040.0 |
   1070.1 |
   1100.2 |
  (15 below, 16 above range)

dispatch_runtime (n=160, range 604.7-1595.3 ns)
    604.7 |############################
    654.2 |
    703.8 |###############################
    753.3 |########################################
    802.8 |
    852.4 |
    901.9 |
    951.4 |
   1001.0 |
   1050.5 |
   1100.0 |
   1149.6 |
   1199.1 |
   1248.6 |
   1298.1 |
   1347.7 |
   1397.2 |
   1446.7 |
   1496.3 |
   1545.8 |
  (18 below, 17 above range)

dispatch_volatile (n=160, range 586.9-1520.3 ns)
    586.9 |#######################
    633.6 |
    680.2 |########################################
    726.9 |###
    773.6 |###
    820.2 |
    866.9 |#
    913.6 |
    960.3 |
   1006.9 |
   1053.6 |
   1100.3 |
   1146.9 |
   1193.6 |
   1240.3 |
   1286.9 |
   1333.6 |
   1380.3 |
   1426.9 |
   1473.6 |
  (8 below, 16 above range)

```

## Diagnostics

- **dispatch_direct**: CV=48.2% (high variance, measurements may be unstable)
- **dispatch_runtime**: CV=54.4% (high variance, measurements may be unstable)
- **dispatch_volatile**: CV=54.6% (high variance, measurements may be unstable)
