# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (2.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 1.22 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 52% faster than the next best (ema_form_q32)

ema_form_intshift (1.22 us) leads ema_form_q32 (1.85 us) by 52%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 42% (significant)

ema_form_intshift is -869 ns (42%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: ema_form_intshift** at 1220.8 ns median (-41.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.71x (fastest 1220.8 ns, slowest 2088.3 ns)

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
| ema_form_float | 4297ns | 4268ns | 4263ns | 4272ns | 4408ns | base |
| ema_form_intshift | 3344ns | 3330ns | 3324ns | 3331ns | 3402ns | -22.19% |
| ema_form_q32 | 3985ns | 3964ns | 3958ns | 3965ns | 4075ns | -7.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 2102ns | 2085ns | 2159ns | base | 0.487 |
| ema_form_intshift | 1225ns | 1215ns | 1247ns | -41.74% | 0.836 |
| ema_form_q32 | 1865ns | 1849ns | 1908ns | -11.31% | 0.549 |

## Performance model

- Peak throughput: **0.843 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.490 | 58.2% |
| ema_form_intshift | 0.839 | 99.5% |
| ema_form_q32 | 0.552 | 65.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 4297ns | 4297ns | base |
| ema_form_intshift | 3344ns | 3344ns | -22.19% |
| ema_form_q32 | 3985ns | 3985ns | -7.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 2088ns | base | --- | [2088, 2089] | --- | --- | --- | --- |
| ema_form_intshift | 1221ns | -868.8ns (-41.6%) | [-870, -868]ns | [1220, 1221] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 1855ns | -234.6ns (-11.2%) | [-235, -234]ns | [1854, 1856] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 2161ns | -43.4% | -11.2% |
| 1 | 2 | 2088ns | -41.5% | -11.2% |
| 1 | 3 | 2089ns | -41.6% | -11.3% |
| 1 | 4 | 2088ns | -41.6% | -11.1% |
| 1 | 5 | 2088ns | -41.6% | -11.1% |
| 1 | 6 | 2088ns | -41.4% | -11.3% |
| 1 | 7 | 2088ns | -41.6% | -11.2% |
| 1 | 8 | 2089ns | -41.6% | -11.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.017 | ok |
| ema_form_intshift | -0.133 | ok |
| ema_form_q32 | -0.023 | ok |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 3.2ns | 2102.2ns | 0.2% |  |
| ema_form_intshift | 2.8ns | 1224.7ns | 0.2% |  |
| ema_form_q32 | 2.9ns | 1864.5ns | 0.2% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 2084.8-2159.1 ns)
   2084.8 |########################################
   2088.5 |#########################
   2092.2 |
   2095.9 |
   2099.6 |
   2103.3 |
   2107.1 |#
   2110.8 |
   2114.5 |#
   2118.2 |
   2121.9 |
   2125.6 |
   2129.4 |
   2133.1 |
   2136.8 |
   2140.5 |
   2144.2 |
   2147.9 |
   2151.7 |#
   2155.4 |##
  (13 below, 20 above range)

ema_form_intshift (n=160, range 1214.8-1246.7 ns)
   1214.8 |#################
   1216.4 |########################
   1218.0 |############################
   1219.6 |########################################
   1221.2 |##################
   1222.8 |########################
   1224.4 |#####
   1226.0 |##
   1227.6 |#
   1229.2 |#
   1230.8 |#
   1232.4 |##
   1233.9 |
   1235.5 |
   1237.1 |####
   1238.7 |#
   1240.3 |#
   1241.9 |##
   1243.5 |#
   1245.1 |
  (14 below, 13 above range)

ema_form_q32 (n=160, range 1849.0-1908.5 ns)
   1849.0 |#########################
   1852.0 |########################################
   1854.9 |#######################################
   1857.9 |############
   1860.9 |
   1863.9 |
   1866.8 |
   1869.8 |#
   1872.8 |
   1875.8 |#
   1878.7 |
   1881.7 |
   1884.7 |
   1887.7 |
   1890.6 |
   1893.6 |
   1896.6 |
   1899.6 |
   1902.5 |
   1905.5 |
  (10 below, 21 above range)

```
