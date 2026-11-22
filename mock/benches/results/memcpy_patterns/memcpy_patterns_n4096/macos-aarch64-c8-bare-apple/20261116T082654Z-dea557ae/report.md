# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 5.37 us to slowest 5.37 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (36 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: memcpy_naive** at 5367.9 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 5367.9 ns, slowest 5369.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4646464650504040 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 7515ns | 7499ns | 7488ns | 7502ns | 7579ns | base |
| memcpy_naive | 7521ns | 7506ns | 7489ns | 7511ns | 7586ns | +0.09% |
| memcpy_unrolled | 7530ns | 7502ns | 7487ns | 7506ns | 7647ns | +0.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 5379ns | 5361ns | 5425ns | base | 0.761 |
| memcpy_naive | 5384ns | 5361ns | 5443ns | +0.09% | 0.761 |
| memcpy_unrolled | 5392ns | 5359ns | 5486ns | +0.23% | 0.760 |

## Performance model

- Peak throughput: **0.764 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.763 | 99.8% |
| memcpy_naive | 0.763 | 99.8% |
| memcpy_unrolled | 0.763 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 7515ns | 7515ns | base |
| memcpy_naive | 7521ns | 7521ns | +0.09% |
| memcpy_unrolled | 7530ns | 7530ns | +0.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 5368ns | base | --- | [5367, 5370] | --- | --- | --- | --- |
| memcpy_naive | 5368ns | no significant difference | [-0, +2]ns | [5367, 5371] | no | 0.4249 | 0.4249 | 3 |
| memcpy_unrolled | 5369ns | no significant difference | [-2, +1]ns | [5368, 5371] | no | 0.4249 | 0.3785 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 5370ns | -0.1% | +0.0% |
| 1 | 2 | 5366ns | +0.0% | +0.0% |
| 1 | 3 | 5367ns | +0.1% | +0.1% |
| 1 | 4 | 5374ns | -0.1% | +0.1% |
| 1 | 5 | 5368ns | +0.0% | +0.0% |
| 1 | 6 | 5367ns | +0.1% | -0.0% |
| 1 | 7 | 5367ns | +0.0% | -0.1% |
| 1 | 8 | 5371ns | +0.0% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.257 | moderate- |
| memcpy_naive | -0.200 | moderate- |
| memcpy_unrolled | -0.042 | ok |

**Consistency summary:**

- **memcpy_naive**: won 0/8, lost 1/8
- **memcpy_unrolled**: won 0/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 3.7ns | 5379.1ns | 0.1% |  |
| memcpy_naive | 3.6ns | 5384.0ns | 0.1% |  |
| memcpy_unrolled | 3.6ns | 5391.5ns | 0.1% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 5361.0-5425.4 ns)
   5361.0 |#################################
   5364.3 |########################################
   5367.5 |#####################################
   5370.7 |###############
   5373.9 |################
   5377.1 |#####
   5380.3 |######
   5383.6 |##
   5386.8 |
   5390.0 |##
   5393.2 |#
   5396.4 |#
   5399.6 |##
   5402.8 |#####
   5406.1 |
   5409.3 |#
   5412.5 |
   5415.7 |##
   5418.9 |##
   5422.1 |####
  (16 below, 12 above range)

memcpy_naive (n=160, range 5360.7-5443.3 ns)
   5360.7 |########################################
   5364.8 |###################################
   5369.0 |#############
   5373.1 |#############
   5377.2 |######
   5381.4 |##
   5385.5 |
   5389.6 |##
   5393.7 |
   5397.9 |##
   5402.0 |#
   5406.1 |#####
   5410.3 |
   5414.4 |##
   5418.5 |####
   5422.7 |####
   5426.8 |###
   5430.9 |####
   5435.1 |##
   5439.2 |##
  (12 below, 9 above range)

memcpy_unrolled (n=160, range 5358.7-5485.9 ns)
   5358.7 |###############################
   5365.1 |########################################
   5371.5 |############
   5377.8 |###########
   5384.2 |######
   5390.5 |
   5396.9 |
   5403.3 |
   5409.6 |#####
   5416.0 |###
   5422.3 |##
   5428.7 |
   5435.1 |#
   5441.4 |
   5447.8 |##
   5454.1 |
   5460.5 |#
   5466.8 |
   5473.2 |
   5479.6 |
  (15 below, 6 above range)

```
