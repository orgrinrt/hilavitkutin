# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 17% faster than the next best (fxpmul_precise)

fxpmul_hot (1.70 us) leads fxpmul_precise (1.98 us) by 17%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (1.70 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 1696.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.17x (fastest 1696.2 ns, slowest 1979.2 ns)

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
| fxpmul_hot | 3878ns | 3919ns | 3668ns | 3894ns | 4040ns | base |
| fxpmul_precise | 4156ns | 4146ns | 3984ns | 4142ns | 4369ns | +7.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 1674ns | 1588ns | 1730ns | base | 9.790 |
| fxpmul_precise | 1989ns | 1904ns | 2097ns | +18.83% | 8.239 |

## Performance model

- Peak throughput: **10.316 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 9.659 | 93.6% |
| fxpmul_precise | 8.278 | 80.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 3878ns | 3878ns | base |
| fxpmul_precise | 4156ns | 4156ns | +7.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 1696ns | base | --- | [1694, 1698] | --- | --- | --- | --- |
| fxpmul_precise | 1979ns | +319.4ns (+18.8%) | [+292, +340]ns | [1966, 1998] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 1642ns | +19.3% |
| 1 | 2 | 1687ns | +21.2% |
| 1 | 3 | 1708ns | +11.7% |
| 1 | 4 | 1700ns | +13.1% |
| 1 | 5 | 1698ns | +20.3% |
| 1 | 6 | 1702ns | +16.4% |
| 1 | 7 | 1627ns | +25.4% |
| 1 | 8 | 1593ns | +20.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | 0.387 | moderate+ |
| fxpmul_precise | -0.363 | moderate- |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 3.6ns | 1673.5ns | 0.2% |  |
| fxpmul_precise | 3.3ns | 1988.6ns | 0.2% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 1588.2-1729.6 ns)
   1588.2 |#######
   1595.2 |####
   1602.3 |#
   1609.4 |#####
   1616.5 |##
   1623.5 |
   1630.6 |#
   1637.7 |#######
   1644.8 |######
   1651.8 |##
   1658.9 |
   1666.0 |
   1673.0 |##
   1680.1 |#####
   1687.2 |###########
   1694.3 |########################################
   1701.3 |##############################
   1708.4 |#####
   1715.5 |##
   1722.5 |####
  (19 below, 8 above range)

fxpmul_precise (n=160, range 1903.8-2097.0 ns)
   1903.8 |###################################
   1913.4 |########
   1923.1 |#####
   1932.8 |###
   1942.4 |##
   1952.1 |#
   1961.7 |#####
   1971.4 |###########
   1981.1 |#########
   1990.7 |###
   2000.4 |
   2010.0 |##
   2019.7 |#
   2029.4 |#######
   2039.0 |########################################
   2048.7 |###
   2058.3 |##
   2068.0 |#
   2077.7 |
   2087.3 |#
  (11 below, 10 above range)

```
