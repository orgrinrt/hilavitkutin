# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 11% faster than the next best (fpt_table_16)

fpt_match_16 (10.26 us) leads fpt_table_16 (11.37 us) by 11%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (10.26 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 10260.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.11x (fastest 10260.5 ns, slowest 11369.4 ns)

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
| fpt_match_16 | 12346ns | 12337ns | 11727ns | 12348ns | 12961ns | base |
| fpt_table_16 | 13471ns | 13486ns | 12613ns | 13413ns | 14501ns | +9.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 10242ns | 9632ns | 10826ns | base | 1.600 |
| fpt_table_16 | 11369ns | 10530ns | 12365ns | +11.00% | 1.441 |

## Performance model

- Peak throughput: **1.701 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 1.597 | 93.9% |
| fpt_table_16 | 1.441 | 84.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 12346ns | 12346ns | base |
| fpt_table_16 | 13471ns | 13471ns | +9.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 10260ns | base | --- | [10187, 10366] | --- | --- | --- | --- |
| fpt_table_16 | 11369ns | +1065.7ns (+10.4%) | [+987, +1197]ns | [11199, 11476] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 10300ns | +11.0% |
| 1 | 2 | 10667ns | +0.8% |
| 1 | 3 | 10474ns | -0.2% |
| 1 | 4 | 10280ns | +10.8% |
| 1 | 5 | 9827ns | +19.0% |
| 1 | 6 | 9988ns | +25.9% |
| 1 | 7 | 10583ns | +2.0% |
| 1 | 8 | 9539ns | +23.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.087 | ok |
| fpt_table_16 | -0.004 | ok |

**Consistency summary:**

- **fpt_table_16**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 3.3ns | 10242.3ns | 0.0% |  |
| fpt_table_16 | 3.4ns | 11368.8ns | 0.0% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 9631.7-10825.6 ns)
   9631.7 |
   9691.4 |###
   9751.1 |##############################
   9810.8 |##############################
   9870.5 |############
   9930.2 |#########
   9989.9 |########################################
  10049.6 |###############
  10109.3 |#########
  10169.0 |###########################
  10228.7 |####################################
  10288.4 |############
  10348.1 |#####################
  10407.8 |###########################
  10467.5 |####################################
  10527.2 |########################
  10586.9 |###########################
  10646.6 |##############################
  10706.3 |#########
  10766.0 |
  (16 below, 12 above range)

fpt_table_16 (n=160, range 10530.5-12365.4 ns)
  10530.5 |##########################
  10622.2 |##########################
  10714.0 |#################################
  10805.7 |########################################
  10897.5 |##########
  10989.2 |#################################
  11081.0 |##########
  11172.7 |##########
  11264.4 |##############################
  11356.2 |########################################
  11447.9 |################
  11539.7 |##########################
  11631.4 |####################################
  11723.2 |#################################
  11814.9 |#############
  11906.7 |#############
  11998.4 |######
  12090.2 |###
  12181.9 |##########
  12273.6 |#############
  (13 below, 17 above range)

```
