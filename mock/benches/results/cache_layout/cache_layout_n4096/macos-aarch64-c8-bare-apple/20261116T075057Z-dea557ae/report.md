# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### layout_aos is fastest but the noisiest (CV 9.4%)

layout_aos wins on median (63 ns) yet has the highest variance (CV 9.4%), while layout_soa is the steadiest (CV 9.3%, 65 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### layout_aos shows alternating (throttle bounce) (autocorr -0.76)

layout_aos's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (6 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (63 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader layout_aos vs stability leader layout_soa (+3% speed for 1.0x steadier)

layout_aos is fastest (63 ns, CV 9.4%); layout_soa gives up 3.3% median for 1.0x lower variance (CV 9.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (layout_aos) is the fastest** at 63.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.03x (fastest 63.3 ns, slowest 65.4 ns)

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
| layout_aos | 2165ns | 2172ns | 2136ns | 2164ns | 2195ns | base |
| layout_soa | 2153ns | 2140ns | 2131ns | 2144ns | 2202ns | -0.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 64ns | 59ns | 70ns | base | 64.289 |
| layout_soa | 66ns | 61ns | 73ns | +3.42% | 62.162 |

## Performance model

- Peak throughput: **69.734 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 64.708 | 92.8% |
| layout_soa | 62.630 | 89.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2165ns | 2165ns | base |
| layout_soa | 2153ns | 2153ns | -0.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 63ns | base | --- | [63, 64] | --- | --- | --- | --- |
| layout_soa | 65ns | +1.9ns (+2.9%) | [+2, +2]ns | [65, 66] | YES | 0.0000 | 0.0000 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 62ns | +2.4% |
| 1 | 2 | 65ns | +1.9% |
| 1 | 3 | 62ns | +4.3% |
| 1 | 4 | 64ns | +0.0% |
| 1 | 5 | 64ns | +3.5% |
| 1 | 6 | 64ns | +1.2% |
| 1 | 7 | 62ns | +4.0% |
| 1 | 8 | 64ns | +4.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.761 | HIGH- (thermal bounce) |
| layout_soa | -0.333 | moderate- |

**Consistency summary:**

- **layout_soa**: won 0/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 2.7ns | 63.7ns | 4.2% |  |
| layout_soa | 2.8ns | 65.9ns | 4.3% |  |

## Distribution (algo ns)

```
layout_aos (n=160, range 58.7-70.0 ns)
     58.7 |###############
     59.3 |#####
     59.9 |########################################
     60.4 |####################
     61.0 |##########
     61.6 |#########################
     62.1 |############
     62.7 |###########################
     63.3 |###########################
     63.8 |####################
     64.4 |###########################
     65.0 |##############################
     65.5 |#######
     66.1 |###############
     66.7 |####################
     67.2 |############
     67.8 |###############
     68.4 |#######
     68.9 |#####
     69.5 |##
  (16 below, 6 above range)

layout_soa (n=160, range 60.8-73.0 ns)
     60.8 |#############
     61.4 |######
     62.0 |########################################
     62.6 |###########
     63.2 |########################################
     63.8 |########
     64.4 |############################
     65.1 |###########
     65.7 |#####################################
     66.3 |###############
     66.9 |#############
     67.5 |######################
     68.1 |####################
     68.7 |#################
     69.3 |########
     69.9 |###########
     70.5 |##
     71.2 |####
     71.8 |
     72.4 |######
  (12 below, 4 above range)

```
