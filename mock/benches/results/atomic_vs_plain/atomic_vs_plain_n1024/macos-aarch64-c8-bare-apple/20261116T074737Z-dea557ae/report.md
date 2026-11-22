# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (24 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.2% of the fastest

All 3 variants sit between 1.25 us and 1.25 us - a 0.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### acc_plain's edge over baseline is significant but tiny (2 ns, 0.14%)

acc_plain differs from baseline acc_atomic_relaxed by 2 ns (0.14%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: acc_atomic_seqcst** at 1247.3 ns median (-0.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 1247.3 ns, slowest 1250.0 ns)

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
| acc_atomic_relaxed | 3373ns | 3380ns | 3317ns | 3377ns | 3417ns | base |
| acc_atomic_seqcst | 3376ns | 3377ns | 3312ns | 3369ns | 3461ns | +0.09% |
| acc_plain | 3386ns | 3384ns | 3355ns | 3384ns | 3423ns | +0.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 1246ns | 1227ns | 1260ns | base | 0.822 |
| acc_atomic_seqcst | 1249ns | 1225ns | 1286ns | +0.26% | 0.820 |
| acc_plain | 1251ns | 1238ns | 1267ns | +0.42% | 0.819 |

## Performance model

- Peak throughput: **0.836 Gops/s** (acc_atomic_seqcst; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.820 | 98.1% |
| acc_atomic_seqcst | 0.821 | 98.2% |
| acc_plain | 0.819 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 3373ns | 3373ns | base |
| acc_atomic_seqcst | 3376ns | 3376ns | +0.09% |
| acc_plain | 3386ns | 3386ns | +0.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 1248ns | base | --- | [1247, 1249] | --- | --- | --- | --- |
| acc_atomic_seqcst | 1247ns | no significant difference | [-2, +0]ns | [1245, 1249] | no | 0.2356 | 0.2356 | 0 |
| acc_plain | 1250ns | +1.7ns (+0.1%) | [+1, +3]ns | [1249, 1251] | YES | 0.0015 | 0.0007 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 1247ns | -1.5% | +0.1% |
| 1 | 2 | 1246ns | +0.2% | +0.3% |
| 1 | 3 | 1250ns | -0.4% | -0.5% |
| 1 | 4 | 1250ns | +0.4% | +0.1% |
| 1 | 5 | 1249ns | -1.6% | +0.0% |
| 1 | 6 | 1248ns | -0.3% | +0.4% |
| 1 | 7 | 1252ns | -0.6% | -0.2% |
| 1 | 8 | 1226ns | +2.2% | +2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.160 | ok |
| acc_atomic_seqcst | -0.326 | moderate- |
| acc_plain | -0.265 | moderate- |

**Consistency summary:**

- **acc_atomic_seqcst**: won 5/8, lost 3/8
- **acc_plain**: won 2/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 3.2ns | 1245.8ns | 0.3% |  |
| acc_atomic_seqcst | 2.8ns | 1249.0ns | 0.2% |  |
| acc_plain | 3.0ns | 1251.1ns | 0.2% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 1227.2-1260.5 ns)
   1227.2 |#######
   1228.9 |#####
   1230.6 |########
   1232.2 |##
   1233.9 |##
   1235.5 |
   1237.2 |
   1238.9 |##
   1240.5 |###
   1242.2 |##
   1243.8 |#############
   1245.5 |#################
   1247.2 |#################
   1248.8 |########################################
   1250.5 |#################
   1252.1 |#################
   1253.8 |#######
   1255.5 |##
   1257.1 |
   1258.8 |##
  (17 below, 6 above range)

acc_atomic_seqcst (n=160, range 1224.9-1286.0 ns)
   1224.9 |######################################
   1228.0 |#########################
   1231.1 |###
   1234.1 |
   1237.2 |
   1240.2 |#########
   1243.3 |####################################
   1246.3 |######################################
   1249.4 |########################################
   1252.4 |###########################
   1255.5 |#######
   1258.5 |#
   1261.6 |
   1264.6 |#
   1267.7 |#
   1270.8 |#####
   1273.8 |
   1276.9 |#
   1279.9 |#
   1283.0 |
  (14 below, 14 above range)

acc_plain (n=160, range 1238.5-1267.1 ns)
   1238.5 |
   1239.9 |
   1241.3 |###
   1242.8 |####
   1244.2 |################
   1245.6 |#########################
   1247.1 |#########################
   1248.5 |###########################
   1249.9 |########################################
   1251.4 |###########################
   1252.8 |####################
   1254.2 |###########
   1255.7 |###
   1257.1 |####
   1258.5 |#
   1260.0 |
   1261.4 |#
   1262.8 |###
   1264.3 |#
   1265.7 |###
  (12 below, 10 above range)

```
