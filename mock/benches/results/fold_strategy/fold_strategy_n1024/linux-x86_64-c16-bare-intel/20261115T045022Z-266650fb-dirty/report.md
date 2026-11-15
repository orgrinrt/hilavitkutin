# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 76% faster than the next best (fold_paired)

fold_quad (45 ns) leads fold_paired (79 ns) by 76%, a clear separation rather than a photo finish. CV 88.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 43% (significant)

fold_quad is -34 ns (43%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.3x slower than the field

fold_sequential (150 ns) is 3.3x the fastest (45 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.3x the fastest

Fastest fold_quad (45 ns) to slowest fold_sequential (150 ns): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fold_sequential is inconsistent: worst-20% is 3.5x its best-20%

fold_sequential's best 20% of batches run at 126 ns but its worst 20% at 445 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fold_quad** at 45.2 ns median (-43.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.31x (fastest 45.2 ns, slowest 149.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x239fb80547ae61c2 |
| Counter frequency | 2112785343 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fold_paired | 2880ns | 1957ns | 1797ns | 2065ns | 6410ns | base |
| fold_quad | 2797ns | 1923ns | 1917ns | 1964ns | 6175ns | -2.89% |
| fold_sequential | 2821ns | 2028ns | 1705ns | 2097ns | 6110ns | -2.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 116ns | 73ns | 257ns | base | 8.811 |
| fold_quad | 65ns | 45ns | 143ns | -43.94% | 15.717 |
| fold_sequential | 207ns | 126ns | 445ns | +78.13% | 4.946 |

## Performance model

- Peak throughput: **22.740 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 12.897 | 56.7% |
| fold_quad | 22.655 | 99.6% |
| fold_sequential | 6.849 | 30.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2880ns | 2880ns | base |
| fold_quad | 2797ns | 2797ns | -2.89% |
| fold_sequential | 2821ns | 2821ns | -2.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 79ns | base | --- | [79, 79] | --- | --- | --- | --- |
| fold_quad | 45ns | -34.3ns (-43.2%) | [-34, -34]ns | [45, 45] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 150ns | +70.2ns (+88.4%) | [+70, +70]ns | [150, 150] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 79ns | -43.1% | +88.4% |
| 1 | 2 | 79ns | -43.2% | +88.3% |
| 1 | 3 | 79ns | -43.1% | +54.2% |
| 1 | 4 | 79ns | -43.1% | +88.4% |
| 1 | 5 | 80ns | -43.1% | +88.2% |
| 1 | 6 | 65ns | -30.3% | +130.1% |
| 1 | 7 | 79ns | -43.0% | +54.2% |
| 1 | 8 | 79ns | -43.0% | +88.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.170 | ok |
| fold_quad | 0.430 | moderate+ |
| fold_sequential | -0.374 | moderate- |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 35.1ns | 116.2ns | 30.2% | HIGH |
| fold_quad | 34.1ns | 65.2ns | 52.3% | HIGH |
| fold_sequential | 32.9ns | 207.0ns | 15.9% | HIGH |

## Distribution (algo ns)

```
fold_paired (n=160, range 73.0-257.0 ns)
     73.0 |########################################
     82.2 |
     91.4 |
    100.6 |
    109.8 |
    119.0 |
    128.2 |
    137.4 |
    146.6 |
    155.8 |
    165.0 |
    174.2 |
    183.4 |
    192.6 |
    201.8 |
    211.0 |
    220.2 |
    229.4 |#
    238.6 |
    247.8 |#
  (14 below, 19 above range)

fold_quad (n=160, range 45.0-143.0 ns)
     45.0 |########################################
     49.9 |
     54.8 |
     59.7 |
     64.6 |
     69.5 |
     74.4 |
     79.3 |
     84.2 |
     89.1 |
     94.0 |
     98.9 |
    103.8 |
    108.7 |
    113.6 |
    118.5 |
    123.4 |
    128.3 |#
    133.2 |#
    138.1 |
  (19 below, 18 above range)

fold_sequential (n=160, range 125.8-445.5 ns)
    125.8 |
    141.7 |########################################
    157.7 |
    173.7 |
    189.7 |
    205.7 |
    221.7 |
    237.7 |
    253.6 |
    269.6 |
    285.6 |
    301.6 |
    317.6 |
    333.6 |
    349.6 |
    365.6 |
    381.5 |#
    397.5 |
    413.5 |##
    429.5 |
  (28 below, 17 above range)

```

## Diagnostics

- **fold_paired**: CV=62.8% (high variance, measurements may be unstable)
- **fold_paired**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **fold_paired**: bridge=30.0% of algo (FFI overhead may distort results)
- **fold_quad**: CV=61.7% (high variance, measurements may be unstable)
- **fold_quad**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **fold_quad**: bridge=52.4% of algo (FFI overhead may distort results)
- **fold_sequential**: CV=60.0% (high variance, measurements may be unstable)
- **fold_sequential**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **fold_sequential**: bridge=15.9% of algo (FFI overhead may distort results)
