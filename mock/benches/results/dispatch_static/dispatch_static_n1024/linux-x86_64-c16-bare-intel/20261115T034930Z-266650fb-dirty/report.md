# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_const_ptr is fastest but the noisiest (CV 87.2%)

dispatch_const_ptr wins on median (150 ns) yet has the highest variance (CV 87.2%), while dispatch_direct is the steadiest (CV 81.8%, 150 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (131 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (150 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_direct's comparison is tie-heavy (22% tied pairs)

22% of paired samples for dispatch_direct are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_const_ptr vs stability leader dispatch_direct (+0% speed for 1.1x steadier)

dispatch_const_ptr is fastest (150 ns, CV 87.2%); dispatch_direct gives up 0.0% median for 1.1x lower variance (CV 81.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_const_ptr is inconsistent: worst-20% is 3.7x its best-20%

dispatch_const_ptr's best 20% of batches run at 124 ns but its worst 20% at 457 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.0% of the fastest

All 3 variants sit between 150 ns and 150 ns - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### dispatch_trait's edge over baseline is significant but tiny (-0 ns, 0.10%)

dispatch_trait differs from baseline dispatch_const_ptr by -0 ns (0.10%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 150.1 ns median
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 150.1 ns, slowest 150.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Static dispatch shapes: direct vs sealed-trait vs const fn pointer |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x7eedfacedeadbeef |
| Counter frequency | 2112812250 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2804ns | 2028ns | 1671ns | 2035ns | 6247ns | base |
| dispatch_direct | 2704ns | 2028ns | 1694ns | 2031ns | 5733ns | -3.58% |
| dispatch_trait | 2764ns | 2028ns | 1655ns | 2015ns | 6120ns | -1.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 207ns | 124ns | 457ns | base | 4.955 |
| dispatch_direct | 199ns | 125ns | 422ns | -3.48% | 5.133 |
| dispatch_trait | 204ns | 123ns | 447ns | -1.51% | 5.031 |

## Performance model

- Peak throughput: **8.339 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 6.822 | 81.8% |
| dispatch_direct | 6.822 | 81.8% |
| dispatch_trait | 6.822 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2804ns | 2804ns | base |
| dispatch_direct | 2704ns | 2704ns | -3.58% |
| dispatch_trait | 2764ns | 2764ns | -1.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 150ns | base | --- | [150, 150] | --- | --- | --- | --- |
| dispatch_direct | 150ns | no significant difference | [-0, +0]ns | [150, 150] | no | 0.2831 | 0.2831 | **35** (22%, HIGH) |
| dispatch_trait | 150ns | -0.1ns (-0.1%) | [-15, -0]ns | [150, 150] | YES | 0.0030 | 0.0015 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 150ns | +0.0% | -0.0% |
| 1 | 2 | 150ns | +0.0% | -18.1% |
| 1 | 3 | 150ns | -18.2% | +0.0% |
| 1 | 4 | 123ns | +0.0% | +22.2% |
| 1 | 5 | 150ns | +0.0% | -18.2% |
| 1 | 6 | 123ns | +22.2% | +22.2% |
| 1 | 7 | 150ns | -0.1% | +0.0% |
| 1 | 8 | 150ns | -0.0% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.374 | moderate- |
| dispatch_direct | 0.291 | moderate+ |
| dispatch_trait | -0.475 | moderate- |

**Consistency summary:**

- **dispatch_direct**: won 1/8, lost 1/8
- **dispatch_trait**: won 3/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 32.5ns | 206.7ns | 15.7% | HIGH |
| dispatch_direct | 31.5ns | 199.5ns | 15.8% | HIGH |
| dispatch_trait | 32.2ns | 203.6ns | 15.8% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 124.0-456.9 ns)
    124.0 |
    140.7 |########################################
    157.3 |##
    173.9 |
    190.6 |
    207.2 |
    223.9 |
    240.5 |
    257.2 |
    273.8 |
    290.4 |
    307.1 |
    323.7 |
    340.4 |
    357.0 |
    373.7 |
    390.3 |
    406.9 |
    423.6 |
    440.2 |#
  (30 below, 20 above range)

dispatch_direct (n=160, range 125.4-421.7 ns)
    125.4 |
    140.2 |########################################
    155.0 |
    169.8 |
    184.6 |
    199.5 |
    214.3 |
    229.1 |
    243.9 |
    258.7 |
    273.5 |
    288.4 |
    303.2 |
    318.0 |
    332.8 |
    347.6 |
    362.4 |
    377.3 |
    392.1 |
    406.9 |#
  (29 below, 20 above range)

dispatch_trait (n=160, range 122.8-446.9 ns)
    122.8 |#######################
    139.0 |########################################
    155.2 |#
    171.4 |
    187.6 |
    203.8 |
    220.0 |
    236.2 |
    252.4 |
    268.6 |
    284.9 |
    301.1 |
    317.3 |
    333.5 |
    349.7 |
    365.9 |
    382.1 |
    398.3 |
    414.5 |####
    430.7 |
  (3 below, 18 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=63.3% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **dispatch_const_ptr**: bridge=15.7% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=61.6% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **dispatch_direct**: bridge=15.7% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=61.7% (high variance, measurements may be unstable)
- **dispatch_trait**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dispatch_trait**: bridge=15.7% of algo (FFI overhead may distort results)
