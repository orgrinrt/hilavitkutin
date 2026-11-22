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

### bc_iter's comparison is tie-heavy (13% tied pairs)

13% of paired samples for bc_iter are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### bc_iter is inconsistent: worst-20% is 2.6x its best-20%

bc_iter's best 20% of batches run at 3 ns but its worst 20% at 9 ns (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bc_index) is the fastest** at 4.6 ns median
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
| bc_index | 2102ns | 2095ns | 2091ns | 2096ns | 2132ns | base |
| bc_iter | 2106ns | 2096ns | 2091ns | 2097ns | 2148ns | +0.20% |
| bc_unchecked | 2102ns | 2096ns | 2091ns | 2097ns | 2130ns | +0.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 5ns | 3ns | 6ns | base | 54.360 |
| bc_iter | 5ns | 3ns | 9ns | +14.90% | 47.309 |
| bc_unchecked | 5ns | 3ns | 7ns | +7.51% | 50.562 |

## Performance model

- Peak throughput: **79.380 Gops/s** (bc_index; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 55.652 | 70.1% |
| bc_iter | 51.200 | 64.5% |
| bc_unchecked | 51.200 | 64.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2102ns | 2102ns | base |
| bc_iter | 2106ns | 2106ns | +0.20% |
| bc_unchecked | 2102ns | 2102ns | +0.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 5ns | base | --- | [5, 5] | --- | --- | --- | --- |
| bc_iter | 5ns | no significant difference | [+0, +0]ns | [5, 5] | no | 0.0635 | 0.0616 | **21** (13%, HIGH) |
| bc_unchecked | 5ns | no significant difference | [+0, +1]ns | [5, 5] | no | 0.0635 | 0.0635 | **19** (12%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 4ns | +4.5% | +13.6% |
| 1 | 2 | 5ns | +0.0% | -12.0% |
| 1 | 3 | 5ns | +0.0% | +4.0% |
| 1 | 4 | 4ns | +22.7% | +13.6% |
| 1 | 5 | 5ns | +17.4% | +17.4% |
| 1 | 6 | 5ns | +8.7% | +8.7% |
| 1 | 7 | 5ns | +0.0% | +0.0% |
| 1 | 8 | 5ns | +0.0% | +8.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.023 | ok |
| bc_iter | 0.255 | moderate+ |
| bc_unchecked | -0.175 | ok |

**Consistency summary:**

- **bc_iter**: won 0/8, lost 4/8
- **bc_unchecked**: won 1/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 2.1ns | 4.7ns | 44.5% | HIGH |
| bc_iter | 2.0ns | 5.4ns | 36.9% | HIGH |
| bc_unchecked | 2.0ns | 5.1ns | 40.4% | HIGH |

## Distribution (algo ns)

```
bc_index (n=160, range 3.2-6.1 ns)
      3.2 |#############
      3.4 |
      3.5 |
      3.7 |#######################
      3.8 |
      4.0 |
      4.1 |###################################
      4.2 |
      4.4 |
      4.5 |##################################
      4.7 |
      4.8 |
      5.0 |########################################
      5.1 |
      5.3 |################################
      5.4 |
      5.6 |
      5.7 |#############
      5.9 |
      6.0 |
  (11 below, 19 above range)

bc_iter (n=160, range 3.3-8.7 ns)
      3.3 |#######
      3.6 |#########################
      3.8 |
      4.1 |################
      4.4 |#############################
      4.6 |
      4.9 |########################################
      5.2 |######################################
      5.4 |
      5.7 |######################
      6.0 |##########
      6.2 |
      6.5 |#############
      6.8 |
      7.1 |#####
      7.3 |#####
      7.6 |
      7.9 |####
      8.1 |
      8.4 |
  (10 below, 2 above range)

bc_unchecked (n=160, range 3.3-7.0 ns)
      3.3 |############################
      3.4 |
      3.6 |############################
      3.8 |
      4.0 |
      4.2 |######################################
      4.4 |
      4.6 |##########################
      4.7 |
      4.9 |####################################
      5.1 |
      5.3 |############################
      5.5 |
      5.7 |########################################
      5.8 |
      6.0 |##############################
      6.2 |
      6.4 |
      6.6 |######################
      6.8 |
  (9 below, 13 above range)

```

## Diagnostics

- **bc_index**: CV=22.2% (high variance, measurements may be unstable)
- **bc_index**: bridge=45.7% of algo (FFI overhead may distort results)
- **bc_iter**: CV=90.8% (high variance, measurements may be unstable)
- **bc_iter**: bridge=42.0% of algo (FFI overhead may distort results)
- **bc_unchecked**: CV=26.4% (high variance, measurements may be unstable)
- **bc_unchecked**: bridge=42.0% of algo (FFI overhead may distort results)
