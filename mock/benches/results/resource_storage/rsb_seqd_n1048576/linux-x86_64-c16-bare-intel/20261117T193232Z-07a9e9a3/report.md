# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### seq_live dominates: 82% faster than the next best (seq_snapshot)

seq_live (821.78 us) leads seq_snapshot (1.49 ms) by 82%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (seq_live)

The baseline seq_live is the fastest (821.78 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (seq_live) is the fastest** at 821775.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.82x (fastest 821775.5 ns, slowest 1493460.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Seq collection member: live-stream vs snapshot-copy across payload size |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112727760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| seq_live | 842513ns | 821959ns | 816552ns | 823340ns | 925993ns | base |
| seq_snapshot | 1470056ns | 1494035ns | 1289522ns | 1499225ns | 1563084ns | +74.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 842336ns | 816385ns | 925807ns | base | 1.245 |
| seq_snapshot | 1469524ns | 1288971ns | 1562540ns | +74.46% | 0.714 |

## Performance model

- Peak throughput: **1.284 Gops/s** (seq_live; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 1.276 | 99.3% |
| seq_snapshot | 0.702 | 54.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 842513ns | 842513ns | base |
| seq_snapshot | 1470056ns | 1470056ns | +74.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 821776ns | base | --- | [820083, 822768] | --- | --- | --- | --- |
| seq_snapshot | 1493460ns | +670393.0ns (+81.6%) | [+666690, +673529]ns | [1490313, 1497635] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 823127ns | +84.9% |
| 1 | 2 | 820899ns | +80.5% |
| 1 | 3 | 818750ns | +81.8% |
| 1 | 4 | 820026ns | +81.7% |
| 1 | 5 | 815870ns | +83.0% |
| 2 | 1 | 817626ns | +82.2% |
| 2 | 2 | 824324ns | +81.4% |
| 2 | 3 | 824690ns | +84.7% |
| 2 | 4 | 822501ns | +81.1% |
| 2 | 5 | 823175ns | +41.6% |
| 3 | 1 | 998197ns | +49.6% |
| 3 | 2 | 820038ns | +46.2% |
| 3 | 3 | 952847ns | +61.4% |
| 3 | 4 | 816937ns | +86.2% |
| 3 | 5 | 817966ns | +82.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | -0.161 | ok |
| seq_snapshot | -0.172 | ok |

**Consistency summary:**

- **seq_snapshot**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 1347398.9ns | 842336.3ns | 160.0% | HIGH |
| seq_snapshot | 1755330.7ns | 1469523.7ns | 119.4% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 816385.3-925807.4 ns)
  816385.3 |########################################
  821856.4 |########################
  827327.5 |
  832798.6 |
  838269.7 |
  843740.8 |####
  849211.9 |#####
  854683.0 |
  860154.1 |
  865625.2 |#
  871096.3 |
  876567.4 |
  882038.5 |
  887509.6 |
  892980.7 |
  898451.8 |
  903923.0 |
  909394.1 |
  914865.2 |
  920336.3 |
  (13 below, 12 above range)

seq_snapshot (n=120, range 1288971.1-1562540.4 ns)
  1288971.1 |
  1302649.6 |
  1316328.0 |
  1330006.5 |
  1343685.0 |
  1357363.4 |
  1371041.9 |
  1384720.4 |
  1398398.8 |
  1412077.3 |
  1425755.8 |
  1439434.2 |
  1453112.7 |
  1466791.2 |##
  1480469.6 |########################################
  1494148.1 |#############
  1507826.6 |################
  1521505.0 |###########
  1535183.5 |###
  1548862.0 |###
  (15 below, 5 above range)

```

## Diagnostics

- **seq_live**: bridge=161.8% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=118.4% of algo (FFI overhead may distort results)
