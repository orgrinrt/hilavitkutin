# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4 ns) is smaller than the fastest variant's own run-to-run std-dev (6 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (memcpy_intrinsic)

The baseline memcpy_intrinsic is the fastest (463 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 1.0% of the fastest

All 3 variants sit between 463 ns and 468 ns - a 1.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_unrolled's edge over baseline is significant but tiny (4 ns, 0.77%)

memcpy_unrolled differs from baseline memcpy_intrinsic by 4 ns (0.77%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (memcpy_intrinsic) is the fastest** at 463.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 463.1 ns, slowest 467.5 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 2566ns | 2564ns | 2553ns | 2563ns | 2588ns | base |
| memcpy_naive | 2567ns | 2563ns | 2553ns | 2563ns | 2595ns | +0.06% |
| memcpy_unrolled | 2591ns | 2570ns | 2560ns | 2571ns | 2679ns | +0.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 464ns | 456ns | 474ns | base | 0.552 |
| memcpy_naive | 464ns | 455ns | 473ns | +0.00% | 0.552 |
| memcpy_unrolled | 468ns | 459ns | 481ns | +0.92% | 0.546 |

## Performance model

- Peak throughput: **0.563 Gops/s** (memcpy_naive; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.553 | 98.2% |
| memcpy_naive | 0.551 | 97.9% |
| memcpy_unrolled | 0.548 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 2566ns | 2566ns | base |
| memcpy_naive | 2567ns | 2567ns | +0.06% |
| memcpy_unrolled | 2591ns | 2591ns | +0.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 463ns | base | --- | [462, 465] | --- | --- | --- | --- |
| memcpy_naive | 464ns | no significant difference | [-0, +2]ns | [463, 465] | no | 0.0945 | 0.0945 | 2 |
| memcpy_unrolled | 468ns | +3.6ns (+0.8%) | [+2, +5]ns | [465, 469] | YES | 0.0000 | 0.0000 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 467ns | -0.2% | -1.1% |
| 1 | 2 | 468ns | -0.0% | -0.4% |
| 1 | 3 | 464ns | +0.5% | -0.1% |
| 1 | 4 | 465ns | -1.0% | +0.7% |
| 1 | 5 | 465ns | -2.1% | +3.5% |
| 1 | 6 | 461ns | +1.2% | +0.2% |
| 1 | 7 | 462ns | +0.0% | +1.9% |
| 1 | 8 | 460ns | +1.3% | +2.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | 0.458 | moderate+ |
| memcpy_naive | 0.054 | ok |
| memcpy_unrolled | -0.240 | moderate- |

**Consistency summary:**

- **memcpy_naive**: won 3/8, lost 3/8
- **memcpy_unrolled**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 4.1ns | 464.2ns | 0.9% |  |
| memcpy_naive | 4.1ns | 464.2ns | 0.9% |  |
| memcpy_unrolled | 3.9ns | 468.4ns | 0.8% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 455.6-473.6 ns)
    455.6 |#########
    456.5 |#########
    457.4 |########################
    458.3 |######
    459.2 |########################
    460.1 |##############################
    461.0 |########################################
    461.9 |####################################
    462.8 |##############################
    463.7 |###############
    464.6 |########################################
    465.5 |###############
    466.4 |#####################
    467.3 |#####################
    468.2 |###############
    469.1 |#########
    470.0 |########################
    470.9 |############
    471.8 |############
    472.7 |#########
  (14 below, 13 above range)

memcpy_naive (n=160, range 454.7-473.4 ns)
    454.7 |##############
    455.7 |#########
    456.6 |###########
    457.5 |##############
    458.5 |##############
    459.4 |#########
    460.3 |########################################
    461.3 |###########
    462.2 |###########
    463.1 |#####################
    464.1 |##################
    465.0 |##############################
    465.9 |#####################
    466.9 |##################
    467.8 |#####################
    468.7 |##################
    469.7 |#########
    470.6 |###########
    471.5 |###########
    472.5 |##
  (9 below, 14 above range)

memcpy_unrolled (n=160, range 459.0-480.8 ns)
    459.0 |#####################
    460.1 |############################
    461.2 |#########################
    462.3 |##################
    463.4 |###########
    464.4 |########################################
    465.5 |#######
    466.6 |#####################
    467.7 |##############
    468.8 |##############
    469.9 |#####################
    471.0 |##################
    472.1 |##################
    473.2 |##############
    474.3 |#######
    475.4 |##############
    476.4 |#########
    477.5 |##
    478.6 |###########
    479.7 |
  (10 below, 14 above range)

```
