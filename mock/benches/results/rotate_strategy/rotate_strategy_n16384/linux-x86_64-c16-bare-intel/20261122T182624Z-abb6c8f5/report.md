# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 3.0x slower than the field

rot_variable_amount (3.43 us) is 3.0x the fastest (1.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (rot_manual_shifts, rot_intrinsic_const) are a dead heat (<1%)

rot_manual_shifts (1.15 us) and rot_intrinsic_const (1.15 us) differ by 0.01%, inside the noise, even though the wider field spreads 198.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### rot_manual_shifts's comparison is tie-heavy (11% tied pairs)

11% of paired samples for rot_manual_shifts are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### rot_intrinsic_const is inconsistent: worst-20% is 1.9x its best-20%

rot_intrinsic_const's best 20% of batches run at 939 ns but its worst 20% at 1.76 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: rot_manual_shifts** at 1147.7 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.99x (fastest 1147.7 ns, slowest 3427.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Rotate strategy: intrinsic const vs manual shifts vs variable amount |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x71f692ab4e08d325 |
| Counter frequency | 2112817215 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rot_intrinsic_const | 3114ns | 3023ns | 2474ns | 2806ns | 4678ns | base |
| rot_manual_shifts | 3047ns | 3020ns | 2473ns | 2798ns | 4368ns | -2.15% |
| rot_variable_amount | 4967ns | 5308ns | 4342ns | 5037ns | 5384ns | +59.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 1178ns | 939ns | 1759ns | base | 13.906 |
| rot_manual_shifts | 1153ns | 939ns | 1644ns | -2.13% | 14.209 |
| rot_variable_amount | 3205ns | 2804ns | 3477ns | +172.02% | 5.112 |

## Performance model

- Peak throughput: **17.447 Gops/s** (rot_manual_shifts; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 14.274 | 81.8% |
| rot_manual_shifts | 14.276 | 81.8% |
| rot_variable_amount | 4.781 | 27.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 3114ns | 3114ns | base |
| rot_manual_shifts | 3047ns | 3047ns | -2.15% |
| rot_variable_amount | 4967ns | 4967ns | +59.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 1148ns | base | --- | [950, 1148] | --- | --- | --- | --- |
| rot_manual_shifts | 1148ns | no significant difference | [-0, +0]ns | [948, 1148] | no | 0.3156 | 0.3156 | **17** (11%, HIGH) |
| rot_variable_amount | 3427ns | +2279.0ns (+198.6%) | [+1877, +2279]ns | [3427, 3427] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 939ns | +22.2% | +198.5% |
| 1 | 2 | 1148ns | -18.2% | +198.6% |
| 1 | 3 | 1148ns | -18.2% | +144.3% |
| 1 | 4 | 1148ns | +0.0% | +198.6% |
| 1 | 5 | 939ns | +22.2% | +198.5% |
| 1 | 6 | 1148ns | +0.0% | +198.6% |
| 1 | 7 | 939ns | +0.0% | +265.6% |
| 1 | 8 | 939ns | +0.0% | +264.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.125 | ok |
| rot_manual_shifts | 0.125 | ok |
| rot_variable_amount | -0.470 | moderate- |

**Consistency summary:**

- **rot_manual_shifts**: won 2/8, lost 2/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 24.4ns | 1178.2ns | 2.1% |  |
| rot_manual_shifts | 24.1ns | 1153.1ns | 2.1% |  |
| rot_variable_amount | 21.9ns | 3205.0ns | 0.7% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 939.1-1759.5 ns)
    939.1 |################################
    980.1 |
   1021.1 |
   1062.1 |
   1103.2 |
   1144.2 |########################################
   1185.2 |
   1226.2 |
   1267.2 |
   1308.3 |
   1349.3 |
   1390.3 |
   1431.3 |
   1472.3 |
   1513.4 |
   1554.4 |
   1595.4 |
   1636.4 |
   1677.4 |
   1718.5 |
  (10 below, 11 above range)

rot_manual_shifts (n=160, range 939.1-1644.4 ns)
    939.1 |##################################
    974.3 |
   1009.6 |
   1044.9 |
   1080.1 |
   1115.4 |########################################
   1150.7 |####
   1185.9 |
   1221.2 |
   1256.5 |
   1291.7 |
   1327.0 |
   1362.3 |
   1397.6 |
   1432.8 |
   1468.1 |
   1503.4 |
   1538.6 |
   1573.9 |
   1609.2 |
  (13 below, 9 above range)

rot_variable_amount (n=160, range 2803.8-3477.0 ns)
   2803.8 |#################
   2837.5 |
   2871.1 |
   2904.8 |
   2938.5 |
   2972.1 |
   3005.8 |
   3039.4 |
   3073.1 |
   3106.8 |
   3140.4 |
   3174.1 |
   3207.8 |
   3241.4 |
   3275.1 |
   3308.7 |
   3342.4 |
   3376.1 |
   3409.7 |########################################
   3443.4 |##
  (19 below, 4 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=41.7% (high variance, measurements may be unstable)
- **rot_manual_shifts**: CV=37.9% (high variance, measurements may be unstable)
