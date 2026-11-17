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
| layout_aos | 2151ns | 2149ns | 2145ns | 2149ns | 2165ns | base |
| layout_soa | 2161ns | 2152ns | 2146ns | 2155ns | 2196ns | +0.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 62ns | 57ns | 67ns | base | 65.879 |
| layout_soa | 64ns | 59ns | 69ns | +3.32% | 63.762 |

## Performance model

- Peak throughput: **71.359 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 65.958 | 92.4% |
| layout_soa | 63.801 | 89.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2151ns | 2151ns | base |
| layout_soa | 2161ns | 2161ns | +0.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 62ns | base | --- | [61, 63] | --- | --- | --- | --- |
| layout_soa | 64ns | +2.0ns (+3.3%) | [+2, +2]ns | [63, 65] | YES | 0.0000 | 0.0000 | 13 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 62ns | +1.0% |
| 1 | 2 | 62ns | +1.3% |
| 1 | 3 | 62ns | +2.7% |
| 1 | 4 | 61ns | +5.3% |
| 1 | 5 | 61ns | +8.2% |
| 1 | 6 | 62ns | +3.7% |
| 1 | 7 | 62ns | +1.7% |
| 1 | 8 | 62ns | +2.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.288 | moderate+ |
| layout_soa | 0.299 | moderate+ |

**Consistency summary:**

- **layout_soa**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 2.6ns | 62.2ns | 4.2% |  |
| layout_soa | 2.6ns | 64.2ns | 4.0% |  |

## Distribution (algo ns)

```
layout_aos (n=160, range 57.4-67.2 ns)
     57.4 |########
     57.9 |####################
     58.4 |#############
     58.9 |####################
     59.4 |########
     59.8 |#############
     60.3 |########################################
     60.8 |######
     61.3 |###############
     61.8 |###########
     62.3 |####################
     62.8 |#################
     63.3 |#############
     63.8 |#############
     64.2 |########
     64.7 |######
     65.2 |###############
     65.7 |########################
     66.2 |
     66.7 |####################
  (12 below, 14 above range)

layout_soa (n=160, range 59.3-69.3 ns)
     59.3 |######
     59.8 |######
     60.3 |########
     60.8 |########################
     61.3 |#################
     61.8 |#############
     62.3 |###########
     62.8 |########################################
     63.3 |#############
     63.8 |#############
     64.3 |########
     64.8 |#################
     65.3 |####################
     65.8 |#################
     66.3 |#################
     66.8 |######################
     67.3 |######
     67.8 |########
     68.3 |###############
     68.8 |####
  (15 below, 12 above range)

```
