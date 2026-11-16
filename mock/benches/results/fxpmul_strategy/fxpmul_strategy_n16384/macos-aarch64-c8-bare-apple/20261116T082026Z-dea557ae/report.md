# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 20% faster than the next best (fxpmul_precise)

fxpmul_hot (1.62 us) leads fxpmul_precise (1.94 us) by 20%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (1.62 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 1617.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.20x (fastest 1617.1 ns, slowest 1943.3 ns)

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
| fxpmul_hot | 3733ns | 3726ns | 3720ns | 3727ns | 3764ns | base |
| fxpmul_precise | 4066ns | 4058ns | 4048ns | 4059ns | 4105ns | +8.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 1620ns | 1611ns | 1636ns | base | 10.116 |
| fxpmul_precise | 1947ns | 1937ns | 1967ns | +20.22% | 8.415 |

## Performance model

- Peak throughput: **10.171 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 10.132 | 99.6% |
| fxpmul_precise | 8.431 | 82.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 3733ns | 3733ns | base |
| fxpmul_precise | 4066ns | 4066ns | +8.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 1617ns | base | --- | [1616, 1618] | --- | --- | --- | --- |
| fxpmul_precise | 1943ns | +326.7ns (+20.2%) | [+326, +327]ns | [1943, 1944] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 1618ns | +20.0% |
| 1 | 2 | 1615ns | +20.3% |
| 1 | 3 | 1617ns | +20.2% |
| 1 | 4 | 1616ns | +20.4% |
| 1 | 5 | 1617ns | +20.2% |
| 1 | 6 | 1619ns | +19.9% |
| 1 | 7 | 1617ns | +20.3% |
| 1 | 8 | 1617ns | +20.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | 0.006 | ok |
| fxpmul_precise | -0.407 | moderate- |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 3.3ns | 1619.5ns | 0.2% |  |
| fxpmul_precise | 3.1ns | 1947.0ns | 0.2% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 1610.9-1635.7 ns)
   1610.9 |######################
   1612.2 |######################
   1613.4 |########################################
   1614.7 |############################
   1615.9 |##############################
   1617.1 |########################################
   1618.4 |####################
   1619.6 |########################
   1620.9 |############
   1622.1 |############
   1623.3 |####
   1624.6 |########
   1625.8 |
   1627.1 |####
   1628.3 |##
   1629.5 |
   1630.8 |
   1632.0 |##
   1633.3 |
   1634.5 |##
  (12 below, 12 above range)

fxpmul_precise (n=160, range 1937.2-1966.8 ns)
   1937.2 |################
   1938.6 |##############################
   1940.1 |#########################
   1941.6 |########################################
   1943.1 |##############################
   1944.6 |#############################
   1946.0 |#######################
   1947.5 |##############
   1949.0 |#####
   1950.5 |#######
   1952.0 |#
   1953.5 |
   1954.9 |
   1956.4 |#
   1957.9 |##########
   1959.4 |###
   1960.9 |
   1962.3 |#
   1963.8 |#
   1965.3 |###
  (13 below, 10 above range)

```
