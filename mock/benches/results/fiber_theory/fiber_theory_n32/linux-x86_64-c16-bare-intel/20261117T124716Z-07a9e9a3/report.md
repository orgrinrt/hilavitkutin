# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 44377% faster than the next best (fib_spectral)

fib_greedy (192 ns) leads fib_spectral (85.62 us) by 44377%, a clear separation rather than a photo finish. CV 74.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 74.0%)

fib_greedy wins on median (192 ns) yet has the highest variance (CV 74.0%), while fib_spectral is the steadiest (CV 20.3%, 85.62 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fib_greedy shows warm-up / thermal drift (autocorr +0.64)

fib_greedy's per-pass series has lag-1 autocorrelation +0.64, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (192 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 444.8x the fastest

Fastest fib_greedy (192 ns) to slowest fib_spectral (85.62 us): 444.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_greedy is inconsistent: worst-20% is 3.3x its best-20%

fib_greedy's best 20% of batches run at 151 ns but its worst 20% at 497 ns (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 192.5 ns median
- 1 variant significantly slower than baseline
- Spread: 444.77x (fastest 192.5 ns, slowest 85618.3 ns)

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
| fib_greedy | 2626ns | 2078ns | 1692ns | 2024ns | 5365ns | base |
| fib_spectral | 84371ns | 87379ns | 64106ns | 85729ns | 100565ns | +3113.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 244ns | 151ns | 497ns | base | 0.131 |
| fib_spectral | 82523ns | 62192ns | 98559ns | +33751.18% | 0.000 |

## Performance model

- Peak throughput: **0.212 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.166 | 78.3% |
| fib_spectral | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2626ns | 2626ns | base |
| fib_spectral | 84371ns | 84371ns | +3113.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 192ns | base | --- | [189, 195] | --- | --- | --- | --- |
| fib_spectral | 85618ns | +85041.9ns (+44177.6%) | [+76490, +93232]ns | [76677, 93431] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 162ns | +47131.5% |
| 1 | 2 | 150ns | +41250.8% |
| 1 | 3 | 156ns | +60553.1% |
| 1 | 4 | 193ns | +48531.1% |
| 1 | 5 | 191ns | +49029.1% |
| 1 | 6 | 206ns | +30271.1% |
| 1 | 7 | 199ns | +38262.9% |
| 1 | 8 | 191ns | +48873.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | 0.636 | HIGH+ (drift/warm-up) |
| fib_spectral | -0.032 | ok |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 291.9ns | 243.8ns | 119.8% | HIGH |
| fib_spectral | 226.0ns | 82523.5ns | 0.3% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 150.8-497.4 ns)
    150.8 |#################
    168.1 |######
    185.4 |########################################
    202.8 |##########
    220.1 |
    237.4 |
    254.8 |
    272.1 |
    289.4 |
    306.8 |
    324.1 |
    341.4 |
    358.8 |
    376.1 |
    393.4 |
    410.8 |
    428.1 |#
    445.4 |
    462.8 |#
    480.1 |#
  (15 below, 20 above range)

fib_spectral (n=160, range 62191.6-98558.8 ns)
  62191.6 |###########
  64009.9 |
  65828.3 |
  67646.7 |
  69465.0 |
  71283.4 |
  73101.8 |
  74920.1 |#################
  76738.5 |###
  78556.8 |
  80375.2 |
  82193.6 |
  84011.9 |
  85830.3 |
  87648.6 |
  89467.0 |
  91285.4 |
  93103.7 |########################################
  94922.1 |##
  96740.5 |
  (19 below, 1 above range)

```

## Diagnostics

- **fib_greedy**: CV=58.5% (high variance, measurements may be unstable)
- **fib_greedy**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **fib_greedy**: autocorrelation=0.64 (measurement drift or warm-up artifact)
- **fib_greedy**: bridge=117.3% of algo (FFI overhead may distort results)
- **fib_spectral**: CV=21.0% (high variance, measurements may be unstable)
