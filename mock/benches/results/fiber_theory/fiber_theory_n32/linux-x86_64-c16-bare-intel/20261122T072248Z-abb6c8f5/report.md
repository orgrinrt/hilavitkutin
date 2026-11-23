# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 40178% faster than the next best (fib_spectral)

fib_greedy (190 ns) leads fib_spectral (76.57 us) by 40178%, a clear separation rather than a photo finish. CV 72.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 72.6%)

fib_greedy wins on median (190 ns) yet has the highest variance (CV 72.6%), while fib_spectral is the steadiest (CV 20.1%, 76.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (190 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 402.8x the fastest

Fastest fib_greedy (190 ns) to slowest fib_spectral (76.57 us): 402.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_greedy is inconsistent: worst-20% is 3.1x its best-20%

fib_greedy's best 20% of batches run at 152 ns but its worst 20% at 478 ns (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 190.1 ns median
- 1 variant significantly slower than baseline
- Spread: 402.78x (fastest 190.1 ns, slowest 76568.4 ns)

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
| fib_greedy | 2641ns | 2078ns | 1692ns | 2122ns | 5149ns | base |
| fib_spectral | 76098ns | 78141ns | 52275ns | 77427ns | 95933ns | +2781.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 244ns | 152ns | 478ns | base | 0.131 |
| fib_spectral | 74434ns | 50698ns | 94004ns | +30366.17% | 0.000 |

## Performance model

- Peak throughput: **0.211 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.168 | 80.0% |
| fib_spectral | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2641ns | 2641ns | base |
| fib_spectral | 76098ns | 76098ns | +2781.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 190ns | base | --- | [187, 192] | --- | --- | --- | --- |
| fib_spectral | 76568ns | +76373.4ns (+40175.4%) | [+76322, +76431]ns | [76525, 76619] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 188ns | +40640.2% |
| 1 | 2 | 149ns | +33830.3% |
| 1 | 3 | 159ns | +59243.8% |
| 1 | 4 | 192ns | +39908.1% |
| 1 | 5 | 156ns | +48864.3% |
| 1 | 6 | 326ns | +15552.4% |
| 1 | 7 | 199ns | +38335.5% |
| 1 | 8 | 190ns | +49094.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.107 | ok |
| fib_spectral | -0.286 | moderate- |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 311.8ns | 244.3ns | 127.6% | HIGH |
| fib_spectral | 202.9ns | 74434.0ns | 0.3% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 152.0-477.7 ns)
    152.0 |########################
    168.3 |##
    184.6 |########################################
    200.9 |###
    217.1 |
    233.4 |
    249.7 |
    266.0 |
    282.3 |
    298.6 |#
    314.9 |#############
    331.2 |
    347.4 |
    363.7 |
    380.0 |
    396.3 |
    412.6 |
    428.9 |
    445.2 |
    461.5 |#
  (14 below, 15 above range)

fib_spectral (n=160, range 50698.5-94003.7 ns)
  50698.5 |############
  52863.7 |
  55029.0 |
  57194.3 |
  59359.5 |
  61524.8 |
  63690.1 |
  65855.3 |
  68020.6 |
  70185.9 |
  72351.1 |
  74516.4 |########################################
  76681.6 |######
  78846.9 |
  81012.2 |
  83177.4 |
  85342.7 |
  87508.0 |
  89673.2 |
  91838.5 |############
  (19 below, 19 above range)

```

## Diagnostics

- **fib_greedy**: CV=56.5% (high variance, measurements may be unstable)
- **fib_greedy**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **fib_greedy**: bridge=120.6% of algo (FFI overhead may distort results)
- **fib_spectral**: CV=20.6% (high variance, measurements may be unstable)
