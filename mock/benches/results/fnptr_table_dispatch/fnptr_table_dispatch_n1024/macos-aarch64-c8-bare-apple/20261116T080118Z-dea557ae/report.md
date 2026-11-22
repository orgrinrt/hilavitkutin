# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 52% faster than the next best (fpt_table_16)

fpt_match_16 (213 ns) leads fpt_table_16 (325 ns) by 52%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (213 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 212.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.52x (fastest 212.9 ns, slowest 324.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fn-pointer table vs match dispatch for 16-variant hot loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x478ab261e0d9f352 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 2334ns | 2318ns | 2304ns | 2321ns | 2404ns | base |
| fpt_table_16 | 2437ns | 2426ns | 2414ns | 2428ns | 2488ns | +4.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 214ns | 207ns | 223ns | base | 4.788 |
| fpt_table_16 | 325ns | 318ns | 334ns | +52.01% | 3.150 |

## Performance model

- Peak throughput: **4.939 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 4.810 | 97.4% |
| fpt_table_16 | 3.155 | 63.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2334ns | 2334ns | base |
| fpt_table_16 | 2437ns | 2437ns | +4.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 213ns | base | --- | [212, 214] | --- | --- | --- | --- |
| fpt_table_16 | 325ns | +110.8ns (+52.0%) | [+110, +112]ns | [324, 325] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 211ns | +52.7% |
| 1 | 2 | 215ns | +52.0% |
| 1 | 3 | 209ns | +51.6% |
| 1 | 4 | 215ns | +53.1% |
| 1 | 5 | 208ns | +55.5% |
| 1 | 6 | 211ns | +52.4% |
| 1 | 7 | 218ns | +50.1% |
| 1 | 8 | 215ns | +53.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.228 | moderate- |
| fpt_table_16 | -0.376 | moderate- |

**Consistency summary:**

- **fpt_table_16**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 3.1ns | 213.9ns | 1.5% |  |
| fpt_table_16 | 3.6ns | 325.1ns | 1.1% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 207.3-222.8 ns)
    207.3 |###############
    208.1 |#########################
    208.9 |###########################
    209.7 |####################
    210.4 |########################################
    211.2 |#######
    212.0 |##############################
    212.8 |#################
    213.5 |#################
    214.3 |#####################################
    215.1 |####################
    215.9 |###############
    216.6 |##########
    217.4 |####################
    218.2 |############
    219.0 |#######
    219.7 |###############
    220.5 |##
    221.3 |##
    222.1 |#######
  (13 below, 7 above range)

fpt_table_16 (n=160, range 317.8-334.3 ns)
    317.8 |##############
    318.6 |#################
    319.4 |#########################
    320.2 |#########################
    321.1 |############################
    321.9 |########
    322.7 |######################
    323.5 |########################################
    324.4 |##################################
    325.2 |######################
    326.0 |##################################
    326.8 |#################
    327.7 |#########################
    328.5 |####################
    329.3 |#################
    330.1 |##############
    331.0 |########
    331.8 |###########
    332.6 |########
    333.4 |#####
  (14 below, 5 above range)

```
