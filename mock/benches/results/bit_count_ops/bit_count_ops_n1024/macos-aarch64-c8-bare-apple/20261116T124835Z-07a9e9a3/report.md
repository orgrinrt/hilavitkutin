# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 2.5x slower than the field

ctz_loop (148 ns) is 2.5x the fastest (60 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ctz_intrinsic, ctz_debruijn) are a dead heat (<1%)

ctz_intrinsic (60 ns) and ctz_debruijn (60 ns) differ by 0.34%, inside the noise, even though the wider field spreads 148.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: ctz_intrinsic** at 59.6 ns median (-0.3% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.49x (fastest 59.6 ns, slowest 148.3 ns)

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
| ctz_debruijn | 2153ns | 2136ns | 2123ns | 2146ns | 2202ns | base |
| ctz_intrinsic | 2158ns | 2165ns | 2125ns | 2156ns | 2195ns | +0.23% |
| ctz_loop | 2243ns | 2246ns | 2207ns | 2240ns | 2287ns | +4.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 60ns | 53ns | 67ns | base | 17.106 |
| ctz_intrinsic | 60ns | 54ns | 68ns | +0.90% | 16.953 |
| ctz_loop | 148ns | 135ns | 159ns | +146.92% | 6.928 |

## Performance model

- Peak throughput: **19.203 Gops/s** (ctz_debruijn; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 17.124 | 89.2% |
| ctz_intrinsic | 17.181 | 89.5% |
| ctz_loop | 6.905 | 36.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2153ns | 2153ns | base |
| ctz_intrinsic | 2158ns | 2158ns | +0.23% |
| ctz_loop | 2243ns | 2243ns | +4.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 60ns | base | --- | [58, 61] | --- | --- | --- | --- |
| ctz_intrinsic | 60ns | no significant difference | [+0, +1]ns | [59, 61] | no | 0.0711 | 0.0711 | 11 |
| ctz_loop | 148ns | +88.1ns (+147.3%) | [+87, +91]ns | [147, 150] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 58ns | +3.3% | +141.5% |
| 1 | 2 | 59ns | -0.7% | +161.7% |
| 1 | 3 | 59ns | +0.3% | +151.6% |
| 1 | 4 | 58ns | +1.7% | +147.0% |
| 1 | 5 | 59ns | +2.9% | +124.3% |
| 1 | 6 | 62ns | -1.7% | +153.1% |
| 1 | 7 | 61ns | -0.3% | +139.1% |
| 1 | 8 | 61ns | -2.7% | +158.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | 0.371 | moderate+ |
| ctz_intrinsic | 0.024 | ok |
| ctz_loop | -0.278 | moderate- |

**Consistency summary:**

- **ctz_intrinsic**: won 4/8, lost 4/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 2.3ns | 59.9ns | 3.9% |  |
| ctz_intrinsic | 2.3ns | 60.4ns | 3.9% |  |
| ctz_loop | 3.3ns | 147.8ns | 2.3% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 53.3-67.1 ns)
     53.3 |####
     54.0 |#################
     54.7 |###########
     55.4 |######################
     56.1 |###############
     56.8 |########
     57.5 |########################################
     58.1 |####################
     58.8 |##
     59.5 |###############
     60.2 |############################
     60.9 |####
     61.6 |##########################
     62.3 |########
     63.0 |###############
     63.7 |###########
     64.3 |#############
     65.0 |####
     65.7 |#################
     66.4 |#############
  (14 below, 10 above range)

ctz_intrinsic (n=160, range 54.1-67.5 ns)
     54.1 |#####
     54.8 |##################################
     55.5 |####################
     56.1 |####################
     56.8 |######################
     57.5 |############################
     58.1 |########################################
     58.8 |####################
     59.5 |#################
     60.1 |##################################
     60.8 |#####
     61.5 |#################
     62.2 |#####
     62.8 |#########################
     63.5 |###########
     64.2 |####################
     64.8 |##############
     65.5 |##############
     66.2 |########
     66.8 |####################
  (11 below, 14 above range)

ctz_loop (n=160, range 135.3-159.4 ns)
    135.3 |################
    136.5 |########
    137.8 |################################
    139.0 |################
    140.2 |####################################
    141.4 |################################
    142.6 |####################
    143.8 |########################
    145.0 |########################
    146.2 |########################
    147.4 |########################################
    148.6 |############################
    149.8 |################################
    151.0 |########################################
    152.2 |################################
    153.4 |################
    154.6 |################################
    155.8 |################################
    157.0 |####################
    158.2 |################################
  (14 below, 12 above range)

```
