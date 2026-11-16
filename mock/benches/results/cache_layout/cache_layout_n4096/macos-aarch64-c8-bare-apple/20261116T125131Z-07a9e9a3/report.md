# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### layout_aos is fastest but the noisiest (CV 5.7%)

layout_aos wins on median (62 ns) yet has the highest variance (CV 5.7%), while layout_soa is the steadiest (CV 5.6%, 64 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (62 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader layout_aos vs stability leader layout_soa (+3% speed for 1.0x steadier)

layout_aos is fastest (62 ns, CV 5.7%); layout_soa gives up 3.4% median for 1.0x lower variance (CV 5.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (layout_aos) is the fastest** at 62.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.03x (fastest 62.1 ns, slowest 64.2 ns)

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
| layout_aos | 2151ns | 2148ns | 2142ns | 2148ns | 2169ns | base |
| layout_soa | 2155ns | 2150ns | 2145ns | 2150ns | 2181ns | +0.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 62ns | 57ns | 67ns | base | 65.987 |
| layout_soa | 64ns | 59ns | 70ns | +3.42% | 63.807 |

## Performance model

- Peak throughput: **71.836 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 65.958 | 91.8% |
| layout_soa | 63.801 | 88.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2151ns | 2151ns | base |
| layout_soa | 2155ns | 2155ns | +0.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 62ns | base | --- | [62, 63] | --- | --- | --- | --- |
| layout_soa | 64ns | +2.1ns (+3.4%) | [+2, +3]ns | [63, 65] | YES | 0.0000 | 0.0000 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 64ns | +0.6% |
| 1 | 2 | 62ns | +4.1% |
| 1 | 3 | 62ns | +2.7% |
| 1 | 4 | 62ns | +1.3% |
| 1 | 5 | 62ns | +3.2% |
| 1 | 6 | 61ns | +3.7% |
| 1 | 7 | 62ns | +3.7% |
| 1 | 8 | 62ns | +3.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.143 | ok |
| layout_soa | 0.075 | ok |

**Consistency summary:**

- **layout_soa**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 2.7ns | 62.1ns | 4.3% |  |
| layout_soa | 2.5ns | 64.2ns | 3.9% |  |

## Distribution (algo ns)

```
layout_aos (n=160, range 57.0-66.9 ns)
     57.0 |######################
     57.5 |############
     58.0 |#######
     58.5 |####################
     59.0 |#####
     59.5 |############
     60.0 |###########################
     60.5 |##########
     61.0 |############
     61.5 |#########################
     62.0 |######################
     62.5 |########################################
     63.0 |#################
     63.5 |############
     64.0 |######################
     64.5 |#######
     65.0 |#####################################
     65.5 |#######
     66.0 |##########
     66.4 |#######
  (11 below, 13 above range)

layout_soa (n=160, range 59.0-69.5 ns)
     59.0 |################
     59.6 |##################
     60.1 |##
     60.6 |###########
     61.1 |####
     61.7 |#########################
     62.2 |################
     62.7 |#######
     63.2 |##############################
     63.8 |########################################
     64.3 |#######################
     64.8 |###########
     65.3 |################################
     65.9 |###########
     66.4 |#########
     66.9 |#######
     67.4 |###########
     67.9 |###########
     68.5 |##############
     69.0 |####
  (12 below, 15 above range)

```
