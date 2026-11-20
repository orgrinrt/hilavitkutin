# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (memcpy_intrinsic) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline memcpy_intrinsic has the worst median (1.23 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest memcpy_unrolled at 1.23 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 1.23 us to slowest 1.23 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### memcpy_naive shows alternating (throttle bounce) (autocorr -0.56)

memcpy_naive's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (473 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader memcpy_unrolled vs stability leader memcpy_intrinsic (+0% speed for 1.2x steadier)

memcpy_unrolled is fastest (1.23 us, CV 38.4%); memcpy_intrinsic gives up 0.1% median for 1.2x lower variance (CV 32.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### memcpy_naive is inconsistent: worst-20% is 1.8x its best-20%

memcpy_naive's best 20% of batches run at 1.01 us but its worst 20% at 1.83 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: memcpy_unrolled** at 1232.5 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 1232.5 ns, slowest 1233.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4646464650504040 |
| Counter frequency | 2112829592 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 3149ns | 3118ns | 2551ns | 2993ns | 4213ns | base |
| memcpy_naive | 3237ns | 3116ns | 2552ns | 2983ns | 4683ns | +2.80% |
| memcpy_unrolled | 3237ns | 3117ns | 2550ns | 2997ns | 4644ns | +2.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 1243ns | 1009ns | 1657ns | base | 0.824 |
| memcpy_naive | 1276ns | 1009ns | 1835ns | +2.69% | 0.802 |
| memcpy_unrolled | 1276ns | 1009ns | 1818ns | +2.66% | 0.802 |

## Performance model

- Peak throughput: **1.015 Gops/s** (memcpy_naive; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.830 | 81.8% |
| memcpy_naive | 0.830 | 81.8% |
| memcpy_unrolled | 0.831 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 3149ns | 3149ns | base |
| memcpy_naive | 3237ns | 3237ns | +2.80% |
| memcpy_unrolled | 3237ns | 3237ns | +2.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 1233ns | base | --- | [1233, 1234] | --- | --- | --- | --- |
| memcpy_naive | 1233ns | no significant difference | [-2, +1]ns | [1233, 1233] | no | 0.5233 | 0.2668 | 1 |
| memcpy_unrolled | 1232ns | no significant difference | [-3, +2]ns | [1231, 1233] | no | 0.5233 | 0.5233 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 1010ns | +22.1% | +22.1% |
| 1 | 2 | 1010ns | +22.2% | +22.2% |
| 1 | 3 | 1234ns | -0.0% | -18.2% |
| 1 | 4 | 1234ns | -18.2% | +0.0% |
| 1 | 5 | 1234ns | -0.0% | -18.2% |
| 1 | 6 | 1234ns | -18.2% | -18.2% |
| 1 | 7 | 1009ns | +26.9% | +22.3% |
| 1 | 8 | 1234ns | -18.3% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | 0.058 | ok |
| memcpy_naive | -0.564 | HIGH- (thermal bounce) |
| memcpy_unrolled | -0.148 | ok |

**Consistency summary:**

- **memcpy_naive**: won 3/8, lost 3/8
- **memcpy_unrolled**: won 4/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 25.1ns | 1243.0ns | 2.0% |  |
| memcpy_naive | 26.0ns | 1276.4ns | 2.0% |  |
| memcpy_unrolled | 25.7ns | 1276.0ns | 2.0% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 1009.1-1656.9 ns)
   1009.1 |####################
   1041.5 |
   1073.9 |
   1106.3 |
   1138.7 |
   1171.1 |
   1203.5 |########################################
   1235.9 |######
   1268.3 |
   1300.6 |
   1333.0 |
   1365.4 |
   1397.8 |
   1430.2 |
   1462.6 |
   1495.0 |
   1527.4 |
   1559.8 |
   1592.2 |
   1624.5 |
  (11 below, 8 above range)

memcpy_naive (n=160, range 1008.9-1834.7 ns)
   1008.9 |####################
   1050.1 |
   1091.4 |
   1132.7 |
   1174.0 |
   1215.3 |########################################
   1256.6 |#
   1297.9 |##
   1339.2 |
   1380.5 |
   1421.8 |
   1463.1 |
   1504.4 |
   1545.7 |
   1587.0 |
   1628.3 |
   1669.6 |
   1710.8 |
   1752.1 |
   1793.4 |
  (12 below, 9 above range)

memcpy_unrolled (n=160, range 1009.0-1817.6 ns)
   1009.0 |##################
   1049.4 |
   1089.8 |
   1130.3 |
   1170.7 |
   1211.1 |########################################
   1251.6 |
   1292.0 |
   1332.4 |
   1372.8 |
   1413.3 |
   1453.7 |
   1494.1 |
   1534.6 |
   1575.0 |
   1615.4 |
   1655.9 |
   1696.3 |
   1736.7 |
   1777.1 |
  (10 below, 11 above range)

```

## Diagnostics

- **memcpy_intrinsic**: CV=32.3% (high variance, measurements may be unstable)
- **memcpy_naive**: CV=38.9% (high variance, measurements may be unstable)
- **memcpy_unrolled**: CV=37.1% (high variance, measurements may be unstable)
