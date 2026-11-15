# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### layout_aos shows alternating (throttle bounce) (autocorr -0.67)

layout_aos's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4 ns) is smaller than the fastest variant's own run-to-run std-dev (5 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (65 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (layout_aos) is the fastest** at 65.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 65.4 ns, slowest 69.0 ns)

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
| layout_aos | 2229ns | 2204ns | 2138ns | 2199ns | 2412ns | base |
| layout_soa | 2291ns | 2245ns | 2131ns | 2281ns | 2483ns | +2.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 65ns | 59ns | 73ns | base | 62.565 |
| layout_soa | 69ns | 62ns | 77ns | +5.80% | 59.133 |

## Performance model

- Peak throughput: **69.336 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 62.630 | 90.3% |
| layout_soa | 59.405 | 85.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2229ns | 2229ns | base |
| layout_soa | 2291ns | 2291ns | +2.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 65ns | base | --- | [64, 66] | --- | --- | --- | --- |
| layout_soa | 69ns | +4.4ns (+6.7%) | [+3, +6]ns | [68, 70] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 62ns | +6.7% |
| 1 | 2 | 65ns | +13.8% |
| 1 | 3 | 64ns | +4.2% |
| 1 | 4 | 68ns | +8.0% |
| 1 | 5 | 62ns | +15.0% |
| 1 | 6 | 68ns | +1.5% |
| 1 | 7 | 64ns | +5.2% |
| 1 | 8 | 67ns | -6.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.672 | HIGH- (thermal bounce) |
| layout_soa | -0.142 | ok |

**Consistency summary:**

- **layout_soa**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 2.9ns | 65.5ns | 4.4% |  |
| layout_soa | 2.8ns | 69.3ns | 4.1% |  |

## Distribution (algo ns)

```
layout_aos (n=160, range 59.1-73.1 ns)
     59.1 |##########
     59.8 |#####################
     60.5 |############
     61.2 |################
     61.9 |##################
     62.6 |######
     63.3 |#######################
     64.0 |############
     64.7 |##############
     65.4 |#######################
     66.1 |#######################
     66.8 |########################################
     67.5 |##########
     68.2 |##################
     68.9 |####
     69.6 |######
     70.3 |########
     71.0 |
     71.7 |######
     72.4 |####
  (13 below, 13 above range)

layout_soa (n=160, range 61.6-77.2 ns)
     61.6 |########################
     62.4 |####################################
     63.1 |################
     63.9 |####################
     64.7 |################################
     65.5 |########################################
     66.3 |########################
     67.1 |####################################
     67.8 |############################
     68.6 |########################################
     69.4 |####################
     70.2 |########################
     71.0 |############################
     71.7 |################################
     72.5 |####################
     73.3 |############################
     74.1 |########################################
     74.9 |####################
     75.7 |####################
     76.4 |############
  (12 below, 13 above range)

```
