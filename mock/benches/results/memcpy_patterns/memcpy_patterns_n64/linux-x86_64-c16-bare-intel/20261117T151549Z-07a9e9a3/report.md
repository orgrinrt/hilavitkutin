# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### memcpy_naive dominates: 22% faster than the next best (memcpy_intrinsic)

memcpy_naive (64 ns) leads memcpy_intrinsic (78 ns) by 22%, a clear separation rather than a photo finish. CV 112.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### memcpy_naive is fastest but the noisiest (CV 112.1%)

memcpy_naive wins on median (64 ns) yet has the highest variance (CV 112.1%), while memcpy_intrinsic is the steadiest (CV 84.0%, 78 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### memcpy_unrolled shows warm-up / thermal drift (autocorr +0.59)

memcpy_unrolled's per-pass series has lag-1 autocorrelation +0.59, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (14 ns) is smaller than the fastest variant's own run-to-run std-dev (71 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### memcpy_unrolled's comparison is tie-heavy (28% tied pairs)

28% of paired samples for memcpy_unrolled are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### memcpy_naive is inconsistent: worst-20% is 3.8x its best-20%

memcpy_naive's best 20% of batches run at 64 ns but its worst 20% at 239 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### memcpy_naive's edge over baseline is significant but tiny (-0 ns, 0.13%)

memcpy_naive differs from baseline memcpy_intrinsic by -0 ns (0.13%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: memcpy_naive** at 63.7 ns median (-18.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.22x (fastest 63.7 ns, slowest 77.9 ns)

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
| Counter frequency | 2112829592 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 2918ns | 1964ns | 1605ns | 2397ns | 5792ns | base |
| memcpy_naive | 2663ns | 1617ns | 1605ns | 1919ns | 5954ns | -8.72% |
| memcpy_unrolled | 2700ns | 1963ns | 1604ns | 1963ns | 6007ns | -7.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 115ns | 64ns | 226ns | base | 0.558 |
| memcpy_naive | 106ns | 64ns | 239ns | -7.32% | 0.602 |
| memcpy_unrolled | 107ns | 64ns | 235ns | -7.10% | 0.600 |

## Performance model

- Peak throughput: **1.006 Gops/s** (memcpy_naive; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.822 | 81.7% |
| memcpy_naive | 1.005 | 99.9% |
| memcpy_unrolled | 0.822 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 2918ns | 2918ns | base |
| memcpy_naive | 2663ns | 2663ns | -8.72% |
| memcpy_unrolled | 2700ns | 2700ns | -7.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 78ns | base | --- | [78, 78] | --- | --- | --- | --- |
| memcpy_naive | 64ns | -0.1ns (-0.1%) | [-0, -0]ns | [64, 78] | YES | 0.0000 | 0.0000 | **29** (18%, HIGH) |
| memcpy_unrolled | 78ns | no significant difference | [+0, +0]ns | [78, 78] | no | 0.9261 | 0.9261 | **44** (28%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 64ns | -0.2% | +0.0% |
| 1 | 2 | 64ns | -0.2% | +0.0% |
| 1 | 3 | 64ns | -0.2% | +0.1% |
| 1 | 4 | 165ns | -61.5% | -52.9% |
| 1 | 5 | 78ns | +0.0% | +0.1% |
| 1 | 6 | 64ns | -0.2% | +22.3% |
| 1 | 7 | 78ns | -18.3% | -0.1% |
| 1 | 8 | 78ns | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.124 | ok |
| memcpy_naive | -0.208 | moderate- |
| memcpy_unrolled | 0.595 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **memcpy_naive**: won 6/8, lost 0/8
- **memcpy_unrolled**: won 2/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 36.1ns | 114.8ns | 31.4% | HIGH |
| memcpy_naive | 32.9ns | 106.4ns | 30.9% | HIGH |
| memcpy_unrolled | 33.4ns | 106.6ns | 31.3% | HIGH |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 63.7-226.1 ns)
     63.7 |##############################
     71.8 |########################################
     80.0 |
     88.1 |
     96.2 |
    104.3 |#
    112.4 |
    120.6 |##
    128.7 |
    136.8 |#
    144.9 |##
    153.0 |
    161.1 |##
    169.3 |###
    177.4 |
    185.5 |####
    193.6 |#######
    201.7 |#
    209.8 |#######
    218.0 |
  (22 below, 13 above range)

memcpy_naive (n=160, range 63.6-239.0 ns)
     63.6 |########################################
     72.4 |####################
     81.2 |
     89.9 |
     98.7 |
    107.5 |
    116.2 |
    125.0 |
    133.8 |
    142.5 |#
    151.3 |
    160.1 |
    168.9 |
    177.6 |
    186.4 |#
    195.2 |##
    203.9 |#
    212.7 |##
    221.5 |#
    230.2 |##
  (26 below, 13 above range)

memcpy_unrolled (n=160, range 63.7-235.3 ns)
     63.7 |###########
     72.3 |########################################
     80.9 |
     89.5 |
     98.0 |#
    106.6 |
    115.2 |
    123.8 |
    132.4 |
    140.9 |
    149.5 |
    158.1 |
    166.7 |
    175.2 |
    183.8 |#
    192.4 |
    201.0 |#
    209.6 |##
    218.1 |#
    226.7 |##
  (22 below, 14 above range)

```

## Diagnostics

- **memcpy_intrinsic**: CV=57.0% (high variance, measurements may be unstable)
- **memcpy_intrinsic**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **memcpy_intrinsic**: bridge=31.3% of algo (FFI overhead may distort results)
- **memcpy_naive**: CV=67.1% (high variance, measurements may be unstable)
- **memcpy_naive**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **memcpy_naive**: bridge=31.6% of algo (FFI overhead may distort results)
- **memcpy_unrolled**: CV=62.8% (high variance, measurements may be unstable)
- **memcpy_unrolled**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **memcpy_unrolled**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **memcpy_unrolled**: bridge=31.3% of algo (FFI overhead may distort results)
