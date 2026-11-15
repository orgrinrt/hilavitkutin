# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_per_iter is an outlier: 2.8x slower than the field

shr_per_iter (193 ns) is 2.8x the fastest (70 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (shr_const, shr_loop_invariant) are a dead heat (<1%)

shr_const (70 ns) and shr_loop_invariant (70 ns) differ by 0.57%, inside the noise, even though the wider field spreads 177.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (70 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (shr_const) is the fastest** at 69.6 ns median
- 1 variant significantly slower than baseline
- Spread: 2.78x (fastest 69.6 ns, slowest 193.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Shift amount: const vs loop-invariant runtime vs per-iter independent |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6a92bf08e314750d |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| shr_const | 2278ns | 2278ns | 2156ns | 2261ns | 2453ns | base |
| shr_loop_invariant | 2217ns | 2176ns | 2130ns | 2201ns | 2350ns | -2.70% |
| shr_per_iter | 2407ns | 2412ns | 2248ns | 2397ns | 2593ns | +5.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 69ns | 62ns | 77ns | base | 58.939 |
| shr_loop_invariant | 70ns | 62ns | 78ns | +0.18% | 58.832 |
| shr_per_iter | 195ns | 180ns | 216ns | +180.90% | 20.982 |

## Performance model

- Peak throughput: **66.541 Gops/s** (shr_loop_invariant; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 58.851 | 88.4% |
| shr_loop_invariant | 58.514 | 87.9% |
| shr_per_iter | 21.190 | 31.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2278ns | 2278ns | base |
| shr_loop_invariant | 2217ns | 2217ns | -2.70% |
| shr_per_iter | 2407ns | 2407ns | +5.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 70ns | base | --- | [69, 71] | --- | --- | --- | --- |
| shr_loop_invariant | 70ns | no significant difference | [-1, +0]ns | [69, 71] | no | 0.1480 | 0.1480 | 5 |
| shr_per_iter | 193ns | +124.5ns (+179.0%) | [+124, +126]ns | [192, 196] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 67ns | -1.9% | +174.2% |
| 1 | 2 | 71ns | -7.9% | +172.9% |
| 1 | 3 | 71ns | +1.7% | +179.8% |
| 1 | 4 | 74ns | +2.2% | +147.2% |
| 1 | 5 | 72ns | -3.2% | +161.9% |
| 1 | 6 | 68ns | +11.0% | +193.7% |
| 1 | 7 | 68ns | -1.0% | +210.8% |
| 1 | 8 | 65ns | +5.1% | +197.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | 0.325 | moderate+ |
| shr_loop_invariant | 0.062 | ok |
| shr_per_iter | 0.149 | ok |

**Consistency summary:**

- **shr_loop_invariant**: won 4/8, lost 4/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 2.5ns | 69.5ns | 3.6% |  |
| shr_loop_invariant | 2.6ns | 69.6ns | 3.7% |  |
| shr_per_iter | 2.9ns | 195.2ns | 1.5% |  |

## Distribution (algo ns)

```
shr_const (n=160, range 62.1-76.9 ns)
     62.1 |############
     62.9 |###############
     63.6 |#####################
     64.3 |##################
     65.1 |#################################
     65.8 |######
     66.5 |###########################
     67.3 |##################
     68.0 |#####################
     68.8 |############
     69.5 |########################################
     70.2 |###########################
     71.0 |#################################
     71.7 |###############
     72.4 |###########################
     73.2 |###########################
     73.9 |########################
     74.7 |#########
     75.4 |############
     76.1 |############
  (13 below, 11 above range)

shr_loop_invariant (n=160, range 61.6-77.8 ns)
     61.6 |##################
     62.4 |###############
     63.2 |#########
     64.0 |######
     64.8 |########################################
     65.6 |###########################
     66.4 |##################
     67.2 |#####################
     68.1 |############
     68.9 |####################################
     69.7 |####################################
     70.5 |###########################
     71.3 |#################################
     72.1 |###########################
     72.9 |###############
     73.8 |############
     74.6 |###############
     75.4 |###
     76.2 |#########
     77.0 |##################
  (12 below, 16 above range)

shr_per_iter (n=160, range 180.5-215.5 ns)
    180.5 |##################
    182.3 |########################
    184.0 |##########################
    185.8 |########################
    187.5 |################
    189.3 |########################
    191.0 |#############################
    192.8 |##################################
    194.5 |########################
    196.3 |########################################
    198.0 |#####################
    199.8 |################
    201.5 |##########
    203.3 |##
    205.0 |##########
    206.8 |##########
    208.5 |#####
    210.3 |#############
    212.0 |########
    213.8 |########
  (13 below, 9 above range)

```
