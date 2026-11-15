# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_debruijn dominates: 22% faster than the next best (ctz_intrinsic)

ctz_debruijn (20 ns) leads ctz_intrinsic (25 ns) by 22%, a clear separation rather than a photo finish. CV 26.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ctz_loop is an outlier: 2.7x slower than the field

ctz_loop (55 ns) is 2.7x the fastest (20 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_debruijn is fastest but the noisiest (CV 26.3%)

ctz_debruijn wins on median (20 ns) yet has the highest variance (CV 26.3%), while ctz_intrinsic is the steadiest (CV 24.1%, 25 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (20 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### ctz_intrinsic is inconsistent: worst-20% is 2.1x its best-20%

ctz_intrinsic's best 20% of batches run at 15 ns but its worst 20% at 32 ns (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 20.4 ns median
- 2 variants significantly slower than baseline
- Spread: 2.69x (fastest 20.4 ns, slowest 54.8 ns)

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
| ctz_debruijn | 2256ns | 2185ns | 2082ns | 2255ns | 2434ns | base |
| ctz_intrinsic | 2330ns | 2416ns | 2090ns | 2374ns | 2437ns | +3.26% |
| ctz_loop | 2350ns | 2431ns | 2122ns | 2387ns | 2468ns | +4.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 21ns | 14ns | 29ns | base | 12.293 |
| ctz_intrinsic | 24ns | 15ns | 32ns | +14.77% | 10.711 |
| ctz_loop | 56ns | 47ns | 69ns | +170.59% | 4.543 |

## Performance model

- Peak throughput: **18.249 Gops/s** (ctz_debruijn; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 12.549 | 68.8% |
| ctz_intrinsic | 10.323 | 56.6% |
| ctz_loop | 4.672 | 25.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2256ns | 2256ns | base |
| ctz_intrinsic | 2330ns | 2330ns | +3.26% |
| ctz_loop | 2350ns | 2350ns | +4.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 20ns | base | --- | [19, 22] | --- | --- | --- | --- |
| ctz_intrinsic | 25ns | +2.1ns (+10.3%) | [+1, +4]ns | [23, 26] | YES | 0.0017 | 0.0017 | 4 |
| ctz_loop | 55ns | +36.0ns (+176.5%) | [+35, +37]ns | [54, 56] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 17ns | -15.6% | +192.5% |
| 1 | 2 | 18ns | +30.5% | +205.1% |
| 1 | 3 | 26ns | +3.3% | +103.5% |
| 1 | 4 | 23ns | -29.4% | +136.9% |
| 1 | 5 | 15ns | +76.0% | +263.6% |
| 1 | 6 | 24ns | +11.3% | +165.3% |
| 1 | 7 | 27ns | -2.4% | +81.0% |
| 1 | 8 | 15ns | +83.6% | +301.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.231 | moderate- |
| ctz_intrinsic | -0.049 | ok |
| ctz_loop | -0.390 | moderate- |

**Consistency summary:**

- **ctz_intrinsic**: won 3/8, lost 5/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 3.0ns | 20.8ns | 14.6% | HIGH |
| ctz_intrinsic | 2.9ns | 23.9ns | 12.3% | HIGH |
| ctz_loop | 2.9ns | 56.4ns | 5.2% | HIGH |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 14.0-28.8 ns)
     14.0 |########################################
     14.8 |#################################
     15.5 |##########################
     16.2 |################
     17.0 |########################################
     17.7 |#######################
     18.5 |##########################
     19.2 |##########################
     20.0 |##########
     20.7 |#######################
     21.4 |##########################
     22.2 |#################################
     22.9 |################
     23.7 |################
     24.4 |##########
     25.1 |##############################
     25.9 |###
     26.6 |##########################
     27.4 |####################
     28.1 |##########
  (9 below, 13 above range)

ctz_intrinsic (n=160, range 14.8-31.8 ns)
     14.8 |##################
     15.7 |###############
     16.5 |#####################
     17.4 |###############
     18.2 |#########
     19.1 |######
     19.9 |#########
     20.8 |###############
     21.6 |###############
     22.4 |########################################
     23.3 |############
     24.1 |########################
     25.0 |####################################
     25.8 |#####################
     26.7 |########################################
     27.5 |####################################
     28.4 |############
     29.2 |##################
     30.1 |###############
     30.9 |##################
  (14 below, 15 above range)

ctz_loop (n=160, range 47.0-69.2 ns)
     47.0 |############################
     48.1 |##############
     49.2 |####################
     50.3 |########################################
     51.4 |#########################
     52.5 |#########################
     53.6 |#####################################
     54.7 |##################################
     55.9 |##############
     57.0 |#################
     58.1 |#################
     59.2 |######################
     60.3 |############################
     61.4 |####################
     62.5 |####################
     63.6 |##############
     64.7 |##############
     65.9 |#####
     67.0 |#####
     68.1 |##
  (13 below, 4 above range)

```

## Diagnostics

- **ctz_debruijn**: CV=25.8% (high variance, measurements may be unstable)
- **ctz_debruijn**: bridge=14.2% of algo (FFI overhead may distort results)
- **ctz_intrinsic**: CV=25.0% (high variance, measurements may be unstable)
- **ctz_intrinsic**: bridge=11.7% of algo (FFI overhead may distort results)
- **ctz_loop**: CV=23.7% (high variance, measurements may be unstable)
