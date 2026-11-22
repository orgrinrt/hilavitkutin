# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sat_wrap dominates: 89% faster than the next best (sat_checked)

sat_wrap (352 ns) leads sat_checked (666 ns) by 89%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 47% (significant)

sat_wrap is -313 ns (47%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: sat_wrap** at 352.1 ns median (-47.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.89x (fastest 352.1 ns, slowest 665.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Overflow strategy: wrapping_add vs saturating_add vs checked_add |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x52b9de7c318fa064 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 2790ns | 2777ns | 2770ns | 2782ns | 2834ns | base |
| sat_saturate | 2793ns | 2777ns | 2771ns | 2782ns | 2847ns | +0.10% |
| sat_wrap | 2472ns | 2464ns | 2458ns | 2465ns | 2509ns | -11.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 667ns | 659ns | 679ns | base | 1.536 |
| sat_saturate | 668ns | 659ns | 683ns | +0.16% | 1.533 |
| sat_wrap | 352ns | 346ns | 358ns | -47.20% | 2.908 |

## Performance model

- Peak throughput: **2.957 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 1.538 | 52.0% |
| sat_saturate | 1.538 | 52.0% |
| sat_wrap | 2.908 | 98.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 2790ns | 2790ns | base |
| sat_saturate | 2793ns | 2793ns | +0.10% |
| sat_wrap | 2472ns | 2472ns | -11.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 666ns | base | --- | [665, 666] | --- | --- | --- | --- |
| sat_saturate | 666ns | no significant difference | [-0, +1]ns | [665, 667] | no | 0.1957 | 0.1957 | 7 |
| sat_wrap | 352ns | -313.4ns (-47.1%) | [-314, -312]ns | [351, 353] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 664ns | +0.2% | -46.9% |
| 1 | 2 | 675ns | -0.8% | -47.7% |
| 1 | 3 | 664ns | -0.1% | -46.9% |
| 1 | 4 | 666ns | -0.3% | -47.2% |
| 1 | 5 | 665ns | +0.0% | -47.2% |
| 1 | 6 | 664ns | +1.2% | -46.9% |
| 1 | 7 | 666ns | +0.0% | -47.1% |
| 1 | 8 | 664ns | +0.2% | -47.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.316 | moderate- |
| sat_saturate | -0.291 | moderate- |
| sat_wrap | 0.165 | ok |

**Consistency summary:**

- **sat_saturate**: won 2/8, lost 3/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 3.1ns | 666.9ns | 0.5% |  |
| sat_saturate | 3.0ns | 667.9ns | 0.4% |  |
| sat_wrap | 2.5ns | 352.1ns | 0.7% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 659.0-678.6 ns)
    659.0 |#####################
    659.9 |################################
    660.9 |###########
    661.9 |##################
    662.9 |#######################
    663.9 |#########################
    664.9 |########################################
    665.8 |############################
    666.8 |###########
    667.8 |#####################
    668.8 |#######################
    669.8 |################
    670.7 |###########
    671.7 |####
    672.7 |#######
    673.7 |#######
    674.7 |####
    675.6 |####
    676.6 |#########
    677.6 |####
  (13 below, 7 above range)

sat_saturate (n=160, range 659.5-683.0 ns)
    659.5 |###################################
    660.7 |###################################
    661.8 |################
    663.0 |##############################
    664.2 |#####################
    665.4 |########################################
    666.5 |################################
    667.7 |################################
    668.9 |################
    670.0 |##################
    671.2 |#######
    672.4 |####
    673.6 |###########
    674.7 |####
    675.9 |####
    677.1 |####
    678.3 |####
    679.4 |##
    680.6 |##
    681.8 |
  (12 below, 9 above range)

sat_wrap (n=160, range 346.3-357.8 ns)
    346.3 |##########
    346.9 |###############
    347.4 |#################
    348.0 |##########
    348.6 |#####
    349.2 |###############
    349.7 |#################
    350.3 |################################
    350.9 |#################
    351.5 |############
    352.0 |########################################
    352.6 |#################
    353.2 |##########
    353.8 |######################
    354.3 |####################
    354.9 |###########################
    355.5 |#######
    356.1 |#######
    356.6 |###########################
    357.2 |#######
  (13 below, 11 above range)

```
