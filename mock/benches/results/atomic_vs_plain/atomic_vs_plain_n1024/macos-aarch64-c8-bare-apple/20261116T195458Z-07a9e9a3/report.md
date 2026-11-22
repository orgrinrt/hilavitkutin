# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.2% (fastest 1.23 us to slowest 1.23 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (18 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: acc_plain** at 1228.7 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 1228.7 ns, slowest 1231.0 ns)

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
| acc_atomic_relaxed | 3360ns | 3333ns | 3325ns | 3346ns | 3437ns | base |
| acc_atomic_seqcst | 3361ns | 3338ns | 3324ns | 3355ns | 3418ns | +0.04% |
| acc_plain | 3353ns | 3334ns | 3325ns | 3343ns | 3408ns | -0.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 1238ns | 1223ns | 1270ns | base | 0.827 |
| acc_atomic_seqcst | 1240ns | 1224ns | 1269ns | +0.17% | 0.826 |
| acc_plain | 1236ns | 1224ns | 1260ns | -0.19% | 0.829 |

## Performance model

- Peak throughput: **0.837 Gops/s** (acc_atomic_relaxed; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.833 | 99.5% |
| acc_atomic_seqcst | 0.832 | 99.4% |
| acc_plain | 0.833 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 3360ns | 3360ns | base |
| acc_atomic_seqcst | 3361ns | 3361ns | +0.04% |
| acc_plain | 3353ns | 3353ns | -0.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 1229ns | base | --- | [1228, 1230] | --- | --- | --- | --- |
| acc_atomic_seqcst | 1231ns | no significant difference | [-0, +3]ns | [1229, 1242] | no | 0.4653 | 0.2326 | 2 |
| acc_plain | 1229ns | no significant difference | [-1, +1]ns | [1228, 1231] | no | 0.8078 | 0.8078 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 1266ns | -3.1% | -2.7% |
| 1 | 2 | 1229ns | +1.7% | -0.2% |
| 1 | 3 | 1227ns | +1.5% | +1.5% |
| 1 | 4 | 1248ns | -1.7% | -1.8% |
| 1 | 5 | 1229ns | -0.0% | +0.3% |
| 1 | 6 | 1228ns | +0.8% | +1.7% |
| 1 | 7 | 1226ns | +1.6% | +0.1% |
| 1 | 8 | 1226ns | +0.2% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.094 | ok |
| acc_atomic_seqcst | -0.139 | ok |
| acc_plain | -0.352 | moderate- |

**Consistency summary:**

- **acc_atomic_seqcst**: won 2/8, lost 5/8
- **acc_plain**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 2.7ns | 1238.1ns | 0.2% |  |
| acc_atomic_seqcst | 2.6ns | 1240.3ns | 0.2% |  |
| acc_plain | 2.6ns | 1235.7ns | 0.2% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 1223.5-1270.1 ns)
   1223.5 |########################################
   1225.8 |#################################
   1228.2 |##################################
   1230.5 |######
   1232.8 |#####
   1235.1 |##
   1237.5 |
   1239.8 |#####
   1242.1 |#
   1244.5 |#########
   1246.8 |########
   1249.1 |#########
   1251.5 |##########
   1253.8 |#
   1256.1 |
   1258.5 |
   1260.8 |#
   1263.1 |
   1265.4 |########
   1267.8 |#
  (15 below, 11 above range)

acc_atomic_seqcst (n=160, range 1224.3-1268.6 ns)
   1224.3 |##############################
   1226.5 |########################################
   1228.8 |###################
   1231.0 |#####
   1233.2 |#####
   1235.4 |#
   1237.6 |##
   1239.8 |#
   1242.1 |########
   1244.3 |######
   1246.5 |######################
   1248.7 |#################
   1250.9 |#############
   1253.1 |####
   1255.4 |####
   1257.6 |##
   1259.8 |#
   1262.0 |
   1264.2 |
   1266.4 |#
  (15 below, 8 above range)

acc_plain (n=160, range 1224.2-1259.9 ns)
   1224.2 |##################################
   1225.9 |########################################
   1227.7 |################################
   1229.5 |##############
   1231.3 |################
   1233.1 |#####
   1234.9 |##
   1236.7 |#
   1238.4 |#
   1240.2 |
   1242.0 |####
   1243.8 |########
   1245.6 |##
   1247.4 |########
   1249.2 |##############
   1250.9 |########
   1252.7 |#######
   1254.5 |##
   1256.3 |
   1258.1 |
  (13 below, 6 above range)

```
