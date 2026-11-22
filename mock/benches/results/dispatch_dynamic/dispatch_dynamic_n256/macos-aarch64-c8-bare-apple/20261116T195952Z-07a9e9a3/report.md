# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 182% faster than the next best (dispatch_volatile)

dispatch_direct (15 ns) leads dispatch_volatile (41 ns) by 182%, a clear separation rather than a photo finish. CV 15.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 3.0x slower than the field

dispatch_runtime (43 ns) is 3.0x the fastest (15 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dispatch_direct is fastest but the noisiest (CV 15.9%)

dispatch_direct wins on median (15 ns) yet has the highest variance (CV 15.9%), while dispatch_runtime is the steadiest (CV 9.0%, 43 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (15 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_direct is inconsistent: worst-20% is 1.5x its best-20%

dispatch_direct's best 20% of batches run at 12 ns but its worst 20% at 18 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 14.6 ns median
- 2 variants significantly slower than baseline
- Spread: 2.97x (fastest 14.6 ns, slowest 43.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x40ffee15d34db33f |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 2112ns | 2106ns | 2101ns | 2106ns | 2142ns | base |
| dispatch_runtime | 2123ns | 2119ns | 2113ns | 2119ns | 2141ns | +0.49% |
| dispatch_volatile | 2125ns | 2119ns | 2114ns | 2119ns | 2154ns | +0.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 15ns | 12ns | 18ns | base | 17.240 |
| dispatch_runtime | 43ns | 37ns | 48ns | +188.32% | 5.979 |
| dispatch_volatile | 41ns | 36ns | 47ns | +178.98% | 6.180 |

## Performance model

- Peak throughput: **21.530 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 17.534 | 81.4% |
| dispatch_runtime | 5.912 | 27.5% |
| dispatch_volatile | 6.214 | 28.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2112ns | 2112ns | base |
| dispatch_runtime | 2123ns | 2123ns | +0.49% |
| dispatch_volatile | 2125ns | 2125ns | +0.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 15ns | base | --- | [14, 15] | --- | --- | --- | --- |
| dispatch_runtime | 43ns | +27.9ns (+191.1%) | [+28, +29]ns | [42, 44] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 41ns | +26.6ns (+182.2%) | [+26, +27]ns | [41, 42] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 14ns | +186.1% | +186.1% |
| 1 | 2 | 15ns | +196.6% | +183.9% |
| 1 | 3 | 15ns | +194.7% | +170.7% |
| 1 | 4 | 14ns | +192.4% | +187.8% |
| 1 | 5 | 15ns | +181.2% | +170.5% |
| 1 | 6 | 15ns | +188.7% | +178.0% |
| 1 | 7 | 15ns | +178.3% | +175.7% |
| 1 | 8 | 15ns | +204.1% | +183.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.182 | ok |
| dispatch_runtime | -0.321 | moderate- |
| dispatch_volatile | 0.150 | ok |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 3.9ns | 14.8ns | 26.1% | HIGH |
| dispatch_runtime | 3.6ns | 42.8ns | 8.3% | HIGH |
| dispatch_volatile | 3.3ns | 41.4ns | 7.9% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 11.9-18.2 ns)
     11.9 |##################
     12.2 |##########################
     12.5 |
     12.8 |################################
     13.2 |########################
     13.5 |
     13.8 |##################
     14.1 |##########################
     14.4 |########################################
     14.8 |#############################
     15.1 |
     15.4 |##################################
     15.7 |#####################
     16.0 |##################
     16.3 |
     16.7 |#############################
     17.0 |################
     17.3 |#############
     17.6 |#############
     17.9 |
  (13 below, 11 above range)

dispatch_runtime (n=160, range 37.4-48.2 ns)
     37.4 |#######
     38.0 |############
     38.5 |#######
     39.1 |#######
     39.6 |########################################
     40.1 |#################
     40.7 |###########################
     41.2 |#######
     41.7 |############
     42.3 |##########
     42.8 |########################################
     43.4 |###############
     43.9 |#################
     44.4 |####################
     45.0 |########################################
     45.5 |#################
     46.1 |##
     46.6 |######################
     47.1 |#####
     47.7 |##
  (14 below, 13 above range)

dispatch_volatile (n=160, range 36.0-46.7 ns)
     36.0 |#########
     36.5 |###
     37.1 |#####################
     37.6 |#########
     38.1 |########################
     38.7 |###########################
     39.2 |#####################
     39.7 |#####################
     40.3 |###########################
     40.8 |########################################
     41.4 |#####################
     41.9 |#####################
     42.4 |########################################
     43.0 |###########################
     43.5 |#########
     44.0 |###############
     44.6 |#################################
     45.1 |########################
     45.7 |############
     46.2 |######
  (14 below, 10 above range)

```

## Diagnostics

- **dispatch_direct**: bridge=25.3% of algo (FFI overhead may distort results)
