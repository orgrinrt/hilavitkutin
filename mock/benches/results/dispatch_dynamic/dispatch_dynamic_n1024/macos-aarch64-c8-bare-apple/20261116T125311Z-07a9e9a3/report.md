# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 43% faster than the next best (dispatch_volatile)

dispatch_direct (110 ns) leads dispatch_volatile (157 ns) by 43%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 2.6x slower than the field

dispatch_runtime (287 ns) is 2.6x the fastest (110 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dispatch_runtime shows alternating (throttle bounce) (autocorr -0.51)

dispatch_runtime's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (110 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 110.0 ns median
- 2 variants significantly slower than baseline
- Spread: 2.61x (fastest 110.0 ns, slowest 287.3 ns)

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
| dispatch_direct | 2208ns | 2206ns | 2201ns | 2206ns | 2221ns | base |
| dispatch_runtime | 2365ns | 2362ns | 2351ns | 2362ns | 2389ns | +7.13% |
| dispatch_volatile | 2236ns | 2233ns | 2228ns | 2233ns | 2251ns | +1.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 110ns | 106ns | 114ns | base | 9.302 |
| dispatch_runtime | 288ns | 277ns | 301ns | +161.73% | 3.554 |
| dispatch_volatile | 157ns | 152ns | 163ns | +42.76% | 6.516 |

## Performance model

- Peak throughput: **9.652 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 9.309 | 96.4% |
| dispatch_runtime | 3.564 | 36.9% |
| dispatch_volatile | 6.518 | 67.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2208ns | 2208ns | base |
| dispatch_runtime | 2365ns | 2365ns | +7.13% |
| dispatch_volatile | 2236ns | 2236ns | +1.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 110ns | base | --- | [110, 111] | --- | --- | --- | --- |
| dispatch_runtime | 287ns | +177.1ns (+161.0%) | [+175, +179]ns | [285, 290] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 157ns | +47.0ns (+42.7%) | [+47, +47]ns | [156, 158] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 110ns | +153.2% | +43.5% |
| 1 | 2 | 109ns | +166.0% | +42.8% |
| 1 | 3 | 110ns | +157.4% | +42.5% |
| 1 | 4 | 111ns | +170.9% | +42.6% |
| 1 | 5 | 111ns | +158.6% | +42.1% |
| 1 | 6 | 110ns | +167.8% | +44.7% |
| 1 | 7 | 110ns | +155.5% | +43.5% |
| 1 | 8 | 110ns | +163.8% | +44.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | 0.385 | moderate+ |
| dispatch_runtime | -0.507 | HIGH- (thermal bounce) |
| dispatch_volatile | 0.211 | moderate+ |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.8ns | 110.1ns | 2.5% |  |
| dispatch_runtime | 3.5ns | 288.1ns | 1.2% |  |
| dispatch_volatile | 3.4ns | 157.2ns | 2.1% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 106.1-114.4 ns)
    106.1 |
    106.5 |#############
    106.9 |##########################
    107.3 |##########
    107.8 |########################################
    108.2 |##########################
    108.6 |##########################
    109.0 |##########################
    109.4 |####################################
    109.8 |#################################
    110.3 |#############
    110.7 |####################################
    111.1 |##############################
    111.5 |########################################
    111.9 |####################
    112.3 |################
    112.8 |####################
    113.2 |##########
    113.6 |##########
    114.0 |###
  (15 below, 13 above range)

dispatch_runtime (n=160, range 277.4-300.9 ns)
    277.4 |##########
    278.6 |###############
    279.8 |########################################
    280.9 |#########################
    282.1 |############
    283.3 |####################
    284.4 |############
    285.6 |######################
    286.8 |####################
    288.0 |#######
    289.1 |#################
    290.3 |############
    291.5 |######################
    292.7 |####################
    293.8 |######################
    295.0 |#######
    296.2 |############
    297.4 |##########
    298.5 |#########################
    299.7 |#####
  (15 below, 9 above range)

dispatch_volatile (n=160, range 151.7-163.0 ns)
    151.7 |#####
    152.3 |#####
    152.8 |###############
    153.4 |#####
    154.0 |#####
    154.5 |######################
    155.1 |############
    155.7 |###########################
    156.2 |######
    156.8 |########
    157.4 |########################################
    157.9 |##########
    158.5 |#####
    159.1 |####################
    159.6 |###############
    160.2 |##########
    160.8 |##########
    161.3 |###
    161.9 |#
    162.5 |#
  (15 below, 10 above range)

```
