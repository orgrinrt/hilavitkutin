# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 3.3x slower than the field

rot_variable_amount (235 ns) is 3.3x the fastest (72 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_intrinsic_const is fastest but the noisiest (CV 10.3%)

rot_intrinsic_const wins on median (72 ns) yet has the highest variance (CV 10.3%), while rot_variable_amount is the steadiest (CV 1.6%, 235 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (rot_intrinsic_const, rot_manual_shifts) are a dead heat (<1%)

rot_intrinsic_const (72 ns) and rot_manual_shifts (72 ns) differ by 0.00%, inside the noise, even though the wider field spreads 227.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (rot_intrinsic_const)

The baseline rot_intrinsic_const is the fastest (72 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.3x the fastest

Fastest rot_intrinsic_const (72 ns) to slowest rot_variable_amount (235 ns): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (rot_intrinsic_const) is the fastest** at 71.7 ns median
- 1 variant significantly slower than baseline
- Spread: 3.27x (fastest 71.7 ns, slowest 234.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Rotate strategy: intrinsic const vs manual shifts vs variable amount |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x71f692ab4e08d325 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rot_intrinsic_const | 2202ns | 2190ns | 2185ns | 2193ns | 2246ns | base |
| rot_manual_shifts | 2196ns | 2191ns | 2184ns | 2191ns | 2225ns | -0.24% |
| rot_variable_amount | 2359ns | 2353ns | 2347ns | 2354ns | 2384ns | +7.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 73ns | 67ns | 82ns | base | 14.035 |
| rot_manual_shifts | 72ns | 67ns | 79ns | -0.77% | 14.144 |
| rot_variable_amount | 235ns | 230ns | 241ns | +222.25% | 4.355 |

## Performance model

- Peak throughput: **15.355 Gops/s** (rot_intrinsic_const; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 14.282 | 93.0% |
| rot_manual_shifts | 14.282 | 93.0% |
| rot_variable_amount | 4.365 | 28.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2202ns | 2202ns | base |
| rot_manual_shifts | 2196ns | 2196ns | -0.24% |
| rot_variable_amount | 2359ns | 2359ns | +7.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 72ns | base | --- | [71, 73] | --- | --- | --- | --- |
| rot_manual_shifts | 72ns | no significant difference | [-1, +0]ns | [70, 73] | no | 0.8724 | 0.8724 | 5 |
| rot_variable_amount | 235ns | +162.5ns (+226.6%) | [+162, +163]ns | [234, 235] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 71ns | +1.8% | +231.1% |
| 1 | 2 | 71ns | +0.8% | +232.3% |
| 1 | 3 | 71ns | -0.3% | +228.6% |
| 1 | 4 | 72ns | -2.6% | +226.5% |
| 1 | 5 | 72ns | +0.9% | +222.7% |
| 1 | 6 | 71ns | +0.9% | +228.6% |
| 1 | 7 | 72ns | -0.8% | +223.6% |
| 1 | 8 | 72ns | -0.8% | +224.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | 0.420 | moderate+ |
| rot_manual_shifts | -0.129 | ok |
| rot_variable_amount | -0.470 | moderate- |

**Consistency summary:**

- **rot_manual_shifts**: won 4/8, lost 4/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 3.1ns | 73.0ns | 4.3% |  |
| rot_manual_shifts | 2.9ns | 72.4ns | 4.1% |  |
| rot_variable_amount | 2.9ns | 235.1ns | 1.2% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 66.7-81.6 ns)
     66.7 |##########
     67.4 |############
     68.2 |###################################
     68.9 |####################
     69.7 |#####################################
     70.4 |####################
     71.2 |########################################
     71.9 |######################
     72.6 |####################
     73.4 |###############
     74.1 |#################
     74.9 |###############
     75.6 |#################
     76.4 |#################
     77.1 |#####
     77.9 |###############
     78.6 |##
     79.3 |############
     80.1 |##########
     80.8 |#######
  (13 below, 6 above range)

rot_manual_shifts (n=160, range 66.8-79.2 ns)
     66.8 |#######
     67.5 |#######################
     68.1 |##############################
     68.7 |###########
     69.3 |##################
     69.9 |########################################
     70.6 |#######
     71.2 |#######################
     71.8 |##############
     72.4 |#########################
     73.0 |#########
     73.6 |####
     74.3 |#######
     74.9 |################################
     75.5 |##############
     76.1 |####
     76.7 |#######
     77.4 |################
     78.0 |#######
     78.6 |#########
  (13 below, 13 above range)

rot_variable_amount (n=160, range 230.3-241.0 ns)
    230.3 |#######
    230.8 |#################
    231.4 |#########
    231.9 |#########
    232.4 |############################
    233.0 |###############
    233.5 |###############
    234.0 |###########
    234.6 |########################################
    235.1 |#######
    235.6 |###################
    236.2 |#####
    236.7 |###########
    237.2 |#####
    237.8 |###############
    238.3 |#########
    238.8 |#######
    239.4 |###
    239.9 |###########
    240.4 |#####
  (13 below, 12 above range)

```
