# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_manual_shifts dominates: 22% faster than the next best (rot_intrinsic_const)

rot_manual_shifts (939 ns) leads rot_intrinsic_const (1.15 us) by 22%, a clear separation rather than a photo finish. CV 54.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rot_variable_amount is an outlier: 3.0x slower than the field

rot_variable_amount (2.80 us) is 3.0x the fastest (939 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_manual_shifts is fastest but the noisiest (CV 54.2%)

rot_manual_shifts wins on median (939 ns) yet has the highest variance (CV 54.2%), while rot_variable_amount is the steadiest (CV 10.3%, 2.80 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### rot_intrinsic_const is inconsistent: worst-20% is 2.0x its best-20%

rot_intrinsic_const's best 20% of batches run at 939 ns but its worst 20% at 1.83 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: rot_manual_shifts** at 939.2 ns median (-18.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.99x (fastest 939.2 ns, slowest 2804.0 ns)

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
| Counter frequency | 2112804954 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rot_intrinsic_const | 3223ns | 3032ns | 2481ns | 2921ns | 4873ns | base |
| rot_manual_shifts | 3009ns | 2485ns | 2479ns | 2593ns | 4788ns | -6.65% |
| rot_variable_amount | 4502ns | 4352ns | 4342ns | 4352ns | 5113ns | +39.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 1217ns | 939ns | 1831ns | base | 13.463 |
| rot_manual_shifts | 1136ns | 939ns | 1798ns | -6.64% | 14.421 |
| rot_variable_amount | 2901ns | 2804ns | 3284ns | +138.41% | 5.647 |

## Performance model

- Peak throughput: **17.448 Gops/s** (rot_manual_shifts; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 14.274 | 81.8% |
| rot_manual_shifts | 17.445 | 100.0% |
| rot_variable_amount | 5.843 | 33.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 3223ns | 3223ns | base |
| rot_manual_shifts | 3009ns | 3009ns | -6.65% |
| rot_variable_amount | 4502ns | 4502ns | +39.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 1148ns | base | --- | [1148, 1148] | --- | --- | --- | --- |
| rot_manual_shifts | 939ns | -135.9ns (-11.8%) | [-208, -0]ns | [939, 939] | YES | 0.0002 | 0.0002 | 12 |
| rot_variable_amount | 2804ns | +1656.2ns (+144.3%) | [+1656, +1663]ns | [2804, 2804] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 1148ns | -18.2% | +144.3% |
| 1 | 2 | 1148ns | -18.2% | +144.3% |
| 1 | 3 | 1158ns | -18.9% | +142.2% |
| 1 | 4 | 939ns | +0.0% | +198.6% |
| 1 | 5 | 939ns | +22.2% | +265.0% |
| 1 | 6 | 1148ns | -18.2% | +144.3% |
| 1 | 7 | 939ns | +0.0% | +198.6% |
| 1 | 8 | 1148ns | -0.0% | +144.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.143 | ok |
| rot_manual_shifts | -0.208 | moderate- |
| rot_variable_amount | -0.161 | ok |

**Consistency summary:**

- **rot_manual_shifts**: won 4/8, lost 1/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 25.2ns | 1216.9ns | 2.1% |  |
| rot_manual_shifts | 23.6ns | 1136.1ns | 2.1% |  |
| rot_variable_amount | 20.2ns | 2901.3ns | 0.7% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 939.1-1831.4 ns)
    939.1 |##################
    983.7 |
   1028.3 |
   1072.9 |
   1117.5 |########################################
   1162.1 |#
   1206.8 |
   1251.4 |#
   1296.0 |
   1340.6 |
   1385.2 |
   1429.8 |
   1474.5 |
   1519.1 |
   1563.7 |
   1608.3 |
   1652.9 |
   1697.6 |
   1742.2 |
   1786.8 |
  (13 below, 10 above range)

rot_manual_shifts (n=160, range 939.0-1797.7 ns)
    939.0 |########################################
    982.0 |
   1024.9 |
   1067.8 |
   1110.8 |###############
   1153.7 |#
   1196.6 |
   1239.6 |
   1282.5 |
   1325.4 |
   1368.3 |
   1411.3 |
   1454.2 |
   1497.1 |
   1540.1 |
   1583.0 |
   1625.9 |
   1668.9 |
   1711.8 |
   1754.7 |
  (21 below, 13 above range)

rot_variable_amount (n=160, range 2803.8-3283.9 ns)
   2803.8 |########################################
   2827.8 |
   2851.8 |
   2875.8 |
   2899.8 |
   2923.8 |
   2947.8 |
   2971.8 |
   2995.8 |
   3019.8 |
   3043.8 |
   3067.8 |
   3091.8 |
   3115.9 |
   3139.9 |
   3163.9 |
   3187.9 |
   3211.9 |
   3235.9 |
   3259.9 |
  (9 below, 21 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=42.1% (high variance, measurements may be unstable)
- **rot_manual_shifts**: CV=44.8% (high variance, measurements may be unstable)
