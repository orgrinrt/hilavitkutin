# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 20% faster than the next best (fxpmul_precise)

fxpmul_hot (1.59 us) leads fxpmul_precise (1.91 us) by 20%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (1.59 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 1585.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.20x (fastest 1585.2 ns, slowest 1906.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 3675ns | 3666ns | 3659ns | 3667ns | 3716ns | base |
| fxpmul_precise | 4003ns | 3991ns | 3984ns | 3991ns | 4059ns | +8.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 1588ns | 1579ns | 1606ns | base | 10.317 |
| fxpmul_precise | 1911ns | 1901ns | 1935ns | +20.33% | 8.574 |

## Performance model

- Peak throughput: **10.378 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 10.336 | 99.6% |
| fxpmul_precise | 8.595 | 82.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 3675ns | 3675ns | base |
| fxpmul_precise | 4003ns | 4003ns | +8.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 1585ns | base | --- | [1585, 1586] | --- | --- | --- | --- |
| fxpmul_precise | 1906ns | +321.0ns (+20.3%) | [+320, +322]ns | [1905, 1907] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 1583ns | +20.3% |
| 1 | 2 | 1585ns | +20.3% |
| 1 | 3 | 1584ns | +20.3% |
| 1 | 4 | 1585ns | +20.2% |
| 1 | 5 | 1588ns | +20.0% |
| 1 | 6 | 1586ns | +22.4% |
| 1 | 7 | 1586ns | +20.1% |
| 1 | 8 | 1585ns | +20.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | 0.222 | moderate+ |
| fxpmul_precise | -0.179 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 3.4ns | 1588.1ns | 0.2% |  |
| fxpmul_precise | 3.2ns | 1910.9ns | 0.2% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 1578.7-1606.4 ns)
   1578.7 |###########
   1580.0 |###########
   1581.4 |###################################
   1582.8 |############################
   1584.2 |########################################
   1585.6 |#################################
   1587.0 |##########################
   1588.4 |###############
   1589.7 |###############
   1591.1 |
   1592.5 |###
   1593.9 |
   1595.3 |
   1596.7 |
   1598.1 |#
   1599.4 |#
   1600.8 |#
   1602.2 |
   1603.6 |
   1605.0 |#
  (14 below, 10 above range)

fxpmul_precise (n=160, range 1900.6-1935.2 ns)
   1900.6 |###########################
   1902.3 |#############################
   1904.0 |########################################
   1905.8 |###################################
   1907.5 |#############################
   1909.2 |#######################
   1911.0 |####
   1912.7 |###
   1914.4 |#
   1916.2 |
   1917.9 |
   1919.6 |
   1921.3 |
   1923.1 |
   1924.8 |###
   1926.5 |#
   1928.3 |
   1930.0 |
   1931.7 |###
   1933.5 |#
  (10 below, 18 above range)

```
