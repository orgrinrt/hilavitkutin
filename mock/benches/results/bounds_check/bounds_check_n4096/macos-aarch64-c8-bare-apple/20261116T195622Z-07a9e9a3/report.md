# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (bc_index, bc_iter) are a dead heat (<1%)

bc_index (92 ns) and bc_iter (93 ns) differ by 0.86%, inside the noise, even though the wider field spreads 53.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (bc_index)

The baseline bc_index is the fastest (92 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (bc_index) is the fastest** at 92.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.53x (fastest 92.5 ns, slowest 141.7 ns)

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
| bc_index | 2190ns | 2169ns | 2161ns | 2179ns | 2250ns | base |
| bc_iter | 2200ns | 2173ns | 2163ns | 2188ns | 2274ns | +0.47% |
| bc_unchecked | 2227ns | 2223ns | 2216ns | 2223ns | 2250ns | +1.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 93ns | 87ns | 100ns | base | 43.987 |
| bc_iter | 94ns | 88ns | 101ns | +0.59% | 43.730 |
| bc_unchecked | 143ns | 137ns | 150ns | +53.07% | 28.736 |

## Performance model

- Peak throughput: **46.941 Gops/s** (bc_index; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 44.281 | 94.3% |
| bc_iter | 43.901 | 93.5% |
| bc_unchecked | 28.906 | 61.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2190ns | 2190ns | base |
| bc_iter | 2200ns | 2200ns | +0.47% |
| bc_unchecked | 2227ns | 2227ns | +1.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 92ns | base | --- | [92, 94] | --- | --- | --- | --- |
| bc_iter | 93ns | no significant difference | [+0, +1]ns | [92, 94] | no (adj: YES) | 0.0184 | 0.0184 | 8 |
| bc_unchecked | 142ns | +48.8ns (+52.8%) | [+48, +50]ns | [141, 142] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 94ns | -2.8% | +51.9% |
| 1 | 2 | 96ns | -3.5% | +48.0% |
| 1 | 3 | 94ns | +1.1% | +49.9% |
| 1 | 4 | 91ns | +0.2% | +55.1% |
| 1 | 5 | 92ns | +3.1% | +52.3% |
| 1 | 6 | 91ns | +4.2% | +55.4% |
| 1 | 7 | 92ns | +1.3% | +55.0% |
| 1 | 8 | 91ns | +1.2% | +54.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | 0.460 | moderate+ |
| bc_iter | -0.233 | moderate- |
| bc_unchecked | -0.013 | ok |

**Consistency summary:**

- **bc_iter**: won 2/8, lost 6/8
- **bc_unchecked**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 2.4ns | 93.1ns | 2.5% |  |
| bc_iter | 2.2ns | 93.7ns | 2.4% |  |
| bc_unchecked | 2.5ns | 142.5ns | 1.7% |  |

## Distribution (algo ns)

```
bc_index (n=160, range 87.3-100.0 ns)
     87.3 |#########
     87.9 |###########################
     88.5 |########################
     89.2 |########################################
     89.8 |########################
     90.4 |############
     91.1 |########################################
     91.7 |#####################
     92.4 |##############################
     93.0 |#########
     93.6 |####################################
     94.3 |
     94.9 |###########################
     95.5 |###############
     96.2 |########################
     96.8 |##################
     97.4 |#####################
     98.1 |######
     98.7 |#####################
     99.4 |###
  (13 below, 12 above range)

bc_iter (n=160, range 87.7-100.9 ns)
     87.7 |##################
     88.3 |##
     89.0 |#############################
     89.7 |#####################
     90.3 |########################
     91.0 |################
     91.6 |################################
     92.3 |#############
     93.0 |########################
     93.6 |########################################
     94.3 |#############
     95.0 |#####################################
     95.6 |##################
     96.3 |########
     96.9 |#####
     97.6 |########
     98.3 |##########
     98.9 |##
     99.6 |#############
    100.3 |##################
  (14 below, 12 above range)

bc_unchecked (n=160, range 137.2-150.2 ns)
    137.2 |####
    137.8 |#############
    138.5 |######
    139.1 |#################################
    139.8 |#####################################
    140.4 |######################
    141.1 |########################################
    141.7 |######################
    142.4 |###############
    143.0 |###########
    143.7 |######################
    144.4 |######################
    145.0 |###########
    145.7 |###########
    146.3 |####
    147.0 |########
    147.6 |######
    148.3 |###########
    148.9 |####
    149.6 |##
  (17 below, 3 above range)

```
