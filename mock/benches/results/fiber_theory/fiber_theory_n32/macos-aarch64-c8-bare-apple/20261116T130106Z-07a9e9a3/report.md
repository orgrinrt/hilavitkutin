# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 43673% faster than the next best (fib_spectral)

fib_greedy (182 ns) leads fib_spectral (79.89 us) by 43673%, a clear separation rather than a photo finish. CV 4.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy shows alternating (throttle bounce) (autocorr -0.61)

fib_greedy's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (182 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 437.7x the fastest

Fastest fib_greedy (182 ns) to slowest fib_spectral (79.89 us): 437.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_spectral is inconsistent: worst-20% is 1.5x its best-20%

fib_spectral's best 20% of batches run at 53.75 us but its worst 20% at 82.88 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 182.5 ns median
- 1 variant significantly slower than baseline
- Spread: 437.73x (fastest 182.5 ns, slowest 79885.8 ns)

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
| fib_greedy | 2270ns | 2257ns | 2245ns | 2257ns | 2334ns | base |
| fib_spectral | 76142ns | 82081ns | 55880ns | 79915ns | 85083ns | +3254.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 184ns | 175ns | 196ns | base | 0.174 |
| fib_spectral | 73971ns | 53749ns | 82876ns | +40183.57% | 0.000 |

## Performance model

- Peak throughput: **0.183 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.175 | 95.7% |
| fib_spectral | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2270ns | 2270ns | base |
| fib_spectral | 76142ns | 76142ns | +3254.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 182ns | base | --- | [181, 184] | --- | --- | --- | --- |
| fib_spectral | 79886ns | +79704.6ns (+43673.8%) | [+78632, +80103]ns | [78815, 80291] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 185ns | +43293.1% |
| 1 | 2 | 178ns | +29765.1% |
| 1 | 3 | 198ns | +41553.0% |
| 1 | 4 | 174ns | +44718.5% |
| 1 | 5 | 184ns | +44179.5% |
| 1 | 6 | 187ns | +28831.5% |
| 1 | 7 | 183ns | +42994.1% |
| 1 | 8 | 179ns | +45441.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.606 | HIGH- (thermal bounce) |
| fib_spectral | -0.421 | moderate- |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 237.2ns | 183.6ns | 129.2% | HIGH |
| fib_spectral | 242.5ns | 73971.5ns | 0.3% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 174.7-196.2 ns)
    174.7 |############
    175.8 |#######
    176.9 |################################
    178.0 |####################
    179.0 |###################################
    180.1 |######################
    181.2 |###########################
    182.2 |#########################
    183.3 |###########################
    184.4 |########################################
    185.5 |############
    186.5 |###############
    187.6 |#######
    188.7 |############
    189.7 |#####
    190.8 |
    191.9 |#####
    193.0 |
    194.0 |#####
    195.1 |##
  (16 below, 18 above range)

fib_spectral (n=160, range 53749.3-82875.8 ns)
  53749.3 |#################
  55205.6 |#####
  56662.0 |
  58118.3 |
  59574.6 |
  61030.9 |
  62487.3 |
  63943.6 |
  65399.9 |
  66856.2 |
  68312.6 |
  69768.9 |
  71225.2 |
  72681.5 |
  74137.9 |
  75594.2 |
  77050.5 |#################
  78506.8 |#################
  79963.2 |########################################
  81419.5 |####################
  (14 below, 9 above range)

```

## Diagnostics

- **fib_greedy**: bridge=129.3% of algo (FFI overhead may distort results)
