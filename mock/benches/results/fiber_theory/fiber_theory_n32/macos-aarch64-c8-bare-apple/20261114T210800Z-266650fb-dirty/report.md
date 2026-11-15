# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 41830% faster than the next best (fib_spectral)

fib_greedy (191 ns) leads fib_spectral (80.28 us) by 41830%, a clear separation rather than a photo finish. CV 5.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (191 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 419.3x the fastest

Fastest fib_greedy (191 ns) to slowest fib_spectral (80.28 us): 419.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_spectral is inconsistent: worst-20% is 1.6x its best-20%

fib_spectral's best 20% of batches run at 53.63 us but its worst 20% at 83.41 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 191.4 ns median
- 1 variant significantly slower than baseline
- Spread: 419.30x (fastest 191.4 ns, slowest 80275.6 ns)

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
| fib_greedy | 2365ns | 2339ns | 2252ns | 2349ns | 2525ns | base |
| fib_spectral | 76232ns | 82365ns | 55740ns | 79946ns | 85582ns | +3123.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 192ns | 179ns | 208ns | base | 0.167 |
| fib_spectral | 74086ns | 53632ns | 83406ns | +38455.74% | 0.000 |

## Performance model

- Peak throughput: **0.179 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.167 | 93.6% |
| fib_spectral | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2365ns | 2365ns | base |
| fib_spectral | 76232ns | 76232ns | +3123.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 191ns | base | --- | [189, 194] | --- | --- | --- | --- |
| fib_spectral | 80276ns | +80071.2ns (+41823.6%) | [+78651, +80245]ns | [78855, 80459] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 206ns | +38976.6% |
| 1 | 2 | 185ns | +28719.9% |
| 1 | 3 | 192ns | +42896.5% |
| 1 | 4 | 180ns | +43274.6% |
| 1 | 5 | 197ns | +41230.3% |
| 1 | 6 | 190ns | +28516.0% |
| 1 | 7 | 202ns | +38939.6% |
| 1 | 8 | 184ns | +44456.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.477 | moderate- |
| fib_spectral | -0.418 | moderate- |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 248.3ns | 192.2ns | 129.2% | HIGH |
| fib_spectral | 250.6ns | 74086.1ns | 0.3% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 179.2-207.6 ns)
    179.2 |#######################
    180.7 |##########
    182.1 |##########################
    183.5 |####################
    184.9 |####################################
    186.3 |##############################
    187.8 |#######################
    189.2 |#######################
    190.6 |####################################
    192.0 |####################
    193.4 |##########
    194.8 |########################################
    196.3 |####################################
    197.7 |#################################
    199.1 |##########################
    200.5 |###
    201.9 |#############
    203.4 |
    204.8 |
    206.2 |###
  (17 below, 18 above range)

fib_spectral (n=160, range 53631.9-83405.9 ns)
  53631.9 |#########################
  55120.6 |
  56609.3 |
  58098.0 |
  59586.7 |
  61075.4 |
  62564.1 |
  64052.8 |
  65541.5 |
  67030.2 |
  68518.9 |
  70007.6 |
  71496.3 |
  72985.0 |
  74473.7 |
  75962.4 |
  77451.1 |######################################
  78939.8 |###################
  80428.5 |########################################
  81917.2 |############################
  (18 below, 9 above range)

```

## Diagnostics

- **fib_greedy**: bridge=129.9% of algo (FFI overhead may distort results)
