# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (2.57 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader dispatch_const_ptr vs stability leader dispatch_trait (+1% speed for 1.5x steadier)

dispatch_const_ptr is fastest (2.57 us, CV 3.4%); dispatch_trait gives up 1.4% median for 1.5x lower variance (CV 2.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 4.4% of the fastest

All 3 variants sit between 2.57 us and 2.68 us - a 4.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### dispatch_direct's edge over baseline is significant but tiny (8 ns, 0.30%)

dispatch_direct differs from baseline dispatch_const_ptr by 8 ns (0.30%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 2570.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 2570.8 ns, slowest 2683.9 ns)

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
| dispatch_const_ptr | 4760ns | 4698ns | 4594ns | 4752ns | 4954ns | base |
| dispatch_direct | 4809ns | 4918ns | 4604ns | 4826ns | 4961ns | +1.01% |
| dispatch_trait | 4820ns | 4798ns | 4667ns | 4824ns | 4959ns | +1.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 2597ns | 2506ns | 2701ns | base | 6.309 |
| dispatch_direct | 2624ns | 2508ns | 2709ns | +1.03% | 6.245 |
| dispatch_trait | 2627ns | 2544ns | 2699ns | +1.17% | 6.237 |

## Performance model

- Peak throughput: **6.537 Gops/s** (dispatch_const_ptr; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 6.373 | 97.5% |
| dispatch_direct | 6.104 | 93.4% |
| dispatch_trait | 6.285 | 96.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 4760ns | 4760ns | base |
| dispatch_direct | 4809ns | 4809ns | +1.01% |
| dispatch_trait | 4820ns | 4820ns | +1.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2571ns | base | --- | [2523, 2648] | --- | --- | --- | --- |
| dispatch_direct | 2684ns | +7.7ns (+0.3%) | [+0, +64]ns | [2631, 2687] | YES (adj: no) | 0.0778 | 0.0389 | 1 |
| dispatch_trait | 2607ns | no significant difference | [-2, +82]ns | [2600, 2635] | no | 0.1506 | 0.1506 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 2512ns | +7.0% | +3.3% |
| 1 | 2 | 2570ns | +3.9% | +4.6% |
| 1 | 3 | 2527ns | +6.6% | +6.6% |
| 1 | 4 | 2689ns | -0.1% | -3.5% |
| 1 | 5 | 2509ns | +7.2% | +5.0% |
| 1 | 6 | 2582ns | +3.9% | -0.3% |
| 1 | 7 | 2689ns | -6.6% | -3.4% |
| 1 | 8 | 2688ns | -6.5% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.047 | ok |
| dispatch_direct | 0.453 | moderate+ |
| dispatch_trait | -0.052 | ok |

**Consistency summary:**

- **dispatch_direct**: won 3/8, lost 5/8
- **dispatch_trait**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 3.0ns | 2596.7ns | 0.1% |  |
| dispatch_direct | 2.9ns | 2623.5ns | 0.1% |  |
| dispatch_trait | 3.0ns | 2627.0ns | 0.1% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 2506.3-2701.4 ns)
   2506.3 |########################################
   2516.1 |###
   2525.9 |#
   2535.6 |##
   2545.4 |
   2555.1 |#
   2564.9 |##
   2574.6 |
   2584.4 |
   2594.1 |###
   2603.9 |
   2613.6 |###
   2623.4 |
   2633.2 |
   2642.9 |
   2652.7 |##
   2662.4 |
   2672.2 |
   2681.9 |############################
   2691.7 |###############
  (15 below, 7 above range)

dispatch_direct (n=160, range 2507.5-2709.3 ns)
   2507.5 |###########################
   2517.6 |#####
   2527.7 |##
   2537.8 |###
   2547.9 |#
   2558.0 |###
   2568.1 |
   2578.2 |
   2588.2 |####
   2598.3 |####
   2608.4 |
   2618.5 |#
   2628.6 |####
   2638.7 |
   2648.8 |##
   2658.9 |##
   2669.0 |##
   2679.0 |##################################
   2689.1 |########################################
   2699.2 |####
  (16 below, 6 above range)

dispatch_trait (n=160, range 2543.6-2698.9 ns)
   2543.6 |###
   2551.3 |###
   2559.1 |
   2566.9 |
   2574.7 |
   2582.4 |#
   2590.2 |########################################
   2598.0 |########################
   2605.7 |####
   2613.5 |####
   2621.3 |#####
   2629.0 |
   2636.8 |##
   2644.6 |###
   2652.3 |
   2660.1 |###
   2667.9 |##
   2675.6 |#
   2683.4 |#############################
   2691.2 |##########################
  (16 below, 8 above range)

```
