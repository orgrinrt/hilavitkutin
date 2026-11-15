# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 3.3x slower than the field

rot_variable_amount (1.01 us) is 3.3x the fastest (309 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_manual_shifts is fastest but the noisiest (CV 7.1%)

rot_manual_shifts wins on median (309 ns) yet has the highest variance (CV 7.1%), while rot_variable_amount is the steadiest (CV 3.2%, 1.01 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3.3x the fastest

Fastest rot_manual_shifts (309 ns) to slowest rot_variable_amount (1.01 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rot_manual_shifts** at 308.8 ns median (-1.6% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 3.28x (fastest 308.8 ns, slowest 1014.2 ns)

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
| rot_intrinsic_const | 2515ns | 2543ns | 2369ns | 2492ns | 2729ns | base |
| rot_manual_shifts | 2486ns | 2498ns | 2371ns | 2470ns | 2649ns | -1.14% |
| rot_variable_amount | 3172ns | 3239ns | 3023ns | 3184ns | 3284ns | +26.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 313ns | 293ns | 340ns | base | 13.095 |
| rot_manual_shifts | 311ns | 292ns | 339ns | -0.64% | 13.179 |
| rot_variable_amount | 995ns | 950ns | 1029ns | +218.04% | 4.117 |

## Performance model

- Peak throughput: **14.006 Gops/s** (rot_manual_shifts; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 13.053 | 93.2% |
| rot_manual_shifts | 13.264 | 94.7% |
| rot_variable_amount | 4.039 | 28.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2515ns | 2515ns | base |
| rot_manual_shifts | 2486ns | 2486ns | -1.14% |
| rot_variable_amount | 3172ns | 3172ns | +26.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 314ns | base | --- | [309, 315] | --- | --- | --- | --- |
| rot_manual_shifts | 309ns | no significant difference | [-1, +0]ns | [302, 312] | no | 0.3010 | 0.3010 | 2 |
| rot_variable_amount | 1014ns | +693.2ns (+220.9%) | [+671, +701]ns | [991, 1017] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 295ns | +1.5% | +224.7% |
| 1 | 2 | 314ns | +1.8% | +223.8% |
| 1 | 3 | 315ns | +1.1% | +223.4% |
| 1 | 4 | 301ns | -0.4% | +238.2% |
| 1 | 5 | 318ns | -7.2% | +208.7% |
| 1 | 6 | 321ns | -1.6% | +197.0% |
| 1 | 7 | 329ns | -4.7% | +197.6% |
| 1 | 8 | 297ns | -0.5% | +243.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.162 | ok |
| rot_manual_shifts | -0.087 | ok |
| rot_variable_amount | 0.139 | ok |

**Consistency summary:**

- **rot_manual_shifts**: won 5/8, lost 3/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 2.7ns | 312.8ns | 0.9% |  |
| rot_manual_shifts | 2.7ns | 310.8ns | 0.9% |  |
| rot_variable_amount | 2.8ns | 994.8ns | 0.3% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 293.1-339.9 ns)
    293.1 |################################
    295.4 |#####################
    297.8 |#######################
    300.1 |#########################
    302.5 |###########
    304.8 |##
    307.1 |##
    309.5 |#######
    311.8 |############################
    314.2 |########################################
    316.5 |################################
    318.8 |#####################
    321.2 |##################
    323.5 |#######
    325.9 |
    328.2 |
    330.5 |
    332.9 |#######
    335.2 |#########
    337.6 |###########
  (17 below, 14 above range)

rot_manual_shifts (n=160, range 292.5-339.2 ns)
    292.5 |#########################
    294.8 |########################################
    297.1 |#######
    299.5 |#######################
    301.8 |##########
    304.2 |#########
    306.5 |#######
    308.8 |###
    311.2 |#########################
    313.5 |#####################
    315.8 |##################
    318.2 |##############
    320.5 |############
    322.9 |##########
    325.2 |###
    327.5 |
    329.9 |#
    332.2 |
    334.6 |
    336.9 |############
  (13 below, 10 above range)

rot_variable_amount (n=160, range 949.6-1028.9 ns)
    949.6 |############
    953.5 |#########
    957.5 |#
    961.5 |##
    965.4 |###
    969.4 |####
    973.4 |##
    977.3 |####
    981.3 |###
    985.3 |##
    989.2 |
    993.2 |##
    997.2 |
   1001.1 |
   1005.1 |
   1009.1 |##
   1013.1 |###############
   1017.0 |########################################
   1021.0 |#############
   1025.0 |######
  (16 below, 4 above range)

```
