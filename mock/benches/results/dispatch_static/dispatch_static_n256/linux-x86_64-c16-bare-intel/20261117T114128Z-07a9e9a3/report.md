# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 43 ns to slowest 43 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### dispatch_direct shows warm-up / thermal drift (autocorr +0.59)

dispatch_direct's per-pass series has lag-1 autocorrelation +0.59, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (37 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (43 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_direct's comparison is tie-heavy (19% tied pairs)

19% of paired samples for dispatch_direct are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### dispatch_trait is inconsistent: worst-20% is 3.9x its best-20%

dispatch_trait's best 20% of batches run at 35 ns but its worst 20% at 137 ns (3.9x) - a bimodal or bursty profile the median hides.

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
| Counter frequency | 2112808726 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2629ns | 1927ns | 1610ns | 1931ns | 5740ns | base |
| dispatch_direct | 2731ns | 1927ns | 1575ns | 2024ns | 6009ns | +3.90% |
| dispatch_trait | 2730ns | 1929ns | 1576ns | 1958ns | 6200ns | +3.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 59ns | 36ns | 127ns | base | 4.372 |
| dispatch_direct | 61ns | 35ns | 134ns | +4.30% | 4.192 |
| dispatch_trait | 61ns | 35ns | 137ns | +3.73% | 4.215 |

## Performance model

- Peak throughput: **7.269 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 5.940 | 81.7% |
| dispatch_direct | 5.940 | 81.7% |
| dispatch_trait | 5.940 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2629ns | 2629ns | base |
| dispatch_direct | 2731ns | 2731ns | +3.90% |
| dispatch_trait | 2730ns | 2730ns | +3.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 43ns | base | --- | [43, 43] | --- | --- | --- | --- |
| dispatch_direct | 43ns | no significant difference | [-0, +0]ns | [43, 43] | no | 0.6612 | 0.6612 | **30** (19%, HIGH) |
| dispatch_trait | 43ns | no significant difference | [-0, +0]ns | [43, 43] | no | 0.6105 | 0.3052 | **23** (14%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 43ns | +0.0% | +0.0% |
| 1 | 2 | 43ns | +0.0% | +0.0% |
| 1 | 3 | 43ns | +0.0% | +3.8% |
| 1 | 4 | 43ns | +0.1% | +0.0% |
| 1 | 5 | 35ns | +22.1% | +0.0% |
| 1 | 6 | 43ns | -18.1% | -18.1% |
| 1 | 7 | 44ns | -19.6% | -19.6% |
| 1 | 8 | 35ns | +0.0% | +22.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.238 | moderate- |
| dispatch_direct | 0.593 | HIGH+ (drift/warm-up) |
| dispatch_trait | 0.437 | moderate+ |

**Consistency summary:**

- **dispatch_direct**: won 2/8, lost 2/8
- **dispatch_trait**: won 2/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 32.3ns | 58.5ns | 55.1% | HIGH |
| dispatch_direct | 33.6ns | 61.1ns | 55.0% | HIGH |
| dispatch_trait | 33.6ns | 60.7ns | 55.4% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 36.0-126.9 ns)
     36.0 |
     40.5 |########################################
     45.1 |
     49.6 |
     54.2 |
     58.7 |
     63.3 |
     67.8 |
     72.4 |
     76.9 |
     81.5 |
     86.0 |
     90.6 |
     95.1 |
     99.7 |
    104.2 |
    108.7 |
    113.3 |
    117.8 |#
    122.4 |
  (29 below, 18 above range)

dispatch_direct (n=160, range 35.3-134.3 ns)
     35.3 |##############
     40.2 |########################################
     45.2 |
     50.1 |
     55.1 |
     60.0 |
     65.0 |
     69.9 |
     74.9 |
     79.8 |
     84.8 |
     89.7 |
     94.7 |
     99.6 |
    104.6 |#
    109.5 |#
    114.5 |###
    119.4 |#
    124.4 |#
    129.3 |#
  (15 below, 14 above range)

dispatch_trait (n=160, range 35.2-137.1 ns)
     35.2 |############
     40.3 |########################################
     45.4 |##
     50.5 |
     55.6 |
     60.7 |#
     65.8 |
     70.9 |
     76.0 |#
     81.1 |
     86.1 |
     91.2 |
     96.3 |
    101.4 |
    106.5 |
    111.6 |#
    116.7 |#
    121.8 |
    126.9 |####
    132.0 |#
  (22 below, 16 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=63.3% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **dispatch_const_ptr**: bridge=54.8% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=63.8% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dispatch_direct**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **dispatch_direct**: bridge=54.8% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=64.5% (high variance, measurements may be unstable)
- **dispatch_trait**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **dispatch_trait**: bridge=54.8% of algo (FFI overhead may distort results)
