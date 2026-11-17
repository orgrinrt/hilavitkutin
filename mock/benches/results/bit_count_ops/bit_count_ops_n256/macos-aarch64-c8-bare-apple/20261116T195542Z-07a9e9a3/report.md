# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 2.2x slower than the field

ctz_loop (51 ns) is 2.2x the fastest (23 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_intrinsic is fastest but the noisiest (CV 15.7%)

ctz_intrinsic wins on median (23 ns) yet has the highest variance (CV 15.7%), while ctz_loop is the steadiest (CV 11.2%, 51 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ctz_intrinsic is inconsistent: worst-20% is 1.5x its best-20%

ctz_intrinsic's best 20% of batches run at 19 ns but its worst 20% at 29 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ctz_intrinsic** at 22.9 ns median (-1.7% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.22x (fastest 22.9 ns, slowest 50.8 ns)

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
| ctz_debruijn | 2143ns | 2107ns | 2098ns | 2125ns | 2244ns | base |
| ctz_intrinsic | 2109ns | 2103ns | 2095ns | 2103ns | 2142ns | -1.60% |
| ctz_loop | 2138ns | 2122ns | 2113ns | 2123ns | 2205ns | -0.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 24ns | 19ns | 29ns | base | 10.773 |
| ctz_intrinsic | 23ns | 19ns | 29ns | -1.48% | 10.935 |
| ctz_loop | 50ns | 42ns | 58ns | +111.89% | 5.084 |

## Performance model

- Peak throughput: **13.588 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 10.987 | 80.9% |
| ctz_intrinsic | 11.179 | 82.3% |
| ctz_loop | 5.039 | 37.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2143ns | 2143ns | base |
| ctz_intrinsic | 2109ns | 2109ns | -1.60% |
| ctz_loop | 2138ns | 2138ns | -0.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 23ns | base | --- | [22, 24] | --- | --- | --- | --- |
| ctz_intrinsic | 23ns | no significant difference | [-1, +0]ns | [22, 24] | no | 0.3190 | 0.3190 | 15 |
| ctz_loop | 51ns | +26.7ns (+114.6%) | [+26, +28]ns | [50, 52] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 22ns | +4.4% | +98.2% |
| 1 | 2 | 25ns | -9.3% | +106.7% |
| 1 | 3 | 23ns | -0.9% | +118.2% |
| 1 | 4 | 24ns | -4.4% | +108.8% |
| 1 | 5 | 23ns | +0.9% | +134.9% |
| 1 | 6 | 24ns | -2.5% | +137.3% |
| 1 | 7 | 23ns | +0.9% | +91.0% |
| 1 | 8 | 23ns | -0.9% | +135.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.477 | moderate- |
| ctz_intrinsic | -0.324 | moderate- |
| ctz_loop | -0.338 | moderate- |

**Consistency summary:**

- **ctz_intrinsic**: won 5/8, lost 3/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 2.3ns | 23.8ns | 9.5% | HIGH |
| ctz_intrinsic | 2.2ns | 23.4ns | 9.2% | HIGH |
| ctz_loop | 2.6ns | 50.4ns | 5.2% | HIGH |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 19.0-29.1 ns)
     19.0 |
     19.5 |############################
     20.0 |################
     20.5 |##############
     21.0 |##############
     21.5 |###########
     22.0 |########################################
     22.5 |##############
     23.0 |##############
     23.5 |#########
     24.0 |################
     24.5 |#########################
     25.0 |###########
     25.5 |###########
     26.0 |##############
     26.5 |###########
     27.0 |############################
     27.5 |####
     28.0 |##
     28.5 |####
  (17 below, 18 above range)

ctz_intrinsic (n=160, range 18.8-28.9 ns)
     18.8 |###############
     19.3 |##################
     19.8 |###########################
     20.3 |####################################
     20.9 |############
     21.4 |###############
     21.9 |##############################
     22.4 |########################################
     22.9 |#################################
     23.4 |##################
     23.9 |############
     24.4 |##################
     24.9 |#####################
     25.4 |##############################
     25.9 |#########
     26.4 |######
     26.9 |############
     27.4 |############
     27.9 |#####################
     28.4 |###############
  (15 below, 12 above range)

ctz_loop (n=160, range 42.1-58.0 ns)
     42.1 |
     42.9 |####################
     43.7 |##############
     44.5 |###########
     45.3 |##############
     46.1 |############################
     46.9 |############################
     47.7 |#################
     48.5 |########
     49.3 |##############
     50.1 |########################################
     50.9 |#####
     51.7 |##################################
     52.5 |###############################
     53.3 |############################
     54.1 |######################
     54.9 |##############
     55.7 |##############
     56.5 |#################
     57.3 |###########
  (14 below, 14 above range)

```
