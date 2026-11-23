# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 4.57 us to slowest 4.57 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (402 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: acc_atomic_seqcst** at 4567.2 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 4567.2 ns, slowest 4567.3 ns)

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
| Counter frequency | 2112780760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 6023ns | 6449ns | 5275ns | 6116ns | 6492ns | base |
| acc_atomic_seqcst | 6025ns | 6448ns | 5277ns | 6121ns | 6487ns | +0.04% |
| acc_plain | 6021ns | 6449ns | 5276ns | 6116ns | 6482ns | -0.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 4263ns | 3737ns | 4590ns | base | 0.961 |
| acc_atomic_seqcst | 4265ns | 3737ns | 4585ns | +0.04% | 0.960 |
| acc_plain | 4261ns | 3737ns | 4582ns | -0.04% | 0.961 |

## Performance model

- Peak throughput: **1.096 Gops/s** (acc_atomic_relaxed; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.897 | 81.8% |
| acc_atomic_seqcst | 0.897 | 81.8% |
| acc_plain | 0.897 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 6023ns | 6023ns | base |
| acc_atomic_seqcst | 6025ns | 6025ns | +0.04% |
| acc_plain | 6021ns | 6021ns | -0.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 4567ns | base | --- | [4567, 4567] | --- | --- | --- | --- |
| acc_atomic_seqcst | 4567ns | no significant difference | [-0, +1]ns | [4567, 4567] | no | 0.6891 | 0.5179 | 7 |
| acc_plain | 4567ns | no significant difference | [-0, +0]ns | [4567, 4567] | no | 0.6891 | 0.6891 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 3737ns | +0.0% | +0.0% |
| 1 | 2 | 4582ns | -0.2% | -18.4% |
| 1 | 3 | 3737ns | +0.2% | +22.5% |
| 1 | 4 | 4578ns | -18.4% | -0.0% |
| 1 | 5 | 3737ns | +22.2% | +22.2% |
| 1 | 6 | 4572ns | -0.1% | -0.1% |
| 1 | 7 | 4568ns | +0.2% | -18.2% |
| 1 | 8 | 4568ns | +0.0% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.490 | moderate- |
| acc_atomic_seqcst | 0.057 | ok |
| acc_plain | 0.056 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 3/8, lost 3/8
- **acc_plain**: won 3/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 22.0ns | 4263.0ns | 0.5% |  |
| acc_atomic_seqcst | 22.0ns | 4264.8ns | 0.5% |  |
| acc_plain | 21.9ns | 4261.4ns | 0.5% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 3736.8-4590.0 ns)
   3736.8 |####################
   3779.4 |
   3822.1 |
   3864.8 |
   3907.4 |
   3950.1 |
   3992.7 |
   4035.4 |
   4078.1 |
   4120.7 |
   4163.4 |
   4206.0 |
   4248.7 |
   4291.3 |
   4334.0 |
   4376.7 |
   4419.3 |
   4462.0 |
   4504.6 |
   4547.3 |########################################
  (10 below, 3 above range)

acc_atomic_seqcst (n=160, range 3736.9-4585.1 ns)
   3736.9 |##################
   3779.3 |
   3821.7 |
   3864.1 |
   3906.5 |
   3948.9 |
   3991.3 |
   4033.7 |
   4076.2 |
   4118.6 |
   4161.0 |
   4203.4 |
   4245.8 |
   4288.2 |
   4330.6 |
   4373.0 |
   4415.4 |
   4457.9 |
   4500.3 |
   4542.7 |########################################
  (12 below, 1 above range)

acc_plain (n=160, range 3736.9-4582.3 ns)
   3736.9 |####################
   3779.2 |
   3821.4 |
   3863.7 |
   3906.0 |
   3948.2 |
   3990.5 |
   4032.8 |
   4075.0 |
   4117.3 |
   4159.6 |
   4201.9 |
   4244.1 |
   4286.4 |
   4328.7 |
   4370.9 |
   4413.2 |
   4455.5 |
   4497.7 |
   4540.0 |########################################
  (11 below, 5 above range)

```
