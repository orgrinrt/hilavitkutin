# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 3.4x slower than the field

ctz_loop (521 ns) is 3.4x the fastest (152 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ctz_debruijn, ctz_intrinsic) are a dead heat (<1%)

ctz_debruijn (152 ns) and ctz_intrinsic (152 ns) differ by 0.00%, inside the noise, even though the wider field spreads 242.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (152 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.4x the fastest

Fastest ctz_debruijn (152 ns) to slowest ctz_loop (521 ns): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 152.1 ns median
- 1 variant significantly slower than baseline
- Spread: 3.43x (fastest 152.1 ns, slowest 521.5 ns)

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
| ctz_debruijn | 2254ns | 2260ns | 2217ns | 2256ns | 2284ns | base |
| ctz_intrinsic | 2248ns | 2259ns | 2216ns | 2248ns | 2282ns | -0.26% |
| ctz_loop | 2618ns | 2622ns | 2583ns | 2615ns | 2664ns | +16.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 153ns | 146ns | 160ns | base | 26.773 |
| ctz_intrinsic | 153ns | 146ns | 163ns | +0.17% | 26.729 |
| ctz_loop | 523ns | 496ns | 554ns | +241.97% | 7.829 |

## Performance model

- Peak throughput: **28.034 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 26.930 | 96.1% |
| ctz_intrinsic | 26.930 | 96.1% |
| ctz_loop | 7.855 | 28.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2254ns | 2254ns | base |
| ctz_intrinsic | 2248ns | 2248ns | -0.26% |
| ctz_loop | 2618ns | 2618ns | +16.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 152ns | base | --- | [151, 154] | --- | --- | --- | --- |
| ctz_intrinsic | 152ns | no significant difference | [-0, +0]ns | [151, 153] | no | 0.6255 | 0.6255 | 9 |
| ctz_loop | 521ns | +368.8ns (+242.4%) | [+367, +371]ns | [520, 524] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 152ns | +1.0% | +234.4% |
| 1 | 2 | 154ns | -0.8% | +245.2% |
| 1 | 3 | 152ns | -0.7% | +267.8% |
| 1 | 4 | 151ns | +0.4% | +243.7% |
| 1 | 5 | 152ns | +0.0% | +222.1% |
| 1 | 6 | 151ns | -0.4% | +241.2% |
| 1 | 7 | 152ns | +1.0% | +248.4% |
| 1 | 8 | 152ns | +0.3% | +242.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | 0.139 | ok |
| ctz_intrinsic | -0.169 | ok |
| ctz_loop | 0.128 | ok |

**Consistency summary:**

- **ctz_intrinsic**: won 3/8, lost 4/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 2.1ns | 153.0ns | 1.4% |  |
| ctz_intrinsic | 2.2ns | 153.2ns | 1.4% |  |
| ctz_loop | 3.0ns | 523.2ns | 0.6% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 146.3-160.0 ns)
    146.3 |####
    146.9 |############
    147.6 |#####################
    148.3 |####
    149.0 |###########################
    149.7 |########
    150.4 |############
    151.1 |########################################
    151.8 |################
    152.5 |############
    153.1 |############
    153.8 |##
    154.5 |######
    155.2 |##########
    155.9 |##
    156.6 |#######################
    157.3 |###########################
    158.0 |######
    158.7 |##############
    159.3 |##################
  (15 below, 10 above range)

ctz_intrinsic (n=160, range 146.1-162.6 ns)
    146.1 |##########
    146.9 |############
    147.8 |####################
    148.6 |########################################
    149.4 |##############################
    150.2 |##############################
    151.1 |###############
    151.9 |####################
    152.7 |#########################
    153.5 |#################
    154.3 |###############
    155.2 |#######
    156.0 |############
    156.8 |####################
    157.6 |####################
    158.5 |#################
    159.3 |############
    160.1 |############
    160.9 |#######
    161.8 |#######
  (14 below, 5 above range)

ctz_loop (n=160, range 496.4-553.9 ns)
    496.4 |####
    499.3 |####
    502.2 |#######
    505.1 |#######
    507.9 |#######
    510.8 |###########
    513.7 |##############################
    516.5 |########################################
    519.4 |########################################
    522.3 |##############################
    525.2 |#####################################
    528.0 |#######################
    530.9 |###########
    533.8 |####
    536.6 |#######
    539.5 |####
    542.4 |####
    545.2 |####
    548.1 |
    551.0 |###########
  (19 below, 16 above range)

```
