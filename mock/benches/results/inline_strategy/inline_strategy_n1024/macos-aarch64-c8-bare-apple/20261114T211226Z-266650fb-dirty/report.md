# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### inline_never shows warm-up / thermal drift (autocorr +0.55)

inline_never's per-pass series has lag-1 autocorrelation +0.55, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5 ns) is smaller than the fastest variant's own run-to-run std-dev (50 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (1.31 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader inline_always vs stability leader inline_default (+0% speed for 1.2x steadier)

inline_always is fastest (1.31 us, CV 3.8%); inline_default gives up 0.1% median for 1.2x lower variance (CV 3.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.4% of the fastest

All 3 variants sit between 1.31 us and 1.32 us - a 0.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### inline_never's edge over baseline is significant but tiny (14 ns, 1.09%)

inline_never differs from baseline inline_always by 14 ns (1.09%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (inline_always) is the fastest** at 1312.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 1312.1 ns, slowest 1317.5 ns)

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
| inline_always | 3513ns | 3551ns | 3321ns | 3518ns | 3688ns | base |
| inline_default | 3502ns | 3553ns | 3318ns | 3529ns | 3605ns | -0.30% |
| inline_never | 3468ns | 3448ns | 3344ns | 3439ns | 3678ns | -1.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 1298ns | 1228ns | 1363ns | base | 0.789 |
| inline_default | 1295ns | 1227ns | 1335ns | -0.20% | 0.791 |
| inline_never | 1322ns | 1271ns | 1407ns | +1.89% | 0.774 |

## Performance model

- Peak throughput: **0.835 Gops/s** (inline_default; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.780 | 93.5% |
| inline_default | 0.779 | 93.4% |
| inline_never | 0.777 | 93.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 3513ns | 3513ns | base |
| inline_default | 3502ns | 3502ns | -0.30% |
| inline_never | 3468ns | 3468ns | -1.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 1312ns | base | --- | [1300, 1314] | --- | --- | --- | --- |
| inline_default | 1314ns | no significant difference | [-1, +4]ns | [1313, 1315] | no | 0.5206 | 0.5206 | 5 |
| inline_never | 1318ns | +14.3ns (+1.1%) | [+1, +44]ns | [1294, 1324] | YES (adj: no) | 0.0649 | 0.0325 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 1316ns | -6.5% | +0.1% |
| 1 | 2 | 1315ns | +0.1% | -3.0% |
| 1 | 3 | 1317ns | -0.3% | -3.0% |
| 1 | 4 | 1276ns | +3.0% | +3.1% |
| 1 | 5 | 1282ns | +2.6% | +2.8% |
| 1 | 6 | 1315ns | -0.1% | +4.1% |
| 1 | 7 | 1231ns | +6.8% | +11.2% |
| 1 | 8 | 1311ns | -6.3% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.270 | moderate- |
| inline_default | -0.058 | ok |
| inline_never | 0.551 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **inline_default**: won 3/8, lost 4/8
- **inline_never**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 3.1ns | 1297.9ns | 0.2% |  |
| inline_default | 2.9ns | 1295.3ns | 0.2% |  |
| inline_never | 3.4ns | 1322.5ns | 0.3% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 1228.3-1362.9 ns)
   1228.3 |##########
   1235.0 |
   1241.8 |##
   1248.5 |###
   1255.2 |
   1261.9 |###
   1268.7 |###########
   1275.4 |##########
   1282.1 |##
   1288.9 |####
   1295.6 |##
   1302.3 |####
   1309.0 |################################
   1315.8 |########################################
   1322.5 |###
   1329.2 |
   1335.9 |#
   1342.7 |
   1349.4 |#
   1356.1 |
  (21 below, 14 above range)

inline_default (n=160, range 1227.0-1334.8 ns)
   1227.0 |##########
   1232.4 |#
   1237.7 |
   1243.1 |#
   1248.5 |
   1253.9 |
   1259.3 |#
   1264.7 |#
   1270.1 |#
   1275.5 |
   1280.9 |
   1286.3 |
   1291.6 |
   1297.0 |
   1302.4 |
   1307.8 |###########
   1313.2 |########################################
   1318.6 |########
   1324.0 |
   1329.4 |
  (13 below, 7 above range)

inline_never (n=160, range 1271.4-1407.1 ns)
   1271.4 |########################################
   1278.1 |############
   1284.9 |########
   1291.7 |####
   1298.5 |####
   1305.3 |##
   1312.1 |#####
   1318.9 |###############
   1325.7 |################
   1332.4 |###
   1339.2 |##
   1346.0 |##
   1352.8 |#####
   1359.6 |#########
   1366.4 |#################
   1373.2 |###
   1380.0 |
   1386.7 |
   1393.5 |
   1400.3 |
  (15 below, 13 above range)

```

## Diagnostics

- **inline_never**: autocorrelation=0.55 (measurement drift or warm-up artifact)
