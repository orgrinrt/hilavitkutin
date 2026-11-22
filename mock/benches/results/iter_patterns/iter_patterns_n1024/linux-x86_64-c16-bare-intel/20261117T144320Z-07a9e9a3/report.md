# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (297 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 25 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 491% faster than the next best (iter_seq)

iter_strided (25 ns) leads iter_seq (149 ns) by 491%, a clear separation rather than a photo finish. CV 88.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 92% (significant)

iter_strided is -272 ns (92%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 11.8x slower than the field

iter_gather (297 ns) is 11.8x the fastest (25 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 88.1%)

iter_strided wins on median (25 ns) yet has the highest variance (CV 88.1%), while iter_gather is the steadiest (CV 78.8%, 297 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 11.8x the fastest

Fastest iter_strided (25 ns) to slowest iter_gather (297 ns): 11.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 3.7x its best-20%

iter_strided's best 20% of batches run at 21 ns but its worst 20% at 77 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 25.3 ns median (-91.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 11.75x (fastest 25.3 ns, slowest 297.3 ns)

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
| Counter frequency | 2112828630 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 2819ns | 2183ns | 1785ns | 2122ns | 5947ns | base |
| iter_seq | 2640ns | 2035ns | 1663ns | 1988ns | 5576ns | -6.35% |
| iter_strided | 2615ns | 1911ns | 1563ns | 1872ns | 5895ns | -7.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 382ns | 243ns | 803ns | base | 2.677 |
| iter_seq | 193ns | 122ns | 404ns | -49.62% | 5.315 |
| iter_strided | 34ns | 21ns | 77ns | -91.00% | 29.743 |

## Performance model

- Peak throughput: **49.536 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.444 | 7.0% |
| iter_seq | 6.854 | 13.8% |
| iter_strided | 40.474 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 2819ns | 2819ns | base |
| iter_seq | 2640ns | 2640ns | -6.35% |
| iter_strided | 2615ns | 2615ns | -7.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 297ns | base | --- | [297, 297] | --- | --- | --- | --- |
| iter_seq | 149ns | -147.8ns (-49.7%) | [-148, -148]ns | [149, 150] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 25ns | -272.1ns (-91.5%) | [-272, -272]ns | [25, 25] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 244ns | -38.6% | -91.5% |
| 1 | 2 | 298ns | -58.9% | -91.5% |
| 1 | 3 | 298ns | -49.7% | -93.0% |
| 1 | 4 | 297ns | -49.7% | -93.0% |
| 1 | 5 | 243ns | -49.8% | -89.6% |
| 1 | 6 | 243ns | -49.7% | -89.6% |
| 1 | 7 | 297ns | -49.7% | -91.5% |
| 1 | 8 | 297ns | -49.7% | -91.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | 0.061 | ok |
| iter_seq | -0.142 | ok |
| iter_strided | 0.058 | ok |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 30.8ns | 382.5ns | 8.1% | HIGH |
| iter_seq | 31.1ns | 192.7ns | 16.1% | HIGH |
| iter_strided | 32.1ns | 34.4ns | 93.2% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 243.2-803.1 ns)
    243.2 |###############
    271.2 |########################################
    299.2 |
    327.2 |
    355.2 |
    383.2 |
    411.2 |
    439.2 |
    467.2 |
    495.2 |
    523.1 |
    551.1 |
    579.1 |
    607.1 |
    635.1 |
    663.1 |
    691.1 |
    719.1 |
    747.1 |
    775.1 |
  (16 below, 18 above range)

iter_seq (n=160, range 122.2-403.8 ns)
    122.2 |#############
    136.3 |########################################
    150.4 |
    164.5 |
    178.6 |
    192.6 |
    206.7 |
    220.8 |
    234.9 |
    248.9 |
    263.0 |
    277.1 |
    291.2 |
    305.2 |
    319.3 |
    333.4 |
    347.5 |
    361.6 |
    375.6 |#
    389.7 |
  (18 below, 16 above range)

iter_strided (n=160, range 20.7-76.9 ns)
     20.7 |####################
     23.5 |########################################
     26.3 |
     29.1 |
     31.9 |
     34.7 |
     37.5 |
     40.3 |
     43.1 |
     46.0 |
     48.8 |
     51.6 |
     54.4 |
     57.2 |
     60.0 |
     62.8 |#
     65.6 |
     68.4 |#
     71.2 |
     74.0 |#
  (8 below, 17 above range)

```

## Diagnostics

- **iter_gather**: CV=61.2% (high variance, measurements may be unstable)
- **iter_gather**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **iter_seq**: CV=62.7% (high variance, measurements may be unstable)
- **iter_seq**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **iter_seq**: bridge=15.9% of algo (FFI overhead may distort results)
- **iter_strided**: CV=64.8% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **iter_strided**: bridge=92.9% of algo (FFI overhead may distort results)
