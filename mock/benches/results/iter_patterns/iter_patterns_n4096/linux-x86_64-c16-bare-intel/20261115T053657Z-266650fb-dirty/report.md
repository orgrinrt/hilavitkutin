# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (1.16 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 78 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 636% faster than the next best (iter_seq)

iter_strided (78 ns) leads iter_seq (577 ns) by 636%, a clear separation rather than a photo finish. CV 93.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 93% (significant)

iter_strided is -1.09 us (93%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 14.8x slower than the field

iter_gather (1.16 us) is 14.8x the fastest (78 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 93.9%)

iter_strided wins on median (78 ns) yet has the highest variance (CV 93.9%), while iter_gather is the steadiest (CV 40.8%, 1.16 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 14.8x the fastest

Fastest iter_strided (78 ns) to slowest iter_gather (1.16 us): 14.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 3.5x its best-20%

iter_strided's best 20% of batches run at 71 ns but its worst 20% at 246 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 78.4 ns median (-93.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 14.85x (fastest 78.4 ns, slowest 1164.1 ns)

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
| Counter frequency | 2112774242 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 3199ns | 3045ns | 2489ns | 3011ns | 4472ns | base |
| iter_seq | 2999ns | 2456ns | 2038ns | 2457ns | 5588ns | -6.24% |
| iter_strided | 2768ns | 1956ns | 1774ns | 1958ns | 6191ns | -13.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 1220ns | 952ns | 1694ns | base | 3.358 |
| iter_seq | 703ns | 479ns | 1304ns | -42.38% | 5.827 |
| iter_strided | 110ns | 71ns | 246ns | -90.95% | 37.095 |

## Performance model

- Peak throughput: **57.536 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.519 | 6.1% |
| iter_seq | 7.098 | 12.3% |
| iter_strided | 52.245 | 90.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 3199ns | 3199ns | base |
| iter_seq | 2999ns | 2999ns | -6.24% |
| iter_strided | 2768ns | 2768ns | -13.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 1164ns | base | --- | [1164, 1164] | --- | --- | --- | --- |
| iter_seq | 577ns | -587.1ns (-50.4%) | [-587, -587]ns | [577, 577] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 78ns | -1085.7ns (-93.3%) | [-1086, -1086]ns | [78, 78] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 952ns | -39.4% | -91.8% |
| 1 | 2 | 952ns | -39.4% | -91.8% |
| 1 | 3 | 1164ns | -59.4% | -93.3% |
| 1 | 4 | 1164ns | -50.4% | -93.3% |
| 1 | 5 | 1164ns | -50.4% | -93.3% |
| 1 | 6 | 1164ns | -59.4% | -93.3% |
| 1 | 7 | 1164ns | -50.4% | -94.5% |
| 1 | 8 | 1164ns | -50.4% | -93.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | 0.458 | moderate+ |
| iter_seq | -0.375 | moderate- |
| iter_strided | -0.161 | ok |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 24.6ns | 1220.0ns | 2.0% |  |
| iter_seq | 28.6ns | 702.9ns | 4.1% |  |
| iter_strided | 33.2ns | 110.4ns | 30.0% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 952.4-1694.1 ns)
    952.4 |#######
    989.5 |
   1026.6 |
   1063.7 |
   1100.7 |
   1137.8 |########################################
   1174.9 |###
   1212.0 |
   1249.1 |
   1286.1 |
   1323.2 |
   1360.3 |
   1397.4 |
   1434.5 |
   1471.6 |
   1508.6 |
   1545.7 |
   1582.8 |
   1619.9 |
   1657.0 |
  (18 below, 9 above range)

iter_seq (n=160, range 479.0-1304.2 ns)
    479.0 |
    520.3 |
    561.5 |########################################
    602.8 |
    644.0 |
    685.3 |
    726.6 |
    767.8 |
    809.1 |
    850.3 |
    891.6 |
    932.9 |
    974.1 |
   1015.4 |
   1056.6 |
   1097.9 |
   1139.2 |
   1180.4 |
   1221.7 |
   1262.9 |
  (29 below, 18 above range)

iter_strided (n=160, range 71.2-245.7 ns)
     71.2 |########################################
     79.9 |
     88.6 |
     97.4 |
    106.1 |
    114.8 |
    123.5 |
    132.3 |
    141.0 |
    149.7 |
    158.4 |
    167.2 |
    175.9 |
    184.6 |
    193.3 |
    202.1 |
    210.8 |
    219.5 |
    228.3 |#
    237.0 |
  (16 below, 21 above range)

```

## Diagnostics

- **iter_gather**: CV=39.0% (high variance, measurements may be unstable)
- **iter_seq**: CV=54.3% (high variance, measurements may be unstable)
- **iter_strided**: CV=66.7% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **iter_strided**: bridge=30.0% of algo (FFI overhead may distort results)
