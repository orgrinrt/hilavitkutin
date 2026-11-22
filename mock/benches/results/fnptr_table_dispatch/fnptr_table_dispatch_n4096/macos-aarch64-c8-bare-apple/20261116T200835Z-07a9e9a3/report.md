# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 58% faster than the next best (fpt_table_16)

fpt_match_16 (787 ns) leads fpt_table_16 (1.24 us) by 58%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fpt_match_16 shows alternating (throttle bounce) (autocorr -0.59)

fpt_match_16's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (787 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 786.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.58x (fastest 786.7 ns, slowest 1242.9 ns)

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
| fpt_match_16 | 2865ns | 2859ns | 2848ns | 2859ns | 2898ns | base |
| fpt_table_16 | 3340ns | 3327ns | 3314ns | 3328ns | 3405ns | +16.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 789ns | 779ns | 803ns | base | 5.195 |
| fpt_table_16 | 1248ns | 1234ns | 1274ns | +58.25% | 3.283 |

## Performance model

- Peak throughput: **5.259 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 5.207 | 99.0% |
| fpt_table_16 | 3.296 | 62.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2865ns | 2865ns | base |
| fpt_table_16 | 3340ns | 3340ns | +16.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 787ns | base | --- | [785, 788] | --- | --- | --- | --- |
| fpt_table_16 | 1243ns | +456.2ns (+58.0%) | [+455, +457]ns | [1240, 1245] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 788ns | +59.5% |
| 1 | 2 | 790ns | +57.4% |
| 1 | 3 | 781ns | +58.3% |
| 1 | 4 | 791ns | +57.7% |
| 1 | 5 | 783ns | +57.7% |
| 1 | 6 | 785ns | +57.8% |
| 1 | 7 | 798ns | +56.4% |
| 1 | 8 | 780ns | +58.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.587 | HIGH- (thermal bounce) |
| fpt_table_16 | -0.190 | ok |

**Consistency summary:**

- **fpt_table_16**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 3.1ns | 788.5ns | 0.4% |  |
| fpt_table_16 | 3.3ns | 1247.8ns | 0.3% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 778.9-802.8 ns)
    778.9 |########################################
    780.1 |#################
    781.3 |#########################
    782.5 |######################
    783.7 |############################
    784.9 |#####################################
    786.1 |###############################
    787.3 |#########################
    788.5 |#########################
    789.7 |###############################
    790.9 |###########
    792.0 |############################
    793.2 |########
    794.4 |##
    795.6 |#################
    796.8 |###########
    798.0 |###########
    799.2 |###########
    800.4 |#####
    801.6 |#####
  (14 below, 6 above range)

fpt_table_16 (n=160, range 1234.3-1274.5 ns)
   1234.3 |########################################
   1236.3 |#####################################
   1238.3 |###########################
   1240.3 |#######################
   1242.3 |#########################
   1244.3 |#############################
   1246.3 |############
   1248.4 |########
   1250.4 |########
   1252.4 |##############
   1254.4 |############
   1256.4 |##################
   1258.4 |####
   1260.4 |
   1262.4 |######
   1264.4 |##
   1266.4 |
   1268.4 |
   1270.5 |
   1272.5 |
  (16 below, 15 above range)

```
