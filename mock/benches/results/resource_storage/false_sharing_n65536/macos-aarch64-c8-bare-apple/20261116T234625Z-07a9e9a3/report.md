# Cross-core false sharing: slots packed in one line vs one line each

2 variants, 20 samples per variant.
Baseline: **fs_padded**

## Highlights

Baseline for all deltas below: **fs_padded**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fs_padded dominates: 202% faster than the next best (fs_packed)

fs_padded (41.28 ms) leads fs_packed (124.61 ms) by 202%, a clear separation rather than a photo finish. CV 7.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fs_padded is fastest but the noisiest (CV 7.1%)

fs_padded wins on median (41.28 ms) yet has the highest variance (CV 7.1%), while fs_packed is the steadiest (CV 2.5%, 124.61 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fs_padded shows alternating (throttle bounce) (autocorr -0.50)

fs_padded's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fs_padded)

The baseline fs_padded is the fastest (41.28 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.0x the fastest

Fastest fs_padded (41.28 ms) to slowest fs_packed (124.61 ms): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fs_padded) is the fastest** at 41283666.6 ns median
- 1 variant significantly slower than baseline
- Spread: 3.02x (fastest 41283666.6 ns, slowest 124608083.3 ns)

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
| fs_packed | 126061769ns | 124609396ns | 123635927ns | 125085135ns | 131417510ns | +200.79% |
| fs_padded | 41910469ns | 41284458ns | 38786583ns | 41353635ns | 46704854ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fs_packed | 126060577ns | 123634646ns | 131416312ns | +200.79% | 0.001 |
| fs_padded | 41909692ns | 38785750ns | 46703750ns | base | 0.002 |

## Performance model

- Peak throughput: **0.002 Gops/s** (fs_padded; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fs_packed | 0.001 | 31.1% |
| fs_padded | 0.002 | 93.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fs_packed | 126061769ns | 126061769ns | +200.79% |
| fs_padded | 41910469ns | 41910469ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fs_padded | 41283667ns | base | --- | [39808875, 42774438] | --- | --- | --- | --- |
| fs_packed | 124608083ns | +84102833.3ns (+203.7%) | [+82139458, +85430854]ns | [124227396, 126349333] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fs_padded | fs_packed |
|---|---|---|---|
| 1 | 1 | 41093688ns | +203.2% |
| 1 | 2 | 41283667ns | +202.3% |

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
| fs_packed | 93.8ns | 126060577.1ns | 0.0% |  |
| fs_padded | 102.1ns | 41909691.7ns | 0.0% |  |

## Distribution (algo ns)

```
fs_packed (n=20, range 123634645.8-131416312.5 ns)
  123634645.8 |################
  124023729.2 |########################################
  124412812.5 |################
  124801895.8 |
  125190979.2 |########
  125580062.5 |
  125969145.8 |########################
  126358229.2 |########
  126747312.5 |
  127136395.8 |
  127525479.2 |
  127914562.5 |
  128303645.8 |
  128692729.2 |########
  129081812.5 |
  129470895.8 |
  129859979.2 |################
  130249062.5 |
  130638145.8 |
  131027229.2 |
  (2 below, 1 above range)

fs_padded (n=20, range 38785750.0-46703750.0 ns)
  38785750.0 |####################
  39181650.0 |
  39577550.0 |########################################
  39973450.0 |##########
  40369350.0 |
  40765250.0 |##########
  41161150.0 |
  41557050.0 |##########
  41952950.0 |
  42348850.0 |##############################
  42744750.0 |##########
  43140650.0 |
  43536550.0 |##########
  43932450.0 |
  44328350.0 |##########
  44724250.0 |
  45120150.0 |
  45516050.0 |
  45911950.0 |
  46307850.0 |##########
  (2 below, 2 above range)

```
