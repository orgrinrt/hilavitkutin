# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 58% faster than the next best (fpt_table_16)

fpt_match_16 (787 ns) leads fpt_table_16 (1.24 us) by 58%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fpt_match_16 shows alternating (throttle bounce) (autocorr -0.54)

fpt_match_16's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (787 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 787.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.58x (fastest 787.1 ns, slowest 1240.0 ns)

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
| fpt_match_16 | 2863ns | 2859ns | 2850ns | 2860ns | 2886ns | base |
| fpt_table_16 | 3331ns | 3322ns | 3312ns | 3324ns | 3370ns | +16.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 788ns | 779ns | 799ns | base | 5.199 |
| fpt_table_16 | 1245ns | 1234ns | 1266ns | +57.99% | 3.291 |

## Performance model

- Peak throughput: **5.257 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 5.204 | 99.0% |
| fpt_table_16 | 3.303 | 62.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2863ns | 2863ns | base |
| fpt_table_16 | 3331ns | 3331ns | +16.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 787ns | base | --- | [786, 789] | --- | --- | --- | --- |
| fpt_table_16 | 1240ns | +453.8ns (+57.7%) | [+452, +455]ns | [1239, 1241] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 788ns | +57.4% |
| 1 | 2 | 790ns | +57.3% |
| 1 | 3 | 781ns | +60.9% |
| 1 | 4 | 791ns | +57.9% |
| 1 | 5 | 783ns | +57.8% |
| 1 | 6 | 786ns | +57.5% |
| 1 | 7 | 799ns | +54.7% |
| 1 | 8 | 781ns | +58.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.544 | HIGH- (thermal bounce) |
| fpt_table_16 | 0.371 | moderate+ |

**Consistency summary:**

- **fpt_table_16**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 2.9ns | 787.8ns | 0.4% |  |
| fpt_table_16 | 3.2ns | 1244.7ns | 0.3% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 779.2-799.1 ns)
    779.2 |################
    780.2 |####################################
    781.2 |####################
    782.2 |################
    783.2 |########################################
    784.2 |#################################
    785.2 |#######################
    786.2 |#################################
    787.2 |####################
    788.2 |#################################
    789.1 |#################################
    790.1 |##########################
    791.1 |#################################
    792.1 |#############
    793.1 |###
    794.1 |################
    795.1 |
    796.1 |#############
    797.1 |######
    798.1 |####################
  (16 below, 12 above range)

fpt_table_16 (n=160, range 1234.1-1266.0 ns)
   1234.1 |####################
   1235.7 |####################################
   1237.3 |########################################
   1238.9 |###########################
   1240.5 |#################
   1242.1 |############
   1243.7 |############
   1245.3 |############
   1246.9 |###
   1248.5 |#####
   1250.1 |###
   1251.7 |###
   1253.3 |######
   1254.8 |#############
   1256.4 |############
   1258.0 |#####
   1259.6 |###
   1261.2 |
   1262.8 |
   1264.4 |#
  (14 below, 9 above range)

```
