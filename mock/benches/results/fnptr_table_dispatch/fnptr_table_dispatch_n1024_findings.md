# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 39% faster than the next best (fpt_table_16)

fpt_match_16 (236 ns) leads fpt_table_16 (328 ns) by 39%, a clear separation rather than a photo finish. CV 5.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (236 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 235.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.39x (fastest 235.8 ns, slowest 328.5 ns)

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
| fpt_match_16 | 2536ns | 2618ns | 2345ns | 2564ns | 2643ns | base |
| fpt_table_16 | 2510ns | 2477ns | 2378ns | 2481ns | 2726ns | -1.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 232ns | 212ns | 247ns | base | 4.412 |
| fpt_table_16 | 334ns | 312ns | 366ns | +43.76% | 3.069 |

## Performance model

- Peak throughput: **4.822 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 4.343 | 90.1% |
| fpt_table_16 | 3.117 | 64.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2536ns | 2536ns | base |
| fpt_table_16 | 2510ns | 2510ns | -1.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 236ns | base | --- | [233, 238] | --- | --- | --- | --- |
| fpt_table_16 | 328ns | +98.4ns (+41.7%) | [+92, +103]ns | [326, 332] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 214ns | +53.3% |
| 1 | 2 | 238ns | +38.2% |
| 1 | 3 | 236ns | +31.3% |
| 1 | 4 | 241ns | +43.0% |
| 1 | 5 | 222ns | +62.0% |
| 1 | 6 | 238ns | +39.7% |
| 1 | 7 | 243ns | +31.3% |
| 1 | 8 | 227ns | +42.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.266 | moderate- |
| fpt_table_16 | 0.162 | ok |

**Consistency summary:**

- **fpt_table_16**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 3.5ns | 232.1ns | 1.5% |  |
| fpt_table_16 | 3.8ns | 333.7ns | 1.1% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 212.4-247.2 ns)
    212.4 |##
    214.1 |#########
    215.9 |##
    217.6 |##############
    219.3 |################
    221.1 |##################
    222.8 |#########
    224.6 |############################
    226.3 |#########
    228.1 |
    229.8 |#######
    231.5 |###########
    233.3 |##################
    235.0 |##################
    236.8 |############################
    238.5 |##############################
    240.2 |########################################
    242.0 |##############################
    243.7 |#######################
    245.5 |#########
  (14 below, 6 above range)

fpt_table_16 (n=160, range 312.3-365.8 ns)
    312.3 |##############################
    315.0 |##########################
    317.7 |########################################
    320.4 |####################################
    323.0 |####################################
    325.7 |########################################
    328.4 |####################
    331.0 |#######################
    333.7 |#############
    336.4 |####################################
    339.1 |##############################
    341.7 |##############################
    344.4 |################
    347.1 |######
    349.7 |
    352.4 |
    355.1 |##########
    357.8 |##########
    360.4 |######
    363.1 |####################
  (17 below, 13 above range)

```
