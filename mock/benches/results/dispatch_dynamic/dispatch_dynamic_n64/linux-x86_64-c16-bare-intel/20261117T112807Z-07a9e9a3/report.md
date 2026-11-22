# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_runtime is fastest but the noisiest (CV 110.6%)

dispatch_runtime wins on median (15 ns) yet has the highest variance (CV 110.6%), while dispatch_volatile is the steadiest (CV 87.9%, 16 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (17 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader dispatch_runtime vs stability leader dispatch_volatile (+6% speed for 1.3x steadier)

dispatch_runtime is fastest (15 ns, CV 110.6%); dispatch_volatile gives up 5.5% median for 1.3x lower variance (CV 87.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_runtime is inconsistent: worst-20% is 3.9x its best-20%

dispatch_runtime's best 20% of batches run at 13 ns but its worst 20% at 50 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: dispatch_runtime** at 15.4 ns median (-4.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 15.4 ns, slowest 16.3 ns)

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
| Counter frequency | 2112761094 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 2620ns | 1897ns | 1553ns | 1855ns | 5979ns | base |
| dispatch_runtime | 2601ns | 1899ns | 1554ns | 1936ns | 5645ns | -0.70% |
| dispatch_volatile | 2724ns | 1902ns | 1748ns | 1951ns | 6020ns | +3.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 22ns | 13ns | 50ns | base | 2.893 |
| dispatch_runtime | 22ns | 13ns | 50ns | +1.00% | 2.864 |
| dispatch_volatile | 23ns | 15ns | 51ns | +5.23% | 2.749 |

## Performance model

- Peak throughput: **5.023 Gops/s** (dispatch_runtime; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 3.975 | 79.1% |
| dispatch_runtime | 4.142 | 82.5% |
| dispatch_volatile | 3.926 | 78.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2620ns | 2620ns | base |
| dispatch_runtime | 2601ns | 2601ns | -0.70% |
| dispatch_volatile | 2724ns | 2724ns | +3.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 16ns | base | --- | [13, 16] | --- | --- | --- | --- |
| dispatch_runtime | 15ns | no significant difference | [-0, +0]ns | [14, 16] | no | 0.8126 | 0.8126 | 0 |
| dispatch_volatile | 16ns | +2.9ns (+18.3%) | [+1, +3]ns | [16, 16] | YES | 0.0000 | 0.0000 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 16ns | -19.9% | -16.8% |
| 1 | 2 | 13ns | +2.3% | +23.5% |
| 1 | 3 | 13ns | +2.3% | +22.7% |
| 1 | 4 | 13ns | -3.8% | +23.5% |
| 1 | 5 | 13ns | +17.0% | +23.5% |
| 1 | 6 | 16ns | -2.5% | +0.6% |
| 1 | 7 | 16ns | -19.5% | +0.9% |
| 1 | 8 | 13ns | +25.8% | +23.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | 0.064 | ok |
| dispatch_runtime | -0.138 | ok |
| dispatch_volatile | -0.024 | ok |

**Consistency summary:**

- **dispatch_runtime**: won 4/8, lost 4/8
- **dispatch_volatile**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 32.8ns | 22.1ns | 148.2% | HIGH |
| dispatch_runtime | 33.2ns | 22.3ns | 148.6% | HIGH |
| dispatch_volatile | 34.7ns | 23.3ns | 149.0% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 13.1-50.2 ns)
     13.1 |########################################
     15.0 |###################################
     16.9 |
     18.7 |
     20.6 |#
     22.4 |
     24.3 |
     26.1 |
     28.0 |
     29.8 |
     31.7 |
     33.5 |
     35.4 |
     37.3 |
     39.1 |#
     41.0 |##
     42.8 |##
     44.7 |#
     46.5 |####
     48.4 |
  (18 below, 16 above range)

dispatch_runtime (n=160, range 12.7-49.6 ns)
     12.7 |########################################
     14.6 |#######################
     16.4 |##############
     18.3 |
     20.1 |
     22.0 |##
     23.8 |
     25.7 |
     27.5 |
     29.3 |#
     31.2 |
     33.0 |
     34.9 |#
     36.7 |##
     38.6 |###
     40.4 |
     42.3 |####
     44.1 |#
     46.0 |###
     47.8 |###
  (17 below, 8 above range)

dispatch_volatile (n=160, range 14.9-51.0 ns)
     14.9 |########################################
     16.7 |
     18.5 |
     20.3 |
     22.1 |
     24.0 |
     25.8 |
     27.6 |
     29.4 |
     31.2 |
     33.0 |
     34.8 |
     36.6 |
     38.4 |
     40.2 |
     42.0 |
     43.8 |#
     45.6 |
     47.4 |##
     49.2 |#
  (14 below, 14 above range)

```

## Diagnostics

- **dispatch_direct**: CV=66.6% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dispatch_direct**: bridge=147.5% of algo (FFI overhead may distort results)
- **dispatch_runtime**: CV=76.4% (high variance, measurements may be unstable)
- **dispatch_runtime**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **dispatch_runtime**: bridge=157.3% of algo (FFI overhead may distort results)
- **dispatch_volatile**: CV=61.5% (high variance, measurements may be unstable)
- **dispatch_volatile**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **dispatch_volatile**: bridge=149.1% of algo (FFI overhead may distort results)
