# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_index is fastest but the noisiest (CV 10.4%)

bc_index wins on median (103 ns) yet has the highest variance (CV 10.4%), while bc_unchecked is the steadiest (CV 6.6%, 150 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (bc_index, bc_iter) are a dead heat (<1%)

bc_index (103 ns) and bc_iter (104 ns) differ by 0.68%, inside the noise, even though the wider field spreads 45.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (bc_index)

The baseline bc_index is the fastest (103 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (bc_index) is the fastest** at 103.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.45x (fastest 103.1 ns, slowest 149.6 ns)

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
| bc_index | 2384ns | 2482ns | 2164ns | 2417ns | 2505ns | base |
| bc_iter | 2397ns | 2485ns | 2162ns | 2440ns | 2501ns | +0.54% |
| bc_unchecked | 2371ns | 2339ns | 2210ns | 2357ns | 2571ns | -0.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 103ns | 91ns | 116ns | base | 39.665 |
| bc_iter | 103ns | 91ns | 113ns | -0.18% | 39.739 |
| bc_unchecked | 152ns | 140ns | 167ns | +47.05% | 26.973 |

## Performance model

- Peak throughput: **44.957 Gops/s** (bc_index; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 39.728 | 88.4% |
| bc_iter | 39.461 | 87.8% |
| bc_unchecked | 27.380 | 60.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2384ns | 2384ns | base |
| bc_iter | 2397ns | 2397ns | +0.54% |
| bc_unchecked | 2371ns | 2371ns | -0.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 103ns | base | --- | [101, 105] | --- | --- | --- | --- |
| bc_iter | 104ns | no significant difference | [-1, +0]ns | [102, 106] | no | 0.3382 | 0.3382 | 3 |
| bc_unchecked | 150ns | +49.8ns (+48.3%) | [+47, +52]ns | [147, 152] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 108ns | -13.5% | +32.6% |
| 1 | 2 | 101ns | +7.2% | +55.9% |
| 1 | 3 | 107ns | -2.8% | +47.9% |
| 1 | 4 | 110ns | -2.9% | +32.7% |
| 1 | 5 | 99ns | +6.6% | +44.0% |
| 1 | 6 | 95ns | -1.9% | +64.8% |
| 1 | 7 | 93ns | +15.1% | +64.2% |
| 1 | 8 | 105ns | -2.8% | +54.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | 0.234 | moderate+ |
| bc_iter | -0.395 | moderate- |
| bc_unchecked | -0.093 | ok |

**Consistency summary:**

- **bc_iter**: won 5/8, lost 3/8
- **bc_unchecked**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 2.7ns | 103.3ns | 2.6% |  |
| bc_iter | 2.5ns | 103.1ns | 2.4% |  |
| bc_unchecked | 2.7ns | 151.9ns | 1.8% |  |

## Distribution (algo ns)

```
bc_index (n=160, range 91.1-116.3 ns)
     91.1 |####################
     92.4 |######################
     93.6 |####################
     94.9 |##
     96.1 |##############
     97.4 |#################
     98.7 |########################################
     99.9 |######################
    101.2 |######################
    102.4 |#################
    103.7 |######################
    105.0 |##################################
    106.2 |####################
    107.5 |#########################
    108.7 |###########
    110.0 |########################################
    111.3 |#################
    112.5 |########
    113.8 |########
    115.0 |###########
  (13 below, 7 above range)

bc_iter (n=160, range 91.5-113.4 ns)
     91.5 |################
     92.6 |################
     93.7 |##################
     94.8 |################
     95.9 |##########
     97.0 |#############
     98.1 |########
     99.2 |##########
    100.3 |########################
    101.4 |##########################
    102.5 |#####################
    103.6 |##################
    104.7 |################
    105.8 |########################################
    106.9 |#############
    108.0 |##########################
    109.0 |########################
    110.1 |##################
    111.2 |################
    112.3 |########
  (11 below, 13 above range)

bc_unchecked (n=160, range 140.0-167.0 ns)
    140.0 |###########################
    141.3 |#####################
    142.7 |#################################
    144.0 |########################################
    145.4 |#####################
    146.7 |###########################
    148.1 |#####################
    149.4 |#####################
    150.8 |##############################
    152.1 |###
    153.5 |#########
    154.8 |##################
    156.2 |############
    157.5 |#########
    158.9 |###############
    160.2 |########################
    161.6 |#####################
    162.9 |######
    164.3 |###############
    165.6 |############
  (16 below, 16 above range)

```
