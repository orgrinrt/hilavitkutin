# Cross-core false sharing: slots packed in one line vs one line each

2 variants, 20 samples per variant.
Baseline: **fs_padded**

## Highlights

Baseline for all deltas below: **fs_padded**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fs_padded dominates: 34% faster than the next best (fs_packed)

fs_padded (1.21 ms) leads fs_packed (1.63 ms) by 34%, a clear separation rather than a photo finish. CV 16.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fs_padded shows alternating (throttle bounce) (autocorr -0.50)

fs_padded's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fs_padded)

The baseline fs_padded is the fastest (1.21 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fs_packed is inconsistent: worst-20% is 5.0x its best-20%

fs_packed's best 20% of batches run at 479.25 us but its worst 20% at 2.39 ms (5.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fs_padded) is the fastest** at 1212949.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.34x (fastest 1212949.6 ns, slowest 1629212.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Cross-core false sharing: slots packed in one line vs one line each |
| Passes | 2 |
| Runs per pass | 10 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef1 |
| Counter frequency | 2112784724 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fs_packed | 1532512ns | 1630314ns | 479715ns | 1597480ns | 2390405ns | +31.19% |
| fs_padded | 1168188ns | 1214904ns | 857879ns | 1199818ns | 1383607ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fs_packed | 1530054ns | 479252ns | 2386097ns | +31.27% | 0.001 |
| fs_padded | 1165594ns | 854871ns | 1379081ns | base | 0.001 |

## Performance model

- Peak throughput: **0.002 Gops/s** (fs_packed; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fs_packed | 0.001 | 29.4% |
| fs_padded | 0.001 | 39.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fs_packed | 1532512ns | 1532512ns | +31.19% |
| fs_padded | 1168188ns | 1168188ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fs_padded | 1212950ns | base | --- | [1128618, 1263758] | --- | --- | --- | --- |
| fs_packed | 1629212ns | +434478.9ns (+35.8%) | [+265004, +700456]ns | [1430057, 1766205] | YES | 0.0414 | 0.0414 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fs_padded | fs_packed |
|---|---|---|---|
| 1 | 1 | 1301036ns | -5.7% |
| 1 | 2 | 1128618ns | +67.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fs_packed | -0.500 | moderate- |
| fs_padded | -0.500 | moderate- |

**Consistency summary:**

- **fs_packed**: won 1/2, lost 1/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fs_packed | 301.7ns | 1530053.8ns | 0.0% |  |
| fs_padded | 363.2ns | 1165594.1ns | 0.0% |  |

## Distribution (algo ns)

```
fs_packed (n=20, range 479252.1-2386097.3 ns)
  479252.1 |##########################
  574594.4 |
  669936.6 |
  765278.9 |
  860621.2 |
  955963.4 |#############
  1051305.7 |
  1146647.9 |
  1241990.2 |
  1337332.5 |#############
  1432674.7 |#############
  1528017.0 |##########################
  1623359.2 |########################################
  1718701.5 |########################################
  1814043.8 |
  1909386.0 |#############
  2004728.3 |
  2100070.5 |#############
  2195412.8 |#############
  2290755.1 |#############
  (2 below, 1 above range)

fs_padded (n=20, range 854871.0-1379081.0 ns)
  854871.0 |
  881081.5 |
  907292.0 |
  933502.5 |##########
  959713.0 |
  985923.5 |####################
  1012134.0 |##########
  1038344.5 |
  1064555.0 |
  1090765.5 |
  1116976.0 |####################
  1143186.5 |##########
  1169397.0 |
  1195607.5 |########################################
  1221818.0 |####################
  1248028.5 |
  1274239.0 |##########
  1300449.5 |##########
  1326660.0 |####################
  1352870.5 |
  (1 below, 2 above range)

```

## Diagnostics

- **fs_packed**: CV=42.3% (high variance, measurements may be unstable)
- **fs_packed**: worst_20/best_20 = 5.0x (possible bimodal distribution)
