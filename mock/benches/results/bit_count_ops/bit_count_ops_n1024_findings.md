# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 2.5x slower than the field

ctz_loop (158 ns) is 2.5x the fastest (64 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (64 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 63.8 ns median
- 2 variants significantly slower than baseline
- Spread: 2.47x (fastest 63.8 ns, slowest 157.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6f1038d492c5ab07 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ctz_debruijn | 2271ns | 2187ns | 2118ns | 2259ns | 2460ns | base |
| ctz_intrinsic | 2313ns | 2430ns | 2116ns | 2334ns | 2446ns | +1.84% |
| ctz_loop | 2400ns | 2374ns | 2207ns | 2408ns | 2571ns | +5.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 63ns | 55ns | 73ns | base | 16.176 |
| ctz_intrinsic | 65ns | 56ns | 75ns | +3.19% | 15.676 |
| ctz_loop | 160ns | 143ns | 180ns | +152.33% | 6.411 |

## Performance model

- Peak throughput: **18.610 Gops/s** (ctz_debruijn; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 16.050 | 86.2% |
| ctz_intrinsic | 15.657 | 84.1% |
| ctz_loop | 6.502 | 34.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2271ns | 2271ns | base |
| ctz_intrinsic | 2313ns | 2313ns | +1.84% |
| ctz_loop | 2400ns | 2400ns | +5.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 64ns | base | --- | [61, 65] | --- | --- | --- | --- |
| ctz_intrinsic | 65ns | +4.4ns (+6.8%) | [+1, +6]ns | [64, 66] | YES | 0.0042 | 0.0042 | 1 |
| ctz_loop | 158ns | +95.2ns (+149.2%) | [+93, +98]ns | [154, 161] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 68ns | -10.8% | +134.5% |
| 1 | 2 | 67ns | -11.7% | +157.6% |
| 1 | 3 | 61ns | +12.2% | +149.3% |
| 1 | 4 | 59ns | +16.1% | +146.5% |
| 1 | 5 | 59ns | +16.1% | +161.3% |
| 1 | 6 | 59ns | +11.7% | +162.0% |
| 1 | 7 | 60ns | +9.2% | +176.7% |
| 1 | 8 | 68ns | -8.1% | +170.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | 0.349 | moderate+ |
| ctz_intrinsic | 0.361 | moderate+ |
| ctz_loop | 0.170 | ok |

**Consistency summary:**

- **ctz_intrinsic**: won 3/8, lost 5/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 2.3ns | 63.3ns | 3.6% |  |
| ctz_intrinsic | 2.7ns | 65.3ns | 4.1% |  |
| ctz_loop | 3.3ns | 159.7ns | 2.1% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 55.0-73.0 ns)
     55.0 |##########
     55.9 |#################
     56.8 |##############################
     57.7 |##########
     58.6 |#################
     59.5 |########################################
     60.4 |#################
     61.3 |#####
     62.2 |############
     63.1 |##########
     64.0 |########################################
     64.9 |####################
     65.8 |###########################
     66.7 |######################
     67.6 |#######
     68.5 |##########
     69.4 |
     70.3 |#####
     71.2 |##
     72.1 |######################
  (14 below, 15 above range)

ctz_intrinsic (n=160, range 55.7-74.9 ns)
     55.7 |#####
     56.6 |###############
     57.6 |############
     58.6 |############
     59.5 |####################
     60.5 |####################
     61.4 |##########
     62.4 |#################
     63.4 |#########################
     64.3 |#########################
     65.3 |########################################
     66.2 |#################
     67.2 |##########
     68.2 |###############
     69.1 |############
     70.1 |####################
     71.1 |####################
     72.0 |###############
     73.0 |##
     73.9 |###############
  (14 below, 14 above range)

ctz_loop (n=160, range 142.9-179.7 ns)
    142.9 |################
    144.7 |#######################
    146.6 |##############################
    148.4 |##########################
    150.3 |########################################
    152.1 |#################################
    154.0 |####################
    155.8 |##############################
    157.6 |################
    159.5 |##########################
    161.3 |####################
    163.2 |##########
    165.0 |#######################
    166.8 |#############
    168.7 |##########################
    170.5 |##########################
    172.4 |#######################
    174.2 |######
    176.0 |#############
    177.9 |###
  (15 below, 16 above range)

```
