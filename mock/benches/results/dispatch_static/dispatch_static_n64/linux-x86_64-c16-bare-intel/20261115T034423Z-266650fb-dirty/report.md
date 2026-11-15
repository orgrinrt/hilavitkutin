# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 16 ns to slowest 16 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (16 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (16 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_direct's comparison is tie-heavy (31% tied pairs)

31% of paired samples for dispatch_direct are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_const_ptr vs stability leader dispatch_direct (+0% speed for 1.1x steadier)

dispatch_const_ptr is fastest (16 ns, CV 96.4%); dispatch_direct gives up 0.0% median for 1.1x lower variance (CV 91.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_trait is inconsistent: worst-20% is 4.0x its best-20%

dispatch_trait's best 20% of batches run at 13 ns but its worst 20% at 53 ns (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 16.1 ns median
- Spread: 1.00x (fastest 16.1 ns, slowest 16.1 ns)

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
| dispatch_const_ptr | 2692ns | 1894ns | 1727ns | 1916ns | 5985ns | base |
| dispatch_direct | 2770ns | 1894ns | 1761ns | 1980ns | 6148ns | +2.90% |
| dispatch_trait | 2782ns | 1891ns | 1580ns | 2050ns | 6180ns | +3.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 23ns | 15ns | 52ns | base | 2.771 |
| dispatch_direct | 23ns | 15ns | 52ns | +1.51% | 2.730 |
| dispatch_trait | 24ns | 13ns | 53ns | +2.98% | 2.691 |

## Performance model

- Peak throughput: **4.776 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 3.975 | 83.2% |
| dispatch_direct | 3.975 | 83.2% |
| dispatch_trait | 3.975 | 83.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2692ns | 2692ns | base |
| dispatch_direct | 2770ns | 2770ns | +2.90% |
| dispatch_trait | 2782ns | 2782ns | +3.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 16ns | base | --- | [16, 16] | --- | --- | --- | --- |
| dispatch_direct | 16ns | no significant difference | [+0, +0]ns | [16, 16] | no | 1.0000 | 1.0000 | **49** (31%, HIGH) |
| dispatch_trait | 16ns | no significant difference | [+0, +0]ns | [16, 16] | no | 1.0000 | 0.9307 | **28** (18%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 16ns | -17.7% | +0.0% |
| 1 | 2 | 16ns | +0.0% | -18.0% |
| 1 | 3 | 16ns | +0.0% | +0.0% |
| 1 | 4 | 13ns | +22.0% | +22.0% |
| 1 | 5 | 16ns | +0.0% | +0.3% |
| 1 | 6 | 16ns | +0.0% | -18.0% |
| 1 | 7 | 16ns | +0.0% | +0.0% |
| 1 | 8 | 16ns | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.161 | ok |
| dispatch_direct | -0.018 | ok |
| dispatch_trait | -0.378 | moderate- |

**Consistency summary:**

- **dispatch_direct**: won 1/8, lost 1/8
- **dispatch_trait**: won 2/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 33.7ns | 23.1ns | 146.0% | HIGH |
| dispatch_direct | 34.6ns | 23.4ns | 147.7% | HIGH |
| dispatch_trait | 34.8ns | 23.8ns | 146.4% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 14.7-52.1 ns)
     14.7 |########################################
     16.5 |
     18.4 |
     20.3 |
     22.1 |
     24.0 |
     25.9 |
     27.7 |
     29.6 |
     31.5 |
     33.4 |
     35.2 |
     37.1 |
     39.0 |
     40.8 |#
     42.7 |
     44.6 |
     46.4 |
     48.3 |
     50.2 |
  (15 below, 17 above range)

dispatch_direct (n=160, range 15.0-51.5 ns)
     15.0 |########################################
     16.9 |
     18.7 |
     20.5 |
     22.3 |
     24.2 |
     26.0 |
     27.8 |
     29.6 |
     31.5 |
     33.3 |
     35.1 |
     36.9 |
     38.8 |
     40.6 |
     42.4 |
     44.2 |#
     46.1 |
     47.9 |
     49.7 |
  (11 below, 19 above range)

dispatch_trait (n=160, range 13.4-53.2 ns)
     13.4 |
     15.4 |########################################
     17.4 |
     19.4 |
     21.4 |
     23.4 |
     25.4 |
     27.3 |
     29.3 |
     31.3 |
     33.3 |
     35.3 |
     37.3 |
     39.3 |
     41.3 |#
     43.3 |
     45.3 |#
     47.3 |##
     49.3 |
     51.3 |##
  (29 below, 14 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=67.2% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dispatch_const_ptr**: bridge=147.8% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=62.8% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **dispatch_direct**: bridge=147.8% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=66.5% (high variance, measurements may be unstable)
- **dispatch_trait**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **dispatch_trait**: bridge=147.2% of algo (FFI overhead may distort results)
