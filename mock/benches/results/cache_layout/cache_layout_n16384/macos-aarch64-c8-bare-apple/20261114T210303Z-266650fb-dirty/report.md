# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### layout_aos is fastest but the noisiest (CV 6.5%)

layout_aos wins on median (371 ns) yet has the highest variance (CV 6.5%), while layout_soa is the steadiest (CV 5.0%, 386 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (15 ns) is smaller than the fastest variant's own run-to-run std-dev (24 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (371 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader layout_aos vs stability leader layout_soa (+4% speed for 1.3x steadier)

layout_aos is fastest (371 ns, CV 6.5%); layout_soa gives up 3.9% median for 1.3x lower variance (CV 5.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### layout_soa's edge over baseline is significant but tiny (5 ns, 1.35%)

layout_soa differs from baseline layout_aos by 5 ns (1.35%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (layout_aos) is the fastest** at 371.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 371.4 ns, slowest 386.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | AoS vs SoA (column-store) layout under partial-field iteration |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eadbeef01234567 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2633ns | 2639ns | 2416ns | 2647ns | 2805ns | base |
| layout_soa | 2691ns | 2782ns | 2470ns | 2727ns | 2806ns | +2.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 365ns | 333ns | 393ns | base | 44.909 |
| layout_soa | 375ns | 344ns | 393ns | +2.86% | 43.659 |

## Performance model

- Peak throughput: **49.213 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 44.108 | 89.6% |
| layout_soa | 42.446 | 86.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2633ns | 2633ns | base |
| layout_soa | 2691ns | 2691ns | +2.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 371ns | base | --- | [354, 383] | --- | --- | --- | --- |
| layout_soa | 386ns | +5.0ns (+1.3%) | [+3, +8]ns | [384, 388] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 335ns | +16.6% |
| 1 | 2 | 339ns | +7.0% |
| 1 | 3 | 383ns | +1.6% |
| 1 | 4 | 385ns | +0.8% |
| 1 | 5 | 353ns | +9.4% |
| 1 | 6 | 346ns | +12.1% |
| 1 | 7 | 388ns | -6.7% |
| 1 | 8 | 385ns | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.184 | ok |
| layout_soa | -0.441 | moderate- |

**Consistency summary:**

- **layout_soa**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 3.3ns | 364.8ns | 0.9% |  |
| layout_soa | 3.1ns | 375.3ns | 0.8% |  |

## Distribution (algo ns)

```
layout_aos (n=160, range 332.9-392.8 ns)
    332.9 |############
    335.9 |################
    338.9 |############
    341.9 |#####
    344.9 |#########################
    347.9 |##############
    350.9 |###
    353.9 |#########
    356.9 |###
    359.9 |#######
    362.9 |
    365.8 |#
    368.8 |
    371.8 |#
    374.8 |#####
    377.8 |###
    380.8 |###########################
    383.8 |########################################
    386.8 |####################################
    389.8 |####################
  (18 below, 6 above range)

layout_soa (n=160, range 343.6-392.9 ns)
    343.6 |
    346.1 |
    348.5 |#
    351.0 |#
    353.5 |####
    355.9 |##
    358.4 |#################
    360.9 |##########
    363.3 |############
    365.8 |####
    368.2 |##
    370.7 |#
    373.2 |
    375.6 |#
    378.1 |
    380.6 |####
    383.0 |###############
    385.5 |#########################
    388.0 |########################################
    390.4 |#####################################
  (21 below, 11 above range)

```
