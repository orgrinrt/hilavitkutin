# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 45844% faster than the next best (fib_spectral)

fib_greedy (36 ns) leads fib_spectral (16.63 us) by 45844%, a clear separation rather than a photo finish. CV 8.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 8.5%)

fib_greedy wins on median (36 ns) yet has the highest variance (CV 8.5%), while fib_spectral is the steadiest (CV 2.2%, 16.63 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (36 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 459.4x the fastest

Fastest fib_greedy (36 ns) to slowest fib_spectral (16.63 us): 459.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 36.2 ns median
- 1 variant significantly slower than baseline
- Spread: 459.44x (fastest 36.2 ns, slowest 16631.9 ns)

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
| fib_greedy | 2185ns | 2131ns | 2094ns | 2137ns | 2422ns | base |
| fib_spectral | 18889ns | 18702ns | 18534ns | 18777ns | 19579ns | +764.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 37ns | 33ns | 41ns | base | 0.218 |
| fib_spectral | 16787ns | 16467ns | 17402ns | +45652.53% | 0.000 |

## Performance model

- Peak throughput: **0.244 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.221 | 90.6% |
| fib_spectral | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2185ns | 2185ns | base |
| fib_spectral | 18889ns | 18889ns | +764.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 36ns | base | --- | [35, 37] | --- | --- | --- | --- |
| fib_spectral | 16632ns | +16597.1ns (+45848.3%) | [+16517, +16718]ns | [16555, 16752] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 36ns | +45851.4% |
| 1 | 2 | 40ns | +42306.7% |
| 1 | 3 | 41ns | +40852.0% |
| 1 | 4 | 35ns | +47061.9% |
| 1 | 5 | 36ns | +47257.9% |
| 1 | 6 | 36ns | +46480.8% |
| 1 | 7 | 35ns | +46930.1% |
| 1 | 8 | 35ns | +47423.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | 0.267 | moderate+ |
| fib_spectral | -0.444 | moderate- |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 79.2ns | 36.7ns | 215.8% | HIGH |
| fib_spectral | 77.7ns | 16786.9ns | 0.5% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 32.8-41.4 ns)
     32.8 |################
     33.2 |################
     33.7 |############
     34.1 |##############
     34.5 |################
     35.0 |##########################
     35.4 |############################
     35.8 |##################
     36.2 |
     36.7 |########################################
     37.1 |##########
     37.5 |####
     38.0 |################
     38.4 |##############
     38.8 |########
     39.3 |############
     39.7 |####
     40.1 |##########
     40.6 |##########
     41.0 |##
  (9 below, 13 above range)

fib_spectral (n=160, range 16466.5-17402.1 ns)
  16466.5 |########################################
  16513.3 |############################
  16560.1 |######
  16606.9 |#########
  16653.7 |######
  16700.4 |####
  16747.2 |#####
  16794.0 |#################
  16840.8 |######
  16887.6 |####
  16934.3 |####
  16981.1 |######
  17027.9 |####
  17074.7 |###########
  17121.5 |####
  17168.2 |##
  17215.0 |##
  17261.8 |##
  17308.6 |#
  17355.4 |
  (21 below, 15 above range)

```

## Diagnostics

- **fib_greedy**: bridge=217.4% of algo (FFI overhead may distort results)
