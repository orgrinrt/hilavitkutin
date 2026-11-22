# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (591 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 591.2 ns median
- 2 variants significantly slower than baseline
- Spread: 1.86x (fastest 591.2 ns, slowest 1099.2 ns)

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
| dispatch_direct | 2691ns | 2687ns | 2681ns | 2687ns | 2712ns | base |
| dispatch_runtime | 3188ns | 3185ns | 3165ns | 3185ns | 3218ns | +18.45% |
| dispatch_volatile | 2716ns | 2713ns | 2706ns | 2713ns | 2734ns | +0.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 592ns | 587ns | 600ns | base | 6.916 |
| dispatch_runtime | 1102ns | 1082ns | 1127ns | +86.04% | 3.717 |
| dispatch_volatile | 638ns | 631ns | 647ns | +7.68% | 6.423 |

## Performance model

- Peak throughput: **6.972 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 6.928 | 99.4% |
| dispatch_runtime | 3.726 | 53.4% |
| dispatch_volatile | 6.431 | 92.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2691ns | 2691ns | base |
| dispatch_runtime | 3188ns | 3188ns | +18.45% |
| dispatch_volatile | 2716ns | 2716ns | +0.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 591ns | base | --- | [591, 592] | --- | --- | --- | --- |
| dispatch_runtime | 1099ns | +506.4ns (+85.7%) | [+505, +508]ns | [1097, 1101] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 637ns | +45.0ns (+7.6%) | [+45, +46]ns | [636, 638] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 591ns | +87.4% | +7.7% |
| 1 | 2 | 591ns | +87.5% | +7.7% |
| 1 | 3 | 592ns | +85.6% | +7.7% |
| 1 | 4 | 591ns | +90.7% | +7.7% |
| 1 | 5 | 592ns | +85.6% | +7.6% |
| 1 | 6 | 592ns | +82.0% | +7.9% |
| 1 | 7 | 592ns | +84.8% | +7.6% |
| 1 | 8 | 591ns | +84.9% | +7.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | 0.023 | ok |
| dispatch_runtime | 0.115 | ok |
| dispatch_volatile | -0.239 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.7ns | 592.3ns | 0.4% |  |
| dispatch_runtime | 3.5ns | 1101.9ns | 0.3% |  |
| dispatch_volatile | 3.1ns | 637.7ns | 0.5% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 587.5-599.8 ns)
    587.5 |############
    588.1 |#########
    588.7 |#######
    589.3 |############
    589.9 |################################
    590.6 |###############
    591.2 |########################################
    591.8 |#############
    592.4 |#######################
    593.0 |############
    593.6 |##########
    594.3 |#######
    594.9 |##########
    595.5 |
    596.1 |####
    596.7 |#
    597.3 |######
    598.0 |
    598.6 |#
    599.2 |
  (13 below, 3 above range)

dispatch_runtime (n=160, range 1081.5-1127.0 ns)
   1081.5 |
   1083.8 |####
   1086.1 |###########
   1088.4 |###########
   1090.6 |#################
   1092.9 |########################################
   1095.2 |###############################
   1097.4 |###################################
   1099.7 |###############
   1102.0 |########
   1104.3 |#################
   1106.5 |###############################
   1108.8 |###############
   1111.1 |#############
   1113.4 |####
   1115.6 |
   1117.9 |
   1120.2 |######
   1122.4 |########
   1124.7 |######
  (18 below, 16 above range)

dispatch_volatile (n=160, range 631.4-646.6 ns)
    631.4 |########################################
    632.2 |#######
    632.9 |#########
    633.7 |################
    634.4 |#######################
    635.2 |############################
    636.0 |########################################
    636.7 |################
    637.5 |###################################
    638.2 |################################
    639.0 |##################
    639.8 |#######################
    640.5 |#######################
    641.3 |##
    642.0 |####
    642.8 |##
    643.6 |#######
    644.3 |##
    645.1 |
    645.8 |
  (10 below, 8 above range)

```
