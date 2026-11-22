# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 58% faster than the next best (fpt_table_16)

fpt_match_16 (805 ns) leads fpt_table_16 (1.27 us) by 58%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fpt_table_16 shows alternating (throttle bounce) (autocorr -0.57)

fpt_table_16's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (805 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 805.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.58x (fastest 805.0 ns, slowest 1268.8 ns)

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
| fpt_match_16 | 2924ns | 2906ns | 2894ns | 2911ns | 2992ns | base |
| fpt_table_16 | 3399ns | 3383ns | 3367ns | 3388ns | 3468ns | +16.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 809ns | 796ns | 837ns | base | 5.060 |
| fpt_table_16 | 1274ns | 1259ns | 1305ns | +57.42% | 3.214 |

## Performance model

- Peak throughput: **5.147 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 5.088 | 98.9% |
| fpt_table_16 | 3.228 | 62.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2924ns | 2924ns | base |
| fpt_table_16 | 3399ns | 3399ns | +16.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 805ns | base | --- | [803, 806] | --- | --- | --- | --- |
| fpt_table_16 | 1269ns | +463.5ns (+57.6%) | [+462, +465]ns | [1266, 1270] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 804ns | +57.2% |
| 1 | 2 | 807ns | +57.2% |
| 1 | 3 | 798ns | +59.2% |
| 1 | 4 | 806ns | +57.8% |
| 1 | 5 | 796ns | +58.1% |
| 1 | 6 | 800ns | +59.7% |
| 1 | 7 | 811ns | +55.9% |
| 1 | 8 | 809ns | +56.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.062 | ok |
| fpt_table_16 | -0.567 | HIGH- (thermal bounce) |

**Consistency summary:**

- **fpt_table_16**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 3.2ns | 809.5ns | 0.4% |  |
| fpt_table_16 | 3.4ns | 1274.3ns | 0.3% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 795.8-836.7 ns)
    795.8 |######################
    797.9 |############################
    799.9 |###################################
    802.0 |########################################
    804.0 |########################################
    806.0 |###################################
    808.1 |##########################
    810.1 |###############################
    812.2 |######
    814.2 |########
    816.3 |########
    818.3 |####
    820.4 |##
    822.4 |####
    824.4 |##
    826.5 |##
    828.5 |
    830.6 |
    832.6 |##
    834.7 |##
  (15 below, 8 above range)

fpt_table_16 (n=160, range 1259.0-1304.7 ns)
   1259.0 |################
   1261.3 |########################################
   1263.6 |##############################
   1265.9 |######
   1268.1 |###################################
   1270.4 |#####################
   1272.7 |#############
   1275.0 |########
   1277.3 |########
   1279.6 |#
   1281.8 |########
   1284.1 |#
   1286.4 |##########
   1288.7 |######
   1291.0 |###
   1293.3 |###
   1295.5 |#
   1297.8 |#####
   1300.1 |#
   1302.4 |###
  (16 below, 8 above range)

```
