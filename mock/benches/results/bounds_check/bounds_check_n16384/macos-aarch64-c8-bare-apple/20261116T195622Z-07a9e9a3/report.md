# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (bc_iter, bc_index) are a dead heat (<1%)

bc_iter (350 ns) and bc_index (352 ns) differ by 0.71%, inside the noise, even though the wider field spreads 78.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: bc_iter** at 350.0 ns median (-0.7% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.78x (fastest 350.0 ns, slowest 623.1 ns)

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
| bc_index | 2457ns | 2431ns | 2420ns | 2444ns | 2534ns | base |
| bc_iter | 2441ns | 2424ns | 2418ns | 2428ns | 2504ns | -0.66% |
| bc_unchecked | 2720ns | 2703ns | 2696ns | 2714ns | 2763ns | +10.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 353ns | 345ns | 364ns | base | 46.351 |
| bc_iter | 351ns | 345ns | 360ns | -0.56% | 46.612 |
| bc_unchecked | 625ns | 616ns | 639ns | +76.89% | 26.204 |

## Performance model

- Peak throughput: **47.458 Gops/s** (bc_iter; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 46.479 | 97.9% |
| bc_iter | 46.811 | 98.6% |
| bc_unchecked | 26.294 | 55.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2457ns | 2457ns | base |
| bc_iter | 2441ns | 2441ns | -0.66% |
| bc_unchecked | 2720ns | 2720ns | +10.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 352ns | base | --- | [351, 354] | --- | --- | --- | --- |
| bc_iter | 350ns | no significant difference | [-2, +0]ns | [350, 351] | no | 0.0566 | 0.0566 | 14 |
| bc_unchecked | 623ns | +271.3ns (+77.0%) | [+270, +272]ns | [622, 625] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 349ns | +0.6% | +78.0% |
| 1 | 2 | 350ns | +0.1% | +77.3% |
| 1 | 3 | 356ns | -2.1% | +74.3% |
| 1 | 4 | 354ns | -1.2% | +78.0% |
| 1 | 5 | 354ns | -0.3% | +77.5% |
| 1 | 6 | 354ns | +0.2% | +75.4% |
| 1 | 7 | 350ns | +0.3% | +77.5% |
| 1 | 8 | 356ns | -1.8% | +77.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.104 | ok |
| bc_iter | 0.338 | moderate+ |
| bc_unchecked | 0.041 | ok |

**Consistency summary:**

- **bc_iter**: won 4/8, lost 3/8
- **bc_unchecked**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 3.3ns | 353.5ns | 0.9% |  |
| bc_iter | 3.3ns | 351.5ns | 0.9% |  |
| bc_unchecked | 3.2ns | 625.2ns | 0.5% |  |

## Distribution (algo ns)

```
bc_index (n=160, range 345.4-364.0 ns)
    345.4 |###########
    346.3 |########
    347.3 |###############
    348.2 |############################
    349.1 |########################################
    350.1 |########
    351.0 |####################
    351.9 |###########
    352.8 |#################
    353.8 |######################
    354.7 |#############
    355.6 |###############
    356.6 |###############
    357.5 |######################
    358.4 |#################
    359.3 |##
    360.3 |########
    361.2 |##
    362.1 |###########
    363.1 |##
  (16 below, 11 above range)

bc_iter (n=160, range 345.2-359.9 ns)
    345.2 |#################
    346.0 |#################
    346.7 |#################
    347.4 |##############################
    348.2 |###########################
    348.9 |########################################
    349.6 |######################
    350.4 |###############
    351.1 |####################
    351.8 |###########################
    352.6 |############
    353.3 |##########
    354.0 |#################
    354.8 |#################
    355.5 |######################
    356.3 |##
    357.0 |#######
    357.7 |##########
    358.5 |#######
    359.2 |##
  (12 below, 10 above range)

bc_unchecked (n=160, range 616.2-638.7 ns)
    616.2 |###############
    617.4 |####################
    618.5 |###############################
    619.6 |######################
    620.7 |########################################
    621.9 |#################
    623.0 |###############
    624.1 |####################
    625.2 |###############
    626.3 |###########
    627.5 |###############
    628.6 |###############
    629.7 |###############
    630.8 |########
    631.9 |######
    633.1 |####
    634.2 |####
    635.3 |########
    636.4 |######
    637.6 |##
  (14 below, 12 above range)

```
