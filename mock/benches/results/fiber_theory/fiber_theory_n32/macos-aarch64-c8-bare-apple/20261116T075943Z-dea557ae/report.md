# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 44180% faster than the next best (fib_spectral)

fib_greedy (185 ns) leads fib_spectral (82.10 us) by 44180%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy shows alternating (throttle bounce) (autocorr -0.55)

fib_greedy's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (185 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 442.8x the fastest

Fastest fib_greedy (185 ns) to slowest fib_spectral (82.10 us): 442.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_spectral is inconsistent: worst-20% is 1.6x its best-20%

fib_spectral's best 20% of batches run at 54.91 us but its worst 20% at 85.79 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 185.4 ns median
- 1 variant significantly slower than baseline
- Spread: 442.80x (fastest 185.4 ns, slowest 82095.4 ns)

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
| fib_greedy | 2308ns | 2296ns | 2279ns | 2298ns | 2367ns | base |
| fib_spectral | 78205ns | 84291ns | 57071ns | 81970ns | 88045ns | +3287.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 187ns | 178ns | 196ns | base | 0.172 |
| fib_spectral | 76011ns | 54910ns | 85793ns | +40638.23% | 0.000 |

## Performance model

- Peak throughput: **0.180 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.173 | 96.0% |
| fib_spectral | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2308ns | 2308ns | base |
| fib_spectral | 78205ns | 78205ns | +3287.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 185ns | base | --- | [185, 187] | --- | --- | --- | --- |
| fib_spectral | 82095ns | +81910.9ns (+44180.6%) | [+80714, +82734]ns | [80914, 82925] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 187ns | +44013.3% |
| 1 | 2 | 182ns | +29827.7% |
| 1 | 3 | 196ns | +43054.3% |
| 1 | 4 | 177ns | +45011.8% |
| 1 | 5 | 192ns | +43306.1% |
| 1 | 6 | 193ns | +28928.8% |
| 1 | 7 | 185ns | +43644.8% |
| 1 | 8 | 182ns | +45648.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.546 | HIGH- (thermal bounce) |
| fib_spectral | -0.427 | moderate- |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 243.8ns | 186.6ns | 130.7% | HIGH |
| fib_spectral | 260.0ns | 76010.9ns | 0.3% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 177.9-196.2 ns)
    177.9 |###############
    178.9 |############
    179.8 |##################
    180.7 |########################
    181.6 |#####################
    182.5 |###########################
    183.4 |########################################
    184.3 |##############################
    185.2 |########################
    186.1 |########################
    187.1 |#####################
    188.0 |###############
    188.9 |############
    189.8 |
    190.7 |##################
    191.6 |########################
    192.5 |###############
    193.4 |###############
    194.3 |###############
    195.2 |#####################
  (15 below, 15 above range)

fib_spectral (n=160, range 54910.2-85793.4 ns)
  54910.2 |###############
  56454.4 |###
  57998.5 |
  59542.7 |
  61086.9 |
  62631.0 |
  64175.2 |
  65719.3 |
  67263.5 |
  68807.7 |
  70351.8 |
  71896.0 |
  73440.1 |
  74984.3 |
  76528.5 |
  78072.6 |####
  79616.8 |#######################
  81160.9 |#############
  82705.1 |########################################
  84249.3 |##############
  (18 below, 7 above range)

```

## Diagnostics

- **fib_greedy**: bridge=131.1% of algo (FFI overhead may distort results)
