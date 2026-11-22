# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### inline_default dominates: 22% faster than the next best (inline_always)

inline_default (939 ns) leads inline_always (1.15 us) by 22%, a clear separation rather than a photo finish. CV 57.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inline_default is fastest but the noisiest (CV 57.6%)

inline_default wins on median (939 ns) yet has the highest variance (CV 57.6%), while inline_always is the steadiest (CV 39.2%, 1.15 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (212 ns) is smaller than the fastest variant's own run-to-run std-dev (541 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### inline_default's comparison is tie-heavy (21% tied pairs)

21% of paired samples for inline_default are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### inline_default is inconsistent: worst-20% is 2.0x its best-20%

inline_default's best 20% of batches run at 939 ns but its worst 20% at 1.89 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### inline_never's edge over baseline is significant but tiny (4 ns, 0.33%)

inline_never differs from baseline inline_always by 4 ns (0.33%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: inline_default** at 939.1 ns median (-18.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.23x (fastest 939.1 ns, slowest 1151.0 ns)

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
| Counter frequency | 2112795803 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 3078ns | 3030ns | 2480ns | 2804ns | 4500ns | base |
| inline_default | 3125ns | 2493ns | 2480ns | 2703ns | 5034ns | +1.51% |
| inline_never | 3267ns | 3036ns | 2485ns | 3015ns | 4805ns | +6.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 1161ns | 939ns | 1687ns | base | 0.882 |
| inline_default | 1178ns | 939ns | 1885ns | +1.45% | 0.869 |
| inline_never | 1235ns | 942ns | 1804ns | +6.32% | 0.829 |

## Performance model

- Peak throughput: **1.091 Gops/s** (inline_always; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.892 | 81.8% |
| inline_default | 1.090 | 100.0% |
| inline_never | 0.890 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 3078ns | 3078ns | base |
| inline_default | 3125ns | 3125ns | +1.51% |
| inline_never | 3267ns | 3267ns | +6.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 1148ns | base | --- | [948, 1148] | --- | --- | --- | --- |
| inline_default | 939ns | no significant difference | [-0, +0]ns | [939, 1148] | no | 0.1296 | 0.1296 | **34** (21%, HIGH) |
| inline_never | 1151ns | +3.8ns (+0.3%) | [+4, +18]ns | [1151, 1151] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 1148ns | +0.0% | +0.3% |
| 1 | 2 | 1148ns | -18.2% | +0.3% |
| 1 | 3 | 939ns | +0.0% | +22.6% |
| 1 | 4 | 939ns | +0.0% | +22.6% |
| 1 | 5 | 939ns | +0.0% | +0.3% |
| 1 | 6 | 1148ns | -0.0% | -17.9% |
| 1 | 7 | 939ns | +0.0% | +22.6% |
| 1 | 8 | 1148ns | +0.0% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.125 | ok |
| inline_default | -0.275 | moderate- |
| inline_never | 0.291 | moderate+ |

**Consistency summary:**

- **inline_default**: won 1/8, lost 0/8
- **inline_never**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 24.2ns | 1161.4ns | 2.1% |  |
| inline_default | 24.6ns | 1178.3ns | 2.1% |  |
| inline_never | 25.9ns | 1234.8ns | 2.1% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 938.9-1687.2 ns)
    938.9 |####################################
    976.3 |
   1013.7 |
   1051.1 |
   1088.6 |
   1126.0 |########################################
   1163.4 |
   1200.8 |
   1238.2 |
   1275.6 |
   1313.1 |
   1350.5 |
   1387.9 |
   1425.3 |
   1462.7 |
   1500.1 |
   1537.6 |
   1575.0 |
   1612.4 |
   1649.8 |
  (6 below, 10 above range)

inline_default (n=160, range 938.9-1885.3 ns)
    938.9 |########################################
    986.2 |
   1033.5 |
   1080.9 |
   1128.2 |##################################
   1175.5 |
   1222.8 |
   1270.1 |
   1317.5 |
   1364.8 |
   1412.1 |
   1459.4 |
   1506.7 |
   1554.1 |
   1601.4 |
   1648.7 |
   1696.0 |
   1743.3 |
   1790.7 |
   1838.0 |
  (27 below, 13 above range)

inline_never (n=160, range 941.8-1804.3 ns)
    941.8 |########
    985.0 |
   1028.1 |
   1071.2 |
   1114.3 |########################################
   1157.5 |##
   1200.6 |
   1243.7 |
   1286.8 |
   1329.9 |
   1373.1 |
   1416.2 |
   1459.3 |
   1502.4 |
   1545.6 |
   1588.7 |
   1631.8 |
   1674.9 |
   1718.0 |
   1761.2 |
  (14 below, 10 above range)

```

## Diagnostics

- **inline_always**: CV=38.7% (high variance, measurements may be unstable)
- **inline_default**: CV=45.9% (high variance, measurements may be unstable)
- **inline_never**: CV=41.0% (high variance, measurements may be unstable)
