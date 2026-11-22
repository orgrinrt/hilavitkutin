# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (7 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (338 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### layout_soa's edge over baseline is significant but tiny (1 ns, 0.38%)

layout_soa differs from baseline layout_aos by 1 ns (0.38%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (layout_aos) is the fastest** at 338.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 338.3 ns, slowest 338.3 ns)

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
| layout_aos | 2439ns | 2434ns | 2405ns | 2435ns | 2485ns | base |
| layout_soa | 2443ns | 2415ns | 2405ns | 2418ns | 2554ns | +0.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 338ns | 331ns | 347ns | base | 48.418 |
| layout_soa | 341ns | 333ns | 357ns | +0.86% | 48.006 |

## Performance model

- Peak throughput: **49.447 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 48.430 | 97.9% |
| layout_soa | 48.430 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2439ns | 2439ns | base |
| layout_soa | 2443ns | 2443ns | +0.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 338ns | base | --- | [337, 339] | --- | --- | --- | --- |
| layout_soa | 338ns | +1.3ns (+0.4%) | [+0, +2]ns | [338, 339] | YES | 0.0010 | 0.0010 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 340ns | -0.9% |
| 1 | 2 | 339ns | +5.9% |
| 1 | 3 | 334ns | +1.0% |
| 1 | 4 | 341ns | -0.2% |
| 1 | 5 | 339ns | -0.3% |
| 1 | 6 | 336ns | +0.4% |
| 1 | 7 | 337ns | +0.2% |
| 1 | 8 | 335ns | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.181 | ok |
| layout_soa | -0.222 | moderate- |

**Consistency summary:**

- **layout_soa**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 2.9ns | 338.4ns | 0.9% |  |
| layout_soa | 2.8ns | 341.3ns | 0.8% |  |

## Distribution (algo ns)

```
layout_aos (n=160, range 331.3-347.2 ns)
    331.3 |##############
    332.1 |#########################
    332.9 |##############
    333.7 |######################
    334.5 |######################
    335.3 |########################################
    336.1 |#################
    336.9 |#########################
    337.7 |###############################
    338.5 |##################################
    339.3 |####################
    340.1 |###############################
    340.9 |##
    341.7 |############################
    342.5 |###########
    343.2 |####################
    344.0 |##
    344.8 |##############
    345.6 |##
    346.4 |########
  (14 below, 9 above range)

layout_soa (n=160, range 333.4-357.0 ns)
    333.4 |###############
    334.6 |#####################
    335.8 |##########################
    336.9 |########################################
    338.1 |#################################
    339.3 |###############################
    340.5 |######
    341.7 |################
    342.8 |########
    344.0 |#####
    345.2 |#
    346.4 |
    347.5 |###
    348.7 |
    349.9 |###
    351.1 |#
    352.3 |###
    353.4 |
    354.6 |
    355.8 |
  (12 below, 17 above range)

```
