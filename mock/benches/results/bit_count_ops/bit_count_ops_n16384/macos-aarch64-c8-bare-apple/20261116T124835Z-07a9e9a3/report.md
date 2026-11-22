# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 4.1x slower than the field

ctz_loop (2.03 us) is 4.1x the fastest (498 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ctz_intrinsic, ctz_debruijn) are a dead heat (<1%)

ctz_intrinsic (498 ns) and ctz_debruijn (499 ns) differ by 0.34%, inside the noise, even though the wider field spreads 307.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ctz_intrinsic shows alternating (throttle bounce) (autocorr -0.56)

ctz_intrinsic's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.1x the fastest

Fastest ctz_intrinsic (498 ns) to slowest ctz_loop (2.03 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ctz_intrinsic** at 497.5 ns median (-0.3% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 4.07x (fastest 497.5 ns, slowest 2026.9 ns)

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
| ctz_debruijn | 2602ns | 2608ns | 2559ns | 2605ns | 2639ns | base |
| ctz_intrinsic | 2597ns | 2607ns | 2557ns | 2599ns | 2634ns | -0.19% |
| ctz_loop | 4139ns | 4137ns | 4061ns | 4136ns | 4227ns | +59.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 500ns | 490ns | 513ns | base | 32.758 |
| ctz_intrinsic | 498ns | 489ns | 509ns | -0.35% | 32.872 |
| ctz_loop | 2032ns | 1978ns | 2096ns | +306.32% | 8.062 |

## Performance model

- Peak throughput: **33.519 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 32.821 | 97.9% |
| ctz_intrinsic | 32.933 | 98.2% |
| ctz_loop | 8.083 | 24.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2602ns | 2602ns | base |
| ctz_intrinsic | 2597ns | 2597ns | -0.19% |
| ctz_loop | 4139ns | 4139ns | +59.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 499ns | base | --- | [498, 500] | --- | --- | --- | --- |
| ctz_intrinsic | 498ns | no significant difference | [-2, +0]ns | [496, 500] | no | 0.1033 | 0.1033 | 9 |
| ctz_loop | 2027ns | +1527.8ns (+306.0%) | [+1516, +1539]ns | [2014, 2036] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 496ns | +1.0% | +304.8% |
| 1 | 2 | 500ns | -0.9% | +298.6% |
| 1 | 3 | 500ns | -1.3% | +320.8% |
| 1 | 4 | 500ns | +0.4% | +297.3% |
| 1 | 5 | 502ns | -1.6% | +298.9% |
| 1 | 6 | 495ns | +0.9% | +318.1% |
| 1 | 7 | 500ns | -0.8% | +315.0% |
| 1 | 8 | 494ns | +0.5% | +312.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.312 | moderate- |
| ctz_intrinsic | -0.557 | HIGH- (thermal bounce) |
| ctz_loop | -0.253 | moderate- |

**Consistency summary:**

- **ctz_intrinsic**: won 4/8, lost 4/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 2.7ns | 500.1ns | 0.5% |  |
| ctz_intrinsic | 2.8ns | 498.4ns | 0.6% |  |
| ctz_loop | 3.0ns | 2032.2ns | 0.2% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 489.9-513.4 ns)
    489.9 |#######
    491.1 |##############
    492.3 |################
    493.4 |############################
    494.6 |#######################
    495.8 |#####################
    497.0 |#######################
    498.1 |#######################
    499.3 |########################################
    500.5 |####
    501.7 |##################
    502.8 |###########
    504.0 |############################
    505.2 |##############################
    506.3 |##################
    507.5 |#########
    508.7 |####
    509.9 |#######
    511.0 |
    512.2 |##
  (15 below, 3 above range)

ctz_intrinsic (n=160, range 488.8-509.2 ns)
    488.8 |##############################
    489.8 |########################################
    490.8 |########################################
    491.9 |#############
    492.9 |#############
    493.9 |##########################
    494.9 |####################################
    495.9 |####################
    497.0 |#######################
    498.0 |################
    499.0 |#############
    500.0 |#######################
    501.0 |######
    502.1 |#############
    503.1 |####################
    504.1 |########################################
    505.1 |#################################
    506.1 |##########################
    507.2 |####################
    508.2 |####################
  (11 below, 6 above range)

ctz_loop (n=160, range 1978.4-2096.0 ns)
   1978.4 |#####################
   1984.3 |############
   1990.2 |###########################
   1996.1 |###############
   2001.9 |########################################
   2007.8 |####################################
   2013.7 |########################
   2019.6 |###############
   2025.4 |###############
   2031.3 |###########################
   2037.2 |##############################
   2043.1 |############
   2049.0 |###############
   2054.8 |######
   2060.7 |#########
   2066.6 |########################
   2072.5 |##############################
   2078.3 |#########
   2084.2 |###############
   2090.1 |###############
  (16 below, 12 above range)

```
