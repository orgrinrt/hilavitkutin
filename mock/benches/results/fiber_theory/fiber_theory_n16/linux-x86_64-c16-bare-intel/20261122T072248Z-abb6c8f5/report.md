# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 65141% faster than the next best (fib_spectral)

fib_greedy (91 ns) leads fib_spectral (59.37 us) by 65141%, a clear separation rather than a photo finish. CV 90.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 90.3%)

fib_greedy wins on median (91 ns) yet has the highest variance (CV 90.3%), while fib_spectral is the steadiest (CV 0.9%, 59.37 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (91 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 652.4x the fastest

Fastest fib_greedy (91 ns) to slowest fib_spectral (59.37 us): 652.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_greedy is inconsistent: worst-20% is 3.6x its best-20%

fib_greedy's best 20% of batches run at 79 ns but its worst 20% at 285 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 91.0 ns median
- 1 variant significantly slower than baseline
- Spread: 652.41x (fastest 91.0 ns, slowest 59369.2 ns)

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
| Counter frequency | 2112925964 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fib_greedy | 2791ns | 1970ns | 1616ns | 2182ns | 5794ns | base |
| fib_spectral | 60939ns | 60950ns | 60237ns | 60960ns | 61578ns | +2083.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 138ns | 79ns | 285ns | base | 0.116 |
| fib_spectral | 59363ns | 58661ns | 60005ns | +42911.13% | 0.000 |

## Performance model

- Peak throughput: **0.203 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.176 | 86.7% |
| fib_spectral | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2791ns | 2791ns | base |
| fib_spectral | 60939ns | 60939ns | +2083.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 91ns | base | --- | [84, 96] | --- | --- | --- | --- |
| fib_spectral | 59369ns | +59272.9ns (+65135.0%) | [+59136, +59362]ns | [59241, 59482] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 82ns | +72069.4% |
| 1 | 2 | 94ns | +62235.0% |
| 1 | 3 | 79ns | +74940.2% |
| 1 | 4 | 157ns | +37363.0% |
| 1 | 5 | 82ns | +72683.2% |
| 1 | 6 | 80ns | +73410.7% |
| 1 | 7 | 225ns | +26531.3% |
| 1 | 8 | 85ns | +70083.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.371 | moderate- |
| fib_spectral | -0.039 | ok |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 169.8ns | 138.0ns | 123.0% | HIGH |
| fib_spectral | 95.7ns | 59362.9ns | 0.2% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 78.9-285.4 ns)
     78.9 |########################################
     89.3 |#############
     99.6 |
    109.9 |
    120.2 |
    130.5 |#
    140.9 |
    151.2 |###########
    161.5 |#
    171.8 |#
    182.2 |#
    192.5 |
    202.8 |
    213.1 |#
    223.5 |#
    233.8 |#
    244.1 |###
    254.4 |#
    264.8 |
    275.1 |##
  (15 below, 14 above range)

fib_spectral (n=160, range 58661.1-60004.6 ns)
  58661.1 |##################
  58728.3 |##
  58795.5 |##########
  58862.7 |#############
  58929.8 |################################
  58997.0 |################
  59064.2 |################
  59131.4 |#############
  59198.5 |#############################
  59265.7 |##########
  59332.9 |########################
  59400.1 |##################
  59467.2 |################
  59534.4 |##########
  59601.6 |#############
  59668.8 |##################
  59735.9 |##################
  59803.1 |########################################
  59870.3 |##################################
  59937.5 |#############
  (14 below, 7 above range)

```

## Diagnostics

- **fib_greedy**: CV=59.5% (high variance, measurements may be unstable)
- **fib_greedy**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fib_greedy**: bridge=117.0% of algo (FFI overhead may distort results)
