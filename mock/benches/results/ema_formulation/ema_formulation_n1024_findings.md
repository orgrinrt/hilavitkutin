# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (2.24 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 1.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 50% faster than the next best (ema_form_q32)

ema_form_intshift (1.29 us) leads ema_form_q32 (1.93 us) by 50%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 42% (significant)

ema_form_intshift is -932 ns (42%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_intshift shows alternating (throttle bounce) (autocorr -0.50)

ema_form_intshift's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: ema_form_intshift** at 1292.1 ns median (-42.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.73x (fastest 1292.1 ns, slowest 2237.1 ns)

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
| ema_form_float | 4482ns | 4558ns | 4269ns | 4513ns | 4603ns | base |
| ema_form_intshift | 3471ns | 3504ns | 3316ns | 3485ns | 3584ns | -22.56% |
| ema_form_q32 | 4141ns | 4117ns | 3957ns | 4138ns | 4336ns | -7.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 2200ns | 2092ns | 2263ns | base | 0.465 |
| ema_form_intshift | 1276ns | 1220ns | 1317ns | -42.02% | 0.803 |
| ema_form_q32 | 1944ns | 1854ns | 2042ns | -11.64% | 0.527 |

## Performance model

- Peak throughput: **0.839 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.458 | 54.5% |
| ema_form_intshift | 0.793 | 94.4% |
| ema_form_q32 | 0.530 | 63.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 4482ns | 4482ns | base |
| ema_form_intshift | 3471ns | 3471ns | -22.56% |
| ema_form_q32 | 4141ns | 4141ns | -7.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 2237ns | base | --- | [2234, 2238] | --- | --- | --- | --- |
| ema_form_intshift | 1292ns | -932.1ns (-41.7%) | [-938, -929]ns | [1267, 1304] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 1932ns | -252.3ns (-11.3%) | [-269, -249]ns | [1921, 1978] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 2091ns | -37.4% | -4.8% |
| 1 | 2 | 2175ns | -43.7% | -14.3% |
| 1 | 3 | 2237ns | -41.5% | -11.2% |
| 1 | 4 | 2237ns | -41.8% | -11.1% |
| 1 | 5 | 2242ns | -43.6% | -14.4% |
| 1 | 6 | 2238ns | -44.4% | -17.1% |
| 1 | 7 | 2239ns | -41.5% | -11.2% |
| 1 | 8 | 2165ns | -41.7% | -11.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | 0.282 | moderate+ |
| ema_form_intshift | -0.504 | HIGH- (thermal bounce) |
| ema_form_q32 | -0.414 | moderate- |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 3.0ns | 2200.4ns | 0.1% |  |
| ema_form_intshift | 2.8ns | 1275.9ns | 0.2% |  |
| ema_form_q32 | 3.0ns | 1944.3ns | 0.2% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 2091.8-2263.2 ns)
   2091.8 |###
   2100.4 |#
   2108.9 |
   2117.5 |##
   2126.1 |#
   2134.6 |
   2143.2 |
   2151.8 |####
   2160.3 |###
   2168.9 |#
   2177.5 |
   2186.0 |
   2194.6 |
   2203.2 |
   2211.8 |#
   2220.3 |
   2228.9 |#############
   2237.5 |########################################
   2246.0 |##
   2254.6 |##
  (24 below, 9 above range)

ema_form_intshift (n=160, range 1220.0-1317.4 ns)
   1220.0 |#####################
   1224.9 |######
   1229.8 |#
   1234.6 |
   1239.5 |
   1244.4 |
   1249.2 |
   1254.1 |###
   1259.0 |#######
   1263.8 |########
   1268.7 |#####
   1273.6 |
   1278.5 |
   1283.3 |#
   1288.2 |##
   1293.1 |
   1297.9 |#####
   1302.8 |###############
   1307.7 |########################################
   1312.5 |#########
  (13 below, 4 above range)

ema_form_q32 (n=160, range 1853.7-2042.2 ns)
   1853.7 |#################
   1863.1 |####
   1872.6 |####
   1882.0 |####
   1891.4 |####
   1900.8 |
   1910.3 |###############
   1919.7 |##############
   1929.1 |####
   1938.6 |#####
   1948.0 |
   1957.4 |##
   1966.8 |##
   1976.3 |#############
   1985.7 |########################################
   1995.1 |##
   2004.5 |##
   2014.0 |#
   2023.4 |#
   2032.8 |
  (16 below, 10 above range)

```
