# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 70418% faster than the next best (fib_spectral)

fib_greedy (91 ns) leads fib_spectral (64.31 us) by 70418%, a clear separation rather than a photo finish. CV 8.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 8.6%)

fib_greedy wins on median (91 ns) yet has the highest variance (CV 8.6%), while fib_spectral is the steadiest (CV 2.4%, 64.31 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (91 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 705.2x the fastest

Fastest fib_greedy (91 ns) to slowest fib_spectral (64.31 us): 705.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 91.2 ns median
- 1 variant significantly slower than baseline
- Spread: 705.18x (fastest 91.2 ns, slowest 64312.7 ns)

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
| fib_greedy | 2179ns | 2166ns | 2156ns | 2168ns | 2233ns | base |
| fib_spectral | 66615ns | 66394ns | 65255ns | 66252ns | 69064ns | +2957.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 95ns | 87ns | 108ns | base | 0.169 |
| fib_spectral | 64463ns | 63112ns | 66870ns | +68026.81% | 0.000 |

## Performance model

- Peak throughput: **0.184 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.175 | 95.5% |
| fib_spectral | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2179ns | 2179ns | base |
| fib_spectral | 66615ns | 66615ns | +2957.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 91ns | base | --- | [90, 93] | --- | --- | --- | --- |
| fib_spectral | 64313ns | +64222.6ns (+70419.5%) | [+63555, +64287]ns | [63657, 64390] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 105ns | +61552.9% |
| 1 | 2 | 88ns | +71267.5% |
| 1 | 3 | 89ns | +70840.2% |
| 1 | 4 | 88ns | +73238.6% |
| 1 | 5 | 96ns | +68454.1% |
| 1 | 6 | 92ns | +68528.8% |
| 1 | 7 | 91ns | +70103.7% |
| 1 | 8 | 110ns | +62145.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.103 | ok |
| fib_spectral | -0.081 | ok |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 124.9ns | 94.6ns | 132.0% | HIGH |
| fib_spectral | 127.3ns | 64462.9ns | 0.2% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 87.1-108.4 ns)
     87.1 |################
     88.1 |##################
     89.2 |########################################
     90.3 |#############################
     91.3 |####
     92.4 |####################
     93.5 |##########
     94.5 |######
     95.6 |#######
     96.7 |#
     97.7 |######
     98.8 |
     99.9 |######
    101.0 |###
    102.0 |####
    103.1 |#
    104.2 |###
    105.2 |#########
    106.3 |###
    107.4 |####
  (15 below, 17 above range)

fib_spectral (n=160, range 63112.3-66869.9 ns)
  63112.3 |########################################
  63300.2 |########################################
  63488.1 |####
  63676.0 |####
  63863.8 |###
  64051.7 |
  64239.6 |####################################
  64427.5 |######################
  64615.3 |#########
  64803.2 |#
  64991.1 |#
  65179.0 |########
  65366.9 |####################
  65554.7 |###########
  65742.6 |####
  65930.5 |
  66118.4 |
  66306.3 |
  66494.1 |#
  66682.0 |
  (15 below, 13 above range)

```

## Diagnostics

- **fib_greedy**: bridge=135.5% of algo (FFI overhead may distort results)
