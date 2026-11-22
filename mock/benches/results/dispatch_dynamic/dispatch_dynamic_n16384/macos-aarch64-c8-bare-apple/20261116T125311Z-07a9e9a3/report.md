# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_runtime is an outlier: 4.3x slower than the field

dispatch_runtime (10.74 us) is 4.3x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dispatch_direct shows alternating (throttle bounce) (autocorr -0.61)

dispatch_direct's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (2.51 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.3x the fastest

Fastest dispatch_direct (2.51 us) to slowest dispatch_runtime (10.74 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dispatch_volatile's edge over baseline is significant but tiny (45 ns, 1.81%)

dispatch_volatile differs from baseline dispatch_direct by 45 ns (1.81%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 2509.2 ns median
- 2 variants significantly slower than baseline
- Spread: 4.28x (fastest 2509.2 ns, slowest 10742.3 ns)

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
| dispatch_direct | 4618ns | 4609ns | 4602ns | 4609ns | 4660ns | base |
| dispatch_runtime | 12897ns | 12826ns | 12603ns | 12884ns | 13229ns | +179.28% |
| dispatch_volatile | 4649ns | 4638ns | 4630ns | 4639ns | 4697ns | +0.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 2515ns | 2505ns | 2541ns | base | 6.515 |
| dispatch_runtime | 10805ns | 10508ns | 11132ns | +329.62% | 1.516 |
| dispatch_volatile | 2560ns | 2550ns | 2585ns | +1.79% | 6.400 |

## Performance model

- Peak throughput: **6.540 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 6.530 | 99.8% |
| dispatch_runtime | 1.525 | 23.3% |
| dispatch_volatile | 6.413 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 4618ns | 4618ns | base |
| dispatch_runtime | 12897ns | 12897ns | +179.28% |
| dispatch_volatile | 4649ns | 4649ns | +0.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 2509ns | base | --- | [2509, 2510] | --- | --- | --- | --- |
| dispatch_runtime | 10742ns | +8227.5ns (+327.9%) | [+8199, +8263]ns | [10710, 10779] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 2555ns | +45.5ns (+1.8%) | [+45, +46]ns | [2554, 2556] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 2509ns | +316.6% | +1.8% |
| 1 | 2 | 2510ns | +339.9% | +1.8% |
| 1 | 3 | 2509ns | +323.6% | +1.9% |
| 1 | 4 | 2509ns | +329.0% | +1.8% |
| 1 | 5 | 2510ns | +324.4% | +1.8% |
| 1 | 6 | 2509ns | +326.8% | +1.8% |
| 1 | 7 | 2510ns | +340.9% | +1.8% |
| 1 | 8 | 2509ns | +337.7% | +1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.613 | HIGH- (thermal bounce) |
| dispatch_runtime | -0.223 | moderate- |
| dispatch_volatile | -0.491 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.8ns | 2514.9ns | 0.1% |  |
| dispatch_runtime | 3.3ns | 10804.6ns | 0.0% |  |
| dispatch_volatile | 3.2ns | 2559.9ns | 0.1% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 2505.3-2540.7 ns)
   2505.3 |#################################
   2507.1 |########################################
   2508.9 |#####################################
   2510.6 |#########################
   2512.4 |#############
   2514.2 |#######
   2515.9 |##
   2517.7 |
   2519.5 |
   2521.2 |
   2523.0 |###
   2524.8 |##
   2526.5 |
   2528.3 |###
   2530.1 |##
   2531.8 |#
   2533.6 |
   2535.4 |
   2537.1 |
   2538.9 |#
  (11 below, 5 above range)

dispatch_runtime (n=160, range 10507.6-11132.3 ns)
  10507.6 |
  10538.9 |#
  10570.1 |#####
  10601.3 |#####################
  10632.6 |##############################
  10663.8 |#######
  10695.0 |########################################
  10726.3 |####################
  10757.5 |############
  10788.7 |###
  10820.0 |#####
  10851.2 |
  10882.4 |#
  10913.7 |#######
  10944.9 |############
  10976.1 |################
  11007.4 |#####################
  11038.6 |##################
  11069.8 |############
  11101.1 |#
  (16 below, 11 above range)

dispatch_volatile (n=160, range 2549.6-2584.7 ns)
   2549.6 |#############################
   2551.4 |############################
   2553.2 |########################################
   2554.9 |########################################
   2556.7 |##################################
   2558.4 |#
   2560.2 |########
   2561.9 |#
   2563.7 |#######
   2565.4 |#
   2567.2 |##
   2568.9 |#
   2570.7 |#
   2572.4 |####
   2574.2 |#
   2576.0 |
   2577.7 |#
   2579.5 |#
   2581.2 |#
   2583.0 |
  (10 below, 9 above range)

```
