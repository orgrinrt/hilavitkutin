# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 47500% faster than the next best (fib_spectral)

fib_greedy (35 ns) leads fib_spectral (16.47 us) by 47500%, a clear separation rather than a photo finish. CV 6.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 6.7%)

fib_greedy wins on median (35 ns) yet has the highest variance (CV 6.7%), while fib_spectral is the steadiest (CV 0.2%, 16.47 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (35 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 476.0x the fastest

Fastest fib_greedy (35 ns) to slowest fib_spectral (16.47 us): 476.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 34.6 ns median
- 1 variant significantly slower than baseline
- Spread: 476.00x (fastest 34.6 ns, slowest 16469.5 ns)

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
| fib_greedy | 2120ns | 2107ns | 2099ns | 2107ns | 2180ns | base |
| fib_spectral | 18558ns | 18544ns | 18517ns | 18548ns | 18628ns | +775.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 35ns | 31ns | 38ns | base | 0.232 |
| fib_spectral | 16484ns | 16449ns | 16544ns | +47620.43% | 0.000 |

## Performance model

- Peak throughput: **0.256 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.231 | 90.4% |
| fib_spectral | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2120ns | 2120ns | base |
| fib_spectral | 18558ns | 18558ns | +775.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 35ns | base | --- | [34, 35] | --- | --- | --- | --- |
| fib_spectral | 16470ns | +16434.3ns (+47498.1%) | [+16422, +16462]ns | [16456, 16496] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 34ns | +48940.5% |
| 1 | 2 | 34ns | +48430.6% |
| 1 | 3 | 35ns | +46938.6% |
| 1 | 4 | 34ns | +48152.3% |
| 1 | 5 | 37ns | +44494.6% |
| 1 | 6 | 34ns | +48942.6% |
| 1 | 7 | 35ns | +47464.5% |
| 1 | 8 | 35ns | +47448.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.377 | moderate- |
| fib_spectral | -0.215 | moderate- |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 77.0ns | 34.5ns | 222.8% | HIGH |
| fib_spectral | 76.5ns | 16484.1ns | 0.5% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 31.3-37.7 ns)
     31.3 |
     31.6 |##########
     31.9 |########################
     32.3 |##########################
     32.6 |
     32.9 |#####################
     33.2 |################################
     33.5 |#############################
     33.9 |
     34.2 |################
     34.5 |########################################
     34.8 |##########################
     35.1 |##########################
     35.5 |
     35.8 |########################
     36.1 |##################
     36.4 |########################
     36.7 |
     37.1 |################
     37.4 |#####################
  (12 below, 14 above range)

fib_spectral (n=160, range 16448.8-16544.5 ns)
  16448.8 |########################################
  16453.6 |#####################
  16458.4 |#
  16463.2 |###
  16468.0 |###
  16472.7 |#
  16477.5 |#
  16482.3 |#
  16487.1 |###
  16491.9 |####
  16496.6 |#######################
  16501.4 |##############
  16506.2 |###
  16511.0 |####
  16515.8 |###
  16520.6 |#
  16525.3 |##
  16530.1 |###
  16534.9 |##
  16539.7 |#
  (15 below, 12 above range)

```

## Diagnostics

- **fib_greedy**: bridge=220.4% of algo (FFI overhead may distort results)
