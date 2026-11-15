# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (acc_atomic_relaxed) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline acc_atomic_relaxed has the worst median (1.31 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest acc_plain at 1.30 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.0% (fastest 1.30 us to slowest 1.31 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### acc_atomic_relaxed shows alternating (throttle bounce) (autocorr -0.70)

acc_atomic_relaxed's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (13 ns) is smaller than the fastest variant's own run-to-run std-dev (54 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: acc_plain** at 1301.7 ns median (-1.0% vs baseline)
- Spread: 1.01x (fastest 1301.7 ns, slowest 1314.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4823d76519bef0a3 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 3541ns | 3560ns | 3324ns | 3530ns | 3793ns | base |
| acc_atomic_seqcst | 3512ns | 3550ns | 3316ns | 3471ns | 3832ns | -0.82% |
| acc_plain | 3502ns | 3516ns | 3315ns | 3495ns | 3709ns | -1.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 1307ns | 1228ns | 1402ns | base | 0.783 |
| acc_atomic_seqcst | 1296ns | 1224ns | 1413ns | -0.84% | 0.790 |
| acc_plain | 1296ns | 1226ns | 1375ns | -0.87% | 0.790 |

## Performance model

- Peak throughput: **0.837 Gops/s** (acc_atomic_seqcst; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.779 | 93.1% |
| acc_atomic_seqcst | 0.781 | 93.3% |
| acc_plain | 0.787 | 94.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 3541ns | 3541ns | base |
| acc_atomic_seqcst | 3512ns | 3512ns | -0.82% |
| acc_plain | 3502ns | 3502ns | -1.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 1314ns | base | --- | [1312, 1315] | --- | --- | --- | --- |
| acc_atomic_seqcst | 1312ns | no significant difference | [-5, +0]ns | [1244, 1315] | no | 0.0945 | 0.0903 | 6 |
| acc_plain | 1302ns | no significant difference | [-33, +0]ns | [1280, 1311] | no | 0.0945 | 0.0945 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 1239ns | -1.0% | -0.7% |
| 1 | 2 | 1380ns | -11.1% | -4.7% |
| 1 | 3 | 1229ns | +7.2% | +7.2% |
| 1 | 4 | 1313ns | +0.3% | -4.1% |
| 1 | 5 | 1315ns | -6.6% | -6.9% |
| 1 | 6 | 1318ns | +0.3% | +0.0% |
| 1 | 7 | 1264ns | -2.2% | +3.1% |
| 1 | 8 | 1321ns | +0.5% | -3.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.704 | HIGH- (thermal bounce) |
| acc_atomic_seqcst | -0.352 | moderate- |
| acc_plain | -0.127 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 4/8, lost 4/8
- **acc_plain**: won 5/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 2.9ns | 1307.0ns | 0.2% |  |
| acc_atomic_seqcst | 2.9ns | 1296.0ns | 0.2% |  |
| acc_plain | 3.0ns | 1295.6ns | 0.2% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 1227.7-1401.6 ns)
   1227.7 |###############
   1236.4 |######
   1245.1 |###
   1253.8 |###
   1262.5 |#######
   1271.2 |######
   1279.9 |#
   1288.6 |
   1297.3 |#
   1306.0 |############################
   1314.6 |########################################
   1323.3 |#######
   1332.0 |##
   1340.7 |##
   1349.4 |#
   1358.1 |#
   1366.8 |
   1375.5 |
   1384.2 |
   1392.9 |#
  (16 below, 23 above range)

acc_atomic_seqcst (n=160, range 1224.0-1413.1 ns)
   1224.0 |########################################
   1233.4 |######
   1242.9 |###
   1252.4 |
   1261.8 |##
   1271.3 |
   1280.7 |
   1290.2 |
   1299.6 |
   1309.1 |#############################
   1318.5 |###########
   1328.0 |##
   1337.4 |
   1346.9 |
   1356.3 |#
   1365.8 |
   1375.3 |
   1384.7 |#
   1394.2 |
   1403.6 |###
  (14 below, 23 above range)

acc_plain (n=160, range 1226.1-1374.6 ns)
   1226.1 |######################
   1233.5 |##
   1241.0 |##
   1248.4 |#
   1255.8 |#####
   1263.2 |################
   1270.6 |###############
   1278.1 |###########
   1285.5 |####
   1292.9 |####
   1300.3 |####
   1307.8 |###################################
   1315.2 |########################################
   1322.6 |
   1330.0 |#####
   1337.4 |
   1344.9 |#
   1352.3 |
   1359.7 |#
   1367.1 |
  (17 below, 17 above range)

```
