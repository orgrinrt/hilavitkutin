# Cross-core false sharing: slots packed in one line vs one line each

2 variants, 20 samples per variant.
Baseline: **fs_padded**

## Highlights

Baseline for all deltas below: **fs_padded**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fs_padded is fastest but the noisiest (CV 121.8%)

fs_padded wins on median (5.53 ms) yet has the highest variance (CV 121.8%), while fs_packed is the steadiest (CV 91.0%, 7.20 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fs_packed shows alternating (throttle bounce) (autocorr -0.50)

fs_packed's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.68 ms) is smaller than the fastest variant's own run-to-run std-dev (6.73 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fs_padded)

The baseline fs_padded is the fastest (5.53 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fs_padded is inconsistent: worst-20% is 4.4x its best-20%

fs_padded's best 20% of batches run at 4.56 ms but its worst 20% at 20.29 ms (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fs_padded) is the fastest** at 5526260.0 ns median
- Spread: 1.30x (fastest 5526260.0 ns, slowest 7202885.2 ns)

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
| fs_packed | 9671709ns | 7203353ns | 5484331ns | 7458050ns | 20500066ns | +5.26% |
| fs_padded | 9188061ns | 5526904ns | 4561274ns | 7027557ns | 20296361ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fs_packed | 9669573ns | 5483735ns | 20498366ns | +5.26% | 0.002 |
| fs_padded | 9186329ns | 4560639ns | 20292205ns | base | 0.002 |

## Performance model

- Peak throughput: **0.004 Gops/s** (fs_padded; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fs_packed | 0.002 | 63.3% |
| fs_padded | 0.003 | 82.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fs_packed | 9671709ns | 9671709ns | +5.26% |
| fs_padded | 9188061ns | 9188061ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fs_padded | 5526260ns | base | --- | [5113448, 10560201] | --- | --- | --- | --- |
| fs_packed | 7202885ns | no significant difference | [-914832, +2574432]ns | [5977400, 9213693] | no | 0.5034 | 0.5034 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fs_padded | fs_packed |
|---|---|---|---|
| 1 | 1 | 5455383ns | +55.1% |
| 1 | 2 | 5905746ns | +5.9% |

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
| fs_packed | 388.1ns | 9669572.7ns | 0.0% |  |
| fs_padded | 392.1ns | 9186328.8ns | 0.0% |  |

## Distribution (algo ns)

```
fs_packed (n=20, range 5483734.8-20498366.2 ns)
  5483734.8 |########################################
  6234466.3 |###########
  6985197.9 |
  7735929.5 |#################
  8486661.1 |#####
  9237392.6 |#####
  9988124.2 |#####
  10738855.8 |
  11489587.3 |
  12240318.9 |#####
  12991050.5 |
  13741782.1 |
  14492513.6 |
  15243245.2 |#####
  15993976.8 |
  16744708.3 |
  17495439.9 |
  18246171.5 |
  18996903.1 |
  19747634.6 |
  (1 below, 2 above range)

fs_padded (n=20, range 4560639.4-20292205.4 ns)
  4560639.4 |########################################
  5347217.7 |################################
  6133796.0 |################
  6920374.3 |
  7706952.6 |
  8493530.9 |########
  9280109.2 |
  10066687.5 |
  10853265.8 |
  11639844.1 |
  12426422.4 |########
  13213000.7 |########
  13999579.0 |########
  14786157.3 |
  15572735.6 |
  16359313.9 |
  17145892.2 |########
  17932470.5 |
  18719048.8 |########
  19505627.1 |
  (2 below, 1 above range)

```

## Diagnostics

- **fs_packed**: CV=67.8% (high variance, measurements may be unstable)
- **fs_packed**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **fs_padded**: CV=73.3% (high variance, measurements may be unstable)
- **fs_padded**: worst_20/best_20 = 4.4x (possible bimodal distribution)
