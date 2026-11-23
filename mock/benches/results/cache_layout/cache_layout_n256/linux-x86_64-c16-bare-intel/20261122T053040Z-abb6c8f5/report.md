# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 12 ns to slowest 12 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (11 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (12 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### layout_soa's comparison is tie-heavy (31% tied pairs)

31% of paired samples for layout_soa are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### layout_soa is inconsistent: worst-20% is 3.9x its best-20%

layout_soa's best 20% of batches run at 10 ns but its worst 20% at 39 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (layout_aos) is the fastest** at 12.2 ns median
- Spread: 1.00x (fastest 12.2 ns, slowest 12.2 ns)

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
| Counter frequency | 2112756649 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2679ns | 1889ns | 1544ns | 2027ns | 5769ns | base |
| layout_soa | 2641ns | 1889ns | 1545ns | 1878ns | 6027ns | -1.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 17ns | 10ns | 37ns | base | 14.813 |
| layout_soa | 17ns | 10ns | 39ns | -1.73% | 15.073 |

## Performance model

- Peak throughput: **25.688 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 20.984 | 81.7% |
| layout_soa | 20.984 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2679ns | 2679ns | base |
| layout_soa | 2641ns | 2641ns | -1.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 12ns | base | --- | [12, 12] | --- | --- | --- | --- |
| layout_soa | 12ns | no significant difference | [+0, +0]ns | [12, 12] | no | 0.6338 | 0.6338 | **50** (31%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 10ns | +0.0% |
| 1 | 2 | 12ns | +0.0% |
| 1 | 3 | 10ns | +0.0% |
| 1 | 4 | 12ns | -0.8% |
| 1 | 5 | 12ns | -0.4% |
| 1 | 6 | 12ns | +0.0% |
| 1 | 7 | 10ns | +0.0% |
| 1 | 8 | 10ns | +22.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.108 | ok |
| layout_soa | -0.480 | moderate- |

**Consistency summary:**

- **layout_soa**: won 2/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 33.8ns | 17.3ns | 195.8% | HIGH |
| layout_soa | 33.2ns | 17.0ns | 195.8% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 10.0-37.2 ns)
     10.0 |###########################
     11.3 |########################################
     12.7 |#
     14.0 |
     15.4 |
     16.8 |
     18.1 |
     19.5 |
     20.9 |
     22.2 |
     23.6 |
     24.9 |
     26.3 |
     27.7 |#
     29.0 |##
     30.4 |###
     31.7 |#
     33.1 |#
     34.5 |#
     35.8 |##
  (11 below, 14 above range)

layout_soa (n=160, range 10.0-38.5 ns)
     10.0 |####################
     11.4 |########################################
     12.8 |
     14.3 |
     15.7 |
     17.1 |
     18.5 |
     20.0 |
     21.4 |#
     22.8 |
     24.3 |
     25.7 |
     27.1 |
     28.5 |#
     30.0 |
     31.4 |#
     32.8 |#
     34.2 |
     35.7 |#
     37.1 |#
  (5 below, 18 above range)

```

## Diagnostics

- **layout_aos**: CV=62.6% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **layout_aos**: bridge=195.1% of algo (FFI overhead may distort results)
- **layout_soa**: CV=66.6% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **layout_soa**: bridge=195.1% of algo (FFI overhead may distort results)
