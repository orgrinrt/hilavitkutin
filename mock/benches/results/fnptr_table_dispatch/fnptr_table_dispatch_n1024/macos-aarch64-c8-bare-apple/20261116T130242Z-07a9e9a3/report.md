# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 52% faster than the next best (fpt_table_16)

fpt_match_16 (208 ns) leads fpt_table_16 (317 ns) by 52%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (208 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 208.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.52x (fastest 208.3 ns, slowest 317.1 ns)

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
| fpt_match_16 | 2306ns | 2286ns | 2274ns | 2297ns | 2366ns | base |
| fpt_table_16 | 2398ns | 2392ns | 2381ns | 2392ns | 2431ns | +3.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 209ns | 202ns | 220ns | base | 4.892 |
| fpt_table_16 | 317ns | 310ns | 324ns | +51.42% | 3.231 |

## Performance model

- Peak throughput: **5.072 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 4.916 | 96.9% |
| fpt_table_16 | 3.229 | 63.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2306ns | 2306ns | base |
| fpt_table_16 | 2398ns | 2398ns | +3.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 208ns | base | --- | [208, 209] | --- | --- | --- | --- |
| fpt_table_16 | 317ns | +108.4ns (+52.0%) | [+108, +110]ns | [316, 318] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 204ns | +55.0% |
| 1 | 2 | 208ns | +53.1% |
| 1 | 3 | 205ns | +51.2% |
| 1 | 4 | 211ns | +51.6% |
| 1 | 5 | 203ns | +55.0% |
| 1 | 6 | 209ns | +50.0% |
| 1 | 7 | 210ns | +53.1% |
| 1 | 8 | 217ns | +47.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.102 | ok |
| fpt_table_16 | -0.384 | moderate- |

**Consistency summary:**

- **fpt_table_16**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 3.3ns | 209.3ns | 1.6% |  |
| fpt_table_16 | 3.6ns | 316.9ns | 1.1% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 201.9-220.3 ns)
    201.9 |##################
    202.8 |#######################
    203.7 |#######################
    204.6 |#####################
    205.6 |################
    206.5 |########
    207.4 |########################################
    208.3 |#############################
    209.2 |##############
    210.2 |#####################
    211.1 |################
    212.0 |############
    212.9 |######
    213.8 |########
    214.8 |##############
    215.7 |######
    216.6 |##
    217.5 |######
    218.4 |######
    219.3 |####
  (12 below, 5 above range)

fpt_table_16 (n=160, range 309.7-324.3 ns)
    309.7 |##########
    310.5 |######
    311.2 |##############
    311.9 |##
    312.7 |########
    313.4 |########
    314.1 |#####################
    314.8 |#############################
    315.6 |##############
    316.3 |##############
    317.0 |########################################
    317.8 |############
    318.5 |#############################
    319.2 |####
    320.0 |#####################
    320.7 |########
    321.4 |################
    322.1 |####
    322.9 |##############
    323.6 |####
  (15 below, 9 above range)

```
