# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 150 ns to slowest 150 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (132 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### dispatch_trait's comparison is tie-heavy (19% tied pairs)

19% of paired samples for dispatch_trait are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_direct vs stability leader dispatch_trait (+0% speed for 1.1x steadier)

dispatch_direct is fastest (150 ns, CV 87.8%); dispatch_trait gives up 0.1% median for 1.1x lower variance (CV 81.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_const_ptr is inconsistent: worst-20% is 3.8x its best-20%

dispatch_const_ptr's best 20% of batches run at 123 ns but its worst 20% at 462 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: dispatch_direct** at 150.0 ns median (-0.1% vs baseline)
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
| Counter frequency | 2112810234 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2812ns | 2028ns | 1658ns | 2028ns | 6315ns | base |
| dispatch_direct | 2911ns | 2026ns | 1657ns | 2212ns | 6261ns | +3.52% |
| dispatch_trait | 2644ns | 2026ns | 1657ns | 1920ns | 5804ns | -5.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 207ns | 123ns | 462ns | base | 4.944 |
| dispatch_direct | 214ns | 123ns | 458ns | +3.38% | 4.782 |
| dispatch_trait | 195ns | 123ns | 425ns | -5.99% | 5.259 |

## Performance model

- Peak throughput: **8.347 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 6.822 | 81.7% |
| dispatch_direct | 6.827 | 81.8% |
| dispatch_trait | 6.822 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2812ns | 2812ns | base |
| dispatch_direct | 2911ns | 2911ns | +3.52% |
| dispatch_trait | 2644ns | 2644ns | -5.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 150ns | base | --- | [150, 150] | --- | --- | --- | --- |
| dispatch_direct | 150ns | no significant difference | [-0, +0]ns | [150, 150] | no (adj: YES) | 0.0081 | 0.0040 | **19** (12%, HIGH) |
| dispatch_trait | 150ns | no significant difference | [-0, +0]ns | [150, 150] | no | 0.4300 | 0.4300 | **30** (19%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 123ns | +22.1% | +0.3% |
| 1 | 2 | 123ns | -0.1% | -0.0% |
| 1 | 3 | 150ns | +151.4% | +0.1% |
| 1 | 4 | 150ns | -0.1% | +0.0% |
| 1 | 5 | 150ns | -18.3% | -18.2% |
| 1 | 6 | 123ns | -0.1% | +22.3% |
| 1 | 7 | 150ns | -0.1% | +0.0% |
| 1 | 8 | 150ns | -0.1% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.057 | ok |
| dispatch_direct | -0.164 | ok |
| dispatch_trait | -0.132 | ok |

**Consistency summary:**

- **dispatch_direct**: won 1/8, lost 2/8
- **dispatch_trait**: won 2/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 32.8ns | 207.1ns | 15.8% | HIGH |
| dispatch_direct | 34.3ns | 214.1ns | 16.0% | HIGH |
| dispatch_trait | 30.9ns | 194.7ns | 15.8% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 122.8-462.0 ns)
    122.8 |###################
    139.8 |########################################
    156.7 |#
    173.7 |
    190.6 |
    207.6 |
    224.6 |
    241.5 |
    258.5 |
    275.5 |##
    292.4 |
    309.4 |
    326.3 |
    343.3 |
    360.3 |
    377.2 |#
    394.2 |#
    411.2 |#
    428.1 |
    445.1 |##
  (7 below, 18 above range)

dispatch_direct (n=160, range 122.7-458.4 ns)
    122.7 |##########################
    139.5 |########################################
    156.3 |
    173.0 |
    189.8 |
    206.6 |#
    223.4 |
    240.2 |
    257.0 |##
    273.8 |
    290.6 |#
    307.3 |
    324.1 |
    340.9 |
    357.7 |
    374.5 |###
    391.3 |
    408.1 |##
    424.9 |
    441.6 |#
  (7 below, 16 above range)

dispatch_trait (n=160, range 122.8-424.7 ns)
    122.8 |################################
    137.9 |########################################
    153.0 |#
    168.1 |
    183.2 |
    198.3 |
    213.4 |
    228.5 |
    243.6 |
    258.7 |
    273.8 |
    288.9 |
    304.0 |
    319.0 |
    334.1 |#
    349.2 |#
    364.3 |
    379.4 |#
    394.5 |
    409.6 |##
  (7 below, 17 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=64.4% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dispatch_const_ptr**: bridge=15.8% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=61.5% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **dispatch_direct**: bridge=15.9% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=63.2% (high variance, measurements may be unstable)
- **dispatch_trait**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **dispatch_trait**: bridge=15.7% of algo (FFI overhead may distort results)
