# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (1.16 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 78 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 636% faster than the next best (iter_seq)

iter_strided (78 ns) leads iter_seq (577 ns) by 636%, a clear separation rather than a photo finish. CV 84.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 84% (significant)

iter_strided is -979 ns (84%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 14.9x slower than the field

iter_gather (1.16 us) is 14.9x the fastest (78 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 84.8%)

iter_strided wins on median (78 ns) yet has the highest variance (CV 84.8%), while iter_gather is the steadiest (CV 43.9%, 1.16 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 14.9x the fastest

Fastest iter_strided (78 ns) to slowest iter_gather (1.16 us): 14.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 3.6x its best-20%

iter_strided's best 20% of batches run at 64 ns but its worst 20% at 233 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 78.4 ns median (-93.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 14.86x (fastest 78.4 ns, slowest 1164.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Column iteration patterns: sequential vs strided vs data-dependent gather |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x3a5c8e91b2d4f607 |
| Counter frequency | 2112768594 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 3190ns | 3047ns | 2492ns | 2854ns | 4897ns | base |
| iter_seq | 3042ns | 2455ns | 2025ns | 2457ns | 5813ns | -4.65% |
| iter_strided | 2785ns | 1957ns | 1600ns | 2137ns | 5913ns | -12.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 1213ns | 953ns | 1849ns | base | 3.377 |
| iter_seq | 712ns | 475ns | 1353ns | -41.31% | 5.754 |
| iter_strided | 111ns | 64ns | 233ns | -90.87% | 36.967 |

## Performance model

- Peak throughput: **63.931 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.517 | 5.5% |
| iter_seq | 7.099 | 11.1% |
| iter_strided | 52.245 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 3190ns | 3190ns | base |
| iter_seq | 3042ns | 3042ns | -4.65% |
| iter_strided | 2785ns | 2785ns | -12.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 1165ns | base | --- | [1070, 1165] | --- | --- | --- | --- |
| iter_seq | 577ns | -519.5ns (-44.6%) | [-588, -379]ns | [577, 577] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 78ns | -979.1ns (-84.1%) | [-1086, -899]ns | [78, 78] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 1166ns | -59.5% | -93.3% |
| 1 | 2 | 954ns | -39.5% | -91.8% |
| 1 | 3 | 954ns | -39.5% | -93.3% |
| 1 | 4 | 1165ns | -59.5% | -93.3% |
| 1 | 5 | 1165ns | -50.5% | -94.5% |
| 1 | 6 | 953ns | -39.5% | -93.3% |
| 1 | 7 | 1166ns | -50.5% | -93.3% |
| 1 | 8 | 954ns | -39.5% | -91.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.377 | moderate- |
| iter_seq | -0.208 | moderate- |
| iter_strided | -0.140 | ok |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 24.6ns | 1213.0ns | 2.0% |  |
| iter_seq | 29.1ns | 711.8ns | 4.1% |  |
| iter_strided | 33.4ns | 110.8ns | 30.1% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 952.8-1849.3 ns)
    952.8 |#############################
    997.6 |
   1042.4 |
   1087.3 |
   1132.1 |########################################
   1176.9 |#
   1221.7 |
   1266.6 |
   1311.4 |
   1356.2 |
   1401.1 |
   1445.9 |
   1490.7 |
   1535.5 |
   1580.4 |
   1625.2 |
   1670.0 |
   1714.8 |
   1759.7 |
   1804.5 |
  (13 below, 13 above range)

iter_seq (n=160, range 475.4-1352.6 ns)
    475.4 |
    519.3 |
    563.1 |########################################
    607.0 |
    650.8 |
    694.7 |
    738.6 |
    782.4 |
    826.3 |
    870.1 |
    914.0 |
    957.9 |
   1001.7 |
   1045.6 |
   1089.4 |
   1133.3 |
   1177.2 |
   1221.0 |
   1264.9 |
   1308.7 |
  (31 below, 19 above range)

iter_strided (n=160, range 64.1-233.2 ns)
     64.1 |#############
     72.5 |########################################
     81.0 |
     89.4 |
     97.9 |
    106.4 |
    114.8 |
    123.3 |
    131.7 |
    140.2 |
    148.7 |
    157.1 |
    165.6 |
    174.0 |#
    182.5 |###
    190.9 |
    199.4 |##
    207.9 |
    216.3 |##
    224.8 |
  (13 below, 16 above range)

```

## Diagnostics

- **iter_gather**: CV=42.1% (high variance, measurements may be unstable)
- **iter_seq**: CV=54.7% (high variance, measurements may be unstable)
- **iter_strided**: CV=60.0% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **iter_strided**: bridge=30.1% of algo (FFI overhead may distort results)
