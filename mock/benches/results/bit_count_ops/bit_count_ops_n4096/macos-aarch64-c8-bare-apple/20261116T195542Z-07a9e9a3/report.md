# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 3.5x slower than the field

ctz_loop (522 ns) is 3.5x the fastest (151 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ctz_intrinsic, ctz_debruijn) are a dead heat (<1%)

ctz_intrinsic (151 ns) and ctz_debruijn (152 ns) differ by 0.86%, inside the noise, even though the wider field spreads 245.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 3.5x the fastest

Fastest ctz_intrinsic (151 ns) to slowest ctz_loop (522 ns): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ctz_intrinsic** at 151.0 ns median (-0.9% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 3.46x (fastest 151.0 ns, slowest 522.1 ns)

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
| ctz_debruijn | 2251ns | 2228ns | 2217ns | 2243ns | 2308ns | base |
| ctz_intrinsic | 2244ns | 2227ns | 2217ns | 2234ns | 2301ns | -0.29% |
| ctz_loop | 2614ns | 2615ns | 2561ns | 2609ns | 2681ns | +16.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 153ns | 146ns | 162ns | base | 26.773 |
| ctz_intrinsic | 152ns | 146ns | 160ns | -0.59% | 26.933 |
| ctz_loop | 521ns | 489ns | 553ns | +240.59% | 7.861 |

## Performance model

- Peak throughput: **28.146 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 26.894 | 95.6% |
| ctz_intrinsic | 27.126 | 96.4% |
| ctz_loop | 7.845 | 27.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2251ns | 2251ns | base |
| ctz_intrinsic | 2244ns | 2244ns | -0.29% |
| ctz_loop | 2614ns | 2614ns | +16.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 152ns | base | --- | [151, 153] | --- | --- | --- | --- |
| ctz_intrinsic | 151ns | no significant difference | [-1, +0]ns | [150, 153] | no | 0.6871 | 0.6871 | 6 |
| ctz_loop | 522ns | +370.9ns (+243.5%) | [+365, +373]ns | [515, 526] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 152ns | -1.8% | +230.6% |
| 1 | 2 | 155ns | +1.5% | +242.8% |
| 1 | 3 | 149ns | +0.8% | +272.3% |
| 1 | 4 | 150ns | +0.7% | +247.9% |
| 1 | 5 | 152ns | -1.0% | +216.8% |
| 1 | 6 | 151ns | -0.6% | +235.1% |
| 1 | 7 | 154ns | -1.8% | +253.6% |
| 1 | 8 | 152ns | -0.7% | +243.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.117 | ok |
| ctz_intrinsic | -0.348 | moderate- |
| ctz_loop | 0.092 | ok |

**Consistency summary:**

- **ctz_intrinsic**: won 5/8, lost 3/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 2.1ns | 153.0ns | 1.4% |  |
| ctz_intrinsic | 2.1ns | 152.1ns | 1.4% |  |
| ctz_loop | 3.1ns | 521.1ns | 0.6% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 146.0-162.4 ns)
    146.0 |########################
    146.8 |########################################
    147.6 |########################
    148.5 |###########################
    149.3 |###########################
    150.1 |####################################
    150.9 |##################
    151.7 |#####################
    152.6 |###########################
    153.4 |############
    154.2 |####################################
    155.0 |#####################
    155.8 |#####################
    156.6 |###############
    157.5 |##################
    158.3 |########################
    159.1 |#################################
    159.9 |#########
    160.7 |###
    161.5 |###
  (13 below, 1 above range)

ctz_intrinsic (n=160, range 145.5-159.7 ns)
    145.5 |##########################
    146.2 |#############
    146.9 |#################################
    147.7 |####################################
    148.4 |####################
    149.1 |####################################
    149.8 |########################################
    150.5 |##########################
    151.2 |################
    151.9 |#######################
    152.6 |#################################
    153.3 |##########
    154.0 |################
    154.7 |##########################
    155.4 |######
    156.1 |##############################
    156.9 |######
    157.6 |####################
    158.3 |################
    159.0 |####################
  (10 below, 12 above range)

ctz_loop (n=160, range 488.9-553.1 ns)
    488.9 |##
    492.1 |
    495.3 |
    498.5 |#########################
    501.7 |######################
    504.9 |####################
    508.1 |####################
    511.3 |##############################
    514.6 |#####
    517.8 |####################
    521.0 |#################
    524.2 |########################################
    527.4 |###########################
    530.6 |#######
    533.8 |############
    537.0 |#######
    540.2 |###############
    543.5 |###############
    546.7 |##########
    549.9 |###############
  (19 below, 16 above range)

```
