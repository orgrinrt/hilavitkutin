# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 55% faster than the next best (fpt_table_16)

fpt_match_16 (841 ns) leads fpt_table_16 (1.30 us) by 55%, a clear separation rather than a photo finish. CV 5.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (841 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 840.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.55x (fastest 840.6 ns, slowest 1299.8 ns)

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
| fpt_match_16 | 3024ns | 3055ns | 2842ns | 3005ns | 3262ns | base |
| fpt_table_16 | 3524ns | 3461ns | 3316ns | 3487ns | 3839ns | +16.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 835ns | 783ns | 904ns | base | 4.904 |
| fpt_table_16 | 1318ns | 1239ns | 1435ns | +57.76% | 3.108 |

## Performance model

- Peak throughput: **5.232 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 4.873 | 93.1% |
| fpt_table_16 | 3.151 | 60.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 3024ns | 3024ns | base |
| fpt_table_16 | 3524ns | 3524ns | +16.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 841ns | base | --- | [838, 843] | --- | --- | --- | --- |
| fpt_table_16 | 1300ns | +468.3ns (+55.7%) | [+455, +485]ns | [1288, 1326] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 791ns | +57.0% |
| 1 | 2 | 846ns | +52.4% |
| 1 | 3 | 835ns | +54.1% |
| 1 | 4 | 848ns | +53.4% |
| 1 | 5 | 784ns | +70.4% |
| 1 | 6 | 843ns | +69.3% |
| 1 | 7 | 855ns | +58.5% |
| 1 | 8 | 840ns | +47.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.246 | moderate- |
| fpt_table_16 | 0.266 | moderate+ |

**Consistency summary:**

- **fpt_table_16**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 3.2ns | 835.3ns | 0.4% |  |
| fpt_table_16 | 3.6ns | 1317.8ns | 0.3% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 782.9-903.7 ns)
    782.9 |########################
    789.0 |######################
    795.0 |#####
    801.0 |####
    807.1 |
    813.1 |##
    819.2 |#
    825.2 |
    831.2 |######
    837.3 |########################################
    843.3 |#############################
    849.3 |################
    855.4 |####
    861.4 |
    867.5 |
    873.5 |#
    879.5 |#
    885.6 |
    891.6 |####
    897.6 |##
  (15 below, 21 above range)

fpt_table_16 (n=160, range 1239.0-1434.6 ns)
   1239.0 |########################################
   1248.8 |########
   1258.6 |##
   1268.3 |#####
   1278.1 |#############
   1287.9 |################
   1297.7 |####
   1307.5 |####
   1317.2 |#############
   1327.0 |##############################
   1336.8 |####
   1346.6 |
   1356.4 |##
   1366.1 |##
   1375.9 |####
   1385.7 |
   1395.5 |
   1405.3 |
   1415.0 |##
   1424.8 |##########################
  (15 below, 13 above range)

```
