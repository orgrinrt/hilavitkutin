# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (layout_aos) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline layout_aos has the worst median (28 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest layout_soa at 28 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### layout_soa shows alternating (throttle bounce) (autocorr -0.68)

layout_soa's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (25 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### layout_soa's comparison is tie-heavy (16% tied pairs)

16% of paired samples for layout_soa are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### layout_soa is inconsistent: worst-20% is 3.8x its best-20%

layout_soa's best 20% of batches run at 23 ns but its worst 20% at 86 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### layout_soa's edge over baseline is significant but tiny (-0 ns, 0.36%)

layout_soa differs from baseline layout_aos by -0 ns (0.36%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: layout_soa** at 27.9 ns median (-0.4% vs baseline)
- 1 variant significantly faster than baseline
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
| Counter frequency | 2112817009 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2921ns | 1907ns | 1900ns | 2063ns | 6517ns | base |
| layout_soa | 2624ns | 1905ns | 1557ns | 1896ns | 5874ns | -10.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 43ns | 28ns | 94ns | base | 24.067 |
| layout_soa | 38ns | 23ns | 86ns | -9.52% | 26.599 |

## Performance model

- Peak throughput: **44.845 Gops/s** (layout_soa; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 36.571 | 81.6% |
| layout_soa | 36.703 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2921ns | 2921ns | base |
| layout_soa | 2624ns | 2624ns | -10.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 28ns | base | --- | [28, 28] | --- | --- | --- | --- |
| layout_soa | 28ns | -0.1ns (-0.4%) | [-1, -0]ns | [28, 28] | YES | 0.0000 | 0.0000 | **25** (16%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 28ns | -0.2% |
| 1 | 2 | 28ns | -18.2% |
| 1 | 3 | 28ns | +0.0% |
| 1 | 4 | 28ns | -18.1% |
| 1 | 5 | 28ns | +0.4% |
| 1 | 6 | 28ns | +0.0% |
| 1 | 7 | 28ns | -18.2% |
| 1 | 8 | 28ns | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.125 | ok |
| layout_soa | -0.678 | HIGH- (thermal bounce) |

**Consistency summary:**

- **layout_soa**: won 4/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 36.0ns | 42.5ns | 84.7% | HIGH |
| layout_soa | 32.3ns | 38.5ns | 83.8% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 27.8-94.3 ns)
     27.8 |########################################
     31.2 |
     34.5 |
     37.8 |
     41.1 |
     44.4 |
     47.8 |
     51.1 |
     54.4 |
     57.7 |
     61.1 |
     64.4 |
     67.7 |
     71.0 |
     74.4 |
     77.7 |
     81.0 |#
     84.3 |##
     87.7 |
     91.0 |##
  (21 below, 17 above range)

layout_soa (n=160, range 22.8-86.3 ns)
     22.8 |############
     26.0 |########################################
     29.2 |
     32.4 |
     35.5 |
     38.7 |
     41.9 |
     45.0 |
     48.2 |
     51.4 |
     54.6 |
     57.7 |
     60.9 |
     64.1 |#
     67.2 |#
     70.4 |#
     73.6 |#
     76.8 |#
     79.9 |
     83.1 |##
  (21 below, 14 above range)

```

## Diagnostics

- **layout_aos**: CV=63.2% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **layout_aos**: bridge=83.9% of algo (FFI overhead may distort results)
- **layout_soa**: CV=64.8% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **layout_soa**: bridge=84.2% of algo (FFI overhead may distort results)
