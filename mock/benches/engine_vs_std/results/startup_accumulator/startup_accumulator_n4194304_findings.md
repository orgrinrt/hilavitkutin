# accumulator, get-ready

2 variants, 8 samples per variant.
Baseline: **engine_startup**

## Highlights

Baseline for all deltas below: **engine_startup**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (engine_startup) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline engine_startup has the worst median (1.73 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std_startup at 1.34 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### std_startup shows alternating (throttle bounce) (autocorr -0.50)

std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### engine_startup is inconsistent: worst-20% is 1.6x its best-20%

engine_startup's best 20% of batches run at 1.25 ms but its worst 20% at 1.97 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_startup** at 1335701.9 ns median (-22.8% vs baseline)
- Spread: 1.29x (fastest 1335701.9 ns, slowest 1729664.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | accumulator, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010003 |
| Counter frequency | 2300408230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine_startup | 1701812ns | 1735045ns | 1252172ns | 1481391ns | 1979537ns | base |
| std_startup | 1392567ns | 1340763ns | 1215141ns | 1261227ns | 1535428ns | -18.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine_startup | 1696967ns | 1249019ns | 1974236ns | base | 2.472 |
| std_startup | 1387956ns | 1210978ns | 1530501ns | -18.21% | 3.022 |

## Performance model

- Peak throughput: **3.464 Gops/s** (std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine_startup | 2.425 | 70.0% |
| std_startup | 3.140 | 90.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine_startup | 1701812ns | 1701812ns | base |
| std_startup | 1392567ns | 1392567ns | -18.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine_startup | 1729665ns | base | --- | [1259644, 2172757] | --- | --- | --- | --- |
| std_startup | 1335702ns | no significant difference | [-803561, +209678]ns | [1221753, 1469322] | no | 0.2891 | 0.2891 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine_startup | std_startup |
|---|---|---|---|
| 1 | 1 | 1968176ns | -37.9% |
| 1 | 2 | 1367021ns | +2.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine_startup | -0.500 | moderate- |
| std_startup | -0.500 | moderate- |

**Consistency summary:**

- **std_startup**: won 1/2, lost 1/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine_startup | 47542915.2ns | 1696967.4ns | 2801.6% | HIGH |
| std_startup | 38428503.5ns | 1387956.2ns | 2768.7% | HIGH |

## Distribution (algo ns)

```
engine_startup (n=8, range 1249019.4-1974235.8 ns)
  1249019.4 |########################################
  1285280.2 |
  1321541.0 |
  1357801.9 |
  1394062.7 |
  1430323.5 |
  1466584.3 |####################
  1502845.1 |
  1539105.9 |
  1575366.8 |
  1611627.6 |
  1647888.4 |
  1684149.2 |####################
  1720410.0 |
  1756670.8 |########################################
  1792931.7 |
  1829192.5 |
  1865453.3 |
  1901714.1 |
  1937974.9 |
  (0 below, 2 above range)

std_startup (n=8, range 1210977.8-1530501.1 ns)
  1210977.8 |########################################
  1226954.0 |
  1242930.1 |
  1258906.3 |
  1274882.5 |
  1290858.6 |
  1306834.8 |
  1322811.0 |#############
  1338787.1 |#############
  1354763.3 |
  1370739.5 |
  1386715.6 |#############
  1402691.8 |
  1418668.0 |
  1434644.1 |
  1450620.3 |
  1466596.5 |#############
  1482572.6 |
  1498548.8 |
  1514525.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **engine_startup**: CV=20.0% (high variance, measurements may be unstable)
- **engine_startup**: bridge=2747.2% of algo (FFI overhead may distort results)
- **std_startup**: bridge=2874.3% of algo (FFI overhead may distort results)
