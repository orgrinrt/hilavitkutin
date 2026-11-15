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

### dispatch_direct's comparison is tie-heavy (19% tied pairs)

19% of paired samples for dispatch_direct are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_const_ptr vs stability leader dispatch_direct (+0% speed for 1.1x steadier)

dispatch_const_ptr is fastest (2 ns, CV 46.2%); dispatch_direct gives up 0.0% median for 1.1x lower variance (CV 42.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_trait is inconsistent: worst-20% is 4.7x its best-20%

dispatch_trait's best 20% of batches run at 1 ns but its worst 20% at 4 ns (4.7x) - a bimodal or bursty profile the median hides.

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
| dispatch_const_ptr | 2250ns | 2237ns | 2080ns | 2249ns | 2427ns | base |
| dispatch_direct | 2269ns | 2395ns | 2076ns | 2284ns | 2421ns | +0.84% |
| dispatch_trait | 2245ns | 2229ns | 2081ns | 2245ns | 2412ns | -0.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 2ns | 1ns | 3ns | base | 30.946 |
| dispatch_direct | 2ns | 1ns | 3ns | -3.11% | 31.940 |
| dispatch_trait | 2ns | 1ns | 4ns | +0.33% | 30.843 |

## Performance model

- Peak throughput: **91.429 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 30.476 | 33.3% |
| dispatch_direct | 30.476 | 33.3% |
| dispatch_trait | 30.476 | 33.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2250ns | 2250ns | base |
| dispatch_direct | 2269ns | 2269ns | +0.84% |
| dispatch_trait | 2245ns | 2245ns | -0.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2ns | base | --- | [2, 2] | --- | --- | --- | --- |
| dispatch_direct | 2ns | no significant difference | [+0, +0]ns | [2, 2] | no | 0.5975 | 0.5975 | **31** (19%, HIGH) |
| dispatch_trait | 2ns | no significant difference | [-0, +0]ns | [2, 2] | no | 0.5975 | 0.5543 | **20** (12%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 2ns | +35.3% | +0.0% |
| 1 | 2 | 2ns | -16.0% | -16.0% |
| 1 | 3 | 2ns | +0.0% | +0.0% |
| 1 | 4 | 2ns | +19.0% | -9.5% |
| 1 | 5 | 2ns | +0.0% | -31.0% |
| 1 | 6 | 2ns | -10.5% | +31.6% |
| 1 | 7 | 2ns | -9.5% | -9.5% |
| 1 | 8 | 1ns | +44.8% | +86.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.217 | moderate- |
| dispatch_direct | 0.200 | moderate+ |
| dispatch_trait | -0.320 | moderate- |

**Consistency summary:**

- **dispatch_direct**: won 3/8, lost 3/8
- **dispatch_trait**: won 4/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 2.7ns | 2.1ns | 130.3% | HIGH |
| dispatch_direct | 2.7ns | 2.0ns | 135.8% | HIGH |
| dispatch_trait | 2.6ns | 2.1ns | 126.4% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 0.8-3.4 ns)
      0.8 |######################
      0.9 |
      1.1 |
      1.2 |###############################
      1.3 |
      1.5 |
      1.6 |########################################
      1.7 |
      1.8 |
      2.0 |################################
      2.1 |
      2.2 |
      2.4 |###############################
      2.5 |
      2.6 |
      2.8 |###########################
      2.9 |
      3.0 |
      3.2 |##################
      3.3 |
  (7 below, 10 above range)

dispatch_direct (n=160, range 0.7-3.2 ns)
      0.7 |########################
      0.8 |
      0.9 |
      1.1 |
      1.2 |######################
      1.3 |
      1.4 |
      1.6 |
      1.7 |###################################
      1.8 |
      1.9 |
      2.1 |#################################
      2.2 |
      2.3 |
      2.4 |########################################
      2.6 |
      2.7 |
      2.8 |##############################
      2.9 |
      3.1 |
  (10 below, 15 above range)

dispatch_trait (n=160, range 0.8-3.6 ns)
      0.8 |########################
      0.9 |
      1.0 |
      1.2 |###################################
      1.3 |
      1.5 |
      1.6 |########################################
      1.8 |
      1.9 |
      2.0 |###################################
      2.2 |
      2.3 |
      2.5 |########################################
      2.6 |
      2.7 |
      2.9 |####################
      3.0 |
      3.2 |################
      3.3 |
      3.4 |
  (8 below, 14 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=47.0% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **dispatch_const_ptr**: bridge=119.0% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=44.8% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **dispatch_direct**: bridge=119.0% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=49.5% (high variance, measurements may be unstable)
- **dispatch_trait**: worst_20/best_20 = 4.7x (possible bimodal distribution)
- **dispatch_trait**: bridge=119.0% of algo (FFI overhead may distort results)
