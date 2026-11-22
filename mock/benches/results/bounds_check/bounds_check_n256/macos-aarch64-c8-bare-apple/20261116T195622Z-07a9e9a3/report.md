# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (1 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bc_index)

The baseline bc_index is the fastest (5 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### bc_iter is inconsistent: worst-20% is 2.1x its best-20%

bc_iter's best 20% of batches run at 3 ns but its worst 20% at 7 ns (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bc_index) is the fastest** at 4.6 ns median
- 2 variants significantly slower than baseline
- Spread: 1.09x (fastest 4.6 ns, slowest 5.0 ns)

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
| bc_index | 2126ns | 2101ns | 2093ns | 2117ns | 2186ns | base |
| bc_iter | 2106ns | 2099ns | 2092ns | 2099ns | 2141ns | -0.94% |
| bc_unchecked | 2114ns | 2100ns | 2095ns | 2101ns | 2172ns | -0.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 5ns | 3ns | 6ns | base | 55.842 |
| bc_iter | 5ns | 3ns | 7ns | +10.25% | 50.649 |
| bc_unchecked | 5ns | 3ns | 7ns | +8.36% | 51.535 |

## Performance model

- Peak throughput: **82.915 Gops/s** (bc_index; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 55.652 | 67.1% |
| bc_iter | 51.200 | 61.7% |
| bc_unchecked | 51.200 | 61.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2126ns | 2126ns | base |
| bc_iter | 2106ns | 2106ns | -0.94% |
| bc_unchecked | 2114ns | 2114ns | -0.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 5ns | base | --- | [4, 5] | --- | --- | --- | --- |
| bc_iter | 5ns | +0.4ns (+8.7%) | [+0, +1]ns | [5, 5] | YES | 0.0015 | 0.0012 | 14 |
| bc_unchecked | 5ns | no significant difference | [+0, +1]ns | [5, 5] | no (adj: YES) | 0.0015 | 0.0015 | 15 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 5ns | +17.4% | +8.7% |
| 1 | 2 | 5ns | -4.0% | +0.0% |
| 1 | 3 | 4ns | +13.6% | +4.5% |
| 1 | 4 | 5ns | +8.3% | +0.0% |
| 1 | 5 | 5ns | +12.0% | +0.0% |
| 1 | 6 | 4ns | +4.8% | +19.0% |
| 1 | 7 | 4ns | +9.1% | +13.6% |
| 1 | 8 | 5ns | +13.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.211 | moderate- |
| bc_iter | -0.228 | moderate- |
| bc_unchecked | -0.045 | ok |

**Consistency summary:**

- **bc_iter**: won 1/8, lost 7/8
- **bc_unchecked**: won 0/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 2.1ns | 4.6ns | 46.0% | HIGH |
| bc_iter | 1.9ns | 5.1ns | 37.8% | HIGH |
| bc_unchecked | 2.1ns | 5.0ns | 41.6% | HIGH |

## Distribution (algo ns)

```
bc_index (n=160, range 3.1-6.2 ns)
      3.1 |
      3.2 |#########################
      3.4 |
      3.6 |#################################
      3.7 |
      3.9 |
      4.0 |
      4.2 |############################
      4.3 |
      4.5 |######################################
      4.6 |
      4.8 |
      4.9 |########################################
      5.1 |
      5.3 |###########################
      5.4 |
      5.6 |
      5.7 |################
      5.9 |
      6.0 |
  (13 below, 16 above range)

bc_iter (n=160, range 3.2-6.9 ns)
      3.2 |##############
      3.4 |
      3.6 |####################
      3.8 |
      4.0 |
      4.1 |##################################
      4.3 |
      4.5 |##################################
      4.7 |
      4.9 |##############################
      5.1 |
      5.3 |########################################
      5.4 |
      5.6 |#############################
      5.8 |
      6.0 |
      6.2 |#########################
      6.4 |
      6.5 |############
      6.7 |
  (12 below, 15 above range)

bc_unchecked (n=160, range 3.3-6.8 ns)
      3.3 |###########
      3.5 |
      3.6 |################
      3.8 |
      4.0 |
      4.2 |########################################
      4.3 |
      4.5 |########################################
      4.7 |
      4.9 |###################################
      5.0 |
      5.2 |
      5.4 |##############################
      5.6 |
      5.8 |############################
      5.9 |
      6.1 |#############
      6.3 |
      6.5 |
      6.6 |######
  (11 below, 16 above range)

```

## Diagnostics

- **bc_index**: CV=24.7% (high variance, measurements may be unstable)
- **bc_index**: bridge=45.7% of algo (FFI overhead may distort results)
- **bc_iter**: CV=25.8% (high variance, measurements may be unstable)
- **bc_iter**: bridge=34.0% of algo (FFI overhead may distort results)
- **bc_unchecked**: CV=25.5% (high variance, measurements may be unstable)
- **bc_unchecked**: bridge=42.0% of algo (FFI overhead may distort results)
