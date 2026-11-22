# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_runtime is an outlier: 4.1x slower than the field

dispatch_runtime (10.33 us) is 4.1x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (2.51 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.1x the fastest

Fastest dispatch_direct (2.51 us) to slowest dispatch_runtime (10.33 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dispatch_volatile's edge over baseline is significant but tiny (45 ns, 1.79%)

dispatch_volatile differs from baseline dispatch_direct by 45 ns (1.79%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 2509.6 ns median
- 2 variants significantly slower than baseline
- Spread: 4.12x (fastest 2509.6 ns, slowest 10329.8 ns)

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
| dispatch_direct | 4625ns | 4611ns | 4605ns | 4612ns | 4685ns | base |
| dispatch_runtime | 12383ns | 12429ns | 11825ns | 12432ns | 12792ns | +167.74% |
| dispatch_volatile | 4652ns | 4637ns | 4629ns | 4637ns | 4717ns | +0.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 2517ns | 2506ns | 2549ns | base | 6.510 |
| dispatch_runtime | 10289ns | 9739ns | 10687ns | +308.82% | 1.592 |
| dispatch_volatile | 2564ns | 2550ns | 2602ns | +1.85% | 6.391 |

## Performance model

- Peak throughput: **6.539 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 6.529 | 99.8% |
| dispatch_runtime | 1.586 | 24.3% |
| dispatch_volatile | 6.413 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 4625ns | 4625ns | base |
| dispatch_runtime | 12383ns | 12383ns | +167.74% |
| dispatch_volatile | 4652ns | 4652ns | +0.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 2510ns | base | --- | [2509, 2511] | --- | --- | --- | --- |
| dispatch_runtime | 10330ns | +7823.8ns (+311.8%) | [+7772, +7900]ns | [10289, 10417] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 2555ns | +45.0ns (+1.8%) | [+44, +46]ns | [2555, 2556] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 2509ns | +326.2% | +1.9% |
| 1 | 2 | 2509ns | +323.1% | +1.8% |
| 1 | 3 | 2509ns | +302.6% | +1.8% |
| 1 | 4 | 2511ns | +286.3% | +1.7% |
| 1 | 5 | 2510ns | +310.1% | +1.8% |
| 1 | 6 | 2511ns | +314.9% | +2.0% |
| 1 | 7 | 2508ns | +310.3% | +1.8% |
| 1 | 8 | 2510ns | +316.1% | +1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.134 | ok |
| dispatch_runtime | 0.275 | moderate+ |
| dispatch_volatile | -0.123 | ok |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.8ns | 2516.9ns | 0.1% |  |
| dispatch_runtime | 3.5ns | 10289.5ns | 0.0% |  |
| dispatch_volatile | 3.4ns | 2563.5ns | 0.1% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 2505.6-2548.6 ns)
   2505.6 |##################
   2507.7 |########################################
   2509.9 |###################
   2512.0 |#################
   2514.2 |##
   2516.3 |
   2518.5 |##
   2520.6 |#
   2522.8 |#####
   2524.9 |
   2527.1 |#
   2529.2 |#
   2531.4 |
   2533.5 |
   2535.7 |
   2537.8 |
   2540.0 |
   2542.1 |##
   2544.3 |
   2546.4 |
  (15 below, 7 above range)

dispatch_runtime (n=160, range 9738.7-10687.0 ns)
   9738.7 |##
   9786.1 |####
   9833.5 |#########
   9880.9 |##
   9928.4 |####
   9975.8 |
  10023.2 |###########
  10070.6 |################
  10118.0 |###########
  10165.4 |#####################
  10212.8 |################
  10260.3 |########################################
  10307.7 |###########
  10355.1 |##################
  10402.5 |#########################
  10449.9 |#######################
  10497.3 |############################
  10544.7 |################
  10592.2 |##############
  10639.6 |#######################
  (17 below, 14 above range)

dispatch_volatile (n=160, range 2549.6-2601.7 ns)
   2549.6 |##################
   2552.2 |########################################
   2554.8 |###################################
   2557.4 |##################
   2560.0 |#####
   2562.6 |###
   2565.2 |####
   2567.8 |##
   2570.4 |
   2573.0 |#
   2575.7 |
   2578.3 |##
   2580.9 |
   2583.5 |#
   2586.1 |##
   2588.7 |
   2591.3 |
   2593.9 |
   2596.5 |
   2599.1 |
  (16 below, 13 above range)

```
