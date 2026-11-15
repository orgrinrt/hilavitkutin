# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (1 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bc_index)

The baseline bc_index is the fastest (5 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader bc_index vs stability leader bc_iter (+8% speed for 1.0x steadier)

bc_index is fastest (5 ns, CV 28.1%); bc_iter gives up 8.0% median for 1.0x lower variance (CV 27.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bc_unchecked is inconsistent: worst-20% is 2.3x its best-20%

bc_unchecked's best 20% of batches run at 4 ns but its worst 20% at 8 ns (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bc_index) is the fastest** at 5.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.16x (fastest 5.0 ns, slowest 5.8 ns)

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
| bc_index | 2247ns | 2228ns | 2084ns | 2244ns | 2420ns | base |
| bc_iter | 2229ns | 2174ns | 2090ns | 2210ns | 2422ns | -0.84% |
| bc_unchecked | 2341ns | 2409ns | 2105ns | 2388ns | 2436ns | +4.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 5ns | 3ns | 7ns | base | 51.213 |
| bc_iter | 5ns | 3ns | 7ns | +6.19% | 48.228 |
| bc_unchecked | 6ns | 4ns | 8ns | +16.05% | 44.128 |

## Performance model

- Peak throughput: **80.949 Gops/s** (bc_index; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 51.200 | 63.2% |
| bc_iter | 47.407 | 58.6% |
| bc_unchecked | 44.138 | 54.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2247ns | 2247ns | base |
| bc_iter | 2229ns | 2229ns | -0.84% |
| bc_unchecked | 2341ns | 2341ns | +4.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 5ns | base | --- | [5, 5] | --- | --- | --- | --- |
| bc_iter | 5ns | no significant difference | [+0, +0]ns | [5, 5] | no | 0.0987 | 0.0987 | 13 |
| bc_unchecked | 6ns | +0.8ns (+16.0%) | [+0, +1]ns | [5, 6] | YES | 0.0001 | 0.0001 | 13 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 4ns | +4.5% | +18.2% |
| 1 | 2 | 6ns | -10.7% | +7.1% |
| 1 | 3 | 5ns | +4.0% | -12.0% |
| 1 | 4 | 5ns | +17.4% | +26.1% |
| 1 | 5 | 6ns | -10.7% | -3.6% |
| 1 | 6 | 5ns | +15.4% | +15.4% |
| 1 | 7 | 5ns | +25.0% | +52.1% |
| 1 | 8 | 5ns | +0.0% | +34.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.285 | moderate- |
| bc_iter | 0.021 | ok |
| bc_unchecked | 0.085 | ok |

**Consistency summary:**

- **bc_iter**: won 2/8, lost 5/8
- **bc_unchecked**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 2.3ns | 5.0ns | 46.3% | HIGH |
| bc_iter | 2.3ns | 5.3ns | 43.1% | HIGH |
| bc_unchecked | 2.5ns | 5.8ns | 42.7% | HIGH |

## Distribution (algo ns)

```
bc_index (n=160, range 3.2-7.1 ns)
      3.2 |##################
      3.4 |
      3.6 |############################
      3.8 |
      4.0 |
      4.2 |####################
      4.4 |
      4.5 |###################################
      4.7 |
      4.9 |########################################
      5.1 |
      5.3 |####################
      5.5 |
      5.7 |#########################
      5.9 |
      6.1 |########
      6.3 |
      6.5 |#####################
      6.7 |
      6.9 |#############
  (13 below, 9 above range)

bc_iter (n=160, range 3.3-7.5 ns)
      3.3 |
      3.5 |###########################
      3.7 |
      3.9 |
      4.1 |#################################
      4.4 |
      4.6 |#############################
      4.8 |
      5.0 |########################################
      5.2 |
      5.4 |########################################
      5.6 |###############################
      5.8 |
      6.0 |#############################
      6.2 |
      6.4 |
      6.7 |################
      6.9 |
      7.1 |#########################
      7.3 |
  (16 below, 14 above range)

bc_unchecked (n=160, range 3.5-8.1 ns)
      3.5 |#########################
      3.8 |
      4.0 |#####################
      4.2 |
      4.5 |#####################################
      4.7 |
      4.9 |##################
      5.1 |
      5.4 |###################################
      5.6 |########################################
      5.8 |
      6.1 |#########################
      6.3 |
      6.5 |#####################################
      6.7 |
      7.0 |################################
      7.2 |
      7.4 |#####################
      7.7 |
      7.9 |#####################
  (14 below, 11 above range)

```

## Diagnostics

- **bc_index**: CV=28.1% (high variance, measurements may be unstable)
- **bc_index**: bridge=42.0% of algo (FFI overhead may distort results)
- **bc_iter**: CV=27.8% (high variance, measurements may be unstable)
- **bc_iter**: bridge=38.9% of algo (FFI overhead may distort results)
- **bc_unchecked**: CV=28.3% (high variance, measurements may be unstable)
- **bc_unchecked**: bridge=43.1% of algo (FFI overhead may distort results)
