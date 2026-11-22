# Cross-core false sharing: slots packed in one line vs one line each

2 variants, 20 samples per variant.
Baseline: **fs_padded**

## Highlights

Baseline for all deltas below: **fs_padded**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fs_padded dominates: 156% faster than the next best (fs_packed)

fs_padded (775.75 us) leads fs_packed (1.98 ms) by 156%, a clear separation rather than a photo finish. CV 5.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fs_padded is fastest but the noisiest (CV 5.6%)

fs_padded wins on median (775.75 us) yet has the highest variance (CV 5.6%), while fs_packed is the steadiest (CV 2.8%, 1.98 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fs_padded shows alternating (throttle bounce) (autocorr -0.50)

fs_padded's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fs_padded)

The baseline fs_padded is the fastest (775.75 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fs_padded) is the fastest** at 775750.0 ns median
- 1 variant significantly slower than baseline
- Spread: 2.56x (fastest 775750.0 ns, slowest 1983312.5 ns)

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
| fs_packed | 1995069ns | 1983583ns | 1964052ns | 1982205ns | 2064677ns | +155.90% |
| fs_padded | 779633ns | 776125ns | 711479ns | 782767ns | 838385ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fs_packed | 1994775ns | 1963729ns | 2064396ns | +155.98% | 0.001 |
| fs_padded | 779281ns | 711115ns | 838052ns | base | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (fs_padded; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fs_packed | 0.001 | 35.9% |
| fs_padded | 0.001 | 91.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fs_packed | 1995069ns | 1995069ns | +155.90% |
| fs_padded | 779633ns | 779633ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fs_padded | 775750ns | base | --- | [771771, 803792] | --- | --- | --- | --- |
| fs_packed | 1983312ns | +1199520.8ns (+154.6%) | [+1183958, +1241646]ns | [1972125, 1990438] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fs_padded | fs_packed |
|---|---|---|---|
| 1 | 1 | 777125ns | +155.7% |
| 1 | 2 | 773854ns | +155.5% |

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
| fs_packed | 31.3ns | 1994775.0ns | 0.0% |  |
| fs_padded | 37.5ns | 779281.3ns | 0.0% |  |

## Distribution (algo ns)

```
fs_packed (n=20, range 1963729.2-2064395.8 ns)
  1963729.2 |########################################
  1968762.5 |########################################
  1973795.8 |##########################
  1978829.2 |#############
  1983862.5 |########################################
  1988895.8 |########################################
  1993929.2 |#############
  1998962.5 |
  2003995.8 |
  2009029.2 |#############
  2014062.5 |
  2019095.8 |
  2024129.2 |#############
  2029162.5 |
  2034195.8 |
  2039229.2 |
  2044262.5 |
  2049295.8 |
  2054329.2 |
  2059362.5 |
  (1 below, 1 above range)

fs_padded (n=20, range 711114.6-838052.1 ns)
  711114.6 |########
  717461.5 |
  723808.3 |
  730155.2 |########
  736502.1 |########
  742849.0 |
  749195.8 |
  755542.7 |
  761889.6 |
  768236.5 |################
  774583.3 |########################################
  780930.2 |
  787277.1 |
  793624.0 |
  799970.8 |########################
  806317.7 |
  812664.6 |########
  819011.5 |
  825358.3 |################
  831705.2 |########
  (2 below, 1 above range)

```
