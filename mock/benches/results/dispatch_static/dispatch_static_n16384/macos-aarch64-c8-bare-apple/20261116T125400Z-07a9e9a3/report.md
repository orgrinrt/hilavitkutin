# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 2.51 us to slowest 2.51 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (43 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (2.51 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 2508.3 ns median
- Spread: 1.00x (fastest 2508.3 ns, slowest 2508.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Static dispatch shapes: direct vs sealed-trait vs const fn pointer |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x7eedfacedeadbeef |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 4622ns | 4606ns | 4599ns | 4607ns | 4689ns | base |
| dispatch_direct | 4615ns | 4608ns | 4601ns | 4608ns | 4649ns | -0.16% |
| dispatch_trait | 4611ns | 4606ns | 4599ns | 4606ns | 4640ns | -0.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 2517ns | 2504ns | 2555ns | base | 6.510 |
| dispatch_direct | 2512ns | 2505ns | 2531ns | -0.18% | 6.521 |
| dispatch_trait | 2511ns | 2504ns | 2527ns | -0.22% | 6.524 |

## Performance model

- Peak throughput: **6.543 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 6.532 | 99.8% |
| dispatch_direct | 6.532 | 99.8% |
| dispatch_trait | 6.531 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 4622ns | 4622ns | base |
| dispatch_direct | 4615ns | 4615ns | -0.16% |
| dispatch_trait | 4611ns | 4611ns | -0.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2508ns | base | --- | [2508, 2509] | --- | --- | --- | --- |
| dispatch_direct | 2508ns | no significant difference | [-0, +1]ns | [2508, 2509] | no | 0.8066 | 0.6278 | 7 |
| dispatch_trait | 2509ns | no significant difference | [-0, +0]ns | [2508, 2510] | no | 0.8066 | 0.8066 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 2509ns | -0.1% | -0.1% |
| 1 | 2 | 2509ns | -0.0% | -0.0% |
| 1 | 3 | 2508ns | +0.1% | +0.0% |
| 1 | 4 | 2508ns | +0.0% | +0.0% |
| 1 | 5 | 2508ns | +0.0% | +0.1% |
| 1 | 6 | 2509ns | +0.0% | -0.0% |
| 1 | 7 | 2508ns | -0.0% | +0.1% |
| 1 | 8 | 2508ns | +0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.146 | ok |
| dispatch_direct | -0.010 | ok |
| dispatch_trait | -0.444 | moderate- |

**Consistency summary:**

- **dispatch_direct**: won 0/8, lost 0/8
- **dispatch_trait**: won 0/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 3.1ns | 2516.9ns | 0.1% |  |
| dispatch_direct | 2.8ns | 2512.5ns | 0.1% |  |
| dispatch_trait | 3.0ns | 2511.2ns | 0.1% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 2504.2-2554.6 ns)
   2504.2 |########################################
   2506.7 |#################################
   2509.2 |#########################
   2511.8 |##########
   2514.3 |###
   2516.8 |###
   2519.3 |
   2521.8 |
   2524.4 |
   2526.9 |
   2529.4 |
   2531.9 |
   2534.5 |#
   2537.0 |
   2539.5 |
   2542.0 |
   2544.5 |
   2547.1 |
   2549.6 |
   2552.1 |
  (8 below, 6 above range)

dispatch_direct (n=160, range 2504.6-2531.5 ns)
   2504.6 |#################################
   2506.0 |########################################
   2507.3 |####################################
   2508.6 |##########################
   2510.0 |#######################
   2511.3 |##############################
   2512.7 |###########
   2514.0 |#####
   2515.4 |
   2516.7 |#
   2518.0 |#
   2519.4 |#
   2520.7 |
   2522.1 |
   2523.4 |
   2524.8 |#
   2526.1 |
   2527.4 |###
   2528.8 |###
   2530.1 |
  (16 below, 12 above range)

dispatch_trait (n=160, range 2504.1-2526.8 ns)
   2504.1 |#################################
   2505.2 |#################################
   2506.3 |######################
   2507.5 |###############################
   2508.6 |########################################
   2509.7 |######################################
   2510.9 |###############
   2512.0 |##########
   2513.1 |############
   2514.3 |###
   2515.4 |
   2516.6 |
   2517.7 |
   2518.8 |
   2520.0 |
   2521.1 |
   2522.2 |
   2523.4 |
   2524.5 |
   2525.6 |#
  (10 below, 11 above range)

```
