# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 2 ns to slowest 2 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (1 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (2 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_trait's comparison is tie-heavy (14% tied pairs)

14% of paired samples for dispatch_trait are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### dispatch_direct is inconsistent: worst-20% is 4.6x its best-20%

dispatch_direct's best 20% of batches run at 1 ns but its worst 20% at 4 ns (4.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 2.1 ns median
- Spread: 1.00x (fastest 2.1 ns, slowest 2.1 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2088ns | 2081ns | 2073ns | 2081ns | 2122ns | base |
| dispatch_direct | 2119ns | 2086ns | 2075ns | 2098ns | 2225ns | +1.48% |
| dispatch_trait | 2088ns | 2084ns | 2075ns | 2084ns | 2114ns | +0.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 2ns | 1ns | 3ns | base | 29.854 |
| dispatch_direct | 2ns | 1ns | 4ns | +0.79% | 29.621 |
| dispatch_trait | 2ns | 1ns | 4ns | -2.45% | 30.604 |

## Performance model

- Peak throughput: **82.581 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 30.476 | 36.9% |
| dispatch_direct | 30.476 | 36.9% |
| dispatch_trait | 30.476 | 36.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2088ns | 2088ns | base |
| dispatch_direct | 2119ns | 2119ns | +1.48% |
| dispatch_trait | 2088ns | 2088ns | +0.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2ns | base | --- | [2, 2] | --- | --- | --- | --- |
| dispatch_direct | 2ns | no significant difference | [-0, +0]ns | [2, 2] | no | 0.6084 | 0.5066 | 15 |
| dispatch_trait | 2ns | no significant difference | [-0, +0]ns | [2, 2] | no | 0.6084 | 0.6084 | **23** (14%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 2ns | -26.1% | -26.1% |
| 1 | 2 | 2ns | -19.0% | +9.5% |
| 1 | 3 | 2ns | +47.6% | -19.0% |
| 1 | 4 | 2ns | +19.0% | -9.5% |
| 1 | 5 | 2ns | +0.0% | +0.0% |
| 1 | 6 | 2ns | -19.0% | -9.5% |
| 1 | 7 | 2ns | +11.8% | +35.3% |
| 1 | 8 | 2ns | +21.1% | +10.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.261 | moderate+ |
| dispatch_direct | 0.110 | ok |
| dispatch_trait | -0.425 | moderate- |

**Consistency summary:**

- **dispatch_direct**: won 3/8, lost 4/8
- **dispatch_trait**: won 4/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 2.5ns | 2.1ns | 118.7% | HIGH |
| dispatch_direct | 2.6ns | 2.2ns | 118.2% | HIGH |
| dispatch_trait | 2.4ns | 2.1ns | 116.6% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 0.9-3.5 ns)
      0.9 |
      1.0 |
      1.2 |################################
      1.3 |
      1.4 |
      1.5 |
      1.7 |######################################
      1.8 |
      1.9 |
      2.1 |########################################
      2.2 |
      2.3 |
      2.4 |########################
      2.6 |
      2.7 |
      2.8 |##################################
      3.0 |
      3.1 |
      3.2 |#################
      3.3 |
  (17 below, 12 above range)

dispatch_direct (n=160, range 0.8-3.9 ns)
      0.8 |
      1.0 |
      1.1 |######################################
      1.3 |
      1.4 |
      1.6 |########################################
      1.7 |
      1.9 |
      2.1 |#####################################
      2.2 |
      2.4 |################################
      2.5 |
      2.7 |
      2.8 |#############
      3.0 |
      3.1 |
      3.3 |#################
      3.4 |
      3.6 |################
      3.7 |
  (19 below, 9 above range)

dispatch_trait (n=160, range 0.8-3.5 ns)
      0.8 |###############
      0.9 |
      1.0 |
      1.2 |#######################
      1.3 |
      1.5 |
      1.6 |#################################
      1.7 |
      1.9 |
      2.0 |########################################
      2.1 |
      2.3 |
      2.4 |#######################
      2.6 |
      2.7 |
      2.8 |#####################
      3.0 |
      3.1 |
      3.2 |################
      3.4 |
  (10 below, 11 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=43.8% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **dispatch_const_ptr**: bridge=119.0% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=51.3% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **dispatch_direct**: bridge=119.0% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=47.1% (high variance, measurements may be unstable)
- **dispatch_trait**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **dispatch_trait**: bridge=119.0% of algo (FFI overhead may distort results)
