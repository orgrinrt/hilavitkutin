# Cross-core false sharing: slots packed in one line vs one line each

2 variants, 20 samples per variant.
Baseline: **fs_padded**

## Highlights

Baseline for all deltas below: **fs_padded**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fs_padded dominates: 175% faster than the next best (fs_packed)

fs_padded (11.54 ms) leads fs_packed (31.70 ms) by 175%, a clear separation rather than a photo finish. CV 6.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fs_padded is fastest but the noisiest (CV 6.5%)

fs_padded wins on median (11.54 ms) yet has the highest variance (CV 6.5%), while fs_packed is the steadiest (CV 2.1%, 31.70 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fs_padded shows alternating (throttle bounce) (autocorr -0.50)

fs_padded's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fs_padded)

The baseline fs_padded is the fastest (11.54 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fs_padded) is the fastest** at 11538250.0 ns median
- 1 variant significantly slower than baseline
- Spread: 2.75x (fastest 11538250.0 ns, slowest 31701166.7 ns)

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
| fs_packed | 31890644ns | 31701646ns | 31382917ns | 31721007ns | 32907281ns | +181.13% |
| fs_padded | 11343592ns | 11538500ns | 10041292ns | 11542413ns | 12049427ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fs_packed | 31890138ns | 31382469ns | 32906635ns | +181.14% | 0.001 |
| fs_padded | 11343221ns | 10040833ns | 12049104ns | base | 0.001 |

## Performance model

- Peak throughput: **0.002 Gops/s** (fs_padded; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fs_packed | 0.001 | 31.7% |
| fs_padded | 0.001 | 87.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fs_packed | 31890644ns | 31890644ns | +181.13% |
| fs_padded | 11343592ns | 11343592ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fs_padded | 11538250ns | base | --- | [11239229, 11889646] | --- | --- | --- | --- |
| fs_packed | 31701167ns | +20426708.4ns (+177.0%) | [+19842312, +20928250]ns | [31537979, 31922833] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fs_padded | fs_packed |
|---|---|---|---|
| 1 | 1 | 11679354ns | +171.6% |
| 1 | 2 | 11506750ns | +175.2% |

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
| fs_packed | 62.5ns | 31890137.5ns | 0.0% |  |
| fs_padded | 47.9ns | 11343220.8ns | 0.0% |  |

## Distribution (algo ns)

```
fs_packed (n=20, range 31382468.8-32906635.4 ns)
  31382468.8 |#############
  31458677.1 |##########################
  31534885.4 |########################################
  31611093.8 |##########################
  31687302.1 |##########################
  31763510.4 |##########################
  31839718.8 |
  31915927.1 |
  31992135.4 |##########################
  32068343.8 |#############
  32144552.1 |
  32220760.4 |#############
  32296968.8 |
  32373177.1 |
  32449385.4 |
  32525593.8 |
  32601802.1 |
  32678010.4 |
  32754218.8 |
  32830427.1 |
  (2 below, 2 above range)

fs_padded (n=20, range 10040833.3-12049104.1 ns)
  10040833.3 |#############
  10141246.9 |
  10241660.4 |
  10342073.9 |
  10442487.5 |
  10542901.0 |
  10643314.6 |
  10743728.1 |
  10844141.7 |#############
  10944555.2 |
  11044968.7 |#############
  11145382.3 |#############
  11245795.8 |##########################
  11346209.4 |
  11446622.9 |##########################
  11547036.4 |##########################
  11647450.0 |
  11747863.5 |#############
  11848277.1 |##########################
  11948690.6 |########################################
  (2 below, 2 above range)

```
