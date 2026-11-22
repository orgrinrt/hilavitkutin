# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (10.97 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 10969.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.07x (fastest 10969.2 ns, slowest 11752.9 ns)

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
| fpt_match_16 | 13047ns | 13051ns | 12788ns | 13048ns | 13304ns | base |
| fpt_table_16 | 13941ns | 13840ns | 13680ns | 13891ns | 14350ns | +6.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 10964ns | 10707ns | 11218ns | base | 1.494 |
| fpt_table_16 | 11852ns | 11594ns | 12253ns | +8.10% | 1.382 |

## Performance model

- Peak throughput: **1.530 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 1.494 | 97.6% |
| fpt_table_16 | 1.394 | 91.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 13047ns | 13047ns | base |
| fpt_table_16 | 13941ns | 13941ns | +6.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 10969ns | base | --- | [10935, 11026] | --- | --- | --- | --- |
| fpt_table_16 | 11753ns | +856.7ns (+7.8%) | [+820, +887]ns | [11738, 11808] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 11072ns | +11.3% |
| 1 | 2 | 10677ns | +10.4% |
| 1 | 3 | 10890ns | +7.6% |
| 1 | 4 | 10790ns | +8.6% |
| 1 | 5 | 10767ns | +7.6% |
| 1 | 6 | 11132ns | +8.3% |
| 1 | 7 | 11168ns | +7.2% |
| 1 | 8 | 11046ns | +5.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | 0.195 | ok |
| fpt_table_16 | -0.052 | ok |

**Consistency summary:**

- **fpt_table_16**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 3.5ns | 10963.8ns | 0.0% |  |
| fpt_table_16 | 3.3ns | 11851.8ns | 0.0% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 10706.8-11218.3 ns)
  10706.8 |
  10732.4 |########
  10757.9 |#########################
  10783.5 |########################################
  10809.1 |############################
  10834.7 |##
  10860.2 |##############
  10885.8 |###########
  10911.4 |########
  10937.0 |####################
  10962.5 |###############################
  10988.1 |###########
  11013.7 |###########
  11039.3 |######################
  11064.8 |#################
  11090.4 |##################################
  11116.0 |###############################
  11141.6 |####################
  11167.1 |#########################
  11192.7 |########
  (19 below, 10 above range)

fpt_table_16 (n=160, range 11594.1-12252.8 ns)
  11594.1 |#####################
  11627.0 |########################################
  11660.0 |############################
  11692.9 |##############################
  11725.8 |##############################
  11758.8 |#######################
  11791.7 |#######
  11824.7 |##################
  11857.6 |#######
  11890.5 |##
  11923.5 |##
  11956.4 |##############
  11989.3 |#######
  12022.3 |#########################
  12055.2 |##############
  12088.1 |#######
  12121.1 |####
  12154.0 |####
  12187.0 |##
  12219.9 |
  (16 below, 20 above range)

```
