# Cross-core false sharing: slots packed in one line vs one line each

2 variants, 20 samples per variant.
Baseline: **fs_padded**

## Highlights

Baseline for all deltas below: **fs_padded**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fs_padded dominates: 38% faster than the next best (fs_packed)

fs_padded (1.18 ms) leads fs_packed (1.63 ms) by 38%, a clear separation rather than a photo finish. CV 24.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fs_padded shows alternating (throttle bounce) (autocorr -0.50)

fs_padded's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fs_padded)

The baseline fs_padded is the fastest (1.18 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fs_packed is inconsistent: worst-20% is 3.0x its best-20%

fs_packed's best 20% of batches run at 902.85 us but its worst 20% at 2.73 ms (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fs_padded) is the fastest** at 1181099.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.38x (fastest 1181099.5 ns, slowest 1629005.9 ns)

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
| Counter frequency | 2112727760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fs_packed | 1770456ns | 1638983ns | 905760ns | 1738375ns | 2731396ns | +53.74% |
| fs_padded | 1151596ns | 1182525ns | 727379ns | 1179100ns | 1493301ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fs_packed | 1767041ns | 902852ns | 2727779ns | +53.81% | 0.001 |
| fs_padded | 1148872ns | 726251ns | 1488502ns | base | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (fs_padded; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fs_packed | 0.001 | 44.6% |
| fs_padded | 0.001 | 61.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fs_packed | 1770456ns | 1770456ns | +53.74% |
| fs_padded | 1151596ns | 1151596ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fs_padded | 1181099ns | base | --- | [1055102, 1295625] | --- | --- | --- | --- |
| fs_packed | 1629006ns | +710815.4ns (+60.2%) | [+19491, +1196197]ns | [1274205, 2257543] | YES (adj: no) | 0.1153 | 0.1153 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fs_padded | fs_packed |
|---|---|---|---|
| 1 | 1 | 1126011ns | +117.5% |
| 1 | 2 | 1273750ns | -4.0% |

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
| fs_packed | 689.5ns | 1767041.3ns | 0.0% |  |
| fs_padded | 398.6ns | 1148872.3ns | 0.0% |  |

## Distribution (algo ns)

```
fs_packed (n=20, range 902852.0-2727778.9 ns)
  902852.0 |
  994098.3 |
  1085344.7 |#############
  1176591.0 |########################################
  1267837.4 |##########################
  1359083.7 |
  1450330.1 |#############
  1541576.4 |#############
  1632822.7 |#############
  1724069.1 |
  1815315.4 |
  1906561.8 |#############
  1997808.1 |#############
  2089054.5 |#############
  2180300.8 |
  2271547.2 |
  2362793.5 |#############
  2454039.9 |##########################
  2545286.2 |
  2636532.6 |#############
  (2 below, 2 above range)

fs_padded (n=20, range 726250.7-1488501.7 ns)
  726250.7 |
  764363.2 |
  802475.8 |
  840588.3 |
  878700.9 |
  916813.4 |
  954926.0 |####################
  993038.5 |########################################
  1031151.1 |########################################
  1069263.6 |####################
  1107376.2 |
  1145488.7 |########################################
  1183601.3 |####################
  1221713.8 |########################################
  1259826.4 |####################
  1297938.9 |########################################
  1336051.5 |
  1374164.0 |####################
  1412276.6 |
  1450389.1 |########################################
  (2 below, 1 above range)

```

## Diagnostics

- **fs_packed**: CV=38.6% (high variance, measurements may be unstable)
- **fs_packed**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **fs_padded**: CV=24.7% (high variance, measurements may be unstable)
