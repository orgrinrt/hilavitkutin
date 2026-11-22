# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (seq_live) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline seq_live has the worst median (997.20 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest seq_snapshot at 866.75 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### seq_snapshot dominates: 15% faster than the next best (seq_live)

seq_snapshot (866.75 us) leads seq_live (997.20 us) by 15%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

## Key findings

- **Fastest: seq_snapshot** at 866753.0 ns median (-13.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.15x (fastest 866753.0 ns, slowest 997204.2 ns)

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
| seq_live | 955183ns | 997298ns | 816662ns | 981624ns | 1014377ns | base |
| seq_snapshot | 871305ns | 866872ns | 865922ns | 867158ns | 889131ns | -8.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 955097ns | 816587ns | 1014288ns | base | 0.069 |
| seq_snapshot | 871188ns | 865803ns | 889015ns | -8.79% | 0.075 |

## Performance model

- Peak throughput: **0.080 Gops/s** (seq_live; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 0.066 | 81.9% |
| seq_snapshot | 0.076 | 94.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 955183ns | 955183ns | base |
| seq_snapshot | 871305ns | 871305ns | -8.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 997204ns | base | --- | [996725, 997428] | --- | --- | --- | --- |
| seq_snapshot | 866753ns | -130120.8ns (-13.0%) | [-130819, -128065]ns | [866578, 867003] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 996528ns | -13.0% |
| 1 | 2 | 998807ns | -13.3% |
| 1 | 3 | 816511ns | +6.2% |
| 1 | 4 | 997832ns | -13.2% |
| 1 | 5 | 997103ns | -12.7% |
| 2 | 1 | 997551ns | -13.2% |
| 2 | 2 | 997255ns | -12.0% |
| 2 | 3 | 816598ns | +7.4% |
| 2 | 4 | 997223ns | -13.1% |
| 2 | 5 | 817783ns | +6.0% |
| 3 | 1 | 816420ns | +6.2% |
| 3 | 2 | 997463ns | -13.2% |
| 3 | 3 | 998003ns | -13.2% |
| 3 | 4 | 997575ns | -13.2% |
| 3 | 5 | 997402ns | -13.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | -0.049 | ok |
| seq_snapshot | 0.331 | moderate+ |

**Consistency summary:**

- **seq_snapshot**: won 11/15, lost 4/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 96255.3ns | 955096.8ns | 10.1% | HIGH |
| seq_snapshot | 109529.1ns | 871188.0ns | 12.6% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 816586.8-1014287.7 ns)
  816586.8 |#####
  826471.8 |
  836356.9 |
  846241.9 |
  856126.9 |
  866012.0 |
  875897.0 |
  885782.1 |
  895667.1 |
  905552.2 |
  915437.2 |
  925322.3 |
  935207.3 |
  945092.4 |
  954977.4 |
  964862.4 |
  974747.5 |
  984632.5 |
  994517.6 |########################################
  1004402.6 |
  (17 below, 5 above range)

seq_snapshot (n=120, range 865803.0-889014.7 ns)
  865803.0 |########################################
  866963.6 |############
  868124.2 |####
  869284.8 |##
  870445.4 |
  871605.9 |#
  872766.5 |
  873927.1 |#
  875087.7 |
  876248.3 |
  877408.9 |
  878569.4 |#
  879730.0 |##
  880890.6 |
  882051.2 |
  883211.8 |
  884372.4 |
  885532.9 |
  886693.5 |
  887854.1 |
  (10 below, 12 above range)

```

## Diagnostics

- **seq_live**: bridge=10.1% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=12.6% of algo (FFI overhead may distort results)
