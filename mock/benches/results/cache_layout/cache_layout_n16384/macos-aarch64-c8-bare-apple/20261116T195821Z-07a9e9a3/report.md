# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4 ns) is smaller than the fastest variant's own run-to-run std-dev (7 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (336 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### layout_soa's edge over baseline is significant but tiny (3 ns, 1.01%)

layout_soa differs from baseline layout_aos by 3 ns (1.01%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (layout_aos) is the fastest** at 335.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 335.8 ns, slowest 340.2 ns)

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
| layout_aos | 2448ns | 2423ns | 2416ns | 2437ns | 2513ns | base |
| layout_soa | 2459ns | 2460ns | 2419ns | 2451ns | 2523ns | +0.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 338ns | 330ns | 349ns | base | 48.518 |
| layout_soa | 341ns | 334ns | 351ns | +1.07% | 48.006 |

## Performance model

- Peak throughput: **49.590 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 48.791 | 98.4% |
| layout_soa | 48.160 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2448ns | 2448ns | base |
| layout_soa | 2459ns | 2459ns | +0.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 336ns | base | --- | [335, 337] | --- | --- | --- | --- |
| layout_soa | 340ns | +3.4ns (+1.0%) | [+2, +4]ns | [339, 341] | YES | 0.0000 | 0.0000 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 334ns | +3.5% |
| 1 | 2 | 336ns | +1.8% |
| 1 | 3 | 334ns | +1.7% |
| 1 | 4 | 334ns | +0.6% |
| 1 | 5 | 335ns | +0.9% |
| 1 | 6 | 347ns | -1.8% |
| 1 | 7 | 345ns | -0.8% |
| 1 | 8 | 334ns | +1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.269 | moderate+ |
| layout_soa | 0.233 | moderate+ |

**Consistency summary:**

- **layout_soa**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 2.8ns | 337.7ns | 0.8% |  |
| layout_soa | 2.9ns | 341.3ns | 0.8% |  |

## Distribution (algo ns)

```
layout_aos (n=160, range 330.4-348.9 ns)
    330.4 |########################################
    331.3 |###############
    332.2 |##############################
    333.2 |##############################
    334.1 |###########################
    335.0 |################################
    335.9 |################################
    336.9 |####################
    337.8 |############
    338.7 |#####
    339.6 |##########
    340.6 |#####
    341.5 |##########
    342.4 |##########
    343.3 |#######
    344.3 |###############
    345.2 |#######
    346.1 |######################
    347.0 |############
    348.0 |##
  (13 below, 8 above range)

layout_soa (n=160, range 333.9-351.3 ns)
    333.9 |##############
    334.7 |##################
    335.6 |#####################
    336.5 |############################
    337.4 |#########################
    338.2 |##################
    339.1 |##################
    340.0 |########################################
    340.8 |##################
    341.7 |#########
    342.6 |#######################
    343.5 |#########
    344.3 |###########
    345.2 |#####################
    346.1 |##############
    346.9 |#######
    347.8 |##
    348.7 |###########
    349.5 |#########
    350.4 |##
  (12 below, 9 above range)

```
