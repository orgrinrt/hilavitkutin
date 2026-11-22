# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 2.5x slower than the field

ctz_loop (148 ns) is 2.5x the fastest (59 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (59 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader ctz_debruijn vs stability leader ctz_intrinsic (+1% speed for 1.0x steadier)

ctz_debruijn is fastest (59 ns, CV 7.9%); ctz_intrinsic gives up 1.4% median for 1.0x lower variance (CV 7.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 59.2 ns median
- 1 variant significantly slower than baseline
- Spread: 2.51x (fastest 59.2 ns, slowest 148.5 ns)

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
| ctz_debruijn | 2151ns | 2137ns | 2124ns | 2148ns | 2189ns | base |
| ctz_intrinsic | 2145ns | 2134ns | 2124ns | 2139ns | 2185ns | -0.29% |
| ctz_loop | 2252ns | 2257ns | 2215ns | 2250ns | 2293ns | +4.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 60ns | 53ns | 66ns | base | 17.137 |
| ctz_intrinsic | 60ns | 53ns | 66ns | -0.17% | 17.165 |
| ctz_loop | 150ns | 138ns | 166ns | +150.85% | 6.832 |

## Performance model

- Peak throughput: **19.276 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 17.297 | 89.7% |
| ctz_intrinsic | 17.067 | 88.5% |
| ctz_loop | 6.896 | 35.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2151ns | 2151ns | base |
| ctz_intrinsic | 2145ns | 2145ns | -0.29% |
| ctz_loop | 2252ns | 2252ns | +4.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 59ns | base | --- | [58, 60] | --- | --- | --- | --- |
| ctz_intrinsic | 60ns | no significant difference | [-1, +0]ns | [58, 61] | no | 0.1927 | 0.1927 | 9 |
| ctz_loop | 148ns | +88.6ns (+149.6%) | [+87, +91]ns | [147, 150] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 59ns | +1.8% | +146.4% |
| 1 | 2 | 59ns | -0.3% | +162.1% |
| 1 | 3 | 60ns | +1.0% | +151.5% |
| 1 | 4 | 58ns | +3.9% | +148.4% |
| 1 | 5 | 60ns | -1.8% | +129.6% |
| 1 | 6 | 61ns | -0.3% | +156.1% |
| 1 | 7 | 58ns | +2.9% | +143.7% |
| 1 | 8 | 59ns | +1.8% | +168.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.280 | moderate- |
| ctz_intrinsic | -0.392 | moderate- |
| ctz_loop | -0.427 | moderate- |

**Consistency summary:**

- **ctz_intrinsic**: won 3/8, lost 5/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 2.2ns | 59.8ns | 3.7% |  |
| ctz_intrinsic | 2.2ns | 59.7ns | 3.8% |  |
| ctz_loop | 3.4ns | 149.9ns | 2.3% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 53.4-66.5 ns)
     53.4 |##
     54.1 |#################
     54.7 |######
     55.4 |#############
     56.1 |#################################
     56.7 |#############
     57.4 |########################################
     58.0 |###########
     58.7 |#############
     59.3 |######
     60.0 |######################
     60.6 |###############
     61.3 |######
     61.9 |#############
     62.6 |###########
     63.2 |############################
     63.9 |########
     64.5 |#############
     65.2 |####
     65.8 |#############
  (13 below, 14 above range)

ctz_intrinsic (n=160, range 53.1-66.2 ns)
     53.1 |######
     53.8 |#####################
     54.4 |########################
     55.1 |##################
     55.7 |###########################
     56.4 |#########
     57.0 |####################################
     57.7 |###########################
     58.3 |###############
     59.0 |############
     59.7 |##################
     60.3 |###########################
     61.0 |###############
     61.6 |########################################
     62.3 |########################
     62.9 |#####################
     63.6 |#####################
     64.2 |
     64.9 |##################
     65.5 |###############
  (13 below, 16 above range)

ctz_loop (n=160, range 138.2-165.8 ns)
    138.2 |#########
    139.6 |########################################
    141.0 |########################################
    142.3 |##################
    143.7 |########################
    145.1 |########################
    146.5 |##################
    147.9 |########################################
    149.2 |#################################
    150.6 |###########################
    152.0 |###########################
    153.4 |###########################
    154.8 |##################
    156.1 |#####################
    157.5 |#####################
    158.9 |###############
    160.3 |#########
    161.7 |############
    163.0 |###
    164.4 |
  (14 below, 5 above range)

```
