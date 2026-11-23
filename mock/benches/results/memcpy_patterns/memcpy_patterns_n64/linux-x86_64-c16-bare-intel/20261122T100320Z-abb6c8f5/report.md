# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 78 ns to slowest 78 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### memcpy_intrinsic is fastest but the noisiest (CV 89.3%)

memcpy_intrinsic wins on median (78 ns) yet has the highest variance (CV 89.3%), while memcpy_unrolled is the steadiest (CV 84.6%, 78 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (70 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (memcpy_intrinsic)

The baseline memcpy_intrinsic is the fastest (78 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### memcpy_unrolled's comparison is tie-heavy (12% tied pairs)

12% of paired samples for memcpy_unrolled are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader memcpy_intrinsic vs stability leader memcpy_unrolled (+0% speed for 1.1x steadier)

memcpy_intrinsic is fastest (78 ns, CV 89.3%); memcpy_unrolled gives up 0.0% median for 1.1x lower variance (CV 84.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### memcpy_intrinsic is inconsistent: worst-20% is 3.8x its best-20%

memcpy_intrinsic's best 20% of batches run at 64 ns but its worst 20% at 242 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (memcpy_intrinsic) is the fastest** at 77.9 ns median
- Spread: 1.00x (fastest 77.9 ns, slowest 77.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4646464650504040 |
| Counter frequency | 2112541357 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 2739ns | 1953ns | 1599ns | 1978ns | 6164ns | base |
| memcpy_naive | 3027ns | 1960ns | 1601ns | 2502ns | 6029ns | +10.52% |
| memcpy_unrolled | 2690ns | 1953ns | 1597ns | 2022ns | 5789ns | -1.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 108ns | 64ns | 242ns | base | 0.590 |
| memcpy_naive | 120ns | 64ns | 237ns | +10.37% | 0.535 |
| memcpy_unrolled | 107ns | 64ns | 227ns | -1.55% | 0.600 |

## Performance model

- Peak throughput: **1.005 Gops/s** (memcpy_naive; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.822 | 81.7% |
| memcpy_naive | 0.822 | 81.7% |
| memcpy_unrolled | 0.822 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 2739ns | 2739ns | base |
| memcpy_naive | 3027ns | 3027ns | +10.52% |
| memcpy_unrolled | 2690ns | 2690ns | -1.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 78ns | base | --- | [78, 78] | --- | --- | --- | --- |
| memcpy_naive | 78ns | no significant difference | [-0, +0]ns | [78, 78] | no | 0.7381 | 0.7381 | **17** (11%, HIGH) |
| memcpy_unrolled | 78ns | no significant difference | [-0, +0]ns | [70, 78] | no | 0.4766 | 0.2383 | **19** (12%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 64ns | +22.1% | +0.0% |
| 1 | 2 | 78ns | +0.0% | -18.1% |
| 1 | 3 | 64ns | +22.1% | +0.0% |
| 1 | 4 | 78ns | -18.2% | -18.1% |
| 1 | 5 | 78ns | -18.3% | -0.1% |
| 1 | 6 | 78ns | +0.0% | -18.1% |
| 1 | 7 | 64ns | +0.0% | +22.3% |
| 1 | 8 | 64ns | +193.0% | +22.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.121 | ok |
| memcpy_naive | -0.088 | ok |
| memcpy_unrolled | 0.059 | ok |

**Consistency summary:**

- **memcpy_naive**: won 2/8, lost 3/8
- **memcpy_unrolled**: won 4/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 34.0ns | 108.4ns | 31.4% | HIGH |
| memcpy_naive | 37.7ns | 119.6ns | 31.5% | HIGH |
| memcpy_unrolled | 33.4ns | 106.7ns | 31.3% | HIGH |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 63.7-242.0 ns)
     63.7 |####################
     72.6 |########################################
     81.5 |
     90.4 |##
     99.4 |
    108.3 |#
    117.2 |
    126.1 |
    135.0 |
    143.9 |#
    152.8 |
    161.8 |
    170.7 |
    179.6 |
    188.5 |
    197.4 |
    206.3 |##
    215.2 |###
    224.2 |
    233.1 |####
  (29 below, 15 above range)

memcpy_naive (n=160, range 63.7-236.7 ns)
     63.7 |#####################
     72.3 |########################################
     81.0 |
     89.6 |
     98.3 |
    106.9 |
    115.6 |#
    124.2 |
    132.9 |
    141.5 |#
    150.2 |
    158.8 |#
    167.5 |##
    176.1 |#
    184.8 |###
    193.4 |##
    202.1 |
    210.7 |####
    219.4 |
    228.0 |#####
  (8 below, 13 above range)

memcpy_unrolled (n=160, range 63.7-227.3 ns)
     63.7 |################################
     71.9 |########################################
     80.1 |
     88.3 |
     96.4 |
    104.6 |
    112.8 |
    121.0 |
    129.2 |
    137.3 |
    145.5 |
    153.7 |
    161.9 |
    170.1 |
    178.3 |
    186.4 |##
    194.6 |####
    202.8 |###
    211.0 |######
    219.2 |###
  (28 below, 13 above range)

```

## Diagnostics

- **memcpy_intrinsic**: CV=64.2% (high variance, measurements may be unstable)
- **memcpy_intrinsic**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **memcpy_intrinsic**: bridge=31.3% of algo (FFI overhead may distort results)
- **memcpy_naive**: CV=57.7% (high variance, measurements may be unstable)
- **memcpy_naive**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **memcpy_naive**: bridge=31.2% of algo (FFI overhead may distort results)
- **memcpy_unrolled**: CV=61.7% (high variance, measurements may be unstable)
- **memcpy_unrolled**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **memcpy_unrolled**: bridge=31.3% of algo (FFI overhead may distort results)
