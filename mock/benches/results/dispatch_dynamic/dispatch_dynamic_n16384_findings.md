# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_runtime is an outlier: 3.6x slower than the field

dispatch_runtime (9.35 us) is 3.6x the fastest (2.60 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (2.60 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.6x the fastest

Fastest dispatch_direct (2.60 us) to slowest dispatch_runtime (9.35 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 2597.5 ns median
- 2 variants significantly slower than baseline
- Spread: 3.60x (fastest 2597.5 ns, slowest 9351.5 ns)

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
| dispatch_direct | 4756ns | 4763ns | 4597ns | 4747ns | 4942ns | base |
| dispatch_runtime | 11452ns | 11431ns | 10525ns | 11425ns | 12460ns | +140.79% |
| dispatch_volatile | 4824ns | 4790ns | 4652ns | 4820ns | 5006ns | +1.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 2595ns | 2508ns | 2697ns | base | 6.314 |
| dispatch_runtime | 9337ns | 8433ns | 10296ns | +259.81% | 1.755 |
| dispatch_volatile | 2660ns | 2566ns | 2754ns | +2.50% | 6.160 |

## Performance model

- Peak throughput: **6.533 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 6.308 | 96.5% |
| dispatch_runtime | 1.752 | 26.8% |
| dispatch_volatile | 6.193 | 94.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 4756ns | 4756ns | base |
| dispatch_runtime | 11452ns | 11452ns | +140.79% |
| dispatch_volatile | 4824ns | 4824ns | +1.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 2598ns | base | --- | [2595, 2602] | --- | --- | --- | --- |
| dispatch_runtime | 9351ns | +6750.2ns (+259.9%) | [+6618, +6902]ns | [9182, 9454] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 2645ns | +65.2ns (+2.5%) | [+46, +110]ns | [2643, 2648] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 2600ns | +259.6% | +5.6% |
| 1 | 2 | 2617ns | +288.6% | +1.0% |
| 1 | 3 | 2579ns | +248.0% | +1.8% |
| 1 | 4 | 2525ns | +245.2% | +8.3% |
| 1 | 5 | 2510ns | +288.8% | +5.3% |
| 1 | 6 | 2532ns | +248.9% | +4.5% |
| 1 | 7 | 2691ns | +264.4% | -2.0% |
| 1 | 8 | 2687ns | +215.0% | +1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | 0.426 | moderate+ |
| dispatch_runtime | -0.454 | moderate- |
| dispatch_volatile | -0.296 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 3.0ns | 2594.8ns | 0.1% |  |
| dispatch_runtime | 3.2ns | 9336.5ns | 0.0% |  |
| dispatch_volatile | 3.6ns | 2659.6ns | 0.1% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 2507.8-2697.2 ns)
   2507.8 |########################################
   2517.3 |###
   2526.8 |##
   2536.2 |######
   2545.7 |##
   2555.2 |##
   2564.6 |###
   2574.1 |
   2583.6 |##
   2593.0 |######################################
   2602.5 |############
   2612.0 |#####
   2621.5 |#####
   2630.9 |##
   2640.4 |
   2649.9 |
   2659.3 |##
   2668.8 |
   2678.3 |#######
   2687.7 |######################################
  (16 below, 6 above range)

dispatch_runtime (n=160, range 8433.3-10296.0 ns)
   8433.3 |#######
   8526.5 |##
   8619.6 |#######################
   8712.7 |########################################
   8805.9 |#########
   8899.0 |#########
   8992.1 |#####################
   9085.3 |#######
   9178.4 |##
   9271.6 |###################################
   9364.7 |#####################
   9457.8 |
   9551.0 |##################
   9644.1 |#######################
   9737.2 |#####################
   9830.4 |###########
   9923.5 |#########
  10016.6 |##############
  10109.8 |################
  10202.9 |##
  (17 below, 17 above range)

dispatch_volatile (n=160, range 2566.3-2754.0 ns)
   2566.3 |##
   2575.7 |#
   2585.0 |###
   2594.4 |#####
   2603.8 |##########
   2613.2 |#
   2622.6 |##
   2632.0 |############
   2641.3 |########################################
   2650.7 |#######
   2660.1 |####
   2669.5 |#
   2678.9 |#
   2688.3 |
   2697.7 |#
   2707.0 |##
   2716.4 |
   2725.8 |#######
   2735.2 |########################
   2744.6 |##############
  (22 below, 6 above range)

```
