# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.4% (fastest 28 ns to slowest 28 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### layout_aos is fastest but the noisiest (CV 87.2%)

layout_aos wins on median (28 ns) yet has the highest variance (CV 87.2%), while layout_soa is the steadiest (CV 85.7%, 28 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (24 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (28 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### layout_soa's comparison is tie-heavy (21% tied pairs)

21% of paired samples for layout_soa are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader layout_aos vs stability leader layout_soa (+0% speed for 1.0x steadier)

layout_aos is fastest (28 ns, CV 87.2%); layout_soa gives up 0.4% median for 1.0x lower variance (CV 85.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### layout_aos is inconsistent: worst-20% is 3.7x its best-20%

layout_aos's best 20% of batches run at 23 ns but its worst 20% at 84 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (layout_aos) is the fastest** at 27.9 ns median
- Spread: 1.00x (fastest 27.9 ns, slowest 28.0 ns)

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
| layout_aos | 2624ns | 1906ns | 1559ns | 1906ns | 5841ns | base |
| layout_soa | 2518ns | 1906ns | 1557ns | 1872ns | 5417ns | -4.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 38ns | 23ns | 84ns | base | 26.777 |
| layout_soa | 37ns | 23ns | 80ns | -3.33% | 27.698 |

## Performance model

- Peak throughput: **44.832 Gops/s** (layout_soa; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 36.703 | 81.9% |
| layout_soa | 36.571 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2624ns | 2624ns | base |
| layout_soa | 2518ns | 2518ns | -4.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 28ns | base | --- | [28, 28] | --- | --- | --- | --- |
| layout_soa | 28ns | no significant difference | [+0, +0]ns | [28, 28] | no | 0.1812 | 0.1812 | **34** (21%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 28ns | +0.0% |
| 1 | 2 | 28ns | -18.2% |
| 1 | 3 | 28ns | +0.0% |
| 1 | 4 | 23ns | +22.3% |
| 1 | 5 | 23ns | +0.0% |
| 1 | 6 | 23ns | +0.0% |
| 1 | 7 | 28ns | +0.0% |
| 1 | 8 | 28ns | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.391 | moderate+ |
| layout_soa | -0.142 | ok |

**Consistency summary:**

- **layout_soa**: won 1/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 32.3ns | 38.2ns | 84.4% | HIGH |
| layout_soa | 31.1ns | 37.0ns | 84.0% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 22.9-84.3 ns)
     22.9 |##############
     25.9 |########################################
     29.0 |
     32.1 |
     35.2 |
     38.2 |
     41.3 |
     44.4 |
     47.5 |
     50.5 |
     53.6 |
     56.7 |
     59.7 |
     62.8 |
     65.9 |
     69.0 |
     72.0 |
     75.1 |
     78.2 |
     81.3 |#
  (13 below, 17 above range)

layout_soa (n=160, range 22.8-79.6 ns)
     22.8 |###########
     25.7 |########################################
     28.5 |
     31.4 |
     34.2 |
     37.0 |
     39.9 |
     42.7 |
     45.6 |
     48.4 |
     51.2 |
     54.1 |
     56.9 |
     59.8 |
     62.6 |
     65.4 |
     68.3 |
     71.1 |#
     73.9 |
     76.8 |##
  (17 below, 17 above range)

```

## Diagnostics

- **layout_aos**: CV=63.7% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **layout_aos**: bridge=84.6% of algo (FFI overhead may distort results)
- **layout_soa**: CV=64.9% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **layout_soa**: bridge=83.9% of algo (FFI overhead may distort results)
