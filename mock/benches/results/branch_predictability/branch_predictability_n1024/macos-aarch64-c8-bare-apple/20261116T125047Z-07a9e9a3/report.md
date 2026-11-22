# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bp_balanced) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bp_balanced has the worst median (1.25 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bp_skew_high at 1.25 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 1.25 us to slowest 1.25 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (15 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: bp_skew_high** at 1253.8 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 1253.8 ns, slowest 1254.2 ns)

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
| bp_balanced | 3364ns | 3346ns | 3338ns | 3347ns | 3440ns | base |
| bp_skew_high | 3349ns | 3343ns | 3337ns | 3343ns | 3380ns | -0.43% |
| bp_skew_low | 3361ns | 3347ns | 3338ns | 3347ns | 3424ns | -0.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 1258ns | 1247ns | 1281ns | base | 0.814 |
| bp_skew_high | 1256ns | 1248ns | 1274ns | -0.15% | 0.815 |
| bp_skew_low | 1259ns | 1248ns | 1285ns | +0.06% | 0.813 |

## Performance model

- Peak throughput: **0.821 Gops/s** (bp_balanced; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.816 | 99.4% |
| bp_skew_high | 0.817 | 99.5% |
| bp_skew_low | 0.817 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 3364ns | 3364ns | base |
| bp_skew_high | 3349ns | 3349ns | -0.43% |
| bp_skew_low | 3361ns | 3361ns | -0.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 1254ns | base | --- | [1254, 1255] | --- | --- | --- | --- |
| bp_skew_high | 1254ns | no significant difference | [-1, +0]ns | [1253, 1255] | no | 0.6640 | 0.3320 | 7 |
| bp_skew_low | 1254ns | no significant difference | [-0, +1]ns | [1253, 1255] | no | 0.6832 | 0.6832 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 1254ns | -0.1% | -0.1% |
| 1 | 2 | 1254ns | -0.1% | -0.0% |
| 1 | 3 | 1253ns | +0.2% | +0.1% |
| 1 | 4 | 1258ns | -0.4% | -0.4% |
| 1 | 5 | 1254ns | -0.0% | +2.3% |
| 1 | 6 | 1254ns | -0.0% | +0.0% |
| 1 | 7 | 1255ns | -0.0% | -0.1% |
| 1 | 8 | 1255ns | -0.0% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.258 | moderate- |
| bp_skew_high | -0.146 | ok |
| bp_skew_low | -0.164 | ok |

**Consistency summary:**

- **bp_skew_high**: won 1/8, lost 1/8
- **bp_skew_low**: won 1/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 2.8ns | 1258.2ns | 0.2% |  |
| bp_skew_high | 2.8ns | 1256.4ns | 0.2% |  |
| bp_skew_low | 2.8ns | 1259.0ns | 0.2% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 1247.2-1281.4 ns)
   1247.2 |###############
   1248.9 |###############
   1250.6 |########################
   1252.3 |##########################
   1254.0 |########################################
   1255.7 |###########################
   1257.5 |################
   1259.2 |#########
   1260.9 |
   1262.6 |#
   1264.3 |
   1266.0 |#
   1267.7 |#
   1269.4 |#
   1271.2 |
   1272.9 |
   1274.6 |#
   1276.3 |#
   1278.0 |
   1279.7 |
  (15 below, 12 above range)

bp_skew_high (n=160, range 1247.6-1273.6 ns)
   1247.6 |############
   1248.9 |##############
   1250.2 |#########################
   1251.5 |##################
   1252.8 |######################
   1254.1 |########################################
   1255.4 |######################
   1256.7 |########
   1258.0 |###########
   1259.3 |#######
   1260.6 |
   1261.9 |##
   1263.2 |
   1264.5 |
   1265.8 |#
   1267.1 |
   1268.4 |
   1269.7 |
   1271.0 |#
   1272.3 |
  (17 below, 10 above range)

bp_skew_low (n=160, range 1247.7-1285.2 ns)
   1247.7 |############
   1249.6 |#######################
   1251.4 |########################################
   1253.3 |#############################
   1255.2 |################
   1257.1 |#####################
   1258.9 |#######
   1260.8 |
   1262.7 |#
   1264.6 |
   1266.5 |
   1268.3 |
   1270.2 |##
   1272.1 |#
   1274.0 |#
   1275.8 |
   1277.7 |#
   1279.6 |
   1281.5 |
   1283.3 |
  (14 below, 16 above range)

```
