# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 20.45 us to slowest 20.47 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (15 ns) is smaller than the fastest variant's own run-to-run std-dev (306 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (memcpy_intrinsic)

The baseline memcpy_intrinsic is the fastest (20.45 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (memcpy_intrinsic) is the fastest** at 20454.8 ns median
- Spread: 1.00x (fastest 20454.8 ns, slowest 20469.8 ns)

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
| memcpy_intrinsic | 22722ns | 22559ns | 22495ns | 22597ns | 23326ns | base |
| memcpy_naive | 22873ns | 22576ns | 22501ns | 22652ns | 23909ns | +0.66% |
| memcpy_unrolled | 22719ns | 22573ns | 22507ns | 22587ns | 23325ns | -0.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 20600ns | 20402ns | 21140ns | base | 0.795 |
| memcpy_naive | 20729ns | 20403ns | 21635ns | +0.63% | 0.790 |
| memcpy_unrolled | 20592ns | 20405ns | 21140ns | -0.04% | 0.796 |

## Performance model

- Peak throughput: **0.803 Gops/s** (memcpy_intrinsic; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.801 | 99.7% |
| memcpy_naive | 0.800 | 99.7% |
| memcpy_unrolled | 0.801 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 22722ns | 22722ns | base |
| memcpy_naive | 22873ns | 22873ns | +0.66% |
| memcpy_unrolled | 22719ns | 22719ns | -0.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 20455ns | base | --- | [20440, 20468] | --- | --- | --- | --- |
| memcpy_naive | 20470ns | no significant difference | [-3, +29]ns | [20457, 20525] | no | 0.3064 | 0.1532 | 1 |
| memcpy_unrolled | 20465ns | no significant difference | [-5, +15]ns | [20456, 20475] | no | 0.4769 | 0.4769 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 20452ns | +1.9% | +0.0% |
| 1 | 2 | 20432ns | +0.1% | +0.2% |
| 1 | 3 | 20852ns | -1.9% | +2.4% |
| 1 | 4 | 20446ns | -0.1% | -0.0% |
| 1 | 5 | 20420ns | +3.2% | +0.1% |
| 1 | 6 | 20464ns | +0.5% | +0.0% |
| 1 | 7 | 20471ns | -0.1% | -0.1% |
| 1 | 8 | 20467ns | +0.4% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.214 | moderate- |
| memcpy_naive | -0.211 | moderate- |
| memcpy_unrolled | -0.159 | ok |

**Consistency summary:**

- **memcpy_naive**: won 3/8, lost 5/8
- **memcpy_unrolled**: won 1/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 3.9ns | 20599.9ns | 0.0% |  |
| memcpy_naive | 4.5ns | 20729.5ns | 0.0% |  |
| memcpy_unrolled | 3.9ns | 20592.2ns | 0.0% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 20402.2-21140.5 ns)
  20402.2 |########################################
  20439.1 |########################
  20476.0 |######
  20512.9 |#####
  20549.9 |
  20586.8 |#
  20623.7 |##
  20660.6 |
  20697.5 |
  20734.4 |#
  20771.3 |####
  20808.2 |##
  20845.2 |###
  20882.1 |
  20919.0 |
  20955.9 |#
  20992.8 |
  21029.7 |
  21066.6 |
  21103.6 |####
  (14 below, 14 above range)

memcpy_naive (n=160, range 20403.4-21635.0 ns)
  20403.4 |########################################
  20465.0 |############
  20526.6 |#######
  20588.2 |#####
  20649.7 |####
  20711.3 |
  20772.9 |##
  20834.5 |##
  20896.1 |
  20957.6 |##
  21019.2 |#
  21080.8 |###
  21142.4 |###
  21203.9 |
  21265.5 |#
  21327.1 |##
  21388.7 |
  21450.3 |
  21511.8 |
  21573.4 |
  (19 below, 12 above range)

memcpy_unrolled (n=160, range 20404.9-21140.4 ns)
  20404.9 |########################################
  20441.7 |#############################
  20478.4 |###############
  20515.2 |##########
  20552.0 |####
  20588.8 |##
  20625.5 |
  20662.3 |
  20699.1 |
  20735.9 |#
  20772.6 |
  20809.4 |
  20846.2 |
  20882.9 |
  20919.7 |
  20956.5 |
  20993.3 |
  21030.0 |
  21066.8 |
  21103.6 |#
  (14 below, 20 above range)

```
