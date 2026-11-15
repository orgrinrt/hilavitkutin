# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 4.57 us to slowest 4.57 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (399 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader acc_plain vs stability leader acc_atomic_seqcst (+0% speed for 1.4x steadier)

acc_plain is fastest (4.57 us, CV 8.7%); acc_atomic_seqcst gives up 0.0% median for 1.4x lower variance (CV 6.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### acc_atomic_relaxed is inconsistent: worst-20% is 1.7x its best-20%

acc_atomic_relaxed's best 20% of batches run at 3.74 us but its worst 20% at 6.40 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: acc_plain** at 4567.3 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 4567.3 ns, slowest 4567.4 ns)

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
| Counter frequency | 2112779839 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 6663ns | 6453ns | 5276ns | 6357ns | 8965ns | base |
| acc_atomic_seqcst | 6314ns | 6454ns | 5718ns | 6453ns | 6496ns | -5.23% |
| acc_plain | 6185ns | 6453ns | 5277ns | 6356ns | 6580ns | -7.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 4731ns | 3739ns | 6405ns | base | 0.866 |
| acc_atomic_seqcst | 4471ns | 4051ns | 4589ns | -5.50% | 0.916 |
| acc_plain | 4379ns | 3739ns | 4654ns | -7.43% | 0.935 |

## Performance model

- Peak throughput: **1.095 Gops/s** (acc_plain; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.897 | 81.9% |
| acc_atomic_seqcst | 0.897 | 81.9% |
| acc_plain | 0.897 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 6663ns | 6663ns | base |
| acc_atomic_seqcst | 6314ns | 6314ns | -5.23% |
| acc_plain | 6185ns | 6185ns | -7.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 4567ns | base | --- | [4567, 4578] | --- | --- | --- | --- |
| acc_atomic_seqcst | 4567ns | no significant difference | [+0, +1]ns | [4567, 4568] | no | 0.0704 | 0.0352 | 7 |
| acc_plain | 4567ns | no significant difference | [-11, +0]ns | [4567, 4567] | no | 0.0769 | 0.0769 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 3737ns | +22.2% | +22.2% |
| 1 | 2 | 4578ns | -0.1% | -18.4% |
| 1 | 3 | 7491ns | -50.1% | -38.9% |
| 1 | 4 | 4567ns | +0.0% | -0.0% |
| 1 | 5 | 4567ns | +0.2% | -18.2% |
| 1 | 6 | 4573ns | -0.1% | -0.1% |
| 1 | 7 | 3737ns | +22.2% | +22.2% |
| 1 | 8 | 4568ns | +0.2% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.036 | ok |
| acc_atomic_seqcst | -0.159 | ok |
| acc_plain | -0.375 | moderate- |

**Consistency summary:**

- **acc_atomic_seqcst**: won 3/8, lost 4/8
- **acc_plain**: won 4/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 24.2ns | 4730.8ns | 0.5% |  |
| acc_atomic_seqcst | 23.0ns | 4470.7ns | 0.5% |  |
| acc_plain | 22.4ns | 4379.3ns | 0.5% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 3739.3-6404.8 ns)
   3739.3 |######
   3872.6 |
   4005.8 |
   4139.1 |
   4272.4 |
   4405.7 |
   4538.9 |########################################
   4672.2 |
   4805.5 |
   4938.8 |
   5072.1 |
   5205.3 |
   5338.6 |
   5471.9 |
   5605.2 |
   5738.4 |
   5871.7 |
   6005.0 |
   6138.3 |
   6271.6 |
  (24 below, 20 above range)

acc_atomic_seqcst (n=160, range 4050.6-4589.3 ns)
   4050.6 |
   4077.5 |
   4104.4 |
   4131.4 |
   4158.3 |
   4185.3 |
   4212.2 |
   4239.1 |
   4266.1 |
   4293.0 |
   4319.9 |
   4346.9 |
   4373.8 |
   4400.8 |
   4427.7 |
   4454.6 |
   4481.6 |
   4508.5 |
   4535.4 |
   4562.4 |########################################
  (20 below, 4 above range)

acc_plain (n=160, range 3739.2-4653.7 ns)
   3739.2 |#####
   3784.9 |
   3830.7 |
   3876.4 |
   3922.1 |
   3967.8 |
   4013.6 |
   4059.3 |
   4105.0 |
   4150.7 |
   4196.5 |
   4242.2 |
   4287.9 |
   4333.6 |
   4379.4 |
   4425.1 |
   4470.8 |
   4516.5 |
   4562.3 |########################################
   4608.0 |
  (24 below, 4 above range)

```

## Diagnostics

- **acc_atomic_relaxed**: CV=23.3% (high variance, measurements may be unstable)
