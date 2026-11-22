# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 60% faster than the next best (fxpmul_precise)

fxpmul_hot (1.43 us) leads fxpmul_precise (2.29 us) by 60%, a clear separation rather than a photo finish. CV 39.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_hot is fastest but the noisiest (CV 39.2%)

fxpmul_hot wins on median (1.43 us) yet has the highest variance (CV 39.2%), while fxpmul_precise is the steadiest (CV 13.6%, 2.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (1.43 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_hot is inconsistent: worst-20% is 1.8x its best-20%

fxpmul_hot's best 20% of batches run at 1.17 us but its worst 20% at 2.11 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 1433.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.60x (fastest 1433.0 ns, slowest 2288.1 ns)

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
| Counter frequency | 2112548743 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 3421ns | 3316ns | 2711ns | 3175ns | 4870ns | base |
| fxpmul_precise | 3949ns | 4168ns | 3413ns | 3965ns | 4441ns | +15.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 1478ns | 1172ns | 2106ns | base | 11.083 |
| fxpmul_precise | 2164ns | 1872ns | 2429ns | +46.37% | 7.572 |

## Performance model

- Peak throughput: **13.974 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 11.433 | 81.8% |
| fxpmul_precise | 7.161 | 51.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 3421ns | 3421ns | base |
| fxpmul_precise | 3949ns | 3949ns | +15.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 1433ns | base | --- | [1433, 1433] | --- | --- | --- | --- |
| fxpmul_precise | 2288ns | +854.6ns (+59.6%) | [+758, +855]ns | [2288, 2288] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 1433ns | +59.7% |
| 1 | 2 | 1173ns | +59.7% |
| 1 | 3 | 1433ns | +30.6% |
| 1 | 4 | 1433ns | +59.8% |
| 1 | 5 | 1433ns | +59.7% |
| 1 | 6 | 1172ns | +59.7% |
| 1 | 7 | 1173ns | +95.1% |
| 1 | 8 | 1433ns | +59.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.142 | ok |
| fxpmul_precise | -0.142 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 24.6ns | 1478.3ns | 1.7% |  |
| fxpmul_precise | 22.7ns | 2163.8ns | 1.0% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 1172.5-2105.7 ns)
   1172.5 |###################
   1219.1 |
   1265.8 |
   1312.5 |
   1359.1 |
   1405.8 |########################################
   1452.4 |
   1499.1 |
   1545.8 |
   1592.4 |
   1639.1 |
   1685.8 |
   1732.4 |
   1779.1 |
   1825.7 |
   1872.4 |
   1919.1 |
   1965.7 |
   2012.4 |
   2059.1 |
  (12 below, 10 above range)

fxpmul_precise (n=160, range 1872.1-2428.7 ns)
   1872.1 |##################################
   1899.9 |
   1927.8 |
   1955.6 |
   1983.4 |
   2011.3 |
   2039.1 |
   2066.9 |
   2094.7 |
   2122.6 |
   2150.4 |
   2178.2 |
   2206.1 |
   2233.9 |
   2261.7 |########################################
   2289.6 |#########################
   2317.4 |
   2345.2 |
   2373.1 |
   2400.9 |
  (8 below, 3 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=38.0% (high variance, measurements may be unstable)
