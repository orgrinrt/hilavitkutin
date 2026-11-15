# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### memcpy_intrinsic is fastest but the noisiest (CV 6.5%)

memcpy_intrinsic wins on median (488 ns) yet has the highest variance (CV 6.5%), while memcpy_naive is the steadiest (CV 5.6%, 491 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4 ns) is smaller than the fastest variant's own run-to-run std-dev (32 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (memcpy_intrinsic)

The baseline memcpy_intrinsic is the fastest (488 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader memcpy_intrinsic vs stability leader memcpy_naive (+1% speed for 1.2x steadier)

memcpy_intrinsic is fastest (488 ns, CV 6.5%); memcpy_naive gives up 0.8% median for 1.2x lower variance (CV 5.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.8% of the fastest

All 3 variants sit between 488 ns and 491 ns - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_naive's edge over baseline is significant but tiny (4 ns, 0.76%)

memcpy_naive differs from baseline memcpy_intrinsic by 4 ns (0.76%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (memcpy_intrinsic) is the fastest** at 487.5 ns median
- 2 variants significantly slower than baseline
- Spread: 1.01x (fastest 487.5 ns, slowest 491.4 ns)

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
| memcpy_intrinsic | 2720ns | 2721ns | 2542ns | 2699ns | 2961ns | base |
| memcpy_naive | 2707ns | 2728ns | 2552ns | 2700ns | 2880ns | -0.50% |
| memcpy_unrolled | 2732ns | 2722ns | 2554ns | 2715ns | 2960ns | +0.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 493ns | 457ns | 541ns | base | 0.519 |
| memcpy_naive | 494ns | 466ns | 535ns | +0.24% | 0.518 |
| memcpy_unrolled | 499ns | 464ns | 546ns | +1.21% | 0.513 |

## Performance model

- Peak throughput: **0.560 Gops/s** (memcpy_intrinsic; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.525 | 93.7% |
| memcpy_naive | 0.521 | 93.0% |
| memcpy_unrolled | 0.522 | 93.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 2720ns | 2720ns | base |
| memcpy_naive | 2707ns | 2707ns | -0.50% |
| memcpy_unrolled | 2732ns | 2732ns | +0.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 488ns | base | --- | [473, 497] | --- | --- | --- | --- |
| memcpy_naive | 491ns | +3.7ns (+0.8%) | [+0, +6]ns | [489, 495] | YES | 0.0389 | 0.0389 | 1 |
| memcpy_unrolled | 491ns | +8.8ns (+1.8%) | [+2, +15]ns | [484, 499] | YES | 0.0342 | 0.0171 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 472ns | -0.4% | +8.7% |
| 1 | 2 | 468ns | +3.2% | +3.1% |
| 1 | 3 | 525ns | -3.2% | -4.6% |
| 1 | 4 | 468ns | +5.2% | +12.8% |
| 1 | 5 | 458ns | +4.6% | +6.1% |
| 1 | 6 | 542ns | -12.0% | -8.6% |
| 1 | 7 | 526ns | -5.7% | -10.6% |
| 1 | 8 | 499ns | +0.1% | -6.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.017 | ok |
| memcpy_naive | 0.116 | ok |
| memcpy_unrolled | 0.095 | ok |

**Consistency summary:**

- **memcpy_naive**: won 4/8, lost 4/8
- **memcpy_unrolled**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 4.0ns | 493.1ns | 0.8% |  |
| memcpy_naive | 4.0ns | 494.3ns | 0.8% |  |
| memcpy_unrolled | 4.2ns | 499.1ns | 0.8% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 457.0-541.0 ns)
    457.0 |############################
    461.2 |###################
    465.4 |########################################
    469.6 |####################
    473.8 |###
    478.0 |#######
    482.2 |###
    486.4 |###########
    490.6 |#########
    494.8 |#################
    499.0 |#########
    503.2 |#########
    507.4 |###
    511.6 |#
    515.8 |
    520.0 |#######
    524.2 |###################
    528.4 |###################
    532.6 |#####
    536.8 |#######
  (14 below, 17 above range)

memcpy_naive (n=160, range 466.3-535.5 ns)
    466.3 |#######################
    469.8 |#####################################
    473.2 |################
    476.7 |#####################
    480.1 |###########
    483.6 |#########
    487.1 |##############################
    490.5 |#########################
    494.0 |########################################
    497.4 |##################
    500.9 |############################
    504.4 |##############
    507.8 |####
    511.3 |#######
    514.7 |#######
    518.2 |
    521.6 |##
    525.1 |##
    528.6 |#######
    532.0 |####
  (13 below, 14 above range)

memcpy_unrolled (n=160, range 464.1-546.1 ns)
    464.1 |#######################
    468.2 |########################################
    472.3 |###########
    476.4 |############################
    480.5 |##################
    484.6 |#######################
    488.7 |#########
    492.8 |##################
    496.9 |################
    501.0 |#######
    505.1 |##############
    509.2 |#########
    513.3 |####
    517.4 |####
    521.5 |##
    525.6 |##############
    529.7 |#########
    533.8 |#########################
    537.9 |#####################
    542.0 |##############
  (16 below, 9 above range)

```
