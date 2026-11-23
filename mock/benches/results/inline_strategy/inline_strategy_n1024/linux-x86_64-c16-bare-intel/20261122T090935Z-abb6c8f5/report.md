# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### inline_default dominates: 22% faster than the next best (inline_always)

inline_default (939 ns) leads inline_always (1.15 us) by 22%, a clear separation rather than a photo finish. CV 56.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inline_default is fastest but the noisiest (CV 56.2%)

inline_default wins on median (939 ns) yet has the highest variance (CV 56.2%), while inline_never is the steadiest (CV 42.2%, 1.15 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (212 ns) is smaller than the fastest variant's own run-to-run std-dev (528 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### inline_always is inconsistent: worst-20% is 2.1x its best-20%

inline_always's best 20% of batches run at 939 ns but its worst 20% at 1.93 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### inline_never's edge over baseline is significant but tiny (4 ns, 0.32%)

inline_never differs from baseline inline_always by 4 ns (0.32%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: inline_default** at 939.2 ns median (-18.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.23x (fastest 939.2 ns, slowest 1151.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x39f847b2c651de80 |
| Counter frequency | 2112764099 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 3324ns | 3027ns | 2476ns | 3005ns | 5128ns | base |
| inline_default | 3040ns | 2487ns | 2473ns | 2681ns | 4685ns | -8.55% |
| inline_never | 3368ns | 3040ns | 3032ns | 3042ns | 4684ns | +1.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 1258ns | 939ns | 1932ns | base | 0.814 |
| inline_default | 1150ns | 939ns | 1765ns | -8.53% | 0.890 |
| inline_never | 1274ns | 1150ns | 1765ns | +1.28% | 0.804 |

## Performance model

- Peak throughput: **1.091 Gops/s** (inline_default; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.892 | 81.8% |
| inline_default | 1.090 | 100.0% |
| inline_never | 0.889 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 3324ns | 3324ns | base |
| inline_default | 3040ns | 3040ns | -8.55% |
| inline_never | 3368ns | 3368ns | +1.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 1148ns | base | --- | [1148, 1148] | --- | --- | --- | --- |
| inline_default | 939ns | -192.0ns (-16.7%) | [-208, -0]ns | [939, 956] | YES | 0.0000 | 0.0000 | 9 |
| inline_never | 1151ns | +3.7ns (+0.3%) | [+4, +4]ns | [1151, 1151] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 1148ns | +0.0% | +0.3% |
| 1 | 2 | 1148ns | -18.2% | +0.3% |
| 1 | 3 | 1148ns | -18.2% | +0.3% |
| 1 | 4 | 939ns | +22.2% | +22.6% |
| 1 | 5 | 1148ns | -18.2% | +0.3% |
| 1 | 6 | 939ns | +0.0% | +22.6% |
| 1 | 7 | 1148ns | -18.2% | +0.3% |
| 1 | 8 | 1148ns | +0.0% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.375 | moderate- |
| inline_default | -0.275 | moderate- |
| inline_never | -0.026 | ok |

**Consistency summary:**

- **inline_default**: won 4/8, lost 1/8
- **inline_never**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 26.2ns | 1257.7ns | 2.1% |  |
| inline_default | 23.9ns | 1150.5ns | 2.1% |  |
| inline_never | 26.7ns | 1273.8ns | 2.1% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 939.1-1931.7 ns)
    939.1 |########
    988.7 |
   1038.4 |
   1088.0 |
   1137.6 |########################################
   1187.2 |
   1236.9 |
   1286.5 |
   1336.1 |
   1385.8 |
   1435.4 |
   1485.0 |
   1534.7 |
   1584.3 |
   1633.9 |
   1683.6 |
   1733.2 |
   1782.8 |
   1832.4 |
   1882.1 |
  (13 below, 12 above range)

inline_default (n=160, range 938.9-1764.6 ns)
    938.9 |########################################
    980.2 |
   1021.5 |
   1062.8 |
   1104.1 |
   1145.4 |############################
   1186.6 |
   1227.9 |
   1269.2 |
   1310.5 |
   1351.8 |
   1393.0 |
   1434.3 |
   1475.6 |
   1516.9 |
   1558.2 |
   1599.4 |
   1640.7 |
   1682.0 |
   1723.3 |
  (15 below, 10 above range)

inline_never (n=160, range 1150.4-1764.6 ns)
   1150.4 |########################################
   1181.1 |
   1211.8 |
   1242.6 |
   1273.3 |
   1304.0 |
   1334.7 |
   1365.4 |
   1396.1 |
   1426.8 |
   1457.5 |
   1488.2 |
   1519.0 |
   1549.7 |
   1580.4 |
   1611.1 |
   1641.8 |
   1672.5 |
   1703.2 |
   1733.9 |
  (7 below, 9 above range)

```

## Diagnostics

- **inline_always**: CV=44.1% (high variance, measurements may be unstable)
- **inline_default**: CV=45.9% (high variance, measurements may be unstable)
- **inline_never**: CV=38.2% (high variance, measurements may be unstable)
