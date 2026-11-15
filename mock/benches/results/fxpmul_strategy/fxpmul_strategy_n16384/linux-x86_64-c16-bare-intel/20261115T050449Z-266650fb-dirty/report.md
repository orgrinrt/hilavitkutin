# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 60% faster than the next best (fxpmul_precise)

fxpmul_hot (1.43 us) leads fxpmul_precise (2.29 us) by 60%, a clear separation rather than a photo finish. CV 41.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_hot is fastest but the noisiest (CV 41.1%)

fxpmul_hot wins on median (1.43 us) yet has the highest variance (CV 41.1%), while fxpmul_precise is the steadiest (CV 16.5%, 2.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (1.43 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_hot is inconsistent: worst-20% is 1.6x its best-20%

fxpmul_hot's best 20% of batches run at 1.43 us but its worst 20% at 2.23 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 1433.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.60x (fastest 1433.1 ns, slowest 2288.1 ns)

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
| Counter frequency | 2112754895 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 3691ns | 3316ns | 3308ns | 3317ns | 5195ns | base |
| fxpmul_precise | 4092ns | 4170ns | 3412ns | 4125ns | 4675ns | +10.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 1593ns | 1433ns | 2232ns | base | 10.284 |
| fxpmul_precise | 2243ns | 1873ns | 2551ns | +40.76% | 7.306 |

## Performance model

- Peak throughput: **11.434 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 11.433 | 100.0% |
| fxpmul_precise | 7.161 | 62.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 3691ns | 3691ns | base |
| fxpmul_precise | 4092ns | 4092ns | +10.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 1433ns | base | --- | [1433, 1433] | --- | --- | --- | --- |
| fxpmul_precise | 2288ns | +854.9ns (+59.7%) | [+855, +855]ns | [2288, 2288] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 1433ns | +59.7% |
| 1 | 2 | 1433ns | +59.7% |
| 1 | 3 | 1433ns | +30.6% |
| 1 | 4 | 1433ns | +59.7% |
| 1 | 5 | 1433ns | +59.7% |
| 1 | 6 | 1433ns | +30.6% |
| 1 | 7 | 1433ns | +59.7% |
| 1 | 8 | 1433ns | +59.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.327 | moderate- |
| fxpmul_precise | -0.375 | moderate- |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 26.6ns | 1593.2ns | 1.7% |  |
| fxpmul_precise | 23.4ns | 2242.5ns | 1.0% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 1432.9-2231.9 ns)
   1432.9 |########################################
   1472.9 |
   1512.8 |
   1552.8 |
   1592.7 |
   1632.7 |
   1672.6 |
   1712.6 |
   1752.5 |
   1792.5 |
   1832.4 |
   1872.4 |
   1912.3 |
   1952.3 |
   1992.2 |
   2032.2 |
   2072.1 |
   2112.1 |
   2152.0 |
   2192.0 |
  (20 below, 11 above range)

fxpmul_precise (n=160, range 1873.0-2551.5 ns)
   1873.0 |###
   1907.0 |
   1940.9 |
   1974.8 |
   2008.7 |
   2042.7 |
   2076.6 |
   2110.5 |
   2144.4 |
   2178.3 |
   2212.3 |
   2246.2 |
   2280.1 |########################################
   2314.0 |
   2348.0 |
   2381.9 |
   2415.8 |
   2449.7 |
   2483.6 |
   2517.6 |
  (29 below, 4 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=36.9% (high variance, measurements may be unstable)
