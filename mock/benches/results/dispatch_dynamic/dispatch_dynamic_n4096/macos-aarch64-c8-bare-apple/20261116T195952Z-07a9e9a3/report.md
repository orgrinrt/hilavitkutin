# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (592 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 592.1 ns median
- 2 variants significantly slower than baseline
- Spread: 1.86x (fastest 592.1 ns, slowest 1103.1 ns)

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
| dispatch_direct | 2717ns | 2688ns | 2682ns | 2698ns | 2811ns | base |
| dispatch_runtime | 3206ns | 3186ns | 3165ns | 3196ns | 3275ns | +17.98% |
| dispatch_volatile | 2757ns | 2760ns | 2710ns | 2749ns | 2828ns | +1.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 598ns | 588ns | 620ns | base | 6.851 |
| dispatch_runtime | 1107ns | 1083ns | 1134ns | +85.19% | 3.699 |
| dispatch_volatile | 647ns | 635ns | 665ns | +8.22% | 6.330 |

## Performance model

- Peak throughput: **6.964 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 6.918 | 99.3% |
| dispatch_runtime | 3.713 | 53.3% |
| dispatch_volatile | 6.348 | 91.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2717ns | 2717ns | base |
| dispatch_runtime | 3206ns | 3206ns | +17.98% |
| dispatch_volatile | 2757ns | 2757ns | +1.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 592ns | base | --- | [592, 593] | --- | --- | --- | --- |
| dispatch_runtime | 1103ns | +509.2ns (+86.0%) | [+508, +512]ns | [1101, 1108] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 645ns | +47.1ns (+8.0%) | [+46, +49]ns | [643, 648] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 592ns | +86.5% | +9.9% |
| 1 | 2 | 591ns | +87.0% | +8.8% |
| 1 | 3 | 591ns | +85.8% | +9.3% |
| 1 | 4 | 591ns | +90.9% | +7.6% |
| 1 | 5 | 619ns | +82.7% | +6.6% |
| 1 | 6 | 595ns | +81.0% | +8.5% |
| 1 | 7 | 598ns | +83.1% | +7.8% |
| 1 | 8 | 591ns | +88.0% | +8.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.092 | ok |
| dispatch_runtime | -0.005 | ok |
| dispatch_volatile | -0.423 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.8ns | 597.9ns | 0.5% |  |
| dispatch_runtime | 3.4ns | 1107.3ns | 0.3% |  |
| dispatch_volatile | 3.2ns | 647.1ns | 0.5% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 588.2-619.7 ns)
    588.2 |############################
    589.8 |########################################
    591.4 |######################################
    592.9 |############
    594.5 |#########
    596.1 |#########
    597.7 |#
    599.2 |###########
    600.8 |#####
    602.4 |######
    604.0 |##
    605.5 |####
    607.1 |#####
    608.7 |
    610.3 |######
    611.9 |##
    613.4 |#
    615.0 |#
    616.6 |#
    618.2 |
  (11 below, 11 above range)

dispatch_runtime (n=160, range 1082.9-1134.3 ns)
   1082.9 |##
   1085.5 |####
   1088.1 |#########
   1090.7 |#####################
   1093.2 |#######################
   1095.8 |#########################
   1098.4 |################################
   1100.9 |################################
   1103.5 |##################
   1106.1 |##
   1108.6 |#########
   1111.2 |################
   1113.8 |##############
   1116.3 |#######
   1118.9 |#######
   1121.5 |###########
   1124.0 |################
   1126.6 |################
   1129.2 |########################################
   1131.7 |#######
  (18 below, 6 above range)

dispatch_volatile (n=160, range 634.5-665.5 ns)
    634.5 |########################################
    636.1 |#########################
    637.6 |#################
    639.2 |#########################
    640.7 |####################
    642.3 |##############################
    643.8 |###############
    645.4 |######################
    646.9 |###########################
    648.5 |####################
    650.0 |###################################
    651.5 |###############
    653.1 |##########
    654.6 |##########
    656.2 |#######
    657.7 |##########
    659.3 |############
    660.8 |
    662.4 |##
    663.9 |#####
  (11 below, 9 above range)

```
