# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 3.3x slower than the field

rot_variable_amount (247 ns) is 3.3x the fastest (74 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (rot_intrinsic_const, rot_manual_shifts) are a dead heat (<1%)

rot_intrinsic_const (74 ns) and rot_manual_shifts (74 ns) differ by 0.54%, inside the noise, even though the wider field spreads 233.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (rot_intrinsic_const)

The baseline rot_intrinsic_const is the fastest (74 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.3x the fastest

Fastest rot_intrinsic_const (74 ns) to slowest rot_variable_amount (247 ns): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (rot_intrinsic_const) is the fastest** at 74.0 ns median
- 1 variant significantly slower than baseline
- Spread: 3.34x (fastest 74.0 ns, slowest 247.1 ns)

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
| rot_intrinsic_const | 2252ns | 2227ns | 2138ns | 2240ns | 2399ns | base |
| rot_manual_shifts | 2262ns | 2275ns | 2142ns | 2254ns | 2405ns | +0.45% |
| rot_variable_amount | 2499ns | 2466ns | 2352ns | 2485ns | 2685ns | +10.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 75ns | 67ns | 84ns | base | 13.703 |
| rot_manual_shifts | 75ns | 67ns | 84ns | +0.24% | 13.670 |
| rot_variable_amount | 251ns | 235ns | 275ns | +236.34% | 4.074 |

## Performance model

- Peak throughput: **15.319 Gops/s** (rot_manual_shifts; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 13.838 | 90.3% |
| rot_manual_shifts | 13.763 | 89.8% |
| rot_variable_amount | 4.144 | 27.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2252ns | 2252ns | base |
| rot_manual_shifts | 2262ns | 2262ns | +0.45% |
| rot_variable_amount | 2499ns | 2499ns | +10.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 74ns | base | --- | [73, 75] | --- | --- | --- | --- |
| rot_manual_shifts | 74ns | no significant difference | [-1, +1]ns | [73, 75] | no | 0.4249 | 0.4249 | 3 |
| rot_variable_amount | 247ns | +172.5ns (+233.1%) | [+170, +175]ns | [246, 248] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 70ns | -0.9% | +271.0% |
| 1 | 2 | 78ns | -7.5% | +217.4% |
| 1 | 3 | 72ns | +5.0% | +250.1% |
| 1 | 4 | 74ns | -3.1% | +220.0% |
| 1 | 5 | 71ns | -0.2% | +243.8% |
| 1 | 6 | 75ns | +2.3% | +248.9% |
| 1 | 7 | 81ns | -3.0% | +207.3% |
| 1 | 8 | 72ns | +7.6% | +242.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.386 | moderate- |
| rot_manual_shifts | 0.362 | moderate+ |
| rot_variable_amount | -0.158 | ok |

**Consistency summary:**

- **rot_manual_shifts**: won 5/8, lost 3/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 3.0ns | 74.7ns | 4.0% |  |
| rot_manual_shifts | 3.2ns | 74.9ns | 4.2% |  |
| rot_variable_amount | 3.2ns | 251.3ns | 1.3% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 67.2-83.6 ns)
     67.2 |############
     68.1 |######################
     68.9 |###############
     69.7 |###########################
     70.5 |######################
     71.3 |###############
     72.2 |#######
     73.0 |######################
     73.8 |########################################
     74.6 |#########################
     75.4 |######################
     76.2 |#####
     77.1 |##########
     77.9 |##########
     78.7 |####################
     79.5 |#####
     80.3 |#################
     81.1 |##########
     82.0 |#######
     82.8 |############
  (14 below, 14 above range)

rot_manual_shifts (n=160, range 66.8-84.1 ns)
     66.8 |#################################
     67.7 |###
     68.6 |################
     69.4 |#################################
     70.3 |#######################
     71.2 |#################################
     72.0 |####################
     72.9 |####################################
     73.8 |########################################
     74.6 |##########################
     75.5 |##########
     76.3 |##############################
     77.2 |####################
     78.1 |##########################
     78.9 |#############
     79.8 |#######################
     80.7 |##########
     81.5 |#############
     82.4 |##########################
     83.3 |###
  (14 below, 13 above range)

rot_variable_amount (n=160, range 235.0-275.1 ns)
    235.0 |###############
    237.0 |###############
    239.0 |#############
    241.0 |#############
    243.0 |########################################
    245.0 |############################
    247.0 |################################
    249.0 |#############
    251.0 |#############
    253.0 |#######
    255.0 |###
    257.1 |###
    259.1 |###
    261.1 |#############
    263.1 |###############
    265.1 |####################
    267.1 |#######
    269.1 |###
    271.1 |#####
    273.1 |#
  (12 below, 5 above range)

```
