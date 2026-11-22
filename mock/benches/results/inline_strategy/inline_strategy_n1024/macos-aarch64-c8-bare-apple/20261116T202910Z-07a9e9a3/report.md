# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (inline_default, inline_always) are a dead heat (<1%)

inline_default (1.23 us) and inline_always (1.23 us) differ by 0.03%, inside the noise, even though the wider field spreads 3.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### inline_default shows alternating (throttle bounce) (autocorr -0.53)

inline_default's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 3.9% of the fastest

All 3 variants sit between 1.23 us and 1.27 us - a 3.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: inline_default** at 1227.1 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 1227.1 ns, slowest 1275.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x39f847b2c651de80 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 3335ns | 3328ns | 3323ns | 3329ns | 3366ns | base |
| inline_default | 3335ns | 3328ns | 3323ns | 3328ns | 3368ns | -0.01% |
| inline_never | 3368ns | 3359ns | 3351ns | 3358ns | 3414ns | +0.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 1229ns | 1223ns | 1243ns | base | 0.833 |
| inline_default | 1231ns | 1223ns | 1250ns | +0.12% | 0.832 |
| inline_never | 1278ns | 1270ns | 1296ns | +3.99% | 0.801 |

## Performance model

- Peak throughput: **0.837 Gops/s** (inline_default; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.834 | 99.6% |
| inline_default | 0.834 | 99.6% |
| inline_never | 0.803 | 95.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 3335ns | 3335ns | base |
| inline_default | 3335ns | 3335ns | -0.01% |
| inline_never | 3368ns | 3368ns | +0.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 1228ns | base | --- | [1227, 1228] | --- | --- | --- | --- |
| inline_default | 1227ns | no significant difference | [-0, +1]ns | [1227, 1228] | no | 0.5206 | 0.5206 | 5 |
| inline_never | 1275ns | +47.9ns (+3.9%) | [+47, +48]ns | [1275, 1276] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 1227ns | -0.0% | +3.9% |
| 1 | 2 | 1227ns | +0.1% | +3.9% |
| 1 | 3 | 1232ns | -0.5% | +3.5% |
| 1 | 4 | 1227ns | +0.1% | +4.0% |
| 1 | 5 | 1228ns | -0.0% | +4.0% |
| 1 | 6 | 1227ns | -0.0% | +3.9% |
| 1 | 7 | 1226ns | +0.2% | +4.0% |
| 1 | 8 | 1227ns | +0.0% | +3.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.163 | ok |
| inline_default | -0.533 | HIGH- (thermal bounce) |
| inline_never | 0.227 | moderate+ |

**Consistency summary:**

- **inline_default**: won 1/8, lost 1/8
- **inline_never**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 2.9ns | 1229.5ns | 0.2% |  |
| inline_default | 2.8ns | 1230.9ns | 0.2% |  |
| inline_never | 3.1ns | 1278.5ns | 0.2% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 1222.7-1242.5 ns)
   1222.7 |###########
   1223.7 |##########################
   1224.7 |####################
   1225.7 |############################
   1226.7 |########################################
   1227.7 |##########################
   1228.7 |#########################
   1229.7 |########
   1230.7 |##########
   1231.6 |##########
   1232.6 |######
   1233.6 |########
   1234.6 |#
   1235.6 |
   1236.6 |
   1237.6 |
   1238.6 |#
   1239.6 |###
   1240.6 |#
   1241.5 |#
  (13 below, 8 above range)

inline_default (n=160, range 1222.7-1250.2 ns)
   1222.7 |#######################
   1224.1 |######################################
   1225.5 |########################################
   1226.8 |######################################
   1228.2 |####################################
   1229.6 |###############################
   1231.0 |#####
   1232.3 |######
   1233.7 |#
   1235.1 |
   1236.4 |
   1237.8 |#
   1239.2 |#
   1240.6 |#
   1241.9 |###
   1243.3 |
   1244.7 |
   1246.0 |
   1247.4 |###
   1248.8 |
  (12 below, 8 above range)

inline_never (n=160, range 1269.7-1296.2 ns)
   1269.7 |##########
   1271.0 |#######
   1272.4 |####################
   1273.7 |########################################
   1275.0 |###############
   1276.3 |########################
   1277.7 |###########
   1279.0 |#########
   1280.3 |#####
   1281.6 |#######
   1283.0 |
   1284.3 |##
   1285.6 |#
   1286.9 |
   1288.3 |#
   1289.6 |#
   1290.9 |
   1292.2 |
   1293.6 |#
   1294.9 |
  (15 below, 10 above range)

```
