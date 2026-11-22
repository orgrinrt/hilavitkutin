# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 68654% faster than the next best (fib_spectral)

fib_greedy (96 ns) leads fib_spectral (65.80 us) by 68654%, a clear separation rather than a photo finish. CV 82.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 82.1%)

fib_greedy wins on median (96 ns) yet has the highest variance (CV 82.1%), while fib_spectral is the steadiest (CV 10.3%, 65.80 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (96 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 687.5x the fastest

Fastest fib_greedy (96 ns) to slowest fib_spectral (65.80 us): 687.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_greedy is inconsistent: worst-20% is 3.6x its best-20%

fib_greedy's best 20% of batches run at 78 ns but its worst 20% at 276 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 95.7 ns median
- 1 variant significantly slower than baseline
- Spread: 687.54x (fastest 95.7 ns, slowest 65797.3 ns)

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
| Counter frequency | 2112788474 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fib_greedy | 2676ns | 1979ns | 1618ns | 2048ns | 5617ns | base |
| fib_spectral | 67561ns | 67545ns | 60123ns | 67590ns | 74911ns | +2424.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 132ns | 78ns | 276ns | base | 0.121 |
| fib_spectral | 65815ns | 58552ns | 72988ns | +49869.73% | 0.000 |

## Performance model

- Peak throughput: **0.206 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.167 | 81.2% |
| fib_spectral | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2676ns | 2676ns | base |
| fib_spectral | 67561ns | 67561ns | +2424.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 96ns | base | --- | [89, 98] | --- | --- | --- | --- |
| fib_spectral | 65797ns | +65655.8ns (+68605.9%) | [+59500, +71684]ns | [59656, 71815] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 100ns | +72510.3% |
| 1 | 2 | 78ns | +74835.8% |
| 1 | 3 | 82ns | +72273.8% |
| 1 | 4 | 102ns | +70198.1% |
| 1 | 5 | 83ns | +72084.3% |
| 1 | 6 | 80ns | +73999.4% |
| 1 | 7 | 94ns | +77332.1% |
| 1 | 8 | 85ns | +85722.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.398 | moderate- |
| fib_spectral | -0.097 | ok |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 155.2ns | 131.7ns | 117.9% | HIGH |
| fib_spectral | 106.9ns | 65815.1ns | 0.2% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 77.7-276.0 ns)
     77.7 |########################################
     87.6 |##############
     97.5 |##################
    107.5 |
    117.4 |#
    127.3 |
    137.2 |
    147.1 |
    157.0 |
    166.9 |
    176.8 |
    186.8 |
    196.7 |
    206.6 |##
    216.5 |#
    226.4 |##
    236.3 |##
    246.2 |##
    256.2 |###
    266.1 |#
  (11 below, 14 above range)

fib_spectral (n=160, range 58552.3-72987.5 ns)
  58552.3 |###############################
  59274.1 |#########################
  59995.8 |
  60717.6 |
  61439.4 |
  62161.1 |
  62882.9 |
  63604.6 |
  64326.4 |
  65048.2 |
  65769.9 |
  66491.7 |
  67213.4 |
  67935.2 |
  68657.0 |
  69378.7 |
  70100.5 |
  70822.2 |
  71544.0 |#################
  72265.8 |########################################
  (17 below, 17 above range)

```

## Diagnostics

- **fib_greedy**: CV=59.6% (high variance, measurements may be unstable)
- **fib_greedy**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fib_greedy**: bridge=116.5% of algo (FFI overhead may distort results)
