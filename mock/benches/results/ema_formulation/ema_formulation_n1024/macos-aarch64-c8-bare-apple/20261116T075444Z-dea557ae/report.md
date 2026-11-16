# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (2.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 1.22 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 52% faster than the next best (ema_form_q32)

ema_form_intshift (1.22 us) leads ema_form_q32 (1.85 us) by 52%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 42% (significant)

ema_form_intshift is -870 ns (42%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: ema_form_intshift** at 1221.9 ns median (-41.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.71x (fastest 1221.9 ns, slowest 2090.0 ns)

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
| ema_form_float | 4313ns | 4266ns | 4250ns | 4285ns | 4461ns | base |
| ema_form_intshift | 3340ns | 3322ns | 3312ns | 3326ns | 3409ns | -22.56% |
| ema_form_q32 | 3965ns | 3956ns | 3943ns | 3956ns | 4014ns | -8.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 2115ns | 2086ns | 2193ns | base | 0.484 |
| ema_form_intshift | 1227ns | 1216ns | 1252ns | -41.97% | 0.834 |
| ema_form_q32 | 1859ns | 1849ns | 1883ns | -12.08% | 0.551 |

## Performance model

- Peak throughput: **0.842 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.490 | 58.2% |
| ema_form_intshift | 0.838 | 99.5% |
| ema_form_q32 | 0.552 | 65.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 4313ns | 4313ns | base |
| ema_form_intshift | 3340ns | 3340ns | -22.56% |
| ema_form_q32 | 3965ns | 3965ns | -8.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 2090ns | base | --- | [2089, 2091] | --- | --- | --- | --- |
| ema_form_intshift | 1222ns | -869.8ns (-41.6%) | [-871, -868]ns | [1221, 1223] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 1855ns | -235.9ns (-11.3%) | [-238, -234]ns | [1854, 1856] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 2088ns | -41.6% | -11.2% |
| 1 | 2 | 2089ns | -41.5% | -11.1% |
| 1 | 3 | 2088ns | -41.6% | -11.2% |
| 1 | 4 | 2088ns | -41.0% | -11.2% |
| 1 | 5 | 2089ns | -41.5% | -11.2% |
| 1 | 6 | 2091ns | -41.5% | -11.2% |
| 1 | 7 | 2097ns | -41.7% | -11.6% |
| 1 | 8 | 2241ns | -45.6% | -17.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | 0.032 | ok |
| ema_form_intshift | -0.217 | moderate- |
| ema_form_q32 | -0.357 | moderate- |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 3.1ns | 2114.7ns | 0.1% |  |
| ema_form_intshift | 2.8ns | 1227.2ns | 0.2% |  |
| ema_form_q32 | 2.9ns | 1859.3ns | 0.2% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 2086.0-2193.1 ns)
   2086.0 |########################################
   2091.4 |##
   2096.7 |
   2102.1 |
   2107.4 |#
   2112.8 |
   2118.1 |##
   2123.5 |######
   2128.9 |###
   2134.2 |
   2139.6 |
   2144.9 |
   2150.3 |
   2155.6 |
   2161.0 |###
   2166.4 |
   2171.7 |
   2177.1 |
   2182.4 |
   2187.8 |
  (13 below, 15 above range)

ema_form_intshift (n=160, range 1215.6-1252.3 ns)
   1215.6 |##############
   1217.4 |####################################
   1219.3 |########################################
   1221.1 |######################################
   1222.9 |####################
   1224.8 |###########
   1226.6 |
   1228.4 |######
   1230.3 |#
   1232.1 |####
   1233.9 |###
   1235.8 |######
   1237.6 |###
   1239.4 |###
   1241.3 |######
   1243.1 |########
   1244.9 |########
   1246.8 |#
   1248.6 |
   1250.4 |#
  (16 below, 9 above range)

ema_form_q32 (n=160, range 1848.8-1882.6 ns)
   1848.8 |###################
   1850.5 |#########################
   1852.2 |########################################
   1853.9 |########################################
   1855.6 |#################################
   1857.3 |######################
   1859.0 |######
   1860.6 |
   1862.3 |#
   1864.0 |####
   1865.7 |###
   1867.4 |#
   1869.1 |###
   1870.8 |
   1872.5 |####
   1874.2 |####
   1875.9 |#
   1877.5 |
   1879.2 |
   1880.9 |#
  (13 below, 13 above range)

```
