# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_const_ptr is fastest but the noisiest (CV 87.7%)

dispatch_const_ptr wins on median (150 ns) yet has the highest variance (CV 87.7%), while dispatch_trait is the steadiest (CV 81.8%, 150 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (132 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (150 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_direct's comparison is tie-heavy (14% tied pairs)

14% of paired samples for dispatch_direct are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_const_ptr vs stability leader dispatch_trait (+0% speed for 1.1x steadier)

dispatch_const_ptr is fastest (150 ns, CV 87.7%); dispatch_trait gives up 0.1% median for 1.1x lower variance (CV 81.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_direct is inconsistent: worst-20% is 3.6x its best-20%

dispatch_direct's best 20% of batches run at 123 ns but its worst 20% at 443 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.1% of the fastest

All 3 variants sit between 150 ns and 150 ns - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### dispatch_direct's edge over baseline is significant but tiny (-0 ns, 0.07%)

dispatch_direct differs from baseline dispatch_const_ptr by -0 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 150.0 ns median
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 150.0 ns, slowest 150.1 ns)

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
| Counter frequency | 2112808726 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2637ns | 2031ns | 1661ns | 1897ns | 5831ns | base |
| dispatch_direct | 2876ns | 2034ns | 1662ns | 2210ns | 6087ns | +9.05% |
| dispatch_trait | 2788ns | 2035ns | 1663ns | 2112ns | 5941ns | +5.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 194ns | 123ns | 426ns | base | 5.288 |
| dispatch_direct | 211ns | 123ns | 443ns | +8.93% | 4.855 |
| dispatch_trait | 205ns | 123ns | 434ns | +5.92% | 4.993 |

## Performance model

- Peak throughput: **8.345 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 6.827 | 81.8% |
| dispatch_direct | 6.827 | 81.8% |
| dispatch_trait | 6.822 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2637ns | 2637ns | base |
| dispatch_direct | 2876ns | 2876ns | +9.05% |
| dispatch_trait | 2788ns | 2788ns | +5.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 150ns | base | --- | [136, 150] | --- | --- | --- | --- |
| dispatch_direct | 150ns | -0.1ns (-0.1%) | [-0, -0]ns | [150, 150] | YES | 0.0000 | 0.0000 | **23** (14%, HIGH) |
| dispatch_trait | 150ns | +5.2ns (+3.4%) | [+0, +27]ns | [150, 150] | YES | 0.0003 | 0.0003 | 16 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 150ns | -0.1% | -18.2% |
| 1 | 2 | 150ns | -0.1% | +0.1% |
| 1 | 3 | 123ns | +149.3% | +22.2% |
| 1 | 4 | 150ns | -0.1% | +9.0% |
| 1 | 5 | 123ns | +0.0% | +0.0% |
| 1 | 6 | 123ns | +0.0% | +22.2% |
| 1 | 7 | 123ns | +0.0% | +22.2% |
| 1 | 8 | 150ns | -0.1% | -18.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.125 | ok |
| dispatch_direct | 0.027 | ok |
| dispatch_trait | -0.306 | moderate- |

**Consistency summary:**

- **dispatch_direct**: won 0/8, lost 1/8
- **dispatch_trait**: won 2/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 31.0ns | 193.6ns | 16.0% | HIGH |
| dispatch_direct | 33.5ns | 210.9ns | 15.9% | HIGH |
| dispatch_trait | 32.5ns | 205.1ns | 15.9% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 122.8-425.8 ns)
    122.8 |########################################
    137.9 |########################################
    153.1 |#
    168.2 |
    183.4 |
    198.5 |
    213.7 |#
    228.8 |
    244.0 |
    259.1 |
    274.3 |
    289.4 |
    304.6 |
    319.7 |#
    334.9 |#
    350.0 |
    365.2 |
    380.3 |
    395.5 |
    410.6 |
  (7 below, 20 above range)

dispatch_direct (n=160, range 122.7-443.0 ns)
    122.7 |############
    138.7 |########################################
    154.7 |
    170.7 |
    186.8 |#
    202.8 |
    218.8 |
    234.8 |#
    250.8 |
    266.8 |#
    282.8 |
    298.9 |#
    314.9 |#
    330.9 |#
    346.9 |
    362.9 |#
    378.9 |
    394.9 |
    410.9 |####
    427.0 |
  (25 below, 17 above range)

dispatch_trait (n=160, range 122.8-434.4 ns)
    122.8 |##################
    138.4 |########################################
    154.0 |####
    169.5 |#
    185.1 |
    200.7 |
    216.3 |
    231.9 |
    247.4 |
    263.0 |
    278.6 |
    294.2 |
    309.8 |
    325.3 |###
    340.9 |
    356.5 |#
    372.1 |#
    387.7 |#
    403.2 |
    418.8 |
  (6 below, 15 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=67.9% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **dispatch_const_ptr**: bridge=15.7% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=59.5% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dispatch_direct**: bridge=15.9% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=59.8% (high variance, measurements may be unstable)
- **dispatch_trait**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **dispatch_trait**: bridge=15.9% of algo (FFI overhead may distort results)
