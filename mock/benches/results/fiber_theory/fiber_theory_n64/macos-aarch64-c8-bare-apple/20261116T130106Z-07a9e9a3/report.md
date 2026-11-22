# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 32639% faster than the next best (fib_spectral)

fib_greedy (379 ns) leads fib_spectral (124.21 us) by 32639%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy shows alternating (throttle bounce) (autocorr -0.74)

fib_greedy's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (379 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 327.4x the fastest

Fastest fib_greedy (379 ns) to slowest fib_spectral (124.21 us): 327.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 379.4 ns median
- 1 variant significantly slower than baseline
- Spread: 327.39x (fastest 379.4 ns, slowest 124210.2 ns)

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
| fib_greedy | 2468ns | 2464ns | 2436ns | 2468ns | 2503ns | base |
| fib_spectral | 126247ns | 126483ns | 124223ns | 126379ns | 127871ns | +5014.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 381ns | 355ns | 403ns | base | 0.168 |
| fib_spectral | 123981ns | 122026ns | 125567ns | +32459.19% | 0.001 |

## Performance model

- Peak throughput: **0.180 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.169 | 93.6% |
| fib_spectral | 0.001 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2468ns | 2468ns | base |
| fib_spectral | 126247ns | 126247ns | +5014.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 379ns | base | --- | [378, 381] | --- | --- | --- | --- |
| fib_spectral | 124210ns | +123824.1ns (+32636.8%) | [+123606, +124014]ns | [123991, 124392] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 398ns | +31067.9% |
| 1 | 2 | 350ns | +34776.6% |
| 1 | 3 | 400ns | +31147.3% |
| 1 | 4 | 365ns | +33407.7% |
| 1 | 5 | 402ns | +31116.4% |
| 1 | 6 | 379ns | +32697.1% |
| 1 | 7 | 379ns | +32805.2% |
| 1 | 8 | 371ns | +33226.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.741 | HIGH- (thermal bounce) |
| fib_spectral | -0.528 | HIGH- (thermal bounce) |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 490.2ns | 380.8ns | 128.7% | HIGH |
| fib_spectral | 490.3ns | 123981.1ns | 0.4% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 354.9-402.6 ns)
    354.9 |##
    357.3 |
    359.7 |##
    362.1 |##############
    364.5 |##################
    366.9 |##############
    369.2 |##############
    371.6 |############
    374.0 |################
    376.4 |######################
    378.8 |##############################
    381.2 |##############
    383.5 |
    385.9 |##
    388.3 |##
    390.7 |
    393.1 |##
    395.4 |######################
    397.8 |######################################
    400.2 |########################################
  (19 below, 9 above range)

fib_spectral (n=160, range 122026.1-125566.6 ns)
  122026.1 |########################################
  122203.1 |####################################
  122380.1 |##########
  122557.1 |#######
  122734.2 |
  122911.2 |
  123088.2 |###
  123265.2 |##############
  123442.3 |#############################
  123619.3 |################################
  123796.3 |#####################
  123973.4 |#############################
  124150.4 |################################
  124327.4 |####################################
  124504.4 |########################################
  124681.5 |########################################
  124858.5 |#############################
  125035.5 |################################
  125212.6 |#########################
  125389.6 |#############################
  (14 below, 11 above range)

```

## Diagnostics

- **fib_greedy**: bridge=128.4% of algo (FFI overhead may distort results)
