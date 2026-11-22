# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 3.4x slower than the field

ctz_loop (52 ns) is 3.4x the fastest (15 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_debruijn is fastest but the noisiest (CV 24.1%)

ctz_debruijn wins on median (15 ns) yet has the highest variance (CV 24.1%), while ctz_loop is the steadiest (CV 11.6%, 52 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (ctz_debruijn, ctz_intrinsic) are a dead heat (<1%)

ctz_debruijn (15 ns) and ctz_intrinsic (15 ns) differ by 0.00%, inside the noise, even though the wider field spreads 238.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (15 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.4x the fastest

Fastest ctz_debruijn (15 ns) to slowest ctz_loop (52 ns): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ctz_debruijn is inconsistent: worst-20% is 1.8x its best-20%

ctz_debruijn's best 20% of batches run at 13 ns but its worst 20% at 22 ns (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 15.4 ns median
- 1 variant significantly slower than baseline
- Spread: 3.38x (fastest 15.4 ns, slowest 52.1 ns)

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
| ctz_debruijn | 2113ns | 2099ns | 2084ns | 2108ns | 2158ns | base |
| ctz_intrinsic | 2122ns | 2126ns | 2085ns | 2122ns | 2158ns | +0.42% |
| ctz_loop | 2154ns | 2144ns | 2102ns | 2139ns | 2249ns | +1.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 17ns | 13ns | 22ns | base | 15.479 |
| ctz_intrinsic | 16ns | 12ns | 19ns | -6.23% | 16.507 |
| ctz_loop | 52ns | 44ns | 61ns | +215.56% | 4.905 |

## Performance model

- Peak throughput: **20.536 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 16.623 | 80.9% |
| ctz_intrinsic | 16.623 | 80.9% |
| ctz_loop | 4.914 | 23.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2113ns | 2113ns | base |
| ctz_intrinsic | 2122ns | 2122ns | +0.42% |
| ctz_loop | 2154ns | 2154ns | +1.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 15ns | base | --- | [15, 16] | --- | --- | --- | --- |
| ctz_intrinsic | 15ns | no significant difference | [-1, +0]ns | [15, 16] | no | 0.1837 | 0.1837 | 15 |
| ctz_loop | 52ns | +35.9ns (+232.8%) | [+34, +38]ns | [51, 54] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 15ns | +8.2% | +213.7% |
| 1 | 2 | 15ns | +3.9% | +212.3% |
| 1 | 3 | 15ns | +5.3% | +247.3% |
| 1 | 4 | 22ns | -33.3% | +127.8% |
| 1 | 5 | 15ns | +5.4% | +289.9% |
| 1 | 6 | 16ns | -1.3% | +276.6% |
| 1 | 7 | 16ns | -7.7% | +180.6% |
| 1 | 8 | 15ns | +0.0% | +265.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.282 | moderate- |
| ctz_intrinsic | 0.198 | ok |
| ctz_loop | -0.065 | ok |

**Consistency summary:**

- **ctz_intrinsic**: won 3/8, lost 4/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 3.0ns | 16.5ns | 18.2% | HIGH |
| ctz_intrinsic | 3.3ns | 15.5ns | 21.3% | HIGH |
| ctz_loop | 2.9ns | 52.2ns | 5.5% | HIGH |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 12.8-22.5 ns)
     12.8 |#########
     13.2 |#####
     13.7 |########################################
     14.2 |###########
     14.7 |###############
     15.2 |################
     15.7 |###########
     16.2 |####
     16.7 |#############
     17.1 |#########
     17.6 |###############
     18.1 |##
     18.6 |####
     19.1 |##
     19.6 |########
     20.1 |##
     20.5 |#####
     21.0 |####
     21.5 |#
     22.0 |####
  (13 below, 11 above range)

ctz_intrinsic (n=160, range 12.5-18.8 ns)
     12.5 |##########
     12.8 |#################
     13.1 |####################
     13.4 |
     13.7 |###############
     14.0 |#####################################
     14.4 |################################
     14.7 |
     15.0 |######################
     15.3 |################################
     15.6 |########################################
     15.9 |######################
     16.2 |
     16.6 |#########################
     16.9 |####################
     17.2 |####################
     17.5 |
     17.8 |#######
     18.1 |############
     18.5 |
  (10 below, 16 above range)

ctz_loop (n=160, range 43.7-60.7 ns)
     43.7 |#######
     44.6 |####################
     45.4 |############
     46.3 |######################
     47.1 |######################
     48.0 |####################
     48.8 |#######
     49.7 |###############
     50.5 |##########
     51.3 |###################################
     52.2 |############
     53.0 |###############
     53.9 |########################################
     54.7 |############
     55.6 |##########
     56.4 |###############
     57.3 |##########
     58.1 |#################
     59.0 |######################
     59.8 |##########
  (13 below, 12 above range)

```

## Diagnostics

- **ctz_debruijn**: CV=22.4% (high variance, measurements may be unstable)
- **ctz_debruijn**: bridge=18.8% of algo (FFI overhead may distort results)
- **ctz_intrinsic**: bridge=21.4% of algo (FFI overhead may distort results)
