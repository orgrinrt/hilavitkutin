# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 4.1x slower than the field

ctz_loop (2.04 us) is 4.1x the fastest (501 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ctz_debruijn, ctz_intrinsic) are a dead heat (<1%)

ctz_debruijn (501 ns) and ctz_intrinsic (503 ns) differ by 0.38%, inside the noise, even though the wider field spreads 306.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (501 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.1x the fastest

Fastest ctz_debruijn (501 ns) to slowest ctz_loop (2.04 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ctz_intrinsic's edge over baseline is significant but tiny (2 ns, 0.42%)

ctz_intrinsic differs from baseline ctz_debruijn by 2 ns (0.42%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 501.0 ns median
- 2 variants significantly slower than baseline
- Spread: 4.07x (fastest 501.0 ns, slowest 2037.1 ns)

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
| ctz_debruijn | 2598ns | 2600ns | 2563ns | 2599ns | 2630ns | base |
| ctz_intrinsic | 2611ns | 2602ns | 2593ns | 2603ns | 2651ns | +0.49% |
| ctz_loop | 4186ns | 4148ns | 4098ns | 4154ns | 4373ns | +61.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 501ns | 492ns | 510ns | base | 32.693 |
| ctz_intrinsic | 506ns | 496ns | 523ns | +1.00% | 32.370 |
| ctz_loop | 2062ns | 2001ns | 2183ns | +311.47% | 7.945 |

## Performance model

- Peak throughput: **33.292 Gops/s** (ctz_debruijn; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 32.703 | 98.2% |
| ctz_intrinsic | 32.579 | 97.9% |
| ctz_loop | 8.043 | 24.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2598ns | 2598ns | base |
| ctz_intrinsic | 2611ns | 2611ns | +0.49% |
| ctz_loop | 4186ns | 4186ns | +61.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 501ns | base | --- | [500, 502] | --- | --- | --- | --- |
| ctz_intrinsic | 503ns | +2.1ns (+0.4%) | [+1, +2]ns | [502, 505] | YES | 0.0005 | 0.0005 | 4 |
| ctz_loop | 2037ns | +1534.8ns (+306.3%) | [+1531, +1540]ns | [2032, 2040] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 503ns | -0.2% | +300.0% |
| 1 | 2 | 505ns | -0.9% | +298.0% |
| 1 | 3 | 494ns | +1.9% | +334.5% |
| 1 | 4 | 502ns | +4.1% | +298.9% |
| 1 | 5 | 502ns | +0.2% | +305.6% |
| 1 | 6 | 501ns | +0.1% | +310.1% |
| 1 | 7 | 502ns | +0.0% | +320.4% |
| 1 | 8 | 498ns | +1.0% | +309.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.421 | moderate- |
| ctz_intrinsic | -0.082 | ok |
| ctz_loop | -0.348 | moderate- |

**Consistency summary:**

- **ctz_intrinsic**: won 2/8, lost 5/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 2.6ns | 501.1ns | 0.5% |  |
| ctz_intrinsic | 2.6ns | 506.1ns | 0.5% |  |
| ctz_loop | 3.0ns | 2062.1ns | 0.1% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 492.1-510.0 ns)
    492.1 |#####
    493.0 |#####
    493.9 |##############
    494.8 |###########
    495.7 |######################
    496.6 |############################
    497.5 |######################
    498.4 |#########################
    499.3 |#####
    500.2 |########################################
    501.1 |####################
    502.0 |#########################
    502.9 |###############################
    503.8 |#################
    504.7 |##############
    505.6 |##################################
    506.5 |#################
    507.4 |#####################################
    508.3 |########
    509.1 |#################
  (16 below, 2 above range)

ctz_intrinsic (n=160, range 496.3-523.5 ns)
    496.3 |################
    497.7 |#########################
    499.1 |#####################################
    500.4 |########################################
    501.8 |########################################
    503.1 |#######################
    504.5 |############################
    505.8 |##################
    507.2 |#####################################
    508.6 |###########
    509.9 |
    511.3 |#########
    512.6 |##
    514.0 |
    515.3 |###########
    516.7 |##
    518.1 |##
    519.4 |####
    520.8 |
    522.1 |####
  (13 below, 12 above range)

ctz_loop (n=160, range 2000.5-2183.5 ns)
   2000.5 |###############################
   2009.7 |#############################
   2018.8 |###################
   2027.9 |##################################
   2037.1 |########################################
   2046.2 |#####
   2055.4 |#####
   2064.5 |########
   2073.7 |##########
   2082.8 |#
   2092.0 |##########
   2101.1 |#################
   2110.3 |#############
   2119.4 |#####
   2128.6 |###
   2137.7 |#
   2146.9 |
   2156.0 |
   2165.2 |#
   2174.3 |
  (13 below, 9 above range)

```
