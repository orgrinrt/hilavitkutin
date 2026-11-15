# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 43 ns to slowest 43 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (39 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (43 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_trait's comparison is tie-heavy (24% tied pairs)

24% of paired samples for dispatch_trait are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_const_ptr vs stability leader dispatch_trait (+0% speed for 1.0x steadier)

dispatch_const_ptr is fastest (43 ns, CV 89.9%); dispatch_trait gives up 0.0% median for 1.0x lower variance (CV 88.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_trait is inconsistent: worst-20% is 3.6x its best-20%

dispatch_trait's best 20% of batches run at 37 ns but its worst 20% at 134 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 43.1 ns median
- Spread: 1.00x (fastest 43.1 ns, slowest 43.1 ns)

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
| dispatch_const_ptr | 2814ns | 1920ns | 1763ns | 2025ns | 6231ns | base |
| dispatch_direct | 2866ns | 1923ns | 1752ns | 2175ns | 6051ns | +1.83% |
| dispatch_trait | 2831ns | 1920ns | 1656ns | 2152ns | 6044ns | +0.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 63ns | 40ns | 138ns | base | 4.072 |
| dispatch_direct | 65ns | 39ns | 137ns | +3.69% | 3.927 |
| dispatch_trait | 63ns | 37ns | 134ns | +0.37% | 4.057 |

## Performance model

- Peak throughput: **6.892 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 5.940 | 86.2% |
| dispatch_direct | 5.940 | 86.2% |
| dispatch_trait | 5.940 | 86.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2814ns | 2814ns | base |
| dispatch_direct | 2866ns | 2866ns | +1.83% |
| dispatch_trait | 2831ns | 2831ns | +0.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 43ns | base | --- | [43, 43] | --- | --- | --- | --- |
| dispatch_direct | 43ns | no significant difference | [-0, +0]ns | [43, 43] | no | 0.1603 | 0.0802 | **29** (18%, HIGH) |
| dispatch_trait | 43ns | no significant difference | [+0, +0]ns | [43, 43] | no | 0.6510 | 0.6510 | **38** (24%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 43ns | +0.0% | +0.0% |
| 1 | 2 | 35ns | +108.2% | +0.3% |
| 1 | 3 | 43ns | -0.1% | -0.2% |
| 1 | 4 | 43ns | -0.2% | +0.0% |
| 1 | 5 | 43ns | -0.2% | +71.6% |
| 1 | 6 | 43ns | +0.0% | +0.0% |
| 1 | 7 | 43ns | -18.1% | +0.2% |
| 1 | 8 | 43ns | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.170 | ok |
| dispatch_direct | -0.075 | ok |
| dispatch_trait | -0.078 | ok |

**Consistency summary:**

- **dispatch_direct**: won 4/8, lost 1/8
- **dispatch_trait**: won 1/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 34.3ns | 62.9ns | 54.6% | HIGH |
| dispatch_direct | 35.0ns | 65.2ns | 53.8% | HIGH |
| dispatch_trait | 34.5ns | 63.1ns | 54.7% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 39.5-137.8 ns)
     39.5 |########################################
     44.5 |
     49.4 |
     54.3 |
     59.2 |
     64.1 |
     69.0 |
     73.9 |
     78.8 |
     83.8 |
     88.7 |
     93.6 |
     98.5 |
    103.4 |
    108.3 |
    113.2 |
    118.1 |#
    123.1 |
    128.0 |###
    132.9 |#
  (14 below, 15 above range)

dispatch_direct (n=160, range 39.3-137.2 ns)
     39.3 |########################################
     44.2 |
     49.1 |
     54.0 |
     58.9 |
     63.8 |
     68.7 |#######
     73.6 |
     78.5 |
     83.4 |
     88.3 |
     93.1 |
     98.0 |
    102.9 |
    107.8 |
    112.7 |#
    117.6 |#
    122.5 |
    127.4 |###
    132.3 |
  (15 below, 14 above range)

dispatch_trait (n=160, range 37.1-133.8 ns)
     37.1 |
     42.0 |########################################
     46.8 |
     51.7 |
     56.5 |
     61.3 |
     66.2 |
     71.0 |
     75.8 |
     80.7 |
     85.5 |
     90.3 |##
     95.2 |#
    100.0 |
    104.8 |
    109.7 |
    114.5 |
    119.3 |
    124.2 |#
    129.0 |##
  (24 below, 16 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=61.7% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **dispatch_const_ptr**: bridge=54.5% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=59.9% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **dispatch_direct**: bridge=54.8% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=60.4% (high variance, measurements may be unstable)
- **dispatch_trait**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dispatch_trait**: bridge=54.5% of algo (FFI overhead may distort results)
