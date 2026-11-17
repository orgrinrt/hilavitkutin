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

### layout_soa's comparison is tie-heavy (21% tied pairs)

21% of paired samples for layout_soa are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### layout_soa is inconsistent: worst-20% is 32.5x its best-20%

layout_soa's best 20% of batches run at 0 ns but its worst 20% at 2 ns (32.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (layout_aos) is the fastest** at 0.8 ns median
- Spread: 1.00x (fastest 0.8 ns, slowest 0.8 ns)

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
| layout_aos | 2122ns | 2096ns | 2089ns | 2101ns | 2220ns | base |
| layout_soa | 2110ns | 2098ns | 2088ns | 2106ns | 2145ns | -0.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 1ns | 0ns | 2ns | base | 327.942 |
| layout_soa | 1ns | 0ns | 2ns | -5.52% | 347.119 |

## Performance model

- Peak throughput: **5120.000 Gops/s** (layout_soa; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 320.000 | 6.2% |
| layout_soa | 320.000 | 6.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2122ns | 2122ns | base |
| layout_soa | 2110ns | 2110ns | -0.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 1ns | base | --- | [0, 1] | --- | --- | --- | --- |
| layout_soa | 1ns | no significant difference | [-0, +0]ns | [0, 1] | no | 0.2467 | 0.2467 | **34** (21%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 1ns | -25.0% |
| 1 | 2 | 1ns | +0.0% |
| 1 | 3 | 0ns | +150.0% |
| 1 | 4 | 0ns | +0.0% |
| 1 | 5 | 0ns | +0.0% |
| 1 | 6 | 1ns | +0.0% |
| 1 | 7 | 1ns | -50.0% |
| 1 | 8 | 1ns | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.392 | moderate+ |
| layout_soa | -0.243 | moderate- |

**Consistency summary:**

- **layout_soa**: won 2/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 3.0ns | 0.8ns | 380.9% | HIGH |
| layout_soa | 3.0ns | 0.7ns | 406.4% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 0.2-1.7 ns)
      0.2 |
      0.3 |
      0.3 |########################################
      0.4 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.8 |###############################
      0.9 |
      0.9 |
      1.0 |
      1.1 |
      1.2 |###############
      1.2 |
      1.3 |
      1.4 |
      1.5 |
      1.5 |
      1.6 |
  (17 below, 21 above range)

layout_soa (n=160, range 0.1-1.6 ns)
      0.1 |
      0.1 |
      0.2 |
      0.3 |
      0.4 |########################################
      0.4 |
      0.5 |
      0.6 |
      0.7 |
      0.8 |################################
      0.8 |
      0.9 |
      1.0 |
      1.1 |
      1.2 |########################
      1.2 |
      1.3 |
      1.4 |
      1.5 |
      1.5 |
  (28 below, 16 above range)

```

## Diagnostics

- **layout_aos**: CV=74.2% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 9.0x (possible bimodal distribution)
- **layout_aos**: bridge=362.5% of algo (FFI overhead may distort results)
- **layout_soa**: CV=81.5% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 32.5x (possible bimodal distribution)
- **layout_soa**: bridge=362.5% of algo (FFI overhead may distort results)
