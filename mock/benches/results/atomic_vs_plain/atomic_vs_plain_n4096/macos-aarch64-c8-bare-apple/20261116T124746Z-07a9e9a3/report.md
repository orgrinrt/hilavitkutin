# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 5.06 us to slowest 5.07 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (48 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: acc_plain** at 5062.9 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 5062.9 ns, slowest 5065.0 ns)

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
| acc_atomic_relaxed | 7204ns | 7168ns | 7160ns | 7181ns | 7318ns | base |
| acc_atomic_seqcst | 7226ns | 7169ns | 7159ns | 7201ns | 7367ns | +0.30% |
| acc_plain | 7202ns | 7166ns | 7159ns | 7176ns | 7323ns | -0.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 5088ns | 5058ns | 5167ns | base | 0.805 |
| acc_atomic_seqcst | 5107ns | 5058ns | 5214ns | +0.38% | 0.802 |
| acc_plain | 5087ns | 5057ns | 5171ns | -0.03% | 0.805 |

## Performance model

- Peak throughput: **0.810 Gops/s** (acc_plain; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.809 | 99.9% |
| acc_atomic_seqcst | 0.809 | 99.8% |
| acc_plain | 0.809 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 7204ns | 7204ns | base |
| acc_atomic_seqcst | 7226ns | 7226ns | +0.30% |
| acc_plain | 7202ns | 7202ns | -0.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 5064ns | base | --- | [5062, 5065] | --- | --- | --- | --- |
| acc_atomic_seqcst | 5065ns | no significant difference | [-1, +2]ns | [5064, 5081] | no | 0.3382 | 0.3382 | 3 |
| acc_plain | 5063ns | no significant difference | [-1, +0]ns | [5062, 5064] | no | 0.2460 | 0.1230 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 5065ns | +0.2% | -0.0% |
| 1 | 2 | 5064ns | +3.4% | -0.0% |
| 1 | 3 | 5062ns | -0.0% | -0.0% |
| 1 | 4 | 5062ns | +0.8% | +0.0% |
| 1 | 5 | 5064ns | +0.0% | -0.0% |
| 1 | 6 | 5065ns | -0.0% | -0.0% |
| 1 | 7 | 5063ns | -0.0% | +0.1% |
| 1 | 8 | 5068ns | -0.0% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | 0.050 | ok |
| acc_atomic_seqcst | -0.202 | moderate- |
| acc_plain | -0.256 | moderate- |

**Consistency summary:**

- **acc_atomic_seqcst**: won 0/8, lost 3/8
- **acc_plain**: won 0/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 2.6ns | 5088.3ns | 0.1% |  |
| acc_atomic_seqcst | 2.6ns | 5107.4ns | 0.1% |  |
| acc_plain | 2.5ns | 5086.7ns | 0.0% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 5057.8-5167.0 ns)
   5057.8 |########################################
   5063.3 |#####################
   5068.7 |#
   5074.2 |#
   5079.6 |#
   5085.1 |
   5090.6 |
   5096.0 |###
   5101.5 |
   5106.9 |#
   5112.4 |
   5117.9 |#
   5123.3 |#
   5128.8 |
   5134.2 |
   5139.7 |#
   5145.2 |
   5150.6 |######
   5156.1 |#######
   5161.5 |##
  (11 below, 6 above range)

acc_atomic_seqcst (n=160, range 5057.6-5214.1 ns)
   5057.6 |########################################
   5065.4 |####
   5073.2 |
   5081.1 |#
   5088.9 |#
   5096.7 |#
   5104.5 |##
   5112.4 |
   5120.2 |#
   5128.0 |
   5135.8 |#
   5143.7 |#
   5151.5 |#########
   5159.3 |###
   5167.2 |
   5175.0 |
   5182.8 |
   5190.6 |#
   5198.5 |#
   5206.3 |#
  (14 below, 17 above range)

acc_plain (n=160, range 5057.2-5170.9 ns)
   5057.2 |########################################
   5062.9 |###############
   5068.6 |#
   5074.3 |#
   5080.0 |##
   5085.6 |
   5091.3 |
   5097.0 |#
   5102.7 |#
   5108.4 |#
   5114.1 |
   5119.8 |#
   5125.4 |
   5131.1 |
   5136.8 |#
   5142.5 |
   5148.2 |##
   5153.9 |####
   5159.5 |####
   5165.2 |
  (14 below, 7 above range)

```
