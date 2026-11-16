# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### layout_aos is fastest but the noisiest (CV 102.2%)

layout_aos wins on median (1 ns) yet has the highest variance (CV 102.2%), while layout_soa is the steadiest (CV 78.1%, 1 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (1 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (1 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### layout_soa's comparison is tie-heavy (17% tied pairs)

17% of paired samples for layout_soa are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### layout_aos is inconsistent: worst-20% is 27.4x its best-20%

layout_aos's best 20% of batches run at 0 ns but its worst 20% at 2 ns (27.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (layout_aos) is the fastest** at 0.6 ns median
- Spread: 1.33x (fastest 0.6 ns, slowest 0.8 ns)

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
| layout_aos | 2096ns | 2093ns | 2087ns | 2093ns | 2113ns | base |
| layout_soa | 2095ns | 2092ns | 2087ns | 2093ns | 2110ns | -0.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 1ns | 0ns | 2ns | base | 339.354 |
| layout_soa | 1ns | 0ns | 2ns | +8.86% | 311.720 |

## Performance model

- Peak throughput: **4096.000 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 426.667 | 10.4% |
| layout_soa | 320.000 | 7.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2096ns | 2096ns | base |
| layout_soa | 2095ns | 2095ns | -0.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 1ns | base | --- | [0, 1] | --- | --- | --- | --- |
| layout_soa | 1ns | no significant difference | [+0, +0]ns | [1, 1] | no | 0.4880 | 0.4880 | **27** (17%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 0ns | +100.0% |
| 1 | 2 | 1ns | -50.0% |
| 1 | 3 | 1ns | +33.3% |
| 1 | 4 | 1ns | +81.2% |
| 1 | 5 | 1ns | -50.0% |
| 1 | 6 | 1ns | -33.3% |
| 1 | 7 | 1ns | +33.3% |
| 1 | 8 | 0ns | +100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.021 | ok |
| layout_soa | -0.356 | moderate- |

**Consistency summary:**

- **layout_soa**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 3.0ns | 0.8ns | 391.5% | HIGH |
| layout_soa | 3.1ns | 0.8ns | 374.6% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 0.1-1.7 ns)
      0.1 |
      0.1 |
      0.2 |
      0.3 |
      0.4 |########################################
      0.5 |
      0.6 |
      0.6 |
      0.7 |########################
      0.8 |
      0.9 |
      1.0 |
      1.1 |
      1.1 |##################
      1.2 |
      1.3 |
      1.4 |
      1.5 |
      1.5 |
      1.6 |###########
  (27 below, 8 above range)

layout_soa (n=160, range 0.1-1.8 ns)
      0.1 |
      0.2 |
      0.3 |
      0.3 |########################################
      0.4 |
      0.5 |
      0.6 |
      0.7 |
      0.8 |########################################
      0.8 |
      0.9 |
      1.0 |
      1.1 |
      1.2 |##########################
      1.3 |
      1.4 |
      1.4 |
      1.5 |
      1.6 |
      1.7 |################
  (25 below, 9 above range)

```

## Diagnostics

- **layout_aos**: CV=81.3% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 27.4x (possible bimodal distribution)
- **layout_aos**: bridge=483.3% of algo (FFI overhead may distort results)
- **layout_soa**: CV=76.1% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 20.4x (possible bimodal distribution)
- **layout_soa**: bridge=362.5% of algo (FFI overhead may distort results)
