# Cross-core false sharing: slots packed in one line vs one line each

2 variants, 20 samples per variant.
Baseline: **fs_padded**

## Highlights

Baseline for all deltas below: **fs_padded**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fs_padded dominates: 27% faster than the next best (fs_packed)

fs_padded (18.44 ms) leads fs_packed (23.43 ms) by 27%, a clear separation rather than a photo finish. CV 18.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fs_padded shows alternating (throttle bounce) (autocorr -0.50)

fs_padded's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fs_padded)

The baseline fs_padded is the fastest (18.44 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fs_packed is inconsistent: worst-20% is 1.6x its best-20%

fs_packed's best 20% of batches run at 22.10 ms but its worst 20% at 34.51 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fs_padded) is the fastest** at 18437418.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.27x (fastest 18437418.3 ns, slowest 23432877.9 ns)

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
| fs_packed | 25972372ns | 23433383ns | 22105603ns | 24415891ns | 34508584ns | +35.24% |
| fs_padded | 19204728ns | 18437974ns | 17178948ns | 18458991ns | 23467718ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fs_packed | 25971012ns | 22104706ns | 34505980ns | +35.24% | 0.003 |
| fs_padded | 19203441ns | 17177268ns | 23464994ns | base | 0.003 |

## Performance model

- Peak throughput: **0.004 Gops/s** (fs_padded; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fs_packed | 0.003 | 73.3% |
| fs_padded | 0.004 | 93.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fs_packed | 25972372ns | 25972372ns | +35.24% |
| fs_padded | 19204728ns | 19204728ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fs_padded | 18437418ns | base | --- | [17709031, 19009553] | --- | --- | --- | --- |
| fs_packed | 23432878ns | +5355964.6ns (+29.0%) | [+4312685, +8192483]ns | [23069894, 26658656] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fs_padded | fs_packed |
|---|---|---|---|
| 1 | 1 | 17546134ns | +32.5% |
| 1 | 2 | 18968136ns | +31.0% |

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
| fs_packed | 242.6ns | 25971012.3ns | 0.0% |  |
| fs_padded | 195.0ns | 19203440.8ns | 0.0% |  |

## Distribution (algo ns)

```
fs_packed (n=20, range 22104706.4-34505979.5 ns)
  22104706.4 |##############################
  22724770.0 |########################################
  23344833.7 |####################
  23964897.3 |##########
  24584961.0 |
  25205024.6 |##########
  25825088.3 |##########
  26445152.0 |
  27065215.6 |####################
  27685279.3 |##########
  28305342.9 |##########
  28925406.6 |
  29545470.3 |
  30165533.9 |
  30785597.6 |
  31405661.2 |
  32025724.9 |
  32645788.6 |
  33265852.2 |
  33885915.9 |
  (2 below, 2 above range)

fs_padded (n=20, range 17177268.0-23464994.4 ns)
  17177268.0 |########################################
  17491654.3 |##########################
  17806040.6 |
  18120426.9 |########################################
  18434813.3 |#############
  18749199.6 |########################################
  19063585.9 |#############
  19377972.2 |
  19692358.6 |#############
  20006744.9 |#############
  20321131.2 |##########################
  20635517.5 |
  20949903.8 |
  21264290.2 |
  21578676.5 |
  21893062.8 |
  22207449.1 |
  22521835.5 |
  22836221.8 |
  23150608.1 |
  (2 below, 1 above range)

```

## Diagnostics

- **fs_packed**: CV=21.7% (high variance, measurements may be unstable)
