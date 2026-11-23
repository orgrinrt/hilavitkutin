# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.2% (fastest 43 ns to slowest 43 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### dispatch_direct shows alternating (throttle bounce) (autocorr -0.68)

dispatch_direct's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (38 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### dispatch_direct's comparison is tie-heavy (28% tied pairs)

28% of paired samples for dispatch_direct are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_trait vs stability leader dispatch_direct (+0% speed for 1.0x steadier)

dispatch_trait is fastest (43 ns, CV 88.8%); dispatch_direct gives up 0.2% median for 1.0x lower variance (CV 87.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_const_ptr is inconsistent: worst-20% is 3.8x its best-20%

dispatch_const_ptr's best 20% of batches run at 35 ns but its worst 20% at 134 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: dispatch_trait** at 43.0 ns median (-0.2% vs baseline)
- Spread: 1.00x (fastest 43.0 ns, slowest 43.1 ns)

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
| dispatch_const_ptr | 2682ns | 1920ns | 1569ns | 1924ns | 6069ns | base |
| dispatch_direct | 2764ns | 1921ns | 1570ns | 2125ns | 5873ns | +3.04% |
| dispatch_trait | 2930ns | 1919ns | 1568ns | 2455ns | 5715ns | +9.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 60ns | 35ns | 134ns | base | 4.272 |
| dispatch_direct | 62ns | 35ns | 131ns | +3.45% | 4.130 |
| dispatch_trait | 65ns | 35ns | 127ns | +9.07% | 3.917 |

## Performance model

- Peak throughput: **7.273 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 5.940 | 81.7% |
| dispatch_direct | 5.940 | 81.7% |
| dispatch_trait | 5.953 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2682ns | 2682ns | base |
| dispatch_direct | 2764ns | 2764ns | +3.04% |
| dispatch_trait | 2930ns | 2930ns | +9.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 43ns | base | --- | [43, 43] | --- | --- | --- | --- |
| dispatch_direct | 43ns | no significant difference | [+0, +0]ns | [43, 43] | no | 1.0000 | 1.0000 | **45** (28%, HIGH) |
| dispatch_trait | 43ns | no significant difference | [-0, +0]ns | [39, 43] | no | 0.0684 | 0.0342 | **31** (19%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 43ns | +0.0% | -18.1% |
| 1 | 2 | 35ns | +0.0% | -0.3% |
| 1 | 3 | 43ns | +0.1% | -18.1% |
| 1 | 4 | 35ns | +0.0% | +0.0% |
| 1 | 5 | 43ns | +0.1% | +139.9% |
| 1 | 6 | 35ns | +0.0% | +0.0% |
| 1 | 7 | 43ns | +0.0% | +0.0% |
| 1 | 8 | 43ns | +0.1% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.678 | HIGH- (thermal bounce) |
| dispatch_direct | -0.679 | HIGH- (thermal bounce) |
| dispatch_trait | -0.216 | moderate- |

**Consistency summary:**

- **dispatch_direct**: won 0/8, lost 3/8
- **dispatch_trait**: won 3/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 32.8ns | 59.9ns | 54.7% | HIGH |
| dispatch_direct | 33.7ns | 62.0ns | 54.4% | HIGH |
| dispatch_trait | 35.7ns | 65.4ns | 54.7% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 35.3-134.4 ns)
     35.3 |#############
     40.2 |########################################
     45.2 |#
     50.1 |
     55.1 |
     60.0 |
     65.0 |
     69.9 |
     74.9 |
     79.9 |
     84.8 |
     89.8 |
     94.7 |
     99.7 |
    104.6 |
    109.6 |#
    114.5 |
    119.5 |
    124.4 |#
    129.4 |
  (13 below, 16 above range)

dispatch_direct (n=160, range 35.2-131.1 ns)
     35.2 |#############
     40.0 |########################################
     44.8 |
     49.6 |
     54.4 |#
     59.2 |
     64.0 |
     68.8 |
     73.6 |
     78.4 |#
     83.2 |
     88.0 |
     92.8 |
     97.6 |#
    102.4 |#
    107.2 |###
    111.9 |#
    116.7 |###
    121.5 |
    126.3 |##
  (14 below, 13 above range)

dispatch_trait (n=160, range 35.2-126.5 ns)
     35.2 |########################################
     39.8 |###################
     44.3 |
     48.9 |
     53.5 |
     58.0 |
     62.6 |
     67.2 |
     71.7 |
     76.3 |#
     80.9 |#
     85.4 |
     90.0 |
     94.6 |#
     99.1 |######
    103.7 |
    108.3 |#####
    112.8 |#
    117.4 |####
    122.0 |#
  (1 below, 13 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=64.1% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dispatch_const_ptr**: bridge=54.8% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=61.0% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **dispatch_direct**: bridge=54.5% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=58.4% (high variance, measurements may be unstable)
- **dispatch_trait**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dispatch_trait**: bridge=54.7% of algo (FFI overhead may distort results)
