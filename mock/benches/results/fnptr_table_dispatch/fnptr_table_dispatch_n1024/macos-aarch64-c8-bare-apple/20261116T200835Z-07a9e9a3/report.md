# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 52% faster than the next best (fpt_table_16)

fpt_match_16 (208 ns) leads fpt_table_16 (317 ns) by 52%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (208 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 208.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.52x (fastest 208.3 ns, slowest 316.7 ns)

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
| fpt_match_16 | 2306ns | 2286ns | 2274ns | 2290ns | 2388ns | base |
| fpt_table_16 | 2396ns | 2388ns | 2380ns | 2388ns | 2436ns | +3.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 209ns | 203ns | 219ns | base | 4.888 |
| fpt_table_16 | 317ns | 310ns | 325ns | +51.22% | 3.232 |

## Performance model

- Peak throughput: **5.048 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 4.916 | 97.4% |
| fpt_table_16 | 3.233 | 64.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2306ns | 2306ns | base |
| fpt_table_16 | 2396ns | 2396ns | +3.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 208ns | base | --- | [208, 209] | --- | --- | --- | --- |
| fpt_table_16 | 317ns | +108.1ns (+51.9%) | [+107, +109]ns | [316, 318] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 204ns | +54.5% |
| 1 | 2 | 208ns | +53.5% |
| 1 | 3 | 208ns | +48.9% |
| 1 | 4 | 210ns | +53.1% |
| 1 | 5 | 204ns | +54.1% |
| 1 | 6 | 208ns | +50.6% |
| 1 | 7 | 211ns | +50.8% |
| 1 | 8 | 221ns | +44.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | 0.145 | ok |
| fpt_table_16 | -0.499 | moderate- |

**Consistency summary:**

- **fpt_table_16**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 3.0ns | 209.5ns | 1.4% |  |
| fpt_table_16 | 3.4ns | 316.8ns | 1.1% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 202.9-219.3 ns)
    202.9 |#########################
    203.7 |######################
    204.5 |####################
    205.3 |############################
    206.1 |########################################
    207.0 |##################################
    207.8 |###############################
    208.6 |########################################
    209.4 |###############################
    210.3 |###########
    211.1 |#################
    211.9 |##############
    212.7 |######################
    213.5 |##############
    214.4 |
    215.2 |#####
    216.0 |#####
    216.8 |#####
    217.6 |########
    218.5 |
  (12 below, 15 above range)

fpt_table_16 (n=160, range 309.8-324.6 ns)
    309.8 |##########################
    310.5 |#############
    311.2 |##########
    312.0 |#################################
    312.7 |################
    313.5 |#######################
    314.2 |#############
    315.0 |####################################
    315.7 |########################################
    316.4 |##############################
    317.2 |####################################
    317.9 |#############
    318.7 |#################################
    319.4 |####################################
    320.1 |#######################
    320.9 |################
    321.6 |##########################
    322.4 |######
    323.1 |##########
    323.8 |######
  (15 below, 9 above range)

```
