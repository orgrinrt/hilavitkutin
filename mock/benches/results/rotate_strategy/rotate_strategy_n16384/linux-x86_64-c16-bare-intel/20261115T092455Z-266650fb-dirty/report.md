# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 3.0x slower than the field

rot_variable_amount (3.43 us) is 3.0x the fastest (1.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (rot_intrinsic_const, rot_manual_shifts) are a dead heat (<1%)

rot_intrinsic_const (1.15 us) and rot_manual_shifts (1.15 us) differ by 0.00%, inside the noise, even though the wider field spreads 198.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (rot_intrinsic_const)

The baseline rot_intrinsic_const is the fastest (1.15 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### rot_manual_shifts's comparison is tie-heavy (12% tied pairs)

12% of paired samples for rot_manual_shifts are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### rot_manual_shifts is inconsistent: worst-20% is 2.1x its best-20%

rot_manual_shifts's best 20% of batches run at 939 ns but its worst 20% at 1.98 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (rot_intrinsic_const) is the fastest** at 1147.8 ns median
- 1 variant significantly slower than baseline
- Spread: 2.99x (fastest 1147.8 ns, slowest 3427.0 ns)

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
| Counter frequency | 2112768874 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rot_intrinsic_const | 3290ns | 3027ns | 2715ns | 3028ns | 4652ns | base |
| rot_manual_shifts | 3296ns | 3024ns | 2473ns | 2913ns | 5268ns | +0.19% |
| rot_variable_amount | 5212ns | 5311ns | 4708ns | 5312ns | 5415ns | +58.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 1245ns | 1032ns | 1752ns | base | 13.156 |
| rot_manual_shifts | 1247ns | 939ns | 1982ns | +0.15% | 13.136 |
| rot_variable_amount | 3362ns | 3039ns | 3484ns | +169.97% | 4.873 |

## Performance model

- Peak throughput: **17.446 Gops/s** (rot_manual_shifts; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 14.274 | 81.8% |
| rot_manual_shifts | 14.274 | 81.8% |
| rot_variable_amount | 4.781 | 27.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 3290ns | 3290ns | base |
| rot_manual_shifts | 3296ns | 3296ns | +0.19% |
| rot_variable_amount | 5212ns | 5212ns | +58.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 1148ns | base | --- | [1148, 1148] | --- | --- | --- | --- |
| rot_manual_shifts | 1148ns | no significant difference | [-0, +0]ns | [1148, 1148] | no (adj: YES) | 0.0222 | 0.0222 | **20** (12%, HIGH) |
| rot_variable_amount | 3427ns | +2279.2ns (+198.6%) | [+2279, +2279]ns | [3427, 3427] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 1148ns | -18.2% | +198.6% |
| 1 | 2 | 939ns | +0.0% | +265.5% |
| 1 | 3 | 1148ns | +0.0% | +198.6% |
| 1 | 4 | 1148ns | -0.0% | +198.5% |
| 1 | 5 | 1148ns | -18.2% | +199.0% |
| 1 | 6 | 1148ns | +0.0% | +198.6% |
| 1 | 7 | 1148ns | +0.0% | +198.6% |
| 1 | 8 | 1148ns | +0.0% | +144.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.160 | ok |
| rot_manual_shifts | 0.059 | ok |
| rot_variable_amount | -0.015 | ok |

**Consistency summary:**

- **rot_manual_shifts**: won 2/8, lost 0/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 25.8ns | 1245.4ns | 2.1% |  |
| rot_manual_shifts | 25.8ns | 1247.3ns | 2.1% |  |
| rot_variable_amount | 23.3ns | 3362.2ns | 0.7% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 1031.9-1751.6 ns)
   1031.9 |
   1067.9 |
   1103.9 |
   1139.9 |########################################
   1175.9 |
   1211.9 |
   1247.8 |
   1283.8 |
   1319.8 |
   1355.8 |
   1391.8 |
   1427.8 |
   1463.8 |
   1499.7 |
   1535.7 |
   1571.7 |
   1607.7 |
   1643.7 |
   1679.7 |
   1715.6 |
  (18 below, 12 above range)

rot_manual_shifts (n=160, range 939.1-1982.5 ns)
    939.1 |#################
    991.3 |
   1043.5 |
   1095.6 |#################
   1147.8 |########################################
   1200.0 |
   1252.1 |
   1304.3 |
   1356.5 |
   1408.6 |
   1460.8 |
   1513.0 |
   1565.1 |
   1617.3 |
   1669.5 |
   1721.6 |
   1773.8 |
   1826.0 |
   1878.1 |
   1930.3 |
  (24 below, 14 above range)

rot_variable_amount (n=160, range 3039.1-3484.4 ns)
   3039.1 |
   3061.4 |
   3083.7 |
   3105.9 |
   3128.2 |
   3150.4 |
   3172.7 |
   3195.0 |
   3217.2 |
   3239.5 |
   3261.8 |
   3284.0 |
   3306.3 |
   3328.6 |
   3350.8 |
   3373.1 |
   3395.4 |
   3417.6 |########################################
   3439.9 |##
   3462.2 |
  (20 below, 1 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=37.2% (high variance, measurements may be unstable)
- **rot_manual_shifts**: CV=44.9% (high variance, measurements may be unstable)
