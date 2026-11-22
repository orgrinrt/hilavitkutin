# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 1 ns to slowest 1 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (1 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (1 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### layout_soa's comparison is tie-heavy (17% tied pairs)

17% of paired samples for layout_soa are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### layout_soa is inconsistent: worst-20% is 5.2x its best-20%

layout_soa's best 20% of batches run at 1 ns but its worst 20% at 3 ns (5.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (layout_aos) is the fastest** at 1.2 ns median
- Spread: 1.00x (fastest 1.2 ns, slowest 1.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | AoS vs SoA (column-store) layout under partial-field iteration |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eadbeef01234567 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2105ns | 2086ns | 2075ns | 2097ns | 2157ns | base |
| layout_soa | 2084ns | 2080ns | 2074ns | 2080ns | 2104ns | -1.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 2ns | 1ns | 3ns | base | 170.241 |
| layout_soa | 1ns | 1ns | 3ns | -3.49% | 176.400 |

## Performance model

- Peak throughput: **512.000 Gops/s** (layout_soa; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 213.333 | 41.7% |
| layout_soa | 213.333 | 41.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2105ns | 2105ns | base |
| layout_soa | 2084ns | 2084ns | -1.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 1ns | base | --- | [1, 2] | --- | --- | --- | --- |
| layout_soa | 1ns | no significant difference | [-0, +0]ns | [1, 2] | no | 0.7289 | 0.7289 | **27** (17%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 1ns | +17.2% |
| 1 | 2 | 1ns | +41.7% |
| 1 | 3 | 1ns | +41.7% |
| 1 | 4 | 2ns | -14.7% |
| 1 | 5 | 1ns | -17.2% |
| 1 | 6 | 1ns | +0.0% |
| 1 | 7 | 1ns | +41.7% |
| 1 | 8 | 1ns | -44.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.069 | ok |
| layout_soa | -0.042 | ok |

**Consistency summary:**

- **layout_soa**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 2.7ns | 1.5ns | 179.3% | HIGH |
| layout_soa | 2.7ns | 1.5ns | 186.1% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 0.6-2.7 ns)
      0.6 |
      0.7 |
      0.8 |############################
      0.9 |
      1.0 |
      1.1 |
      1.2 |########################################
      1.3 |
      1.4 |
      1.5 |
      1.6 |##########################
      1.7 |
      1.8 |
      1.9 |
      2.0 |###################
      2.2 |
      2.3 |
      2.4 |
      2.5 |##################
      2.6 |
  (18 below, 11 above range)

layout_soa (n=160, range 0.5-2.6 ns)
      0.5 |
      0.6 |
      0.7 |#####################################
      0.8 |
      0.9 |
      1.0 |
      1.1 |########################################
      1.2 |
      1.3 |
      1.4 |
      1.5 |
      1.6 |#############################
      1.7 |
      1.9 |
      2.0 |
      2.1 |###########################
      2.2 |
      2.3 |
      2.4 |
      2.5 |#################
  (19 below, 8 above range)

```

## Diagnostics

- **layout_aos**: CV=52.0% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 4.9x (possible bimodal distribution)
- **layout_aos**: bridge=208.3% of algo (FFI overhead may distort results)
- **layout_soa**: CV=54.3% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 5.2x (possible bimodal distribution)
- **layout_soa**: bridge=208.3% of algo (FFI overhead may distort results)
