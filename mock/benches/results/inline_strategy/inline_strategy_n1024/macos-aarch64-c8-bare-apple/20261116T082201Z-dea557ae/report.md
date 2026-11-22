# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (inline_always, inline_default) are a dead heat (<1%)

inline_always (1.25 us) and inline_default (1.25 us) differ by 0.02%, inside the noise, even though the wider field spreads 4.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (1.25 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 4.3% of the fastest

All 3 variants sit between 1.25 us and 1.31 us - a 4.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (inline_always) is the fastest** at 1251.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 1251.7 ns, slowest 1305.4 ns)

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
| inline_always | 3415ns | 3387ns | 3369ns | 3396ns | 3518ns | base |
| inline_default | 3392ns | 3382ns | 3323ns | 3387ns | 3478ns | -0.67% |
| inline_never | 3433ns | 3418ns | 3367ns | 3425ns | 3521ns | +0.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 1262ns | 1244ns | 1308ns | base | 0.811 |
| inline_default | 1255ns | 1227ns | 1293ns | -0.55% | 0.816 |
| inline_never | 1309ns | 1281ns | 1345ns | +3.72% | 0.782 |

## Performance model

- Peak throughput: **0.835 Gops/s** (inline_default; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.818 | 98.0% |
| inline_default | 0.818 | 98.0% |
| inline_never | 0.784 | 94.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 3415ns | 3415ns | base |
| inline_default | 3392ns | 3392ns | -0.67% |
| inline_never | 3433ns | 3433ns | +0.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 1252ns | base | --- | [1251, 1253] | --- | --- | --- | --- |
| inline_default | 1252ns | no significant difference | [-3, +0]ns | [1250, 1252] | no | 0.1705 | 0.1705 | 6 |
| inline_never | 1305ns | +50.6ns (+4.0%) | [+49, +53]ns | [1302, 1308] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 1251ns | +0.1% | +4.0% |
| 1 | 2 | 1250ns | +0.4% | +4.1% |
| 1 | 3 | 1251ns | -0.3% | +4.7% |
| 1 | 4 | 1256ns | -0.4% | +4.3% |
| 1 | 5 | 1268ns | +0.1% | +1.5% |
| 1 | 6 | 1250ns | +0.3% | +5.7% |
| 1 | 7 | 1254ns | -1.2% | +5.1% |
| 1 | 8 | 1251ns | -1.2% | +2.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | 0.009 | ok |
| inline_default | 0.279 | moderate+ |
| inline_never | -0.304 | moderate- |

**Consistency summary:**

- **inline_default**: won 4/8, lost 3/8
- **inline_never**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 2.9ns | 1262.2ns | 0.2% |  |
| inline_default | 3.1ns | 1255.3ns | 0.2% |  |
| inline_never | 3.4ns | 1309.1ns | 0.3% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 1244.2-1307.8 ns)
   1244.2 |############
   1247.3 |########################################
   1250.5 |######################
   1253.7 |###############
   1256.9 |###
   1260.1 |####
   1263.3 |###
   1266.4 |#####
   1269.6 |
   1272.8 |
   1276.0 |###
   1279.2 |
   1282.4 |
   1285.5 |#
   1288.7 |
   1291.9 |#
   1295.1 |
   1298.3 |#
   1301.5 |
   1304.6 |
  (5 below, 13 above range)

inline_default (n=160, range 1227.1-1292.8 ns)
   1227.1 |##########
   1230.4 |###
   1233.6 |
   1236.9 |##
   1240.2 |#
   1243.5 |##########
   1246.8 |##################################
   1250.1 |########################################
   1253.4 |#####################
   1256.7 |######
   1260.0 |##
   1263.2 |###
   1266.5 |#######
   1269.8 |#########
   1273.1 |###
   1276.4 |##
   1279.7 |###
   1283.0 |#
   1286.3 |#
   1289.6 |
  (19 below, 11 above range)

inline_never (n=160, range 1281.4-1345.5 ns)
   1281.4 |#
   1284.6 |#######
   1287.8 |#
   1291.0 |###########
   1294.2 |#############
   1297.4 |########################################
   1300.6 |##################################
   1303.8 |###################
   1307.0 |##########################
   1310.3 |####################
   1313.5 |###################
   1316.7 |###
   1319.9 |#################
   1323.1 |#############
   1326.3 |#############
   1329.5 |#####
   1332.7 |###
   1335.9 |
   1339.1 |#
   1342.3 |#
  (18 below, 7 above range)

```
