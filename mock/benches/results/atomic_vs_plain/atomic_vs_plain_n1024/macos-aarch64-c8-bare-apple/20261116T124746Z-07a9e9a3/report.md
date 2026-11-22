# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.8% (fastest 1.24 us to slowest 1.25 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (10 ns) is smaller than the fastest variant's own run-to-run std-dev (15 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (acc_atomic_relaxed)

The baseline acc_atomic_relaxed is the fastest (1.24 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (acc_atomic_relaxed) is the fastest** at 1237.9 ns median
- Spread: 1.01x (fastest 1237.9 ns, slowest 1247.5 ns)

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
| acc_atomic_relaxed | 3365ns | 3375ns | 3325ns | 3363ns | 3412ns | base |
| acc_atomic_seqcst | 3373ns | 3388ns | 3328ns | 3375ns | 3413ns | +0.24% |
| acc_plain | 3382ns | 3388ns | 3325ns | 3374ns | 3463ns | +0.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 1239ns | 1225ns | 1257ns | base | 0.826 |
| acc_atomic_seqcst | 1244ns | 1226ns | 1263ns | +0.35% | 0.823 |
| acc_plain | 1246ns | 1225ns | 1277ns | +0.56% | 0.822 |

## Performance model

- Peak throughput: **0.836 Gops/s** (acc_atomic_relaxed; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.827 | 98.9% |
| acc_atomic_seqcst | 0.821 | 98.2% |
| acc_plain | 0.821 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 3365ns | 3365ns | base |
| acc_atomic_seqcst | 3373ns | 3373ns | +0.24% |
| acc_plain | 3382ns | 3382ns | +0.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 1238ns | base | --- | [1232, 1245] | --- | --- | --- | --- |
| acc_atomic_seqcst | 1248ns | no significant difference | [-0, +2]ns | [1246, 1248] | no | 0.2917 | 0.2917 | 8 |
| acc_plain | 1247ns | no significant difference | [+0, +2]ns | [1245, 1248] | no | 0.1763 | 0.0882 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 1247ns | +0.2% | -1.4% |
| 1 | 2 | 1236ns | +1.0% | +1.0% |
| 1 | 3 | 1230ns | -0.3% | +1.4% |
| 1 | 4 | 1228ns | +1.7% | +3.6% |
| 1 | 5 | 1231ns | +1.2% | +0.7% |
| 1 | 6 | 1243ns | +0.3% | -1.0% |
| 1 | 7 | 1241ns | -0.9% | +0.5% |
| 1 | 8 | 1251ns | -0.2% | -1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | 0.346 | moderate+ |
| acc_atomic_seqcst | -0.397 | moderate- |
| acc_plain | -0.038 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 3/8, lost 5/8
- **acc_plain**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 2.7ns | 1239.4ns | 0.2% |  |
| acc_atomic_seqcst | 2.7ns | 1243.7ns | 0.2% |  |
| acc_plain | 3.0ns | 1246.3ns | 0.2% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 1224.8-1256.6 ns)
   1224.8 |########################################
   1226.4 |###################################
   1227.9 |##############
   1229.5 |##############
   1231.1 |############
   1232.7 |##########
   1234.3 |####
   1235.9 |####
   1237.5 |####
   1239.1 |##
   1240.7 |######
   1242.3 |####
   1243.9 |##############
   1245.5 |##########
   1247.0 |#######################
   1248.6 |#############################
   1250.2 |#######################
   1251.8 |#################################
   1253.4 |##########
   1255.0 |##
  (14 below, 3 above range)

acc_atomic_seqcst (n=160, range 1225.9-1262.6 ns)
   1225.9 |###########################
   1227.7 |#############
   1229.6 |##################
   1231.4 |####
   1233.2 |####
   1235.1 |
   1236.9 |#
   1238.8 |
   1240.6 |#
   1242.4 |###
   1244.3 |##########
   1246.1 |###################################
   1247.9 |########################################
   1249.8 |########################
   1251.6 |########################
   1253.4 |######
   1255.3 |###
   1257.1 |#
   1258.9 |
   1260.8 |
  (12 below, 4 above range)

acc_plain (n=160, range 1225.2-1276.7 ns)
   1225.2 |########################
   1227.8 |##########################
   1230.4 |###############
   1232.9 |###
   1235.5 |######
   1238.1 |###
   1240.7 |######
   1243.2 |####################
   1245.8 |########################
   1248.4 |########################################
   1251.0 |###############################
   1253.6 |#####
   1256.1 |
   1258.7 |
   1261.3 |###
   1263.9 |#
   1266.4 |#####
   1269.0 |######
   1271.6 |#####
   1274.2 |#
  (16 below, 10 above range)

```
