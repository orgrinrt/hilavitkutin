# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (297 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 25 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 493% faster than the next best (iter_seq)

iter_strided (25 ns) leads iter_seq (150 ns) by 493%, a clear separation rather than a photo finish. CV 101.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 92% (significant)

iter_strided is -272 ns (92%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 11.8x slower than the field

iter_gather (297 ns) is 11.8x the fastest (25 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 101.5%)

iter_strided wins on median (25 ns) yet has the highest variance (CV 101.5%), while iter_gather is the steadiest (CV 80.8%, 297 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 11.8x the fastest

Fastest iter_strided (25 ns) to slowest iter_gather (297 ns): 11.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 4.0x its best-20%

iter_strided's best 20% of batches run at 21 ns but its worst 20% at 83 ns (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 25.2 ns median (-91.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 11.80x (fastest 25.2 ns, slowest 297.4 ns)

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
| iter_gather | 2937ns | 2174ns | 1777ns | 2164ns | 6416ns | base |
| iter_seq | 2833ns | 2027ns | 1670ns | 2094ns | 6214ns | -3.54% |
| iter_strided | 2949ns | 1901ns | 1554ns | 2391ns | 6018ns | +0.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 398ns | 243ns | 864ns | base | 2.576 |
| iter_seq | 208ns | 124ns | 454ns | -47.67% | 4.923 |
| iter_strided | 40ns | 21ns | 83ns | -90.02% | 25.816 |

## Performance model

- Peak throughput: **49.716 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.443 | 6.9% |
| iter_seq | 6.849 | 13.8% |
| iter_strided | 40.635 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 2937ns | 2937ns | base |
| iter_seq | 2833ns | 2833ns | -3.54% |
| iter_strided | 2949ns | 2949ns | +0.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 297ns | base | --- | [297, 298] | --- | --- | --- | --- |
| iter_seq | 150ns | -148.0ns (-49.8%) | [-148, -148]ns | [150, 150] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 25ns | -272.4ns (-91.6%) | [-276, -272]ns | [25, 25] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 300ns | -59.2% | -91.6% |
| 1 | 2 | 244ns | -38.6% | -91.5% |
| 1 | 3 | 244ns | -38.6% | -74.2% |
| 1 | 4 | 297ns | -49.7% | -93.0% |
| 1 | 5 | 298ns | -49.7% | -91.5% |
| 1 | 6 | 244ns | -38.6% | -91.5% |
| 1 | 7 | 298ns | -49.8% | -93.0% |
| 1 | 8 | 297ns | -58.8% | -91.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.151 | ok |
| iter_seq | -0.041 | ok |
| iter_strided | -0.259 | moderate- |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 31.8ns | 397.5ns | 8.0% | HIGH |
| iter_seq | 33.1ns | 208.0ns | 15.9% | HIGH |
| iter_strided | 36.3ns | 39.7ns | 91.5% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 243.4-863.7 ns)
    243.4 |#####################
    274.4 |########################################
    305.4 |##
    336.4 |#
    367.4 |
    398.5 |#
    429.5 |
    460.5 |
    491.5 |
    522.5 |
    553.5 |
    584.6 |
    615.6 |
    646.6 |
    677.6 |
    708.6 |
    739.6 |
    770.6 |#
    801.7 |##
    832.7 |#
  (8 below, 18 above range)

iter_seq (n=160, range 123.5-453.8 ns)
    123.5 |
    140.0 |########################################
    156.6 |
    173.1 |
    189.6 |
    206.1 |
    222.6 |
    239.1 |#
    255.6 |
    272.2 |
    288.7 |
    305.2 |
    321.7 |
    338.2 |
    354.7 |
    371.2 |
    387.8 |
    404.3 |##
    420.8 |#
    437.3 |##
  (30 below, 13 above range)

iter_strided (n=160, range 20.6-82.7 ns)
     20.6 |########################################
     23.7 |################################
     26.8 |
     29.9 |
     33.0 |
     36.1 |
     39.2 |
     42.3 |
     45.4 |#
     48.5 |
     51.6 |
     54.7 |##
     57.8 |####
     61.0 |
     64.1 |######
     67.2 |##
     70.3 |##
     73.4 |####
     76.5 |#
     79.6 |#
  (5 below, 14 above range)

```

## Diagnostics

- **iter_gather**: CV=60.4% (high variance, measurements may be unstable)
- **iter_gather**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **iter_seq**: CV=60.7% (high variance, measurements may be unstable)
- **iter_seq**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **iter_seq**: bridge=15.9% of algo (FFI overhead may distort results)
- **iter_strided**: CV=64.5% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **iter_strided**: bridge=93.3% of algo (FFI overhead may distort results)
