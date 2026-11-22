# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_volatile shows warm-up / thermal drift (autocorr +0.59)

dispatch_volatile's per-pass series has lag-1 autocorrelation +0.59, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (593 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 592.7 ns median
- 2 variants significantly slower than baseline
- Spread: 1.86x (fastest 592.7 ns, slowest 1101.2 ns)

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
| dispatch_direct | 2694ns | 2680ns | 2669ns | 2687ns | 2744ns | base |
| dispatch_runtime | 3197ns | 3179ns | 3157ns | 3191ns | 3258ns | +18.66% |
| dispatch_volatile | 2733ns | 2745ns | 2697ns | 2732ns | 2772ns | +1.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 595ns | 588ns | 609ns | base | 6.883 |
| dispatch_runtime | 1108ns | 1089ns | 1139ns | +86.16% | 3.697 |
| dispatch_volatile | 645ns | 634ns | 658ns | +8.46% | 6.346 |

## Performance model

- Peak throughput: **6.969 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 6.911 | 99.2% |
| dispatch_runtime | 3.720 | 53.4% |
| dispatch_volatile | 6.341 | 91.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2694ns | 2694ns | base |
| dispatch_runtime | 3197ns | 3197ns | +18.66% |
| dispatch_volatile | 2733ns | 2733ns | +1.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 593ns | base | --- | [591, 594] | --- | --- | --- | --- |
| dispatch_runtime | 1101ns | +509.2ns (+85.9%) | [+507, +511]ns | [1099, 1104] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 646ns | +48.7ns (+8.2%) | [+48, +52]ns | [643, 648] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 602ns | +85.7% | +7.8% |
| 1 | 2 | 590ns | +86.6% | +10.2% |
| 1 | 3 | 593ns | +85.4% | +9.6% |
| 1 | 4 | 592ns | +94.0% | +9.8% |
| 1 | 5 | 590ns | +85.2% | +8.2% |
| 1 | 6 | 591ns | +84.3% | +8.1% |
| 1 | 7 | 600ns | +82.8% | +6.4% |
| 1 | 8 | 592ns | +86.4% | +8.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.239 | moderate- |
| dispatch_runtime | -0.206 | moderate- |
| dispatch_volatile | 0.595 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.9ns | 595.1ns | 0.5% |  |
| dispatch_runtime | 3.6ns | 1107.8ns | 0.3% |  |
| dispatch_volatile | 3.4ns | 645.5ns | 0.5% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 587.7-608.5 ns)
    587.7 |##############
    588.8 |########################################
    589.8 |################################
    590.9 |##############
    591.9 |################
    592.9 |###################
    594.0 |###################
    595.0 |########
    596.0 |#########
    597.1 |######
    598.1 |#
    599.2 |########
    600.2 |#########
    601.2 |###
    602.3 |############
    603.3 |######
    604.4 |
    605.4 |###
    606.4 |###
    607.5 |####
  (10 below, 5 above range)

dispatch_runtime (n=160, range 1088.9-1139.2 ns)
   1088.9 |################################
   1091.4 |###################################
   1093.9 |##############################
   1096.4 |################################
   1098.9 |###################################
   1101.4 |###########################
   1104.0 |##########
   1106.5 |#######
   1109.0 |#####
   1111.5 |#######
   1114.0 |########################################
   1116.5 |####################
   1119.0 |###############
   1121.6 |##########
   1124.1 |
   1126.6 |##
   1129.1 |##
   1131.6 |##
   1134.1 |
   1136.6 |##
  (15 below, 18 above range)

dispatch_volatile (n=160, range 634.2-657.8 ns)
    634.2 |####################
    635.4 |##############
    636.5 |####################
    637.7 |############################
    638.9 |##############
    640.1 |#########################
    641.3 |###########
    642.4 |######################
    643.6 |######################
    644.8 |########
    646.0 |#########################
    647.2 |########################################
    648.4 |##############
    649.5 |########################################
    650.7 |###############################
    651.9 |######################
    653.1 |#################
    654.3 |#################
    655.4 |##
    656.6 |
  (14 below, 6 above range)

```

## Diagnostics

- **dispatch_volatile**: autocorrelation=0.59 (measurement drift or warm-up artifact)
