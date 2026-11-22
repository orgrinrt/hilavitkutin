# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (335 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### layout_soa's edge over baseline is significant but tiny (3 ns, 0.99%)

layout_soa differs from baseline layout_aos by 3 ns (0.99%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (layout_aos) is the fastest** at 334.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 334.8 ns, slowest 338.3 ns)

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
| layout_aos | 2431ns | 2422ns | 2415ns | 2423ns | 2470ns | base |
| layout_soa | 2456ns | 2425ns | 2417ns | 2435ns | 2555ns | +1.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 335ns | 330ns | 342ns | base | 48.852 |
| layout_soa | 341ns | 332ns | 357ns | +1.69% | 48.042 |

## Performance model

- Peak throughput: **49.602 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 48.937 | 98.7% |
| layout_soa | 48.430 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2431ns | 2431ns | base |
| layout_soa | 2456ns | 2456ns | +1.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 335ns | base | --- | [334, 336] | --- | --- | --- | --- |
| layout_soa | 338ns | +3.3ns (+1.0%) | [+3, +4]ns | [337, 339] | YES | 0.0000 | 0.0000 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 334ns | +0.5% |
| 1 | 2 | 335ns | +0.4% |
| 1 | 3 | 333ns | +1.3% |
| 1 | 4 | 334ns | +0.4% |
| 1 | 5 | 334ns | +0.7% |
| 1 | 6 | 334ns | +1.0% |
| 1 | 7 | 334ns | +7.0% |
| 1 | 8 | 342ns | +1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.007 | ok |
| layout_soa | 0.298 | moderate+ |

**Consistency summary:**

- **layout_soa**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 3.0ns | 335.4ns | 0.9% |  |
| layout_soa | 2.8ns | 341.0ns | 0.8% |  |

## Distribution (algo ns)

```
layout_aos (n=160, range 330.3-341.8 ns)
    330.3 |##################
    330.9 |##########
    331.5 |########
    332.0 |#####################################
    332.6 |#####################
    333.2 |########################
    333.7 |########################################
    334.3 |################
    334.9 |################################
    335.5 |################
    336.0 |#####################
    336.6 |################################
    337.2 |#####
    337.8 |########################
    338.3 |##########
    338.9 |##
    339.5 |########################
    340.1 |
    340.6 |########
    341.2 |##
  (14 below, 13 above range)

layout_soa (n=160, range 331.8-357.3 ns)
    331.8 |#################
    333.1 |####################
    334.4 |###################
    335.7 |########################################
    336.9 |######################
    338.2 |##########################
    339.5 |##############################
    340.8 |#########
    342.0 |#####
    343.3 |#####
    344.6 |###############
    345.8 |#########
    347.1 |#######
    348.4 |#######
    349.7 |#
    350.9 |###
    352.2 |
    353.5 |
    354.8 |#
    356.0 |#####
  (16 below, 12 above range)

```
