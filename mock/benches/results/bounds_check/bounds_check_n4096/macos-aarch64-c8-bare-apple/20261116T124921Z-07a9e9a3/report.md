# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (bc_index, bc_iter) are a dead heat (<1%)

bc_index (92 ns) and bc_iter (92 ns) differ by 0.43%, inside the noise, even though the wider field spreads 54.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (bc_index)

The baseline bc_index is the fastest (92 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### bc_iter's edge over baseline is significant but tiny (1 ns, 0.87%)

bc_iter differs from baseline bc_index by 1 ns (0.87%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (bc_index) is the fastest** at 92.1 ns median
- 2 variants significantly slower than baseline
- Spread: 1.54x (fastest 92.1 ns, slowest 142.1 ns)

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
| bc_index | 2170ns | 2164ns | 2159ns | 2165ns | 2198ns | base |
| bc_iter | 2175ns | 2168ns | 2162ns | 2168ns | 2210ns | +0.24% |
| bc_unchecked | 2233ns | 2224ns | 2217ns | 2224ns | 2276ns | +2.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 92ns | 87ns | 99ns | base | 44.395 |
| bc_iter | 94ns | 87ns | 102ns | +1.42% | 43.774 |
| bc_unchecked | 143ns | 137ns | 153ns | +55.29% | 28.589 |

## Performance model

- Peak throughput: **47.262 Gops/s** (bc_index; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 44.473 | 94.1% |
| bc_iter | 44.281 | 93.7% |
| bc_unchecked | 28.825 | 61.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2170ns | 2170ns | base |
| bc_iter | 2175ns | 2175ns | +0.24% |
| bc_unchecked | 2233ns | 2233ns | +2.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 92ns | base | --- | [91, 93] | --- | --- | --- | --- |
| bc_iter | 92ns | +0.8ns (+0.9%) | [+0, +1]ns | [92, 93] | YES | 0.0011 | 0.0011 | 9 |
| bc_unchecked | 142ns | +49.6ns (+53.9%) | [+49, +50]ns | [141, 143] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 92ns | +1.8% | +55.3% |
| 1 | 2 | 92ns | +6.8% | +54.7% |
| 1 | 3 | 92ns | +0.2% | +53.5% |
| 1 | 4 | 91ns | +0.8% | +55.3% |
| 1 | 5 | 90ns | +1.2% | +57.2% |
| 1 | 6 | 92ns | +0.0% | +51.8% |
| 1 | 7 | 90ns | +1.1% | +57.8% |
| 1 | 8 | 93ns | +0.2% | +52.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.461 | moderate- |
| bc_iter | 0.173 | ok |
| bc_unchecked | 0.051 | ok |

**Consistency summary:**

- **bc_iter**: won 0/8, lost 7/8
- **bc_unchecked**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 2.5ns | 92.3ns | 2.7% |  |
| bc_iter | 2.2ns | 93.6ns | 2.3% |  |
| bc_unchecked | 2.4ns | 143.3ns | 1.6% |  |

## Distribution (algo ns)

```
bc_index (n=160, range 86.7-99.0 ns)
     86.7 |###############
     87.3 |##
     87.9 |######################
     88.5 |#############
     89.1 |###############################
     89.7 |#################
     90.4 |######################
     91.0 |########
     91.6 |####################
     92.2 |#############
     92.8 |########################################
     93.4 |########
     94.0 |########################
     94.7 |########
     95.3 |#############
     95.9 |##
     96.5 |#############
     97.1 |##
     97.7 |######
     98.4 |####
  (13 below, 16 above range)

bc_iter (n=160, range 87.3-102.3 ns)
     87.3 |########################
     88.1 |#####################
     88.8 |##########
     89.6 |########################
     90.3 |#####################################
     91.1 |########################################
     91.8 |##########################
     92.6 |##################################
     93.3 |##################
     94.1 |##########################
     94.8 |##################
     95.6 |##################
     96.3 |##
     97.1 |#############
     97.8 |##########
     98.6 |##########
     99.3 |##########
    100.0 |##
    100.8 |##
    101.5 |##
  (12 below, 14 above range)

bc_unchecked (n=160, range 137.2-153.2 ns)
    137.2 |################################
    138.0 |######################
    138.8 |#########################
    139.6 |#####################################
    140.4 |##############################
    141.2 |######################
    142.0 |################################
    142.8 |########################################
    143.6 |#########################
    144.4 |################################
    145.2 |############
    146.0 |#####
    146.8 |###############
    147.6 |#######
    148.4 |###############
    149.2 |#####
    150.0 |##
    150.8 |
    151.6 |##
    152.4 |
  (11 below, 3 above range)

```
