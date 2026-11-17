# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 20% faster than the next best (fxpmul_precise)

fxpmul_hot (1.58 us) leads fxpmul_precise (1.91 us) by 20%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (1.58 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 1585.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.20x (fastest 1585.0 ns, slowest 1906.7 ns)

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
| fxpmul_hot | 3677ns | 3669ns | 3661ns | 3669ns | 3715ns | base |
| fxpmul_precise | 4011ns | 3992ns | 3983ns | 3994ns | 4092ns | +9.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 1588ns | 1579ns | 1604ns | base | 10.319 |
| fxpmul_precise | 1915ns | 1901ns | 1956ns | +20.63% | 8.554 |

## Performance model

- Peak throughput: **10.374 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 10.337 | 99.6% |
| fxpmul_precise | 8.593 | 82.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 3677ns | 3677ns | base |
| fxpmul_precise | 4011ns | 4011ns | +9.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 1585ns | base | --- | [1584, 1586] | --- | --- | --- | --- |
| fxpmul_precise | 1907ns | +321.3ns (+20.3%) | [+320, +322]ns | [1906, 1907] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 1583ns | +20.3% |
| 1 | 2 | 1586ns | +20.1% |
| 1 | 3 | 1585ns | +20.3% |
| 1 | 4 | 1585ns | +20.3% |
| 1 | 5 | 1585ns | +20.4% |
| 1 | 6 | 1584ns | +23.3% |
| 1 | 7 | 1588ns | +20.0% |
| 1 | 8 | 1586ns | +20.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.228 | moderate- |
| fxpmul_precise | -0.153 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 3.2ns | 1587.8ns | 0.2% |  |
| fxpmul_precise | 3.3ns | 1915.4ns | 0.2% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 1579.3-1604.3 ns)
   1579.3 |######################
   1580.5 |########################
   1581.8 |##############################
   1583.0 |########################################
   1584.3 |########################################
   1585.5 |##########################
   1586.8 |################################
   1588.0 |####################
   1589.3 |################
   1590.5 |##################
   1591.8 |####
   1593.0 |####
   1594.3 |
   1595.5 |##
   1596.8 |
   1598.0 |##
   1599.3 |##
   1600.5 |
   1601.8 |##
   1603.0 |##
  (12 below, 5 above range)

fxpmul_precise (n=160, range 1901.1-1955.6 ns)
   1901.1 |#################################
   1903.9 |#################################
   1906.6 |########################################
   1909.3 |##########
   1912.0 |######
   1914.7 |#
   1917.5 |##
   1920.2 |#
   1922.9 |#
   1925.6 |
   1928.4 |
   1931.1 |##
   1933.8 |#
   1936.5 |#
   1939.2 |#
   1942.0 |#
   1944.7 |
   1947.4 |#
   1950.1 |
   1952.9 |#
  (9 below, 16 above range)

```
