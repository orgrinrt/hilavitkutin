# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 66628% faster than the next best (fib_spectral)

fib_greedy (96 ns) leads fib_spectral (64.19 us) by 66628%, a clear separation rather than a photo finish. CV 8.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 8.8%)

fib_greedy wins on median (96 ns) yet has the highest variance (CV 8.8%), while fib_spectral is the steadiest (CV 1.9%, 64.19 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (96 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 667.3x the fastest

Fastest fib_greedy (96 ns) to slowest fib_spectral (64.19 us): 667.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 96.2 ns median
- 1 variant significantly slower than baseline
- Spread: 667.28x (fastest 96.2 ns, slowest 64192.3 ns)

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
| fib_greedy | 2272ns | 2300ns | 2147ns | 2274ns | 2392ns | base |
| fib_spectral | 66533ns | 66337ns | 65218ns | 66349ns | 68397ns | +2827.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 99ns | 89ns | 112ns | base | 0.161 |
| fib_spectral | 64395ns | 63101ns | 66240ns | +64697.87% | 0.000 |

## Performance model

- Peak throughput: **0.179 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.166 | 92.9% |
| fib_spectral | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2272ns | 2272ns | base |
| fib_spectral | 66533ns | 66533ns | +2827.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 96ns | base | --- | [95, 98] | --- | --- | --- | --- |
| fib_spectral | 64192ns | +64097.8ns (+66629.7%) | [+63681, +64406]ns | [63778, 64509] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 108ns | +59568.9% |
| 1 | 2 | 94ns | +66819.7% |
| 1 | 3 | 88ns | +71798.4% |
| 1 | 4 | 94ns | +68127.8% |
| 1 | 5 | 108ns | +60629.1% |
| 1 | 6 | 93ns | +68083.8% |
| 1 | 7 | 96ns | +65889.9% |
| 1 | 8 | 113ns | +58004.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.094 | ok |
| fib_spectral | -0.136 | ok |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 130.0ns | 99.4ns | 130.8% | HIGH |
| fib_spectral | 134.6ns | 64394.5ns | 0.2% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 89.4-111.9 ns)
     89.4 |#########
     90.5 |#####################
     91.6 |############################
     92.8 |###################################
     93.9 |########################################
     95.0 |################
     96.1 |#######################
     97.3 |##################
     98.4 |
     99.5 |####
    100.6 |#######
    101.7 |#######
    102.9 |
    104.0 |####
    105.1 |###########
    106.2 |#######
    107.4 |################
    108.5 |#########################
    109.6 |####
    110.7 |#########################
  (14 below, 15 above range)

fib_spectral (n=160, range 63101.2-66239.9 ns)
  63101.2 |######
  63258.1 |###################
  63415.1 |########################################
  63572.0 |#################
  63729.0 |########
  63885.9 |######
  64042.8 |##########
  64199.8 |#############
  64356.7 |########
  64513.6 |###########################
  64670.6 |######
  64827.5 |######
  64984.5 |
  65141.4 |#
  65298.3 |######
  65455.3 |####################
  65612.2 |##########################
  65769.1 |###
  65926.1 |#
  66083.0 |
  (17 below, 8 above range)

```

## Diagnostics

- **fib_greedy**: bridge=135.1% of algo (FFI overhead may distort results)
