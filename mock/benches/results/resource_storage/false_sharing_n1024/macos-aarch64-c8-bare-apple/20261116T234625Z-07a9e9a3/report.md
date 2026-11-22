# Cross-core false sharing: slots packed in one line vs one line each

2 variants, 20 samples per variant.
Baseline: **fs_padded**

## Highlights

Baseline for all deltas below: **fs_padded**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fs_padded dominates: 138% faster than the next best (fs_packed)

fs_padded (844.42 us) leads fs_packed (2.01 ms) by 138%, a clear separation rather than a photo finish. CV 8.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fs_padded shows alternating (throttle bounce) (autocorr -0.50)

fs_padded's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fs_padded)

The baseline fs_padded is the fastest (844.42 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fs_packed is inconsistent: worst-20% is 1.5x its best-20%

fs_packed's best 20% of batches run at 1.98 ms but its worst 20% at 3.00 ms (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fs_padded) is the fastest** at 844416.7 ns median
- 1 variant significantly slower than baseline
- Spread: 2.38x (fastest 844416.7 ns, slowest 2011729.1 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fs_packed | 2264304ns | 2012104ns | 1976760ns | 2114163ns | 3002271ns | +165.94% |
| fs_padded | 851446ns | 844771ns | 749562ns | 853535ns | 947062ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fs_packed | 2263867ns | 1976469ns | 3001604ns | +166.01% | 0.000 |
| fs_padded | 851048ns | 749146ns | 946688ns | base | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (fs_padded; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fs_packed | 0.001 | 37.2% |
| fs_padded | 0.001 | 88.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fs_packed | 2264304ns | 2264304ns | +165.94% |
| fs_padded | 851446ns | 851446ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fs_padded | 844417ns | base | --- | [818062, 896500] | --- | --- | --- | --- |
| fs_packed | 2011729ns | +1207354.1ns (+143.0%) | [+1148458, +1506063]ns | [1983604, 2325042] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fs_padded | fs_packed |
|---|---|---|---|
| 1 | 1 | 899708ns | +180.9% |
| 1 | 2 | 818062ns | +142.1% |

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
| fs_packed | 52.1ns | 2263866.7ns | 0.0% |  |
| fs_padded | 39.6ns | 851047.9ns | 0.0% |  |

## Distribution (algo ns)

```
fs_packed (n=20, range 1976468.8-3001604.2 ns)
  1976468.8 |########################################
  2027725.5 |########
  2078982.3 |
  2130239.1 |
  2181495.8 |
  2232752.6 |####
  2284009.4 |
  2335266.1 |
  2386522.9 |####
  2437779.7 |
  2489036.5 |
  2540293.2 |
  2591550.0 |
  2642806.8 |####
  2694063.5 |########
  2745320.3 |
  2796577.1 |
  2847833.8 |
  2899090.6 |
  2950347.4 |
  (2 below, 2 above range)

fs_padded (n=20, range 749145.8-946687.5 ns)
  749145.8 |
  759022.9 |##########################
  768900.0 |#############
  778777.1 |
  788654.2 |
  798531.2 |#############
  808408.3 |
  818285.4 |#############
  828162.5 |#############
  838039.6 |########################################
  847916.7 |
  857793.7 |
  867670.8 |
  877547.9 |##########################
  887425.0 |#############
  897302.1 |##########################
  907179.2 |#############
  917056.2 |##########################
  926933.3 |
  936810.4 |
  (2 below, 1 above range)

```
