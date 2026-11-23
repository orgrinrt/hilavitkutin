# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 16 ns to slowest 16 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### dispatch_const_ptr is fastest but the noisiest (CV 89.6%)

dispatch_const_ptr wins on median (16 ns) yet has the highest variance (CV 89.6%), while dispatch_direct is the steadiest (CV 86.6%, 16 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (14 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (16 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_direct's comparison is tie-heavy (31% tied pairs)

31% of paired samples for dispatch_direct are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_const_ptr vs stability leader dispatch_direct (+0% speed for 1.0x steadier)

dispatch_const_ptr is fastest (16 ns, CV 89.6%); dispatch_direct gives up 0.0% median for 1.0x lower variance (CV 86.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_trait is inconsistent: worst-20% is 3.8x its best-20%

dispatch_trait's best 20% of batches run at 13 ns but its worst 20% at 49 ns (3.8x) - a bimodal or bursty profile the median hides.

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
| Counter frequency | 2112810234 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2728ns | 1893ns | 1546ns | 2082ns | 5851ns | base |
| dispatch_direct | 2652ns | 1893ns | 1547ns | 1972ns | 5796ns | -2.80% |
| dispatch_trait | 2626ns | 1892ns | 1548ns | 1899ns | 5883ns | -3.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 23ns | 13ns | 49ns | base | 2.771 |
| dispatch_direct | 22ns | 13ns | 49ns | -2.68% | 2.848 |
| dispatch_trait | 22ns | 13ns | 49ns | -3.84% | 2.882 |

## Performance model

- Peak throughput: **4.862 Gops/s** (dispatch_const_ptr; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 3.975 | 81.8% |
| dispatch_direct | 3.975 | 81.8% |
| dispatch_trait | 3.975 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2728ns | 2728ns | base |
| dispatch_direct | 2652ns | 2652ns | -2.80% |
| dispatch_trait | 2626ns | 2626ns | -3.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 16ns | base | --- | [16, 16] | --- | --- | --- | --- |
| dispatch_direct | 16ns | no significant difference | [+0, +0]ns | [16, 16] | no | 0.5692 | 0.5692 | **49** (31%, HIGH) |
| dispatch_trait | 16ns | no significant difference | [+0, +0]ns | [16, 16] | no | 0.0741 | 0.0370 | **27** (17%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 16ns | +0.0% | +0.0% |
| 1 | 2 | 13ns | +1.1% | +0.0% |
| 1 | 3 | 16ns | +0.3% | -18.0% |
| 1 | 4 | 16ns | +0.0% | +0.0% |
| 1 | 5 | 13ns | +21.5% | -0.4% |
| 1 | 6 | 16ns | -0.6% | -0.6% |
| 1 | 7 | 13ns | +0.0% | +22.0% |
| 1 | 8 | 13ns | +0.0% | +22.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.376 | moderate- |
| dispatch_direct | 0.077 | ok |
| dispatch_trait | -0.168 | ok |

**Consistency summary:**

- **dispatch_direct**: won 1/8, lost 3/8
- **dispatch_trait**: won 3/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 34.2ns | 23.1ns | 148.0% | HIGH |
| dispatch_direct | 33.4ns | 22.5ns | 148.8% | HIGH |
| dispatch_trait | 32.9ns | 22.2ns | 147.9% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 13.2-49.2 ns)
     13.2 |###############################
     15.0 |########################################
     16.8 |
     18.6 |
     20.4 |
     22.2 |
     24.0 |
     25.8 |#
     27.6 |
     29.4 |
     31.2 |
     33.0 |
     34.8 |
     36.6 |
     38.4 |###
     40.2 |##
     42.0 |#
     43.8 |####
     45.6 |##
     47.4 |####
  (11 below, 13 above range)

dispatch_direct (n=160, range 13.2-48.7 ns)
     13.2 |##############
     14.9 |########################################
     16.7 |
     18.5 |
     20.3 |
     22.1 |
     23.8 |##
     25.6 |
     27.4 |
     29.2 |
     30.9 |
     32.7 |
     34.5 |#
     36.3 |
     38.0 |#
     39.8 |##
     41.6 |#
     43.4 |
     45.2 |#
     46.9 |#
  (11 below, 15 above range)

dispatch_trait (n=160, range 13.2-49.4 ns)
     13.2 |##################
     15.0 |########################################
     16.8 |
     18.6 |
     20.4 |
     22.2 |
     24.0 |
     25.9 |
     27.7 |
     29.5 |
     31.3 |
     33.1 |
     34.9 |
     36.7 |
     38.6 |
     40.4 |#
     42.2 |
     44.0 |###
     45.8 |#
     47.6 |##
  (9 below, 14 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=62.5% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **dispatch_const_ptr**: bridge=147.8% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=62.0% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **dispatch_direct**: bridge=147.8% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=64.1% (high variance, measurements may be unstable)
- **dispatch_trait**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dispatch_trait**: bridge=147.8% of algo (FFI overhead may distort results)
