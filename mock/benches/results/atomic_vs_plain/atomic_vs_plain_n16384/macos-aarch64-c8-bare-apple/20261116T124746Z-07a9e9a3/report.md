# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 20.41 us to slowest 20.41 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (115 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: acc_plain** at 20406.2 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 20406.2 ns, slowest 20407.3 ns)

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
| acc_atomic_relaxed | 22559ns | 22521ns | 22502ns | 22529ns | 22705ns | base |
| acc_atomic_seqcst | 22565ns | 22523ns | 22503ns | 22531ns | 22727ns | +0.03% |
| acc_plain | 22555ns | 22523ns | 22502ns | 22529ns | 22685ns | -0.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 20446ns | 20398ns | 20577ns | base | 0.801 |
| acc_atomic_seqcst | 20451ns | 20399ns | 20593ns | +0.02% | 0.801 |
| acc_plain | 20437ns | 20399ns | 20537ns | -0.04% | 0.802 |

## Performance model

- Peak throughput: **0.803 Gops/s** (acc_atomic_relaxed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.803 | 100.0% |
| acc_atomic_seqcst | 0.803 | 100.0% |
| acc_plain | 0.803 | 100.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 22559ns | 22559ns | base |
| acc_atomic_seqcst | 22565ns | 22565ns | +0.03% |
| acc_plain | 22555ns | 22555ns | -0.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 20407ns | base | --- | [20405, 20414] | --- | --- | --- | --- |
| acc_atomic_seqcst | 20407ns | no significant difference | [-1, +2]ns | [20405, 20417] | no | 1.0000 | 1.0000 | 4 |
| acc_plain | 20406ns | no significant difference | [-2, +1]ns | [20405, 20409] | no | 1.0000 | 0.9366 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 20405ns | +0.0% | +0.0% |
| 1 | 2 | 20441ns | +0.1% | -0.1% |
| 1 | 3 | 20406ns | -0.0% | +0.1% |
| 1 | 4 | 20407ns | +0.0% | -0.0% |
| 1 | 5 | 20404ns | +0.0% | +0.0% |
| 1 | 6 | 20406ns | +0.0% | -0.0% |
| 1 | 7 | 20442ns | -0.2% | -0.2% |
| 1 | 8 | 20410ns | -0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.334 | moderate- |
| acc_atomic_seqcst | -0.148 | ok |
| acc_plain | -0.005 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 1/8, lost 1/8
- **acc_plain**: won 2/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 2.9ns | 20446.1ns | 0.0% |  |
| acc_atomic_seqcst | 2.9ns | 20450.6ns | 0.0% |  |
| acc_plain | 2.8ns | 20437.0ns | 0.0% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 20398.5-20576.6 ns)
  20398.5 |########################################
  20407.4 |#######
  20416.3 |##
  20425.2 |####
  20434.1 |##
  20443.0 |#####
  20451.9 |##
  20460.8 |#####
  20469.7 |#
  20478.6 |###
  20487.5 |#
  20496.5 |
  20505.4 |#
  20514.3 |
  20523.2 |#
  20532.1 |
  20541.0 |
  20549.9 |
  20558.8 |
  20567.7 |
  (14 below, 8 above range)

acc_atomic_seqcst (n=160, range 20399.1-20593.3 ns)
  20399.1 |########################################
  20408.8 |#####
  20418.6 |#
  20428.3 |###
  20438.0 |###
  20447.7 |######
  20457.4 |########
  20467.1 |####
  20476.8 |
  20486.5 |#
  20496.2 |
  20505.9 |
  20515.6 |#
  20525.4 |#
  20535.1 |
  20544.8 |
  20554.5 |
  20564.2 |
  20573.9 |
  20583.6 |#
  (15 below, 7 above range)

acc_plain (n=160, range 20399.2-20537.5 ns)
  20399.2 |########################################
  20406.1 |##########
  20413.1 |#
  20420.0 |#
  20426.9 |#####
  20433.8 |#
  20440.7 |######
  20447.6 |####
  20454.5 |#
  20461.4 |###
  20468.3 |####
  20475.3 |#
  20482.2 |#
  20489.1 |#
  20496.0 |#
  20502.9 |
  20509.8 |#
  20516.7 |
  20523.6 |
  20530.5 |
  (16 below, 3 above range)

```
