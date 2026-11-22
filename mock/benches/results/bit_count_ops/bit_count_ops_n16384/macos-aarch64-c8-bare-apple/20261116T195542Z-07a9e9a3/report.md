# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 4.1x slower than the field

ctz_loop (2.00 us) is 4.1x the fastest (493 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.1x the fastest

Fastest ctz_intrinsic (493 ns) to slowest ctz_loop (2.00 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ctz_intrinsic's edge over baseline is significant but tiny (-8 ns, 1.59%)

ctz_intrinsic differs from baseline ctz_debruijn by -8 ns (1.59%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ctz_intrinsic** at 492.9 ns median (-1.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.06x (fastest 492.9 ns, slowest 2001.7 ns)

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
| ctz_debruijn | 2604ns | 2606ns | 2559ns | 2595ns | 2678ns | base |
| ctz_intrinsic | 2576ns | 2563ns | 2555ns | 2566ns | 2626ns | -1.09% |
| ctz_loop | 4101ns | 4090ns | 4047ns | 4096ns | 4169ns | +57.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 500ns | 489ns | 516ns | base | 32.772 |
| ctz_intrinsic | 495ns | 486ns | 508ns | -1.07% | 33.126 |
| ctz_loop | 2013ns | 1966ns | 2071ns | +302.65% | 8.139 |

## Performance model

- Peak throughput: **33.700 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 32.906 | 97.6% |
| ctz_intrinsic | 33.240 | 98.6% |
| ctz_loop | 8.185 | 24.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2604ns | 2604ns | base |
| ctz_intrinsic | 2576ns | 2576ns | -1.09% |
| ctz_loop | 4101ns | 4101ns | +57.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 498ns | base | --- | [496, 499] | --- | --- | --- | --- |
| ctz_intrinsic | 493ns | -7.9ns (-1.6%) | [-9, -7]ns | [492, 494] | YES | 0.0000 | 0.0000 | 1 |
| ctz_loop | 2002ns | +1507.3ns (+302.7%) | [+1498, +1515]ns | [1997, 2010] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 505ns | -2.4% | +290.5% |
| 1 | 2 | 494ns | -0.2% | +298.8% |
| 1 | 3 | 499ns | -1.3% | +315.2% |
| 1 | 4 | 491ns | +2.0% | +301.1% |
| 1 | 5 | 499ns | -1.3% | +299.8% |
| 1 | 6 | 501ns | -1.8% | +306.0% |
| 1 | 7 | 498ns | -1.5% | +311.9% |
| 1 | 8 | 500ns | -1.5% | +300.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.317 | moderate- |
| ctz_intrinsic | -0.113 | ok |
| ctz_loop | -0.223 | moderate- |

**Consistency summary:**

- **ctz_intrinsic**: won 7/8, lost 1/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 2.9ns | 499.9ns | 0.6% |  |
| ctz_intrinsic | 2.7ns | 494.6ns | 0.6% |  |
| ctz_loop | 3.0ns | 2013.0ns | 0.1% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 488.7-516.0 ns)
    488.7 |############
    490.0 |#########################
    491.4 |#######
    492.8 |################################
    494.1 |#####################################
    495.5 |##############################
    496.9 |######################
    498.2 |########################################
    499.6 |######################
    501.0 |############
    502.4 |###############
    503.7 |####################
    505.1 |#################
    506.5 |####################
    507.8 |############
    509.2 |###############
    510.6 |#####
    511.9 |##
    513.3 |##
    514.7 |
  (14 below, 5 above range)

ctz_intrinsic (n=160, range 486.2-508.3 ns)
    486.2 |####
    487.3 |##############################
    488.4 |##################
    489.5 |########################################
    490.6 |###################################
    491.7 |################
    492.8 |#####################################
    493.9 |#########################
    495.0 |################################
    496.1 |##################
    497.2 |################
    498.4 |###########
    499.5 |##############
    500.6 |####
    501.7 |####
    502.8 |#######
    503.9 |##
    505.0 |##
    506.1 |####
    507.2 |
  (16 below, 4 above range)

ctz_loop (n=160, range 1965.9-2070.7 ns)
   1965.9 |########################################
   1971.1 |#####################
   1976.4 |######
   1981.6 |######
   1986.9 |########
   1992.1 |#################################
   1997.3 |#############################
   2002.6 |##############
   2007.8 |########
   2013.0 |
   2018.3 |
   2023.5 |############
   2028.8 |########
   2034.0 |#######################
   2039.2 |######
   2044.5 |##################
   2049.7 |##########
   2054.9 |##########
   2060.2 |##########
   2065.4 |############
  (13 below, 13 above range)

```
