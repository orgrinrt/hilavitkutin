# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 2.2x slower than the field

ctz_loop (51 ns) is 2.2x the fastest (23 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_intrinsic is fastest but the noisiest (CV 15.1%)

ctz_intrinsic wins on median (23 ns) yet has the highest variance (CV 15.1%), while ctz_loop is the steadiest (CV 14.0%, 51 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ctz_intrinsic is inconsistent: worst-20% is 1.5x its best-20%

ctz_intrinsic's best 20% of batches run at 19 ns but its worst 20% at 29 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ctz_intrinsic** at 22.9 ns median (-1.7% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.21x (fastest 22.9 ns, slowest 50.6 ns)

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
| ctz_debruijn | 2110ns | 2102ns | 2097ns | 2104ns | 2144ns | base |
| ctz_intrinsic | 2113ns | 2102ns | 2096ns | 2105ns | 2155ns | +0.14% |
| ctz_loop | 2127ns | 2121ns | 2113ns | 2121ns | 2160ns | +0.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 24ns | 19ns | 28ns | base | 10.845 |
| ctz_intrinsic | 24ns | 19ns | 29ns | -0.43% | 10.892 |
| ctz_loop | 50ns | 42ns | 60ns | +113.10% | 5.089 |

## Performance model

- Peak throughput: **13.458 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 10.987 | 81.6% |
| ctz_intrinsic | 11.179 | 83.1% |
| ctz_loop | 5.059 | 37.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2110ns | 2110ns | base |
| ctz_intrinsic | 2113ns | 2113ns | +0.14% |
| ctz_loop | 2127ns | 2127ns | +0.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 23ns | base | --- | [22, 24] | --- | --- | --- | --- |
| ctz_intrinsic | 23ns | no significant difference | [-0, +0]ns | [22, 24] | no | 1.0000 | 1.0000 | 13 |
| ctz_loop | 51ns | +26.2ns (+112.4%) | [+25, +28]ns | [49, 52] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 23ns | +1.8% | +91.9% |
| 1 | 2 | 24ns | +0.2% | +98.7% |
| 1 | 3 | 24ns | -7.0% | +105.8% |
| 1 | 4 | 23ns | +1.7% | +114.7% |
| 1 | 5 | 23ns | +0.9% | +140.7% |
| 1 | 6 | 23ns | -1.7% | +147.2% |
| 1 | 7 | 23ns | +0.9% | +94.8% |
| 1 | 8 | 24ns | -4.3% | +124.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.025 | ok |
| ctz_intrinsic | -0.411 | moderate- |
| ctz_loop | 0.040 | ok |

**Consistency summary:**

- **ctz_intrinsic**: won 3/8, lost 5/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 2.2ns | 23.6ns | 9.2% | HIGH |
| ctz_intrinsic | 2.4ns | 23.5ns | 10.2% | HIGH |
| ctz_loop | 2.6ns | 50.3ns | 5.2% | HIGH |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 19.2-28.4 ns)
     19.2 |###########
     19.7 |#####################
     20.2 |############################
     20.6 |#########
     21.1 |##############
     21.5 |##################
     22.0 |####
     22.5 |########################################
     22.9 |################
     23.4 |###########
     23.8 |##############
     24.3 |#######
     24.7 |###########
     25.2 |##############
     25.7 |##################
     26.1 |##############
     26.6 |##############
     27.0 |#######
     27.5 |############################
     27.9 |####
  (14 below, 14 above range)

ctz_intrinsic (n=160, range 19.0-28.6 ns)
     19.0 |##################
     19.5 |##################
     20.0 |##################################
     20.5 |#############
     20.9 |########################
     21.4 |#####################
     21.9 |################
     22.4 |##################
     22.9 |########################################
     23.3 |#############
     23.8 |########
     24.3 |##########################
     24.8 |##########
     25.3 |##########
     25.7 |##################
     26.2 |
     26.7 |##################
     27.2 |################
     27.7 |#############
     28.1 |########################
  (11 below, 12 above range)

ctz_loop (n=160, range 41.6-59.7 ns)
     41.6 |################
     42.5 |####################
     43.4 |####################
     44.3 |#######################
     45.2 |####################
     46.1 |##############################
     47.1 |####################
     48.0 |##########################
     48.9 |####################
     49.8 |#######################
     50.7 |####################################
     51.6 |########################################
     52.5 |########################################
     53.4 |##########################
     54.3 |################
     55.2 |##########
     56.1 |#######################
     57.0 |####################
     57.9 |###
     58.8 |################
  (14 below, 10 above range)

```

## Diagnostics

- **ctz_intrinsic**: bridge=10.9% of algo (FFI overhead may distort results)
