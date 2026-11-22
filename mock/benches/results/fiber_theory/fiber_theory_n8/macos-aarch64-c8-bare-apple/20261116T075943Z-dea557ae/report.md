# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 48602% faster than the next best (fib_spectral)

fib_greedy (35 ns) leads fib_spectral (16.85 us) by 48602%, a clear separation rather than a photo finish. CV 6.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 6.2%)

fib_greedy wins on median (35 ns) yet has the highest variance (CV 6.2%), while fib_spectral is the steadiest (CV 3.1%, 16.85 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (35 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 487.0x the fastest

Fastest fib_greedy (35 ns) to slowest fib_spectral (16.85 us): 487.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 34.6 ns median
- 1 variant significantly slower than baseline
- Spread: 487.02x (fastest 34.6 ns, slowest 16850.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fiber grouping theories: greedy topo walk vs spectral bisection |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3f1be00000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fib_greedy | 2145ns | 2139ns | 2112ns | 2141ns | 2191ns | base |
| fib_spectral | 19060ns | 18961ns | 18823ns | 18978ns | 19544ns | +788.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 35ns | 32ns | 38ns | base | 0.230 |
| fib_spectral | 16936ns | 16727ns | 17371ns | +48694.18% | 0.000 |

## Performance model

- Peak throughput: **0.252 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.231 | 91.6% |
| fib_spectral | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2145ns | 2145ns | base |
| fib_spectral | 19060ns | 19060ns | +788.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 35ns | base | --- | [34, 35] | --- | --- | --- | --- |
| fib_spectral | 16851ns | +16815.8ns (+48600.7%) | [+16796, +16835]ns | [16832, 16871] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 35ns | +48677.7% |
| 1 | 2 | 35ns | +48054.3% |
| 1 | 3 | 35ns | +48073.3% |
| 1 | 4 | 34ns | +49287.7% |
| 1 | 5 | 35ns | +47525.8% |
| 1 | 6 | 35ns | +48296.8% |
| 1 | 7 | 34ns | +49548.1% |
| 1 | 8 | 36ns | +46334.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.494 | moderate- |
| fib_spectral | 0.405 | moderate+ |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 78.7ns | 34.7ns | 226.9% | HIGH |
| fib_spectral | 79.3ns | 16936.2ns | 0.5% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 31.7-37.5 ns)
     31.7 |#####################
     32.0 |#####################
     32.3 |################
     32.6 |
     32.9 |##################################
     33.2 |##########################
     33.4 |
     33.7 |#############################
     34.0 |#####################
     34.3 |#############################
     34.6 |
     34.9 |########################
     35.2 |##################
     35.5 |
     35.8 |##################################
     36.1 |##########################
     36.3 |
     36.6 |########################################
     36.9 |#############################
     37.2 |#############
  (6 below, 9 above range)

fib_spectral (n=160, range 16726.7-17370.7 ns)
  16726.7 |###
  16758.9 |######################################
  16791.1 |########################################
  16823.3 |###################################
  16855.5 |###########################
  16887.7 |##################
  16919.9 |######
  16952.1 |############
  16984.3 |#########
  17016.5 |###
  17048.7 |#######
  17080.9 |####
  17113.1 |####
  17145.3 |###
  17177.5 |#
  17209.7 |#
  17241.9 |###
  17274.1 |###
  17306.3 |####
  17338.5 |
  (8 below, 4 above range)

```

## Diagnostics

- **fib_greedy**: bridge=222.8% of algo (FFI overhead may distort results)
