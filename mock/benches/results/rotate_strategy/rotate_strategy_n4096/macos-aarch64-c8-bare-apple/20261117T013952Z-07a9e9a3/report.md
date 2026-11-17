# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 3.2x slower than the field

rot_variable_amount (953 ns) is 3.2x the fastest (297 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (rot_intrinsic_const, rot_manual_shifts) are a dead heat (<1%)

rot_intrinsic_const (297 ns) and rot_manual_shifts (298 ns) differ by 0.34%, inside the noise, even though the wider field spreads 220.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (rot_intrinsic_const)

The baseline rot_intrinsic_const is the fastest (297 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.2x the fastest

Fastest rot_intrinsic_const (297 ns) to slowest rot_variable_amount (953 ns): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (rot_intrinsic_const) is the fastest** at 297.1 ns median
- 1 variant significantly slower than baseline
- Spread: 3.21x (fastest 297.1 ns, slowest 953.3 ns)

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
| rot_intrinsic_const | 2434ns | 2400ns | 2376ns | 2412ns | 2558ns | base |
| rot_manual_shifts | 2444ns | 2390ns | 2376ns | 2419ns | 2586ns | +0.40% |
| rot_variable_amount | 3080ns | 3039ns | 3027ns | 3060ns | 3190ns | +26.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 301ns | 290ns | 318ns | base | 13.630 |
| rot_manual_shifts | 308ns | 291ns | 349ns | +2.45% | 13.305 |
| rot_variable_amount | 962ns | 946ns | 999ns | +220.24% | 4.256 |

## Performance model

- Peak throughput: **14.104 Gops/s** (rot_intrinsic_const; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 13.787 | 97.8% |
| rot_manual_shifts | 13.740 | 97.4% |
| rot_variable_amount | 4.297 | 30.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2434ns | 2434ns | base |
| rot_manual_shifts | 2444ns | 2444ns | +0.40% |
| rot_variable_amount | 3080ns | 3080ns | +26.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 297ns | base | --- | [296, 299] | --- | --- | --- | --- |
| rot_manual_shifts | 298ns | no significant difference | [-0, +2]ns | [297, 300] | no | 0.3010 | 0.3010 | 2 |
| rot_variable_amount | 953ns | +655.8ns (+220.7%) | [+655, +657]ns | [951, 955] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 293ns | +0.6% | +223.7% |
| 1 | 2 | 299ns | -1.7% | +217.2% |
| 1 | 3 | 309ns | -3.1% | +217.5% |
| 1 | 4 | 300ns | +4.5% | +216.2% |
| 1 | 5 | 295ns | +6.0% | +223.8% |
| 1 | 6 | 299ns | -0.4% | +225.4% |
| 1 | 7 | 294ns | +1.3% | +225.1% |
| 1 | 8 | 298ns | -0.6% | +219.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | 0.080 | ok |
| rot_manual_shifts | 0.407 | moderate+ |
| rot_variable_amount | -0.350 | moderate- |

**Consistency summary:**

- **rot_manual_shifts**: won 4/8, lost 4/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 2.7ns | 300.5ns | 0.9% |  |
| rot_manual_shifts | 2.6ns | 307.9ns | 0.9% |  |
| rot_variable_amount | 2.6ns | 962.3ns | 0.3% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 290.4-318.3 ns)
    290.4 |#########################
    291.8 |#########################
    293.2 |########################################
    294.6 |#########################
    296.0 |#####################################
    297.4 |#####################
    298.8 |##################
    300.2 |#########################
    301.6 |#########################
    303.0 |#########
    304.4 |################
    305.8 |##################
    307.2 |###########
    308.6 |#######
    309.9 |##
    311.3 |##
    312.7 |##
    314.1 |#######
    315.5 |
    316.9 |
  (15 below, 7 above range)

rot_manual_shifts (n=160, range 291.3-349.2 ns)
    291.3 |######################################
    294.2 |########################################
    297.1 |######################################
    300.0 |#####################
    302.9 |###############
    305.8 |#######
    308.7 |##################
    311.6 |#########
    314.5 |###############
    317.3 |#######
    320.2 |###
    323.1 |
    326.0 |
    328.9 |
    331.8 |
    334.7 |
    337.6 |
    340.5 |
    343.4 |
    346.3 |
  (15 below, 5 above range)

rot_variable_amount (n=160, range 945.8-999.3 ns)
    945.8 |########################################
    948.5 |###################################
    951.2 |############################
    953.8 |####################
    956.5 |#######
    959.2 |#####
    961.9 |#####
    964.5 |##########
    967.2 |#####
    969.9 |#######
    972.6 |##
    975.2 |#####
    977.9 |########
    980.6 |####
    983.3 |########
    985.9 |#
    988.6 |#
    991.3 |
    994.0 |
    996.6 |
  (15 below, 9 above range)

```
