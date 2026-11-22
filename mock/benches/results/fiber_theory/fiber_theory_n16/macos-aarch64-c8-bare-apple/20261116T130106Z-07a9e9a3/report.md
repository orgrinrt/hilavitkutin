# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 69992% faster than the next best (fib_spectral)

fib_greedy (92 ns) leads fib_spectral (64.27 us) by 69992%, a clear separation rather than a photo finish. CV 8.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 8.5%)

fib_greedy wins on median (92 ns) yet has the highest variance (CV 8.5%), while fib_spectral is the steadiest (CV 1.6%, 64.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (92 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 700.9x the fastest

Fastest fib_greedy (92 ns) to slowest fib_spectral (64.27 us): 700.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 91.7 ns median
- 1 variant significantly slower than baseline
- Spread: 700.92x (fastest 91.7 ns, slowest 64274.0 ns)

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
| fib_greedy | 2168ns | 2164ns | 2154ns | 2164ns | 2194ns | base |
| fib_spectral | 66313ns | 66424ns | 65220ns | 66184ns | 67795ns | +2958.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 94ns | 86ns | 107ns | base | 0.170 |
| fib_spectral | 64150ns | 63073ns | 65603ns | +67996.32% | 0.000 |

## Performance model

- Peak throughput: **0.185 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.174 | 94.1% |
| fib_spectral | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2168ns | 2168ns | base |
| fib_spectral | 66313ns | 66313ns | +2958.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 92ns | base | --- | [91, 93] | --- | --- | --- | --- |
| fib_spectral | 64274ns | +64185.7ns (+69995.3%) | [+63370, +64240]ns | [63475, 64327] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 102ns | +63144.2% |
| 1 | 2 | 88ns | +71692.4% |
| 1 | 3 | 89ns | +70805.9% |
| 1 | 4 | 86ns | +74488.2% |
| 1 | 5 | 96ns | +68232.0% |
| 1 | 6 | 92ns | +68371.6% |
| 1 | 7 | 90ns | +70346.1% |
| 1 | 8 | 110ns | +59061.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.113 | ok |
| fib_spectral | -0.116 | ok |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 123.6ns | 94.2ns | 131.2% | HIGH |
| fib_spectral | 126.4ns | 64149.7ns | 0.2% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 86.3-107.4 ns)
     86.3 |#########
     87.3 |########################################
     88.4 |##############
     89.4 |################
     90.5 |##############
     91.5 |########################
     92.6 |####
     93.7 |#################
     94.7 |####
     95.8 |#########
     96.8 |####
     97.9 |####
     98.9 |####
    100.0 |####
    101.1 |#########
    102.1 |#
    103.2 |####
    104.2 |####
    105.3 |###
    106.3 |###
  (17 below, 17 above range)

fib_spectral (n=160, range 63072.5-65603.3 ns)
  63072.5 |################
  63199.1 |########################################
  63325.6 |###########################
  63452.1 |########
  63578.7 |###
  63705.2 |#
  63831.8 |#
  63958.3 |
  64084.8 |
  64211.4 |###########################
  64337.9 |###########################
  64464.5 |######
  64591.0 |####
  64717.6 |
  64844.1 |#
  64970.6 |
  65097.2 |
  65223.7 |####################
  65350.3 |##############################
  65476.8 |########
  (15 below, 5 above range)

```

## Diagnostics

- **fib_greedy**: bridge=133.4% of algo (FFI overhead may distort results)
