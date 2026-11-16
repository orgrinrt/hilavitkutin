# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_index dominates: 24% faster than the next best (bc_iter)

bc_index (5 ns) leads bc_iter (6 ns) by 24%, a clear separation rather than a photo finish. CV 29.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bc_index is fastest but the noisiest (CV 29.1%)

bc_index wins on median (5 ns) yet has the highest variance (CV 29.1%), while bc_unchecked is the steadiest (CV 23.9%, 7 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (bc_index)

The baseline bc_index is the fastest (5 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### bc_index is inconsistent: worst-20% is 2.4x its best-20%

bc_index's best 20% of batches run at 3 ns but its worst 20% at 7 ns (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bc_index) is the fastest** at 5.0 ns median
- 2 variants significantly slower than baseline
- Spread: 1.34x (fastest 5.0 ns, slowest 6.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Bounds-check elision: iter() vs explicit index vs get_unchecked |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5a3ec07491bf2d68 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 2096ns | 2086ns | 2078ns | 2090ns | 2130ns | base |
| bc_iter | 2108ns | 2115ns | 2080ns | 2106ns | 2143ns | +0.59% |
| bc_unchecked | 2124ns | 2123ns | 2088ns | 2123ns | 2163ns | +1.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 5ns | 3ns | 7ns | base | 52.797 |
| bc_iter | 6ns | 4ns | 8ns | +28.20% | 41.182 |
| bc_unchecked | 7ns | 4ns | 9ns | +34.76% | 39.177 |

## Performance model

- Peak throughput: **88.658 Gops/s** (bc_index; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 51.200 | 57.7% |
| bc_iter | 41.290 | 46.6% |
| bc_unchecked | 38.209 | 43.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2096ns | 2096ns | base |
| bc_iter | 2108ns | 2108ns | +0.59% |
| bc_unchecked | 2124ns | 2124ns | +1.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 5ns | base | --- | [5, 5] | --- | --- | --- | --- |
| bc_iter | 6ns | +1.3ns (+26.0%) | [+1, +2]ns | [6, 7] | YES | 0.0000 | 0.0000 | 13 |
| bc_unchecked | 7ns | +2.1ns (+42.0%) | [+2, +2]ns | [6, 7] | YES | 0.0000 | 0.0000 | 9 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 5ns | +46.0% | -4.0% |
| 1 | 2 | 4ns | +9.5% | +53.6% |
| 1 | 3 | 5ns | +34.4% | +39.6% |
| 1 | 4 | 5ns | +11.1% | +27.8% |
| 1 | 5 | 5ns | +4.0% | +34.0% |
| 1 | 6 | 5ns | +45.7% | +50.0% |
| 1 | 7 | 5ns | +29.0% | +34.0% |
| 1 | 8 | 5ns | +34.4% | +39.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.108 | ok |
| bc_iter | -0.465 | moderate- |
| bc_unchecked | 0.136 | ok |

**Consistency summary:**

- **bc_iter**: won 0/8, lost 8/8
- **bc_unchecked**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 2.3ns | 4.8ns | 48.0% | HIGH |
| bc_iter | 2.2ns | 6.2ns | 35.2% | HIGH |
| bc_unchecked | 2.2ns | 6.5ns | 32.9% | HIGH |

## Distribution (algo ns)

```
bc_index (n=160, range 2.9-7.0 ns)
      2.9 |######
      3.1 |
      3.3 |############################
      3.5 |################################
      3.7 |
      3.9 |
      4.1 |############################
      4.3 |
      4.5 |########################################
      4.7 |
      4.9 |####################################
      5.1 |
      5.3 |########################################
      5.5 |
      5.7 |############################
      5.9 |
      6.1 |################
      6.3 |
      6.6 |############
      6.8 |
  (12 below, 15 above range)

bc_iter (n=160, range 4.0-8.4 ns)
      4.0 |##############
      4.3 |
      4.5 |################################
      4.7 |
      4.9 |################################
      5.1 |
      5.4 |##############################
      5.6 |
      5.8 |########################################
      6.0 |#####################
      6.2 |
      6.5 |
      6.7 |########################################
      6.9 |##############################
      7.1 |
      7.3 |##############################
      7.6 |
      7.8 |###################################
      8.0 |
      8.2 |################
  (13 below, 9 above range)

bc_unchecked (n=160, range 4.4-8.8 ns)
      4.4 |######################
      4.7 |
      4.9 |###################################
      5.1 |
      5.3 |########################
      5.5 |
      5.7 |###################################
      6.0 |
      6.2 |############################
      6.4 |
      6.6 |#################################
      6.8 |
      7.1 |########################################
      7.3 |
      7.5 |####################
      7.7 |#####################################
      7.9 |
      8.2 |###########
      8.4 |
      8.6 |#################
  (12 below, 10 above range)

```

## Diagnostics

- **bc_index**: CV=30.1% (high variance, measurements may be unstable)
- **bc_index**: bridge=50.0% of algo (FFI overhead may distort results)
- **bc_iter**: CV=25.6% (high variance, measurements may be unstable)
- **bc_iter**: bridge=33.9% of algo (FFI overhead may distort results)
- **bc_unchecked**: CV=24.5% (high variance, measurements may be unstable)
- **bc_unchecked**: bridge=31.3% of algo (FFI overhead may distort results)
