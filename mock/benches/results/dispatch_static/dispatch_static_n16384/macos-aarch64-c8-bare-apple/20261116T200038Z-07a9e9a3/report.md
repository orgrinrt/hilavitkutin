# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (11 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.1% of the fastest

All 3 variants sit between 2.51 us and 2.51 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### dispatch_direct's edge over baseline is significant but tiny (-1 ns, 0.05%)

dispatch_direct differs from baseline dispatch_const_ptr by -1 ns (0.05%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: dispatch_direct** at 2508.8 ns median (-0.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 2508.8 ns, slowest 2510.4 ns)

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
| dispatch_const_ptr | 4644ns | 4608ns | 4601ns | 4614ns | 4779ns | base |
| dispatch_direct | 4614ns | 4608ns | 4602ns | 4608ns | 4646ns | -0.65% |
| dispatch_trait | 4645ns | 4612ns | 4603ns | 4615ns | 4777ns | +0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 2528ns | 2505ns | 2601ns | base | 6.481 |
| dispatch_direct | 2511ns | 2504ns | 2526ns | -0.66% | 6.524 |
| dispatch_trait | 2525ns | 2505ns | 2589ns | -0.12% | 6.488 |

## Performance model

- Peak throughput: **6.542 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 6.527 | 99.8% |
| dispatch_direct | 6.531 | 99.8% |
| dispatch_trait | 6.526 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 4644ns | 4644ns | base |
| dispatch_direct | 4614ns | 4614ns | -0.65% |
| dispatch_trait | 4645ns | 4645ns | +0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2510ns | base | --- | [2510, 2511] | --- | --- | --- | --- |
| dispatch_direct | 2509ns | -1.2ns (-0.0%) | [-2, -0]ns | [2508, 2510] | YES | 0.0032 | 0.0016 | 6 |
| dispatch_trait | 2510ns | no significant difference | [-1, +1]ns | [2509, 2511] | no | 0.5728 | 0.5728 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 2622ns | -4.3% | -0.7% |
| 1 | 2 | 2510ns | -0.0% | +0.0% |
| 1 | 3 | 2510ns | -0.0% | -0.0% |
| 1 | 4 | 2509ns | +0.1% | +0.0% |
| 1 | 5 | 2509ns | -0.0% | +0.1% |
| 1 | 6 | 2508ns | +0.1% | +0.1% |
| 1 | 7 | 2513ns | -0.2% | -0.2% |
| 1 | 8 | 2510ns | -0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.019 | ok |
| dispatch_direct | -0.089 | ok |
| dispatch_trait | -0.018 | ok |

**Consistency summary:**

- **dispatch_direct**: won 2/8, lost 0/8
- **dispatch_trait**: won 2/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 3.0ns | 2528.1ns | 0.1% |  |
| dispatch_direct | 2.9ns | 2511.5ns | 0.1% |  |
| dispatch_trait | 2.9ns | 2525.2ns | 0.1% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 2504.8-2600.9 ns)
   2504.8 |########################################
   2509.6 |#####################################
   2514.4 |#####
   2519.2 |
   2524.0 |#
   2528.8 |
   2533.6 |#
   2538.4 |
   2543.2 |
   2548.0 |###
   2552.9 |#
   2557.7 |#
   2562.5 |
   2567.3 |
   2572.1 |
   2576.9 |#
   2581.7 |
   2586.5 |
   2591.3 |#
   2596.1 |##
  (12 below, 14 above range)

dispatch_direct (n=160, range 2504.3-2526.1 ns)
   2504.3 |################
   2505.4 |#########################
   2506.5 |###################
   2507.6 |################################
   2508.7 |########################################
   2509.8 |######################
   2510.9 |###########################
   2511.9 |############
   2513.0 |###
   2514.1 |########
   2515.2 |#
   2516.3 |#
   2517.4 |
   2518.5 |
   2519.6 |
   2520.7 |
   2521.8 |
   2522.8 |#
   2523.9 |###
   2525.0 |
  (15 below, 11 above range)

dispatch_trait (n=160, range 2504.6-2589.1 ns)
   2504.6 |########################################
   2508.8 |########################################
   2513.1 |#############
   2517.3 |###
   2521.5 |##
   2525.7 |#
   2530.0 |#
   2534.2 |##
   2538.4 |
   2542.6 |
   2546.9 |
   2551.1 |
   2555.3 |
   2559.5 |
   2563.8 |
   2568.0 |
   2572.2 |
   2576.4 |
   2580.7 |
   2584.9 |
  (15 below, 20 above range)

```
