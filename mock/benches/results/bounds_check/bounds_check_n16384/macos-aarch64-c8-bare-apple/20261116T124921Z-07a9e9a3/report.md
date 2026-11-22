# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (bc_index, bc_iter) are a dead heat (<1%)

bc_index (352 ns) and bc_iter (353 ns) differ by 0.11%, inside the noise, even though the wider field spreads 77.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (bc_index)

The baseline bc_index is the fastest (352 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (bc_index) is the fastest** at 352.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.78x (fastest 352.5 ns, slowest 627.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Bounds-check elision: iter() vs explicit index vs get_unchecked |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5a3ec07491bf2d68 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 2453ns | 2431ns | 2419ns | 2444ns | 2513ns | base |
| bc_iter | 2452ns | 2466ns | 2420ns | 2453ns | 2482ns | -0.03% |
| bc_unchecked | 2732ns | 2746ns | 2698ns | 2731ns | 2771ns | +11.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 354ns | 346ns | 365ns | base | 46.300 |
| bc_iter | 354ns | 347ns | 362ns | -0.03% | 46.314 |
| bc_unchecked | 627ns | 618ns | 639ns | +77.26% | 26.120 |

## Performance model

- Peak throughput: **47.379 Gops/s** (bc_index; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 46.479 | 98.1% |
| bc_iter | 46.427 | 98.0% |
| bc_unchecked | 26.127 | 55.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2453ns | 2453ns | base |
| bc_iter | 2452ns | 2452ns | -0.03% |
| bc_unchecked | 2732ns | 2732ns | +11.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 352ns | base | --- | [351, 354] | --- | --- | --- | --- |
| bc_iter | 353ns | no significant difference | [-0, +1]ns | [352, 355] | no | 1.0000 | 1.0000 | 11 |
| bc_unchecked | 627ns | +273.0ns (+77.4%) | [+272, +274]ns | [624, 629] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 352ns | +0.7% | +79.1% |
| 1 | 2 | 354ns | -0.1% | +75.8% |
| 1 | 3 | 350ns | +1.1% | +77.8% |
| 1 | 4 | 351ns | +0.2% | +79.5% |
| 1 | 5 | 362ns | -2.6% | +74.4% |
| 1 | 6 | 351ns | +0.2% | +78.4% |
| 1 | 7 | 349ns | +1.7% | +79.5% |
| 1 | 8 | 350ns | +0.5% | +77.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.093 | ok |
| bc_iter | -0.194 | ok |
| bc_unchecked | -0.044 | ok |

**Consistency summary:**

- **bc_iter**: won 2/8, lost 6/8
- **bc_unchecked**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 3.3ns | 353.9ns | 0.9% |  |
| bc_iter | 3.1ns | 353.8ns | 0.9% |  |
| bc_unchecked | 3.2ns | 627.2ns | 0.5% |  |

## Distribution (algo ns)

```
bc_index (n=160, range 345.8-365.0 ns)
    345.8 |######
    346.8 |############
    347.7 |##########
    348.7 |########################################
    349.6 |###############
    350.6 |#################
    351.6 |############
    352.5 |############
    353.5 |##########
    354.4 |########
    355.4 |#################
    356.4 |#####
    357.3 |########
    358.3 |##########
    359.2 |#############
    360.2 |###
    361.2 |#############
    362.1 |###
    363.1 |#
    364.0 |######
  (17 below, 10 above range)

bc_iter (n=160, range 346.8-361.9 ns)
    346.8 |#################################
    347.5 |#######################
    348.3 |##########
    349.1 |####################################
    349.8 |##############################
    350.6 |##############################
    351.3 |####################
    352.1 |##############################
    352.8 |##########################
    353.6 |####################
    354.4 |#######################
    355.1 |########################################
    355.9 |######
    356.6 |##########################
    357.4 |#################################
    358.2 |###
    358.9 |#############
    359.7 |#######################
    360.4 |###
    361.2 |##########
  (12 below, 15 above range)

bc_unchecked (n=160, range 617.5-639.2 ns)
    617.5 |#####
    618.6 |#####################################
    619.7 |########################
    620.8 |#############################
    621.8 |##########################
    622.9 |#####################
    624.0 |##########
    625.1 |#####
    626.2 |################
    627.3 |#############
    628.3 |#####################
    629.4 |##################
    630.5 |########################################
    631.6 |#####################
    632.7 |########
    633.8 |################
    634.9 |##################
    635.9 |##########
    637.0 |##################
    638.1 |########
  (16 below, 5 above range)

```
