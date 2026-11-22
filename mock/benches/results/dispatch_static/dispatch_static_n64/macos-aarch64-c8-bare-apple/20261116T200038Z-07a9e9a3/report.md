# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 2 ns to slowest 2 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### dispatch_const_ptr is fastest but the noisiest (CV 54.8%)

dispatch_const_ptr wins on median (2 ns) yet has the highest variance (CV 54.8%), while dispatch_trait is the steadiest (CV 49.6%, 2 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (1 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (2 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_trait's comparison is tie-heavy (15% tied pairs)

15% of paired samples for dispatch_trait are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_const_ptr vs stability leader dispatch_trait (+0% speed for 1.1x steadier)

dispatch_const_ptr is fastest (2 ns, CV 54.8%); dispatch_trait gives up 0.0% median for 1.1x lower variance (CV 49.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_const_ptr is inconsistent: worst-20% is 4.7x its best-20%

dispatch_const_ptr's best 20% of batches run at 1 ns but its worst 20% at 3 ns (4.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 1.7 ns median
- Spread: 1.00x (fastest 1.7 ns, slowest 1.7 ns)

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
| dispatch_const_ptr | 2122ns | 2096ns | 2089ns | 2103ns | 2210ns | base |
| dispatch_direct | 2101ns | 2095ns | 2089ns | 2095ns | 2134ns | -0.95% |
| dispatch_trait | 2104ns | 2095ns | 2088ns | 2095ns | 2148ns | -0.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 2ns | 1ns | 3ns | base | 34.641 |
| dispatch_direct | 2ns | 1ns | 3ns | +3.79% | 33.377 |
| dispatch_trait | 2ns | 1ns | 3ns | -0.47% | 34.806 |

## Performance model

- Peak throughput: **91.429 Gops/s** (dispatch_const_ptr; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 37.647 | 41.2% |
| dispatch_direct | 37.647 | 41.2% |
| dispatch_trait | 37.647 | 41.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2122ns | 2122ns | base |
| dispatch_direct | 2101ns | 2101ns | -0.95% |
| dispatch_trait | 2104ns | 2104ns | -0.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2ns | base | --- | [2, 2] | --- | --- | --- | --- |
| dispatch_direct | 2ns | no significant difference | [+0, +0]ns | [2, 2] | no | 0.3553 | 0.1776 | **19** (12%, HIGH) |
| dispatch_trait | 2ns | no significant difference | [+0, +0]ns | [2, 2] | no | 0.9317 | 0.9317 | **24** (15%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 2ns | +0.0% | +11.8% |
| 1 | 2 | 2ns | +23.5% | -14.7% |
| 1 | 3 | 2ns | +23.5% | +0.0% |
| 1 | 4 | 1ns | +58.3% | +75.0% |
| 1 | 5 | 2ns | -10.5% | -10.5% |
| 1 | 6 | 2ns | -10.5% | +10.5% |
| 1 | 7 | 2ns | +0.0% | -19.0% |
| 1 | 8 | 2ns | +35.3% | +11.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.015 | ok |
| dispatch_direct | 0.178 | ok |
| dispatch_trait | -0.370 | moderate- |

**Consistency summary:**

- **dispatch_direct**: won 2/8, lost 4/8
- **dispatch_trait**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 2.6ns | 1.8ns | 139.6% | HIGH |
| dispatch_direct | 2.6ns | 1.9ns | 133.4% | HIGH |
| dispatch_trait | 2.5ns | 1.8ns | 135.8% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 0.7-3.3 ns)
      0.7 |######################################
      0.8 |
      1.0 |
      1.1 |###################################
      1.2 |
      1.3 |
      1.5 |
      1.6 |###################################
      1.7 |
      1.9 |
      2.0 |########################################
      2.1 |
      2.2 |
      2.4 |
      2.5 |####################
      2.6 |
      2.8 |
      2.9 |################
      3.0 |
      3.1 |
  (7 below, 17 above range)

dispatch_direct (n=160, range 0.7-3.1 ns)
      0.7 |####################
      0.8 |
      1.0 |
      1.1 |
      1.2 |####################
      1.3 |
      1.4 |
      1.6 |
      1.7 |########################################
      1.8 |
      1.9 |
      2.0 |############################
      2.2 |
      2.3 |
      2.4 |############################
      2.5 |
      2.6 |
      2.8 |
      2.9 |############
      3.0 |
  (9 below, 16 above range)

dispatch_trait (n=160, range 0.7-3.0 ns)
      0.7 |#################################
      0.8 |
      0.9 |
      1.1 |
      1.2 |####################################
      1.3 |
      1.4 |
      1.5 |
      1.6 |##############################
      1.8 |
      1.9 |
      2.0 |########################################
      2.1 |
      2.2 |
      2.3 |
      2.4 |##################################
      2.6 |
      2.7 |
      2.8 |#########
      2.9 |
  (7 below, 15 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=50.4% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: worst_20/best_20 = 4.7x (possible bimodal distribution)
- **dispatch_const_ptr**: bridge=147.1% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=44.9% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **dispatch_direct**: bridge=147.1% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=45.8% (high variance, measurements may be unstable)
- **dispatch_trait**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **dispatch_trait**: bridge=147.1% of algo (FFI overhead may distort results)
