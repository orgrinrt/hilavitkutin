# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (bp_balanced)

The baseline bp_balanced is the fastest (1.27 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### bp_skew_high's edge over baseline is significant but tiny (24 ns, 1.87%)

bp_skew_high differs from baseline bp_balanced by 24 ns (1.87%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (bp_balanced) is the fastest** at 1265.6 ns median
- 2 variants significantly slower than baseline
- Spread: 1.06x (fastest 1265.6 ns, slowest 1342.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x1b41e7c35fa28d06 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bp_balanced | 3444ns | 3393ns | 3331ns | 3422ns | 3621ns | base |
| bp_skew_high | 3531ns | 3570ns | 3335ns | 3526ns | 3738ns | +2.52% |
| bp_skew_low | 3560ns | 3573ns | 3343ns | 3561ns | 3773ns | +3.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 1294ns | 1249ns | 1365ns | base | 0.791 |
| bp_skew_high | 1328ns | 1254ns | 1413ns | +2.56% | 0.771 |
| bp_skew_low | 1339ns | 1255ns | 1429ns | +3.43% | 0.765 |

## Performance model

- Peak throughput: **0.820 Gops/s** (bp_balanced; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.809 | 98.7% |
| bp_skew_high | 0.765 | 93.3% |
| bp_skew_low | 0.763 | 93.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 3444ns | 3444ns | base |
| bp_skew_high | 3531ns | 3531ns | +2.52% |
| bp_skew_low | 3560ns | 3560ns | +3.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 1266ns | base | --- | [1257, 1297] | --- | --- | --- | --- |
| bp_skew_high | 1338ns | +23.7ns (+1.9%) | [+3, +40]ns | [1315, 1342] | YES | 0.0000 | 0.0000 | 0 |
| bp_skew_low | 1342ns | +46.6ns (+3.7%) | [+25, +73]ns | [1339, 1344] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 1256ns | +3.0% | +6.7% |
| 1 | 2 | 1252ns | +7.4% | +7.5% |
| 1 | 3 | 1301ns | +1.5% | +3.4% |
| 1 | 4 | 1345ns | +5.3% | -4.2% |
| 1 | 5 | 1306ns | +1.5% | +0.8% |
| 1 | 6 | 1256ns | +7.0% | +7.1% |
| 1 | 7 | 1254ns | +0.5% | +7.3% |
| 1 | 8 | 1258ns | +0.0% | +3.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | 0.449 | moderate+ |
| bp_skew_high | 0.115 | ok |
| bp_skew_low | -0.000 | ok |

**Consistency summary:**

- **bp_skew_high**: won 0/8, lost 7/8
- **bp_skew_low**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 2.8ns | 1294.4ns | 0.2% |  |
| bp_skew_high | 3.0ns | 1327.5ns | 0.2% |  |
| bp_skew_low | 2.9ns | 1338.8ns | 0.2% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 1249.1-1365.3 ns)
   1249.1 |########################################
   1254.9 |##########################
   1260.7 |#####
   1266.5 |#
   1272.3 |##
   1278.1 |#
   1283.9 |##
   1289.7 |####
   1295.6 |######
   1301.4 |####
   1307.2 |##
   1313.0 |##
   1318.8 |###
   1324.6 |
   1330.4 |#
   1336.2 |#############
   1342.1 |#########################
   1347.9 |######
   1353.7 |
   1359.5 |
  (13 below, 8 above range)

bp_skew_high (n=160, range 1253.7-1413.1 ns)
   1253.7 |#############
   1261.6 |#####
   1269.6 |###
   1277.6 |###
   1285.6 |#######
   1293.5 |##############
   1301.5 |#####
   1309.5 |###
   1317.5 |##
   1325.4 |#
   1333.4 |####################
   1341.4 |########################################
   1349.3 |#####
   1357.3 |
   1365.3 |
   1373.3 |##
   1381.2 |##
   1389.2 |#
   1397.2 |#
   1405.2 |
  (16 below, 18 above range)

bp_skew_low (n=160, range 1255.1-1428.9 ns)
   1255.1 |############
   1263.8 |
   1272.5 |##
   1281.2 |
   1289.9 |
   1298.6 |###
   1307.2 |#
   1315.9 |
   1324.6 |##
   1333.3 |########################
   1342.0 |########################################
   1350.7 |###
   1359.4 |#
   1368.1 |
   1376.8 |
   1385.4 |
   1394.1 |
   1402.8 |
   1411.5 |#
   1420.2 |
  (15 below, 19 above range)

```
