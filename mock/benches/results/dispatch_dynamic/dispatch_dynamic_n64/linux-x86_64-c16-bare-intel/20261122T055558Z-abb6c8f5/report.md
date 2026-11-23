# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (14 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### dispatch_volatile's comparison is tie-heavy (10% tied pairs)

10% of paired samples for dispatch_volatile are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_runtime vs stability leader dispatch_direct (+3% speed for 1.1x steadier)

dispatch_runtime is fastest (16 ns, CV 88.1%); dispatch_direct gives up 2.5% median for 1.1x lower variance (CV 83.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_volatile is inconsistent: worst-20% is 4.0x its best-20%

dispatch_volatile's best 20% of batches run at 13 ns but its worst 20% at 53 ns (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 3.8% of the fastest

All 3 variants sit between 16 ns and 16 ns - a 3.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### dispatch_volatile's edge over baseline is significant but tiny (0 ns, 1.24%)

dispatch_volatile differs from baseline dispatch_direct by 0 ns (1.24%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: dispatch_runtime** at 15.7 ns median (-2.5% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 15.7 ns, slowest 16.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x40ffee15d34db33f |
| Counter frequency | 2112860326 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 2529ns | 1892ns | 1548ns | 1818ns | 5644ns | base |
| dispatch_runtime | 2598ns | 1891ns | 1547ns | 1922ns | 5676ns | +2.71% |
| dispatch_volatile | 2922ns | 1896ns | 1549ns | 2357ns | 5988ns | +15.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 21ns | 13ns | 48ns | base | 2.985 |
| dispatch_runtime | 22ns | 13ns | 47ns | +1.50% | 2.941 |
| dispatch_volatile | 25ns | 13ns | 53ns | +18.44% | 2.520 |

## Performance model

- Peak throughput: **5.077 Gops/s** (dispatch_runtime; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 3.975 | 78.3% |
| dispatch_runtime | 4.076 | 80.3% |
| dispatch_volatile | 3.926 | 77.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2529ns | 2529ns | base |
| dispatch_runtime | 2598ns | 2598ns | +2.71% |
| dispatch_volatile | 2922ns | 2922ns | +15.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 16ns | base | --- | [16, 16] | --- | --- | --- | --- |
| dispatch_runtime | 16ns | no significant difference | [-1, +0]ns | [14, 16] | no | 0.7512 | 0.7512 | 1 |
| dispatch_volatile | 16ns | +0.2ns (+1.2%) | [+0, +0]ns | [16, 16] | YES | 0.0000 | 0.0000 | 16 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 13ns | +18.6% | +0.8% |
| 1 | 2 | 13ns | +25.0% | +1.5% |
| 1 | 3 | 13ns | +1.5% | +166.5% |
| 1 | 4 | 16ns | -22.2% | +1.2% |
| 1 | 5 | 13ns | -4.5% | +23.5% |
| 1 | 6 | 16ns | -19.8% | +0.9% |
| 1 | 7 | 16ns | -20.4% | -17.6% |
| 1 | 8 | 16ns | +2.5% | +0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | 0.128 | ok |
| dispatch_runtime | 0.261 | moderate+ |
| dispatch_volatile | -0.170 | ok |

**Consistency summary:**

- **dispatch_runtime**: won 4/8, lost 4/8
- **dispatch_volatile**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 31.6ns | 21.4ns | 147.2% | HIGH |
| dispatch_runtime | 33.5ns | 21.8ns | 153.9% | HIGH |
| dispatch_volatile | 37.3ns | 25.4ns | 146.8% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 13.2-47.5 ns)
     13.2 |####################################
     14.9 |########################################
     16.6 |
     18.3 |#
     20.0 |
     21.8 |
     23.5 |
     25.2 |
     26.9 |
     28.6 |
     30.3 |
     32.1 |
     33.8 |
     35.5 |
     37.2 |
     38.9 |#
     40.6 |#
     42.4 |
     44.1 |###
     45.8 |###
  (7 below, 15 above range)

dispatch_runtime (n=160, range 12.6-47.3 ns)
     12.6 |########################################
     14.3 |#################
     16.1 |#############################
     17.8 |
     19.6 |
     21.3 |
     23.0 |#
     24.8 |
     26.5 |
     28.2 |
     30.0 |
     31.7 |
     33.5 |
     35.2 |##
     36.9 |#
     38.7 |###
     40.4 |###
     42.1 |###
     43.9 |###
     45.6 |#
  (25 below, 13 above range)

dispatch_volatile (n=160, range 13.3-53.0 ns)
     13.3 |####################
     15.3 |########################################
     17.2 |
     19.2 |
     21.2 |
     23.2 |#
     25.2 |#
     27.2 |#
     29.1 |
     31.1 |#
     33.1 |#
     35.1 |##
     37.1 |##
     39.1 |##
     41.1 |#
     43.0 |###
     45.0 |
     47.0 |#
     49.0 |##
     51.0 |
  (8 below, 14 above range)

```

## Diagnostics

- **dispatch_direct**: CV=62.8% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dispatch_direct**: bridge=147.8% of algo (FFI overhead may distort results)
- **dispatch_runtime**: CV=63.6% (high variance, measurements may be unstable)
- **dispatch_runtime**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dispatch_runtime**: bridge=154.1% of algo (FFI overhead may distort results)
- **dispatch_volatile**: CV=65.0% (high variance, measurements may be unstable)
- **dispatch_volatile**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **dispatch_volatile**: bridge=149.1% of algo (FFI overhead may distort results)
