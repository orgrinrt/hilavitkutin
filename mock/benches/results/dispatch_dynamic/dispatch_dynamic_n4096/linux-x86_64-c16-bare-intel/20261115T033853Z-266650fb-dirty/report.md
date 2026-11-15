# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 24% faster than the next best (dispatch_volatile)

dispatch_direct (578 ns) leads dispatch_volatile (718 ns) by 24%, a clear separation rather than a photo finish. CV 65.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (164 ns) is smaller than the fastest variant's own run-to-run std-dev (379 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (578 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_direct is inconsistent: worst-20% is 2.7x its best-20%

dispatch_direct's best 20% of batches run at 474 ns but its worst 20% at 1.27 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 577.5 ns median
- 2 variants significantly slower than baseline
- Spread: 1.28x (fastest 577.5 ns, slowest 741.2 ns)

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
| Counter frequency | 2112780910 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 2974ns | 2456ns | 2021ns | 2457ns | 5477ns | base |
| dispatch_runtime | 3000ns | 2620ns | 2140ns | 2469ns | 5453ns | +0.88% |
| dispatch_volatile | 3068ns | 2599ns | 2128ns | 2586ns | 5455ns | +3.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 696ns | 474ns | 1273ns | base | 5.889 |
| dispatch_runtime | 852ns | 605ns | 1548ns | +22.50% | 4.807 |
| dispatch_volatile | 843ns | 587ns | 1490ns | +21.26% | 4.856 |

## Performance model

- Peak throughput: **8.638 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 7.093 | 82.1% |
| dispatch_runtime | 5.526 | 64.0% |
| dispatch_volatile | 5.709 | 66.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2974ns | 2974ns | base |
| dispatch_runtime | 3000ns | 3000ns | +0.88% |
| dispatch_volatile | 3068ns | 3068ns | +3.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 578ns | base | --- | [577, 578] | --- | --- | --- | --- |
| dispatch_runtime | 741ns | +160.2ns (+27.7%) | [+137, +164]ns | [738, 743] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 718ns | +140.0ns (+24.2%) | [+139, +140]ns | [717, 718] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 578ns | +27.9% | +1.6% |
| 1 | 2 | 578ns | +5.2% | +24.3% |
| 1 | 3 | 473ns | +29.5% | +51.8% |
| 1 | 4 | 578ns | +30.8% | +24.1% |
| 1 | 5 | 578ns | +5.0% | +24.2% |
| 1 | 6 | 472ns | +57.3% | +24.3% |
| 1 | 7 | 578ns | +31.4% | +24.4% |
| 1 | 8 | 577ns | +6.6% | +24.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.375 | moderate- |
| dispatch_runtime | -0.379 | moderate- |
| dispatch_volatile | -0.212 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 28.6ns | 695.5ns | 4.1% |  |
| dispatch_runtime | 28.1ns | 852.0ns | 3.3% |  |
| dispatch_volatile | 29.1ns | 843.4ns | 3.4% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 474.2-1272.8 ns)
    474.2 |#
    514.1 |
    554.0 |########################################
    594.0 |
    633.9 |
    673.8 |
    713.8 |
    753.7 |
    793.6 |
    833.6 |
    873.5 |
    913.4 |
    953.4 |
    993.3 |
   1033.2 |
   1073.2 |
   1113.1 |
   1153.0 |
   1192.9 |
   1232.9 |
  (29 below, 19 above range)

dispatch_runtime (n=160, range 605.2-1548.1 ns)
    605.2 |########################################
    652.4 |
    699.5 |###############################
    746.7 |#############################
    793.8 |
    841.0 |
    888.1 |
    935.3 |
    982.4 |
   1029.5 |
   1076.7 |
   1123.8 |
   1171.0 |
   1218.1 |
   1265.3 |
   1312.4 |
   1359.5 |
   1406.7 |
   1453.8 |
   1501.0 |
  (16 below, 19 above range)

dispatch_volatile (n=160, range 586.8-1489.8 ns)
    586.8 |########
    632.0 |
    677.1 |########################################
    722.3 |##
    767.4 |
    812.6 |
    857.7 |
    902.9 |
    948.0 |
    993.2 |
   1038.3 |
   1083.5 |
   1128.6 |
   1173.8 |
   1218.9 |
   1264.0 |
   1309.2 |
   1354.3 |
   1399.5 |
   1444.6 |
  (13 below, 15 above range)

```

## Diagnostics

- **dispatch_direct**: CV=54.6% (high variance, measurements may be unstable)
- **dispatch_runtime**: CV=52.8% (high variance, measurements may be unstable)
- **dispatch_volatile**: CV=56.1% (high variance, measurements may be unstable)
