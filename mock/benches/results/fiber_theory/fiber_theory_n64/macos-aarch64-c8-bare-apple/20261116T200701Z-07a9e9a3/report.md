# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 32475% faster than the next best (fib_spectral)

fib_greedy (381 ns) leads fib_spectral (124.26 us) by 32475%, a clear separation rather than a photo finish. CV 4.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy shows alternating (throttle bounce) (autocorr -0.74)

fib_greedy's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (381 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 325.8x the fastest

Fastest fib_greedy (381 ns) to slowest fib_spectral (124.26 us): 325.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 381.4 ns median
- 1 variant significantly slower than baseline
- Spread: 325.75x (fastest 381.4 ns, slowest 124258.6 ns)

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
| fib_greedy | 2472ns | 2475ns | 2439ns | 2472ns | 2507ns | base |
| fib_spectral | 126341ns | 126525ns | 124395ns | 126399ns | 128113ns | +5010.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 382ns | 355ns | 403ns | base | 0.168 |
| fib_spectral | 124086ns | 122136ns | 125880ns | +32408.88% | 0.001 |

## Performance model

- Peak throughput: **0.180 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.168 | 93.2% |
| fib_spectral | 0.001 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2472ns | 2472ns | base |
| fib_spectral | 126341ns | 126341ns | +5010.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 381ns | base | --- | [378, 387] | --- | --- | --- | --- |
| fib_spectral | 124259ns | +123869.4ns (+32473.3%) | [+123641, +124042]ns | [124027, 124433] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 396ns | +31207.9% |
| 1 | 2 | 350ns | +34825.5% |
| 1 | 3 | 399ns | +31219.6% |
| 1 | 4 | 366ns | +33330.0% |
| 1 | 5 | 404ns | +31012.2% |
| 1 | 6 | 379ns | +32696.7% |
| 1 | 7 | 386ns | +32191.3% |
| 1 | 8 | 372ns | +33101.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.739 | HIGH- (thermal bounce) |
| fib_spectral | -0.564 | HIGH- (thermal bounce) |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 491.2ns | 381.7ns | 128.7% | HIGH |
| fib_spectral | 493.1ns | 124086.0ns | 0.4% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 355.3-403.4 ns)
    355.3 |######
    357.7 |
    360.1 |#########
    362.5 |###############
    364.9 |########################
    367.3 |##################
    369.8 |#####################
    372.2 |###########################
    374.6 |#########
    377.0 |########################################
    379.4 |#####################
    381.8 |#########
    384.2 |#####################
    386.6 |#####################
    389.0 |######
    391.4 |######
    393.8 |###########################
    396.2 |########################################
    398.6 |####################################
    401.0 |###########################
  (18 below, 15 above range)

fib_spectral (n=160, range 122135.7-125880.4 ns)
  122135.7 |######################
  122323.0 |#########################
  122510.2 |#####
  122697.4 |#####
  122884.7 |
  123071.9 |#######
  123259.1 |#######
  123446.4 |###############
  123633.6 |############
  123820.8 |################################
  124008.1 |######################
  124195.3 |###################################
  124382.5 |########################################
  124569.8 |######################
  124757.0 |######################
  124944.2 |#################
  125131.5 |####################
  125318.7 |############
  125505.9 |############
  125693.2 |#######
  (14 below, 8 above range)

```

## Diagnostics

- **fib_greedy**: bridge=128.4% of algo (FFI overhead may distort results)
