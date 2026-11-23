# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 59% faster than the next best (fxpmul_precise)

fxpmul_hot (364 ns) leads fxpmul_precise (578 ns) by 59%, a clear separation rather than a photo finish. CV 78.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_hot is fastest but the noisiest (CV 78.1%)

fxpmul_hot wins on median (364 ns) yet has the highest variance (CV 78.1%), while fxpmul_precise is the steadiest (CV 62.5%, 578 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (214 ns) is smaller than the fastest variant's own run-to-run std-dev (285 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (364 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_hot is inconsistent: worst-20% is 3.1x its best-20%

fxpmul_hot's best 20% of batches run at 298 ns but its worst 20% at 919 ns (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 364.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.59x (fastest 364.3 ns, slowest 578.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate) |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5555666677770007 |
| Counter frequency | 2112817198 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 2817ns | 2243ns | 1834ns | 2170ns | 5740ns | base |
| fxpmul_precise | 2835ns | 2456ns | 2010ns | 2372ns | 5046ns | +0.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 455ns | 298ns | 919ns | base | 9.009 |
| fxpmul_precise | 664ns | 473ns | 1176ns | +46.14% | 6.165 |

## Performance model

- Peak throughput: **13.743 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 11.243 | 81.8% |
| fxpmul_precise | 7.087 | 51.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2817ns | 2817ns | base |
| fxpmul_precise | 2835ns | 2835ns | +0.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 364ns | base | --- | [364, 364] | --- | --- | --- | --- |
| fxpmul_precise | 578ns | +213.7ns (+58.7%) | [+214, +214]ns | [578, 578] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 364ns | +58.6% |
| 1 | 2 | 364ns | +58.7% |
| 1 | 3 | 364ns | +29.8% |
| 1 | 4 | 298ns | +93.9% |
| 1 | 5 | 364ns | +29.8% |
| 1 | 6 | 298ns | +59.2% |
| 1 | 7 | 364ns | +58.7% |
| 1 | 8 | 298ns | +93.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.475 | moderate- |
| fxpmul_precise | -0.145 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 30.1ns | 454.7ns | 6.6% | HIGH |
| fxpmul_precise | 27.5ns | 664.4ns | 4.1% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 298.0-919.3 ns)
    298.0 |#############
    329.1 |
    360.2 |########################################
    391.2 |
    422.3 |
    453.3 |
    484.4 |
    515.5 |
    546.5 |
    577.6 |
    608.7 |
    639.7 |
    670.8 |
    701.8 |
    732.9 |
    764.0 |
    795.0 |
    826.1 |
    857.1 |
    888.2 |
  (20 below, 20 above range)

fxpmul_precise (n=160, range 472.9-1176.0 ns)
    472.9 |####################
    508.1 |
    543.2 |########################################
    578.4 |##
    613.5 |
    648.7 |
    683.8 |
    719.0 |
    754.1 |
    789.3 |
    824.4 |
    859.6 |
    894.7 |
    929.9 |
    965.1 |
   1000.2 |
   1035.4 |
   1070.5 |
   1105.7 |
   1140.8 |
  (7 below, 17 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=62.6% (high variance, measurements may be unstable)
- **fxpmul_hot**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **fxpmul_precise**: CV=54.4% (high variance, measurements may be unstable)
