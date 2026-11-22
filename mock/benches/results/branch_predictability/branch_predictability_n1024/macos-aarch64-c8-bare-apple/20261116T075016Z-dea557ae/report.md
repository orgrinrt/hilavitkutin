# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bp_balanced) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bp_balanced has the worst median (1.28 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bp_skew_low at 1.26 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole field within 1.5% of the fastest

All 3 variants sit between 1.26 us and 1.28 us - a 1.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bp_skew_high's edge over baseline is significant but tiny (-3 ns, 0.20%)

bp_skew_high differs from baseline bp_balanced by -3 ns (0.20%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bp_skew_low** at 1257.3 ns median (-1.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.01x (fastest 1257.3 ns, slowest 1276.0 ns)

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
| bp_balanced | 3401ns | 3392ns | 3328ns | 3391ns | 3507ns | base |
| bp_skew_high | 3388ns | 3375ns | 3326ns | 3369ns | 3505ns | -0.40% |
| bp_skew_low | 3362ns | 3342ns | 3326ns | 3355ns | 3418ns | -1.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 1278ns | 1251ns | 1317ns | base | 0.801 |
| bp_skew_high | 1274ns | 1250ns | 1322ns | -0.33% | 0.804 |
| bp_skew_low | 1263ns | 1249ns | 1285ns | -1.21% | 0.811 |

## Performance model

- Peak throughput: **0.820 Gops/s** (bp_skew_low; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.803 | 97.9% |
| bp_skew_high | 0.809 | 98.7% |
| bp_skew_low | 0.814 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 3401ns | 3401ns | base |
| bp_skew_high | 3388ns | 3388ns | -0.40% |
| bp_skew_low | 3362ns | 3362ns | -1.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 1276ns | base | --- | [1272, 1278] | --- | --- | --- | --- |
| bp_skew_high | 1265ns | -2.5ns (-0.2%) | [-4, -0]ns | [1258, 1273] | YES | 0.0192 | 0.0192 | 6 |
| bp_skew_low | 1257ns | -14.4ns (-1.1%) | [-20, -4]ns | [1256, 1259] | YES | 0.0000 | 0.0000 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 1274ns | -0.2% | -1.5% |
| 1 | 2 | 1264ns | -0.8% | -0.8% |
| 1 | 3 | 1313ns | +0.4% | -3.1% |
| 1 | 4 | 1253ns | +1.6% | +0.6% |
| 1 | 5 | 1256ns | +1.4% | -0.2% |
| 1 | 6 | 1274ns | -1.5% | -0.2% |
| 1 | 7 | 1272ns | -0.0% | -1.5% |
| 1 | 8 | 1297ns | -3.1% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.307 | moderate- |
| bp_skew_high | -0.255 | moderate- |
| bp_skew_low | -0.441 | moderate- |

**Consistency summary:**

- **bp_skew_high**: won 4/8, lost 3/8
- **bp_skew_low**: won 7/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 2.8ns | 1278.4ns | 0.2% |  |
| bp_skew_high | 3.0ns | 1274.3ns | 0.2% |  |
| bp_skew_low | 2.8ns | 1262.9ns | 0.2% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 1251.0-1317.1 ns)
   1251.0 |###############################
   1254.3 |########################################
   1257.6 |#################
   1260.9 |###########
   1264.2 |##
   1267.5 |###########
   1270.8 |####################
   1274.1 |#####################################
   1277.4 |####################
   1280.7 |#############
   1284.1 |##
   1287.4 |##
   1290.7 |###############
   1294.0 |####################
   1297.3 |####################
   1300.6 |###########
   1303.9 |########
   1307.2 |########
   1310.5 |
   1313.8 |##
  (15 below, 12 above range)

bp_skew_high (n=160, range 1250.1-1322.1 ns)
   1250.1 |###########################
   1253.7 |########################################
   1257.3 |#########
   1260.9 |###
   1264.5 |#####
   1268.1 |###########
   1271.7 |############
   1275.3 |###############
   1278.9 |###############
   1282.5 |##
   1286.1 |#
   1289.7 |#
   1293.3 |#####
   1296.9 |#########
   1300.5 |###
   1304.1 |
   1307.7 |##
   1311.3 |#
   1314.9 |###
   1318.5 |#
  (16 below, 10 above range)

bp_skew_low (n=160, range 1248.9-1284.8 ns)
   1248.9 |####################
   1250.7 |####################
   1252.5 |######################################
   1254.3 |########################################
   1256.1 |##############
   1257.9 |########################
   1259.7 |##############
   1261.4 |##
   1263.2 |
   1265.0 |
   1266.8 |##
   1268.6 |######
   1270.4 |############
   1272.2 |##########
   1274.0 |####################
   1275.8 |##################
   1277.6 |########
   1279.4 |##################
   1281.2 |
   1283.0 |
  (17 below, 10 above range)

```
