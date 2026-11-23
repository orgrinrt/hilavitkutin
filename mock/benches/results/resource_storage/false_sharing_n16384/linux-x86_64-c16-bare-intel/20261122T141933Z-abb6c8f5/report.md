# Cross-core false sharing: slots packed in one line vs one line each

2 variants, 20 samples per variant.
Baseline: **fs_padded**

## Highlights

Baseline for all deltas below: **fs_padded**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fs_padded shows alternating (throttle bounce) (autocorr -0.50)

fs_padded's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.28 ms) is smaller than the fastest variant's own run-to-run std-dev (6.84 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fs_padded)

The baseline fs_padded is the fastest (5.51 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fs_packed is inconsistent: worst-20% is 5.1x its best-20%

fs_packed's best 20% of batches run at 5.53 ms but its worst 20% at 28.01 ms (5.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fs_padded) is the fastest** at 5509521.2 ns median
- Spread: 1.23x (fastest 5509521.2 ns, slowest 6787824.4 ns)

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
| fs_packed | 11130376ns | 6788877ns | 5529426ns | 7371173ns | 28008936ns | +34.43% |
| fs_padded | 8279523ns | 5512652ns | 4569377ns | 5836541ns | 19318614ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fs_packed | 11128710ns | 5528665ns | 28005819ns | +34.44% | 0.001 |
| fs_padded | 8277925ns | 4567995ns | 19315102ns | base | 0.002 |

## Performance model

- Peak throughput: **0.004 Gops/s** (fs_padded; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fs_packed | 0.002 | 67.3% |
| fs_padded | 0.003 | 82.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fs_packed | 11130376ns | 11130376ns | +34.43% |
| fs_padded | 8279523ns | 8279523ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fs_padded | 5509521ns | base | --- | [4867912, 7024356] | --- | --- | --- | --- |
| fs_packed | 6787824ns | no significant difference | [-364868, +2544935]ns | [6381287, 9011045] | no | 0.2632 | 0.2632 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fs_padded | fs_packed |
|---|---|---|---|
| 1 | 1 | 5612426ns | +13.7% |
| 1 | 2 | 5392894ns | +67.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fs_packed | -0.500 | moderate- |
| fs_padded | -0.500 | moderate- |

**Consistency summary:**

- **fs_packed**: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fs_packed | 312.2ns | 11128710.0ns | 0.0% |  |
| fs_padded | 253.0ns | 8277925.5ns | 0.0% |  |

## Distribution (algo ns)

```
fs_packed (n=20, range 5528664.8-28005819.0 ns)
  5528664.8 |########################################
  6652522.5 |#################
  7776380.2 |###########
  8900237.9 |#####
  10024095.6 |#####
  11147953.3 |
  12271811.0 |#####
  13395668.7 |
  14519526.4 |
  15643384.2 |
  16767241.9 |
  17891099.6 |
  19014957.3 |
  20138815.0 |
  21262672.7 |
  22386530.4 |
  23510388.1 |
  24634245.9 |
  25758103.6 |#####
  26881961.3 |
  (2 below, 2 above range)

fs_padded (n=20, range 4567994.7-19315102.3 ns)
  4567994.7 |#################################
  5305350.1 |########################################
  6042705.5 |######
  6780060.9 |######
  7517416.2 |
  8254771.6 |######
  8992127.0 |
  9729482.4 |######
  10466837.8 |
  11204193.1 |
  11941548.5 |######
  12678903.9 |
  13416259.3 |
  14153614.6 |
  14890970.0 |
  15628325.4 |
  16365680.8 |
  17103036.2 |
  17840391.5 |
  18577746.9 |
  (2 below, 2 above range)

```

## Diagnostics

- **fs_packed**: CV=87.3% (high variance, measurements may be unstable)
- **fs_packed**: worst_20/best_20 = 5.1x (possible bimodal distribution)
- **fs_padded**: CV=82.7% (high variance, measurements may be unstable)
- **fs_padded**: worst_20/best_20 = 4.2x (possible bimodal distribution)
