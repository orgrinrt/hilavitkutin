# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (14 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### dispatch_volatile is inconsistent: worst-20% is 4.0x its best-20%

dispatch_volatile's best 20% of batches run at 13 ns but its worst 20% at 52 ns (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 2.5% of the fastest

All 3 variants sit between 16 ns and 16 ns - a 2.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### dispatch_runtime's edge over baseline is significant but tiny (-0 ns, 1.86%)

dispatch_runtime differs from baseline dispatch_direct by -0 ns (1.86%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: dispatch_runtime** at 15.8 ns median (-1.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.03x (fastest 15.8 ns, slowest 16.2 ns)

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
| Counter frequency | 2112780910 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 2737ns | 1894ns | 1741ns | 1927ns | 6165ns | base |
| dispatch_runtime | 2736ns | 1894ns | 1580ns | 2027ns | 6021ns | -0.04% |
| dispatch_volatile | 2773ns | 1893ns | 1549ns | 2051ns | 6164ns | +1.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 23ns | 15ns | 52ns | base | 2.766 |
| dispatch_runtime | 23ns | 13ns | 50ns | -0.95% | 2.793 |
| dispatch_volatile | 24ns | 13ns | 52ns | +2.37% | 2.702 |

## Performance model

- Peak throughput: **4.834 Gops/s** (dispatch_runtime; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 3.975 | 82.2% |
| dispatch_runtime | 4.051 | 83.8% |
| dispatch_volatile | 3.951 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2737ns | 2737ns | base |
| dispatch_runtime | 2736ns | 2736ns | -0.04% |
| dispatch_volatile | 2773ns | 2773ns | +1.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 16ns | base | --- | [16, 16] | --- | --- | --- | --- |
| dispatch_runtime | 16ns | -0.3ns (-1.9%) | [-0, -0]ns | [16, 16] | YES | 0.0171 | 0.0171 | 1 |
| dispatch_volatile | 16ns | +0.1ns (+0.6%) | [+0, +0]ns | [16, 16] | YES | 0.0048 | 0.0024 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 16ns | -3.1% | -17.1% |
| 1 | 2 | 13ns | +2.3% | +0.8% |
| 1 | 3 | 16ns | +1.9% | +1.2% |
| 1 | 4 | 16ns | -4.3% | +1.2% |
| 1 | 5 | 16ns | -21.7% | +1.2% |
| 1 | 6 | 16ns | -1.9% | +1.9% |
| 1 | 7 | 16ns | -1.2% | +0.6% |
| 1 | 8 | 16ns | +3.1% | -17.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.161 | ok |
| dispatch_runtime | -0.221 | moderate- |
| dispatch_volatile | 0.271 | moderate+ |

**Consistency summary:**

- **dispatch_runtime**: won 5/8, lost 3/8
- **dispatch_volatile**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 34.2ns | 23.1ns | 147.9% | HIGH |
| dispatch_runtime | 35.3ns | 22.9ns | 153.8% | HIGH |
| dispatch_volatile | 35.3ns | 23.7ns | 149.2% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 14.8-51.9 ns)
     14.8 |########################################
     16.6 |
     18.5 |
     20.3 |
     22.2 |
     24.0 |
     25.9 |
     27.8 |
     29.6 |
     31.5 |
     33.3 |
     35.2 |
     37.0 |
     38.9 |
     40.7 |
     42.6 |
     44.5 |
     46.3 |
     48.2 |#
     50.0 |
  (14 below, 20 above range)

dispatch_runtime (n=160, range 13.2-50.1 ns)
     13.2 |######
     15.1 |########################################
     16.9 |
     18.8 |
     20.6 |
     22.5 |
     24.3 |
     26.1 |
     28.0 |
     29.8 |
     31.7 |
     33.5 |
     35.4 |
     37.2 |
     39.0 |
     40.9 |
     42.7 |#
     44.6 |#
     46.4 |#
     48.3 |#
  (14 below, 14 above range)

dispatch_volatile (n=160, range 13.3-52.4 ns)
     13.3 |####################
     15.2 |########################################
     17.2 |
     19.1 |
     21.1 |#
     23.1 |
     25.0 |
     27.0 |
     28.9 |
     30.9 |#
     32.8 |
     34.8 |
     36.8 |
     38.7 |
     40.7 |
     42.6 |#
     44.6 |#
     46.5 |##
     48.5 |##
     50.5 |##
  (10 below, 15 above range)

```

## Diagnostics

- **dispatch_direct**: CV=64.2% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **dispatch_direct**: bridge=147.8% of algo (FFI overhead may distort results)
- **dispatch_runtime**: CV=62.7% (high variance, measurements may be unstable)
- **dispatch_runtime**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dispatch_runtime**: bridge=153.8% of algo (FFI overhead may distort results)
- **dispatch_volatile**: CV=64.7% (high variance, measurements may be unstable)
- **dispatch_volatile**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **dispatch_volatile**: bridge=149.4% of algo (FFI overhead may distort results)
