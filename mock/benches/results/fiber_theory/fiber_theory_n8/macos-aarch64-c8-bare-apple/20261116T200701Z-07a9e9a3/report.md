# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 47584% faster than the next best (fib_spectral)

fib_greedy (35 ns) leads fib_spectral (16.50 us) by 47584%, a clear separation rather than a photo finish. CV 6.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 6.0%)

fib_greedy wins on median (35 ns) yet has the highest variance (CV 6.0%), while fib_spectral is the steadiest (CV 0.7%, 16.50 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (35 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 476.8x the fastest

Fastest fib_greedy (35 ns) to slowest fib_spectral (16.50 us): 476.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 34.6 ns median
- 1 variant significantly slower than baseline
- Spread: 476.84x (fastest 34.6 ns, slowest 16498.8 ns)

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
| fib_greedy | 2119ns | 2108ns | 2100ns | 2109ns | 2167ns | base |
| fib_spectral | 18610ns | 18575ns | 18524ns | 18576ns | 18796ns | +778.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 34ns | 31ns | 37ns | base | 0.233 |
| fib_spectral | 16530ns | 16453ns | 16699ns | +48032.55% | 0.000 |

## Performance model

- Peak throughput: **0.255 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.231 | 90.6% |
| fib_spectral | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2119ns | 2119ns | base |
| fib_spectral | 18610ns | 18610ns | +778.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 35ns | base | --- | [34, 35] | --- | --- | --- | --- |
| fib_spectral | 16499ns | +16464.4ns (+47585.0%) | [+16462, +16471]ns | [16496, 16506] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 35ns | +47573.4% |
| 1 | 2 | 34ns | +48172.4% |
| 1 | 3 | 34ns | +47767.6% |
| 1 | 4 | 35ns | +47234.8% |
| 1 | 5 | 34ns | +48439.9% |
| 1 | 6 | 35ns | +47031.4% |
| 1 | 7 | 36ns | +46057.5% |
| 1 | 8 | 34ns | +48945.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.289 | moderate- |
| fib_spectral | 0.320 | moderate+ |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 77.2ns | 34.3ns | 224.8% | HIGH |
| fib_spectral | 76.7ns | 16529.9ns | 0.5% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 31.4-37.0 ns)
     31.4 |
     31.6 |#########
     31.9 |#####################
     32.2 |
     32.5 |################
     32.8 |############################
     33.0 |##################
     33.3 |
     33.6 |################
     33.9 |
     34.2 |########################################
     34.5 |##############################
     34.7 |########################################
     35.0 |
     35.3 |#######################
     35.6 |###################################
     35.9 |
     36.1 |#####################
     36.4 |#####################
     36.7 |
  (12 below, 11 above range)

fib_spectral (n=160, range 16453.0-16698.7 ns)
  16453.0 |######################################
  16465.3 |################
  16477.6 |############
  16489.9 |########################################
  16502.2 |#######################
  16514.4 |#####################
  16526.7 |#######
  16539.0 |#############
  16551.3 |#########
  16563.6 |#
  16575.9 |#
  16588.2 |######
  16600.5 |#
  16612.7 |
  16625.0 |###
  16637.3 |#
  16649.6 |#
  16661.9 |
  16674.2 |
  16686.5 |#
  (17 below, 12 above range)

```

## Diagnostics

- **fib_greedy**: bridge=220.2% of algo (FFI overhead may distort results)
