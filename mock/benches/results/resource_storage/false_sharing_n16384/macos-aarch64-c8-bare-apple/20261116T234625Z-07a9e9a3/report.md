# Cross-core false sharing: slots packed in one line vs one line each

2 variants, 20 samples per variant.
Baseline: **fs_padded**

## Highlights

Baseline for all deltas below: **fs_padded**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fs_padded dominates: 170% faster than the next best (fs_packed)

fs_padded (11.64 ms) leads fs_packed (31.42 ms) by 170%, a clear separation rather than a photo finish. CV 7.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fs_padded is fastest but the noisiest (CV 7.6%)

fs_padded wins on median (11.64 ms) yet has the highest variance (CV 7.6%), while fs_packed is the steadiest (CV 5.1%, 31.42 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fs_padded shows alternating (throttle bounce) (autocorr -0.50)

fs_padded's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fs_padded)

The baseline fs_padded is the fastest (11.64 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fs_padded) is the fastest** at 11643458.3 ns median
- 1 variant significantly slower than baseline
- Spread: 2.70x (fastest 11643458.3 ns, slowest 31419895.8 ns)

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
| fs_packed | 32217069ns | 31420604ns | 30936979ns | 31700615ns | 35046521ns | +185.05% |
| fs_padded | 11302110ns | 11643771ns | 9807104ns | 11529149ns | 12116000ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fs_packed | 32216404ns | 30936458ns | 35045781ns | +185.06% | 0.001 |
| fs_padded | 11301583ns | 9806698ns | 12115146ns | base | 0.001 |

## Performance model

- Peak throughput: **0.002 Gops/s** (fs_padded; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fs_packed | 0.001 | 31.2% |
| fs_padded | 0.001 | 84.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fs_packed | 32217069ns | 32217069ns | +185.05% |
| fs_padded | 11302110ns | 11302110ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fs_padded | 11643458ns | base | --- | [11291271, 11892250] | --- | --- | --- | --- |
| fs_packed | 31419896ns | +19728354.2ns (+169.4%) | [+19399479, +21791854]ns | [31187000, 32029250] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fs_padded | fs_packed |
|---|---|---|---|
| 1 | 1 | 11463312ns | +175.4% |
| 1 | 2 | 11787771ns | +165.6% |

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
| fs_packed | 89.6ns | 32216404.2ns | 0.0% |  |
| fs_padded | 85.4ns | 11301583.3ns | 0.0% |  |

## Distribution (algo ns)

```
fs_packed (n=20, range 30936458.3-35045781.2 ns)
  30936458.3 |########################################
  31141924.5 |##############################
  31347390.6 |##############################
  31552856.8 |##########
  31758322.9 |##########
  31963789.1 |
  32169255.2 |##########
  32374721.3 |
  32580187.5 |
  32785653.6 |
  32991119.8 |
  33196585.9 |
  33402052.1 |
  33607518.2 |
  33812984.4 |
  34018450.5 |
  34223916.7 |####################
  34429382.8 |
  34634849.0 |
  34840315.1 |##########
  (2 below, 2 above range)

fs_padded (n=20, range 9806697.9-12115145.8 ns)
  9806697.9 |
  9922120.3 |
  10037542.7 |########################
  10152965.1 |
  10268387.5 |
  10383809.9 |########
  10499232.3 |
  10614654.7 |
  10730077.1 |
  10845499.5 |
  10960921.9 |
  11076344.3 |
  11191766.7 |########
  11307189.1 |########################
  11422611.5 |
  11538033.9 |########
  11653456.2 |################
  11768878.6 |########
  11884301.0 |########################################
  11999723.4 |########
  (1 below, 1 above range)

```
