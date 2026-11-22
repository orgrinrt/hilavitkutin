# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (2.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 1.22 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 52% faster than the next best (ema_form_q32)

ema_form_intshift (1.22 us) leads ema_form_q32 (1.86 us) by 52%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 42% (significant)

ema_form_intshift is -872 ns (42%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: ema_form_intshift** at 1220.2 ns median (-41.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.71x (fastest 1220.2 ns, slowest 2091.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6c97b3e25a1d8f04 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_form_float | 4354ns | 4280ns | 4264ns | 4314ns | 4568ns | base |
| ema_form_intshift | 3341ns | 3331ns | 3323ns | 3331ns | 3387ns | -23.28% |
| ema_form_q32 | 4018ns | 3970ns | 3958ns | 4000ns | 4136ns | -7.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 2129ns | 2086ns | 2232ns | base | 0.481 |
| ema_form_intshift | 1224ns | 1214ns | 1244ns | -42.53% | 0.837 |
| ema_form_q32 | 1879ns | 1851ns | 1939ns | -11.72% | 0.545 |

## Performance model

- Peak throughput: **0.843 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.490 | 58.1% |
| ema_form_intshift | 0.839 | 99.5% |
| ema_form_q32 | 0.552 | 65.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 4354ns | 4354ns | base |
| ema_form_intshift | 3341ns | 3341ns | -23.28% |
| ema_form_q32 | 4018ns | 4018ns | -7.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 2091ns | base | --- | [2090, 2103] | --- | --- | --- | --- |
| ema_form_intshift | 1220ns | -872.1ns (-41.7%) | [-879, -870]ns | [1220, 1221] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 1857ns | -235.9ns (-11.3%) | [-238, -235]ns | [1855, 1859] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 2089ns | -41.6% | -9.5% |
| 1 | 2 | 2090ns | -41.6% | -11.4% |
| 1 | 3 | 2168ns | -43.7% | -14.5% |
| 1 | 4 | 2093ns | -41.7% | -11.4% |
| 1 | 5 | 2088ns | -41.6% | -11.1% |
| 1 | 6 | 2089ns | -41.6% | -11.2% |
| 1 | 7 | 2088ns | -41.6% | -8.1% |
| 1 | 8 | 2179ns | -44.0% | -11.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.180 | ok |
| ema_form_intshift | 0.229 | moderate+ |
| ema_form_q32 | 0.374 | moderate+ |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 3.2ns | 2129.1ns | 0.2% |  |
| ema_form_intshift | 2.8ns | 1223.5ns | 0.2% |  |
| ema_form_q32 | 2.8ns | 1879.5ns | 0.1% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 2086.0-2231.8 ns)
   2086.0 |########################################
   2093.3 |##
   2100.6 |###
   2107.8 |
   2115.1 |
   2122.4 |####
   2129.7 |
   2137.0 |##
   2144.3 |
   2151.6 |#
   2158.9 |#########
   2166.2 |#
   2173.5 |#
   2180.8 |
   2188.1 |#
   2195.3 |
   2202.6 |
   2209.9 |####
   2217.2 |#
   2224.5 |
  (15 below, 9 above range)

ema_form_intshift (n=160, range 1214.2-1243.6 ns)
   1214.2 |#######
   1215.7 |#############################
   1217.2 |#############################
   1218.6 |###############################
   1220.1 |########################################
   1221.6 |###############################
   1223.0 |##############
   1224.5 |##########
   1226.0 |##
   1227.4 |#
   1228.9 |
   1230.4 |
   1231.9 |#
   1233.3 |##
   1234.8 |#
   1236.3 |#
   1237.7 |
   1239.2 |
   1240.7 |#
   1242.1 |
  (14 below, 6 above range)

ema_form_q32 (n=160, range 1850.6-1939.3 ns)
   1850.6 |########################################
   1855.1 |###################
   1859.5 |###
   1863.9 |#
   1868.4 |
   1872.8 |#
   1877.2 |##
   1881.7 |##
   1886.1 |#
   1890.5 |
   1895.0 |#
   1899.4 |
   1903.8 |
   1908.3 |###
   1912.7 |#####
   1917.1 |############
   1921.6 |#####
   1926.0 |###
   1930.4 |
   1934.9 |
  (14 below, 6 above range)

```
