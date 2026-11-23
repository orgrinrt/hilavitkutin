# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (layout_aos) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline layout_aos has the worst median (10 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest layout_soa at 9 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### layout_soa dominates: 22% faster than the next best (layout_aos)

layout_soa (9 ns) leads layout_aos (10 ns) by 22%, a clear separation rather than a photo finish. CV 113.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### layout_soa is fastest but the noisiest (CV 113.8%)

layout_soa wins on median (9 ns) yet has the highest variance (CV 113.8%), while layout_aos is the steadiest (CV 88.0%, 10 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (10 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### layout_soa's comparison is tie-heavy (10% tied pairs)

10% of paired samples for layout_soa are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### layout_soa is inconsistent: worst-20% is 3.7x its best-20%

layout_soa's best 20% of batches run at 8 ns but its worst 20% at 32 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### layout_soa's edge over baseline is significant but tiny (-0 ns, 0.95%)

layout_soa differs from baseline layout_aos by -0 ns (0.95%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: layout_soa** at 8.6 ns median (-18.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.22x (fastest 8.6 ns, slowest 10.5 ns)

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
| layout_aos | 2714ns | 1889ns | 1545ns | 2139ns | 5607ns | base |
| layout_soa | 2634ns | 1561ns | 1546ns | 1994ns | 5645ns | -2.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 15ns | 9ns | 31ns | base | 4.220 |
| layout_soa | 15ns | 8ns | 32ns | -3.58% | 4.377 |

## Performance model

- Peak throughput: **7.566 Gops/s** (layout_soa; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 6.095 | 80.6% |
| layout_soa | 7.442 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2714ns | 2714ns | base |
| layout_soa | 2634ns | 2634ns | -2.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 10ns | base | --- | [10, 11] | --- | --- | --- | --- |
| layout_soa | 9ns | -0.1ns (-1.0%) | [-0, -0]ns | [9, 10] | YES | 0.0000 | 0.0000 | 16 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 11ns | -19.8% |
| 1 | 2 | 9ns | -1.7% |
| 1 | 3 | 9ns | +20.7% |
| 1 | 4 | 9ns | +0.0% |
| 1 | 5 | 9ns | -1.7% |
| 1 | 6 | 11ns | -0.9% |
| 1 | 7 | 11ns | -18.5% |
| 1 | 8 | 9ns | -0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.055 | ok |
| layout_soa | -0.357 | moderate- |

**Consistency summary:**

- **layout_soa**: won 6/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 33.9ns | 15.2ns | 223.4% | HIGH |
| layout_soa | 33.3ns | 14.6ns | 227.5% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 8.6-31.0 ns)
      8.6 |########################################
      9.7 |#############################
     10.8 |
     12.0 |
     13.1 |
     14.2 |
     15.3 |
     16.4 |
     17.6 |
     18.7 |
     19.8 |
     20.9 |
     22.1 |
     23.2 |
     24.3 |
     25.4 |
     26.5 |###
     27.7 |#
     28.8 |#########
     29.9 |##
  (5 below, 13 above range)

layout_soa (n=160, range 8.5-31.7 ns)
      8.5 |########################################
      9.6 |##################
     10.8 |
     11.9 |
     13.1 |
     14.3 |#
     15.4 |
     16.6 |
     17.8 |
     18.9 |
     20.1 |
     21.3 |
     22.4 |#
     23.6 |
     24.7 |#
     25.9 |###
     27.1 |#
     28.2 |#####
     29.4 |#
     30.6 |###
  (13 below, 9 above range)

```

## Diagnostics

- **layout_aos**: CV=60.9% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **layout_aos**: bridge=225.7% of algo (FFI overhead may distort results)
- **layout_soa**: CV=66.9% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **layout_soa**: bridge=230.2% of algo (FFI overhead may distort results)
