# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 34% faster than the next best (dispatch_volatile)

dispatch_direct (124 ns) leads dispatch_volatile (166 ns) by 34%, a clear separation rather than a photo finish. CV 7.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 2.6x slower than the field

dispatch_runtime (319 ns) is 2.6x the fastest (124 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (124 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 123.5 ns median
- 2 variants significantly slower than baseline
- Spread: 2.58x (fastest 123.5 ns, slowest 319.2 ns)

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
| dispatch_direct | 2417ns | 2527ns | 2195ns | 2448ns | 2544ns | base |
| dispatch_runtime | 2550ns | 2532ns | 2347ns | 2559ns | 2725ns | +5.49% |
| dispatch_volatile | 2365ns | 2356ns | 2217ns | 2345ns | 2573ns | -2.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 121ns | 109ns | 133ns | base | 8.436 |
| dispatch_runtime | 312ns | 280ns | 345ns | +157.37% | 3.278 |
| dispatch_volatile | 168ns | 154ns | 186ns | +38.19% | 6.105 |

## Performance model

- Peak throughput: **9.415 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 8.288 | 88.0% |
| dispatch_runtime | 3.208 | 34.1% |
| dispatch_volatile | 6.176 | 65.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2417ns | 2417ns | base |
| dispatch_runtime | 2550ns | 2550ns | +5.49% |
| dispatch_volatile | 2365ns | 2365ns | -2.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 124ns | base | --- | [121, 125] | --- | --- | --- | --- |
| dispatch_runtime | 319ns | +197.3ns (+159.7%) | [+184, +202]ns | [311, 325] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 166ns | +44.4ns (+35.9%) | [+41, +50]ns | [162, 169] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 127ns | +118.3% | +24.9% |
| 1 | 2 | 125ns | +160.5% | +28.4% |
| 1 | 3 | 111ns | +154.8% | +45.2% |
| 1 | 4 | 125ns | +174.2% | +28.0% |
| 1 | 5 | 111ns | +200.0% | +51.2% |
| 1 | 6 | 125ns | +151.7% | +25.8% |
| 1 | 7 | 127ns | +130.1% | +36.8% |
| 1 | 8 | 116ns | +185.3% | +57.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.404 | moderate- |
| dispatch_runtime | -0.339 | moderate- |
| dispatch_volatile | 0.187 | ok |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.9ns | 121.4ns | 2.4% |  |
| dispatch_runtime | 3.9ns | 312.4ns | 1.3% |  |
| dispatch_volatile | 3.7ns | 167.7ns | 2.2% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 108.8-133.2 ns)
    108.8 |########
    110.0 |######################
    111.2 |####################
    112.4 |##############
    113.6 |#################
    114.9 |######################
    116.1 |###########
    117.3 |#####
    118.5 |###########
    119.7 |##############
    121.0 |##############
    122.2 |#################
    123.4 |#####################################
    124.6 |############################
    125.8 |###############################
    127.1 |########################################
    128.3 |###############################
    129.5 |#####
    130.7 |#################
    131.9 |########
  (17 below, 10 above range)

dispatch_runtime (n=160, range 279.6-344.7 ns)
    279.6 |########################
    282.9 |####################
    286.1 |############################
    289.4 |###########
    292.6 |######################
    295.9 |####
    299.1 |
    302.4 |##
    305.7 |
    308.9 |##
    312.2 |########
    315.4 |###############
    318.7 |#############
    321.9 |#############
    325.2 |########################################
    328.4 |####################
    331.7 |############################
    335.0 |####################
    338.2 |###############
    341.5 |####
  (16 below, 11 above range)

dispatch_volatile (n=160, range 154.3-186.3 ns)
    154.3 |##################
    155.9 |################################
    157.5 |########################################
    159.1 |##########################
    160.7 |##################
    162.3 |#####################
    163.9 |################
    165.5 |################
    167.1 |#####################
    168.7 |##########################
    170.3 |################
    171.9 |##########
    173.5 |##########
    175.1 |################
    176.7 |#############
    178.3 |########
    179.9 |##########
    181.5 |########################
    183.1 |################
    184.7 |#####
  (14 below, 8 above range)

```
