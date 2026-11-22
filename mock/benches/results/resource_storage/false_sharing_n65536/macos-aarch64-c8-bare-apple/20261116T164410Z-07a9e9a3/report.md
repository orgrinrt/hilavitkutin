# Cross-core false sharing: slots packed in one line vs one line each

2 variants, 20 samples per variant.
Baseline: **fs_padded**

## Highlights

Baseline for all deltas below: **fs_padded**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fs_padded dominates: 211% faster than the next best (fs_packed)

fs_padded (40.56 ms) leads fs_packed (126.02 ms) by 211%, a clear separation rather than a photo finish. CV 10.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fs_padded is fastest but the noisiest (CV 10.3%)

fs_padded wins on median (40.56 ms) yet has the highest variance (CV 10.3%), while fs_packed is the steadiest (CV 0.3%, 126.02 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fs_padded shows alternating (throttle bounce) (autocorr -0.50)

fs_padded's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fs_padded)

The baseline fs_padded is the fastest (40.56 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.1x the fastest

Fastest fs_padded (40.56 ms) to slowest fs_packed (126.02 ms): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fs_padded) is the fastest** at 40555479.2 ns median
- 1 variant significantly slower than baseline
- Spread: 3.11x (fastest 40555479.2 ns, slowest 126015875.0 ns)

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
| fs_packed | 125944733ns | 126016438ns | 125397562ns | 125961413ns | 126441865ns | +215.94% |
| fs_padded | 39862890ns | 40556083ns | 34053146ns | 39967191ns | 45359729ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fs_packed | 125944085ns | 125397031ns | 126441271ns | +215.95% | 0.001 |
| fs_padded | 39862367ns | 34052562ns | 45359031ns | base | 0.002 |

## Performance model

- Peak throughput: **0.002 Gops/s** (fs_padded; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fs_packed | 0.001 | 27.0% |
| fs_padded | 0.002 | 84.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fs_packed | 125944733ns | 125944733ns | +215.94% |
| fs_padded | 39862890ns | 39862890ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fs_padded | 40555479ns | base | --- | [37275854, 42062667] | --- | --- | --- | --- |
| fs_packed | 126015875ns | +85008750.0ns (+209.6%) | [+83777896, +88557854]ns | [125684771, 126192000] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fs_padded | fs_packed |
|---|---|---|---|
| 1 | 1 | 38432292ns | +227.9% |
| 1 | 2 | 41195750ns | +205.6% |

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
| fs_packed | 85.4ns | 125944085.4ns | 0.0% |  |
| fs_padded | 50.0ns | 39862366.7ns | 0.0% |  |

## Distribution (algo ns)

```
fs_packed (n=20, range 125397031.2-126441270.9 ns)
  125397031.2 |
  125449243.2 |
  125501455.2 |########################################
  125553667.2 |
  125605879.2 |########################################
  125658091.2 |
  125710303.1 |####################
  125762515.1 |####################
  125814727.1 |
  125866939.1 |
  125919151.1 |####################
  125971363.0 |####################
  126023575.0 |########################################
  126075787.0 |####################
  126127999.0 |
  126180211.0 |########################################
  126232422.9 |####################
  126284634.9 |####################
  126336846.9 |########################################
  126389058.9 |
  (2 below, 1 above range)

fs_padded (n=20, range 34052562.5-45359031.3 ns)
  34052562.5 |
  34617885.9 |##########################
  35183209.4 |#############
  35748532.8 |
  36313856.3 |#############
  36879179.7 |##########################
  37444503.1 |#############
  38009826.6 |
  38575150.0 |
  39140473.4 |#############
  39705796.9 |
  40271120.3 |##########################
  40836443.8 |##########################
  41401767.2 |#############
  41967090.6 |
  42532414.1 |#############
  43097737.5 |
  43663061.0 |########################################
  44228384.4 |
  44793707.8 |
  (1 below, 2 above range)

```
