# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.3% (fastest 1.25 us to slowest 1.26 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (17 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: bp_skew_high** at 1255.0 ns median (-0.3% vs baseline)
- Spread: 1.00x (fastest 1255.0 ns, slowest 1258.3 ns)

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
| bp_balanced | 3381ns | 3360ns | 3341ns | 3372ns | 3448ns | base |
| bp_skew_high | 3366ns | 3349ns | 3341ns | 3355ns | 3424ns | -0.45% |
| bp_skew_low | 3430ns | 3359ns | 3341ns | 3374ns | 3688ns | +1.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 1265ns | 1249ns | 1291ns | base | 0.809 |
| bp_skew_high | 1261ns | 1248ns | 1286ns | -0.37% | 0.812 |
| bp_skew_low | 1279ns | 1250ns | 1354ns | +1.07% | 0.801 |

## Performance model

- Peak throughput: **0.820 Gops/s** (bp_skew_high; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.814 | 99.2% |
| bp_skew_high | 0.816 | 99.5% |
| bp_skew_low | 0.814 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 3381ns | 3381ns | base |
| bp_skew_high | 3366ns | 3366ns | -0.45% |
| bp_skew_low | 3430ns | 3430ns | +1.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 1258ns | base | --- | [1256, 1263] | --- | --- | --- | --- |
| bp_skew_high | 1255ns | no significant difference | [-2, +0]ns | [1254, 1256] | no | 0.0585 | 0.0292 | 6 |
| bp_skew_low | 1258ns | no significant difference | [-2, +1]ns | [1257, 1270] | no | 0.8732 | 0.8732 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 1258ns | -0.3% | +1.2% |
| 1 | 2 | 1258ns | -0.2% | -0.2% |
| 1 | 3 | 1257ns | -0.2% | +0.5% |
| 1 | 4 | 1284ns | -2.3% | -1.6% |
| 1 | 5 | 1256ns | +1.1% | -0.2% |
| 1 | 6 | 1254ns | +0.0% | +7.7% |
| 1 | 7 | 1256ns | +1.4% | -0.1% |
| 1 | 8 | 1278ns | -1.8% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.209 | moderate- |
| bp_skew_high | -0.412 | moderate- |
| bp_skew_low | -0.343 | moderate- |

**Consistency summary:**

- **bp_skew_high**: won 5/8, lost 2/8
- **bp_skew_low**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 2.9ns | 1265.3ns | 0.2% |  |
| bp_skew_high | 2.8ns | 1260.6ns | 0.2% |  |
| bp_skew_low | 2.9ns | 1278.9ns | 0.2% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 1249.1-1291.0 ns)
   1249.1 |##########################
   1251.2 |####################
   1253.3 |######################################
   1255.4 |########################################
   1257.5 |##################
   1259.6 |##########
   1261.7 |####
   1263.7 |######
   1265.8 |####
   1267.9 |##########
   1270.0 |########
   1272.1 |##############
   1274.2 |############
   1276.3 |##############
   1278.4 |####################
   1280.5 |##############
   1282.6 |####
   1284.7 |##
   1286.8 |######
   1288.9 |##
  (15 below, 9 above range)

bp_skew_high (n=160, range 1248.4-1286.3 ns)
   1248.4 |#########
   1250.3 |################
   1252.2 |########################
   1254.1 |########################################
   1256.0 |#########################
   1257.9 |########
   1259.8 |###
   1261.7 |
   1263.6 |#
   1265.5 |#
   1267.4 |###
   1269.3 |####
   1271.2 |##
   1273.1 |####
   1275.0 |#######
   1276.9 |###
   1278.7 |#
   1280.6 |######
   1282.5 |
   1284.4 |##
  (16 below, 6 above range)

bp_skew_low (n=160, range 1249.7-1353.6 ns)
   1249.7 |########################################
   1254.9 |#######################
   1260.1 |###
   1265.3 |#########
   1270.5 |##########
   1275.7 |##############
   1280.8 |########
   1286.0 |
   1291.2 |
   1296.4 |
   1301.6 |
   1306.8 |
   1312.0 |
   1317.2 |
   1322.4 |
   1327.6 |
   1332.8 |
   1338.0 |
   1343.2 |#####
   1348.4 |####
  (14 below, 8 above range)

```
