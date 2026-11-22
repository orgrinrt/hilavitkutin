# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_runtime is an outlier: 3.9x slower than the field

dispatch_runtime (9.86 us) is 3.9x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (2.51 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.9x the fastest

Fastest dispatch_direct (2.51 us) to slowest dispatch_runtime (9.86 us): 3.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dispatch_volatile's edge over baseline is significant but tiny (47 ns, 1.86%)

dispatch_volatile differs from baseline dispatch_direct by 47 ns (1.86%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 2511.2 ns median
- 2 variants significantly slower than baseline
- Spread: 3.93x (fastest 2511.2 ns, slowest 9863.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x40ffee15d34db33f |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 4626ns | 4601ns | 4589ns | 4609ns | 4712ns | base |
| dispatch_runtime | 11935ns | 11954ns | 10696ns | 12026ns | 12899ns | +157.99% |
| dispatch_volatile | 4674ns | 4635ns | 4621ns | 4647ns | 4808ns | +1.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 2524ns | 2506ns | 2573ns | base | 6.492 |
| dispatch_runtime | 9829ns | 8568ns | 10796ns | +289.44% | 1.667 |
| dispatch_volatile | 2580ns | 2551ns | 2654ns | +2.21% | 6.351 |

## Performance model

- Peak throughput: **6.538 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 6.524 | 99.8% |
| dispatch_runtime | 1.661 | 25.4% |
| dispatch_volatile | 6.405 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 4626ns | 4626ns | base |
| dispatch_runtime | 11935ns | 11935ns | +157.99% |
| dispatch_volatile | 4674ns | 4674ns | +1.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 2511ns | base | --- | [2510, 2512] | --- | --- | --- | --- |
| dispatch_runtime | 9863ns | +7343.6ns (+292.4%) | [+7149, +7708]ns | [9672, 10217] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 2558ns | +46.7ns (+1.9%) | [+45, +48]ns | [2557, 2561] | YES | 0.0000 | 0.0000 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 2512ns | +214.5% | +2.0% |
| 1 | 2 | 2510ns | +340.9% | +3.0% |
| 1 | 3 | 2512ns | +270.5% | +1.7% |
| 1 | 4 | 2511ns | +319.1% | +1.9% |
| 1 | 5 | 2509ns | +314.8% | +3.9% |
| 1 | 6 | 2510ns | +284.1% | +1.9% |
| 1 | 7 | 2512ns | +272.4% | +1.7% |
| 1 | 8 | 2520ns | +313.8% | +1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | 0.146 | ok |
| dispatch_runtime | -0.474 | moderate- |
| dispatch_volatile | -0.281 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.9ns | 2523.9ns | 0.1% |  |
| dispatch_runtime | 3.3ns | 9828.8ns | 0.0% |  |
| dispatch_volatile | 3.4ns | 2579.6ns | 0.1% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 2505.9-2572.9 ns)
   2505.9 |########################################
   2509.2 |#####################################
   2512.6 |########
   2515.9 |####
   2519.3 |####
   2522.6 |###
   2526.0 |##
   2529.3 |#
   2532.7 |#
   2536.0 |
   2539.4 |####
   2542.7 |
   2546.1 |
   2549.4 |
   2552.8 |####
   2556.1 |####
   2559.5 |###
   2562.8 |
   2566.2 |
   2569.5 |
  (15 below, 10 above range)

dispatch_runtime (n=160, range 8567.7-10796.1 ns)
   8567.7 |
   8679.1 |
   8790.6 |
   8902.0 |
   9013.4 |######
   9124.8 |###########
   9236.2 |####################
   9347.7 |#################
   9459.1 |########
   9570.5 |########################
   9681.9 |############
   9793.3 |######
   9904.8 |####
  10016.2 |####
  10127.6 |########
  10239.0 |################
  10350.5 |########################################
  10461.9 |################
  10573.3 |################
  10684.7 |
  (15 below, 12 above range)

dispatch_volatile (n=160, range 2551.3-2653.6 ns)
   2551.3 |########################################
   2556.4 |#########################
   2561.5 |#######
   2566.6 |##
   2571.7 |###
   2576.8 |
   2582.0 |
   2587.1 |
   2592.2 |###
   2597.3 |####
   2602.4 |######
   2607.5 |##
   2612.6 |
   2617.8 |
   2622.9 |#
   2628.0 |
   2633.1 |
   2638.2 |
   2643.3 |#
   2648.5 |###
  (12 below, 12 above range)

```
