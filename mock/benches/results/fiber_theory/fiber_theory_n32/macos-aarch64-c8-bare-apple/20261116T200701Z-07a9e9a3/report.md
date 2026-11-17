# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 43446% faster than the next best (fib_spectral)

fib_greedy (183 ns) leads fib_spectral (79.65 us) by 43446%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy shows alternating (throttle bounce) (autocorr -0.59)

fib_greedy's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (183 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 435.5x the fastest

Fastest fib_greedy (183 ns) to slowest fib_spectral (79.65 us): 435.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_spectral is inconsistent: worst-20% is 1.5x its best-20%

fib_spectral's best 20% of batches run at 53.63 us but its worst 20% at 82.46 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 182.9 ns median
- 1 variant significantly slower than baseline
- Spread: 435.46x (fastest 182.9 ns, slowest 79646.2 ns)

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
| fib_greedy | 2261ns | 2257ns | 2246ns | 2257ns | 2286ns | base |
| fib_spectral | 75981ns | 81864ns | 55738ns | 79837ns | 84654ns | +3260.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 182ns | 174ns | 190ns | base | 0.175 |
| fib_spectral | 73820ns | 53628ns | 82462ns | +40360.43% | 0.000 |

## Performance model

- Peak throughput: **0.184 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.175 | 95.2% |
| fib_spectral | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2261ns | 2261ns | base |
| fib_spectral | 75981ns | 75981ns | +3260.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 183ns | base | --- | [182, 184] | --- | --- | --- | --- |
| fib_spectral | 79646ns | +79462.2ns (+43445.7%) | [+78643, +80238]ns | [78826, 80421] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 184ns | +43539.1% |
| 1 | 2 | 178ns | +29729.2% |
| 1 | 3 | 191ns | +43111.3% |
| 1 | 4 | 173ns | +44994.1% |
| 1 | 5 | 184ns | +44107.0% |
| 1 | 6 | 187ns | +28838.4% |
| 1 | 7 | 183ns | +43000.3% |
| 1 | 8 | 179ns | +45294.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.590 | HIGH- (thermal bounce) |
| fib_spectral | -0.422 | moderate- |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 236.3ns | 182.5ns | 129.5% | HIGH |
| fib_spectral | 239.1ns | 73820.3ns | 0.3% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 174.2-190.2 ns)
    174.2 |########
    175.0 |##########
    175.8 |##
    176.6 |#####
    177.4 |#############
    178.2 |#####################################
    179.0 |################
    179.8 |##########
    180.6 |################
    181.4 |##################################
    182.2 |#####################
    183.0 |########################################
    183.8 |##########################
    184.6 |################
    185.4 |#############
    186.2 |##########
    187.0 |##########################
    187.8 |##########
    188.6 |########
    189.4 |##########
  (17 below, 16 above range)

fib_spectral (n=160, range 53628.5-82462.1 ns)
  53628.5 |#################
  55070.1 |
  56511.8 |
  57953.5 |
  59395.2 |
  60836.9 |
  62278.5 |
  63720.2 |
  65161.9 |
  66603.6 |
  68045.3 |
  69486.9 |
  70928.6 |
  72370.3 |
  73812.0 |
  75253.7 |
  76695.4 |################
  78137.0 |################
  79578.7 |################
  81020.4 |########################################
  (18 below, 11 above range)

```

## Diagnostics

- **fib_greedy**: bridge=129.0% of algo (FFI overhead may distort results)
