# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### memcpy_naive is fastest but the noisiest (CV 110.7%)

memcpy_naive wins on median (78 ns) yet has the highest variance (CV 110.7%), while memcpy_unrolled is the steadiest (CV 87.7%, 78 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (86 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### memcpy_unrolled's comparison is tie-heavy (20% tied pairs)

20% of paired samples for memcpy_unrolled are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader memcpy_naive vs stability leader memcpy_unrolled (+0% speed for 1.3x steadier)

memcpy_naive is fastest (78 ns, CV 110.7%); memcpy_unrolled gives up 0.1% median for 1.3x lower variance (CV 87.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### memcpy_unrolled is inconsistent: worst-20% is 3.7x its best-20%

memcpy_unrolled's best 20% of batches run at 64 ns but its worst 20% at 235 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.1% of the fastest

All 3 variants sit between 78 ns and 78 ns - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_naive's edge over baseline is significant but tiny (-0 ns, 0.13%)

memcpy_naive differs from baseline memcpy_intrinsic by -0 ns (0.13%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: memcpy_naive** at 77.8 ns median (-0.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.00x (fastest 77.8 ns, slowest 77.9 ns)

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
| Counter frequency | 2112632730 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 2828ns | 1957ns | 1788ns | 2034ns | 6252ns | base |
| memcpy_naive | 2737ns | 1959ns | 1800ns | 1965ns | 5991ns | -3.21% |
| memcpy_unrolled | 2776ns | 1953ns | 1598ns | 2118ns | 5927ns | -1.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 112ns | 72ns | 246ns | base | 0.570 |
| memcpy_naive | 113ns | 72ns | 258ns | +0.42% | 0.568 |
| memcpy_unrolled | 110ns | 64ns | 235ns | -1.56% | 0.579 |

## Performance model

- Peak throughput: **1.005 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.822 | 81.8% |
| memcpy_naive | 0.823 | 81.9% |
| memcpy_unrolled | 0.822 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 2828ns | 2828ns | base |
| memcpy_naive | 2737ns | 2737ns | -3.21% |
| memcpy_unrolled | 2776ns | 2776ns | -1.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 78ns | base | --- | [78, 78] | --- | --- | --- | --- |
| memcpy_naive | 78ns | -0.1ns (-0.1%) | [-0, -0]ns | [78, 78] | YES | 0.0000 | 0.0000 | **21** (13%, HIGH) |
| memcpy_unrolled | 78ns | -0.1ns (-0.1%) | [-5, -0]ns | [78, 78] | YES | 0.0000 | 0.0000 | **32** (20%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 78ns | -0.1% | -18.2% |
| 1 | 2 | 64ns | +22.1% | +0.0% |
| 1 | 3 | 78ns | -0.2% | -0.1% |
| 1 | 4 | 78ns | -18.2% | -18.1% |
| 1 | 5 | 78ns | -0.1% | +0.0% |
| 1 | 6 | 78ns | -0.1% | -18.1% |
| 1 | 7 | 78ns | -0.2% | -0.1% |
| 1 | 8 | 78ns | -0.1% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.168 | ok |
| memcpy_naive | -0.161 | ok |
| memcpy_unrolled | -0.373 | moderate- |

**Consistency summary:**

- **memcpy_naive**: won 4/8, lost 1/8
- **memcpy_unrolled**: won 4/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 35.1ns | 112.2ns | 31.3% | HIGH |
| memcpy_naive | 33.8ns | 112.7ns | 30.0% | HIGH |
| memcpy_unrolled | 34.4ns | 110.4ns | 31.1% | HIGH |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 71.6-246.2 ns)
     71.6 |########################################
     80.3 |
     89.0 |
     97.8 |
    106.5 |
    115.2 |
    124.0 |
    132.7 |
    141.4 |
    150.2 |
    158.9 |
    167.6 |
    176.4 |
    185.1 |
    193.8 |
    202.6 |
    211.3 |#
    220.0 |
    228.7 |#
    237.5 |
  (14 below, 18 above range)

memcpy_naive (n=160, range 71.5-257.5 ns)
     71.5 |########################################
     80.8 |
     90.1 |
     99.4 |
    108.7 |
    118.0 |
    127.3 |
    136.6 |
    145.9 |
    155.2 |
    164.5 |
    173.8 |
    183.1 |
    192.4 |
    201.7 |
    211.0 |
    220.3 |#
    229.6 |#
    238.9 |
    248.2 |##
  (14 below, 14 above range)

memcpy_unrolled (n=160, range 63.7-234.9 ns)
     63.7 |#################
     72.3 |########################################
     80.8 |
     89.4 |#
     97.9 |
    106.5 |
    115.1 |
    123.6 |
    132.2 |
    140.7 |
    149.3 |
    157.8 |
    166.4 |
    175.0 |
    183.5 |#
    192.1 |##
    200.6 |##
    209.2 |####
    217.8 |#
    226.3 |###
  (28 below, 13 above range)

```

## Diagnostics

- **memcpy_intrinsic**: CV=61.7% (high variance, measurements may be unstable)
- **memcpy_intrinsic**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **memcpy_intrinsic**: bridge=31.3% of algo (FFI overhead may distort results)
- **memcpy_naive**: CV=76.4% (high variance, measurements may be unstable)
- **memcpy_naive**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **memcpy_naive**: bridge=31.4% of algo (FFI overhead may distort results)
- **memcpy_unrolled**: CV=61.9% (high variance, measurements may be unstable)
- **memcpy_unrolled**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **memcpy_unrolled**: bridge=31.2% of algo (FFI overhead may distort results)
