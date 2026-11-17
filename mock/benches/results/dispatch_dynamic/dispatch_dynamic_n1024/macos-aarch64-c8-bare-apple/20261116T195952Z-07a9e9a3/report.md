# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 44% faster than the next best (dispatch_volatile)

dispatch_direct (111 ns) leads dispatch_volatile (160 ns) by 44%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 2.6x slower than the field

dispatch_runtime (288 ns) is 2.6x the fastest (111 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (111 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 111.2 ns median
- 2 variants significantly slower than baseline
- Spread: 2.59x (fastest 111.2 ns, slowest 287.9 ns)

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
| dispatch_direct | 2236ns | 2236ns | 2205ns | 2230ns | 2284ns | base |
| dispatch_runtime | 2389ns | 2367ns | 2349ns | 2377ns | 2468ns | +6.88% |
| dispatch_volatile | 2280ns | 2277ns | 2231ns | 2276ns | 2342ns | +1.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 111ns | 106ns | 116ns | base | 9.212 |
| dispatch_runtime | 290ns | 277ns | 306ns | +161.01% | 3.529 |
| dispatch_volatile | 160ns | 154ns | 169ns | +44.32% | 6.383 |

## Performance model

- Peak throughput: **9.628 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 9.209 | 95.6% |
| dispatch_runtime | 3.557 | 36.9% |
| dispatch_volatile | 6.416 | 66.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2236ns | 2236ns | base |
| dispatch_runtime | 2389ns | 2389ns | +6.88% |
| dispatch_volatile | 2280ns | 2280ns | +1.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 111ns | base | --- | [111, 112] | --- | --- | --- | --- |
| dispatch_runtime | 288ns | +177.1ns (+159.3%) | [+173, +180]ns | [285, 292] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 160ns | +48.5ns (+43.7%) | [+48, +50]ns | [159, 160] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 111ns | +149.5% | +41.8% |
| 1 | 2 | 110ns | +157.1% | +43.7% |
| 1 | 3 | 113ns | +150.8% | +42.1% |
| 1 | 4 | 110ns | +177.8% | +45.7% |
| 1 | 5 | 112ns | +159.5% | +39.8% |
| 1 | 6 | 112ns | +164.0% | +42.3% |
| 1 | 7 | 113ns | +148.9% | +43.7% |
| 1 | 8 | 110ns | +174.2% | +48.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.441 | moderate- |
| dispatch_runtime | -0.111 | ok |
| dispatch_volatile | 0.343 | moderate+ |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.8ns | 111.2ns | 2.5% |  |
| dispatch_runtime | 3.7ns | 290.1ns | 1.3% |  |
| dispatch_volatile | 3.4ns | 160.4ns | 2.1% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 106.4-115.7 ns)
    106.4 |#########
    106.8 |#####
    107.3 |###############
    107.8 |###
    108.2 |###############
    108.7 |
    109.2 |##########################
    109.6 |###########
    110.1 |#############
    110.6 |###################
    111.0 |###############
    111.5 |###################
    112.0 |#########
    112.5 |########################################
    112.9 |###################
    113.4 |#####
    113.9 |#############
    114.3 |#######
    114.8 |###
    115.3 |#####
  (12 below, 12 above range)

dispatch_runtime (n=160, range 276.8-306.0 ns)
    276.8 |#######################
    278.3 |##############################
    279.7 |#######################
    281.2 |####################################
    282.7 |#################################
    284.1 |####################################
    285.6 |################
    287.0 |####################
    288.5 |##########
    289.9 |################
    291.4 |#############
    292.9 |##############################
    294.3 |#############
    295.8 |################
    297.2 |################
    298.7 |####################
    300.1 |########################################
    301.6 |###
    303.1 |##########
    304.5 |#######################
  (16 below, 14 above range)

dispatch_volatile (n=160, range 154.0-169.0 ns)
    154.0 |##############
    154.8 |######################
    155.5 |######################
    156.3 |####################
    157.0 |########################################
    157.7 |##################################
    158.5 |#####################################
    159.2 |###########
    160.0 |#####################################
    160.7 |######################
    161.5 |#########################
    162.2 |###############################
    163.0 |########
    163.7 |#########################
    164.5 |##
    165.2 |########
    166.0 |##############
    166.7 |#####
    167.5 |#####
    168.2 |#####
  (11 below, 10 above range)

```
