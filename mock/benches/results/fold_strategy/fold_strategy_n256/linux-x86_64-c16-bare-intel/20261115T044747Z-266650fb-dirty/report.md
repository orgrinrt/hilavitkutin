# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 23% faster than the next best (fold_paired)

fold_quad (21 ns) leads fold_paired (26 ns) by 23%, a clear separation rather than a photo finish. CV 87.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_sequential is an outlier: 2.0x slower than the field

fold_sequential (43 ns) is 2.0x the fastest (21 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### fold_sequential is inconsistent: worst-20% is 3.8x its best-20%

fold_sequential's best 20% of batches run at 35 ns but its worst 20% at 133 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fold_quad** at 21.2 ns median (-18.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.03x (fastest 21.2 ns, slowest 43.0 ns)

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
| fold_paired | 2775ns | 1904ns | 1610ns | 2141ns | 5843ns | base |
| fold_quad | 2573ns | 1894ns | 1551ns | 1821ns | 5853ns | -7.28% |
| fold_sequential | 2636ns | 1911ns | 1567ns | 1895ns | 5927ns | -5.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 38ns | 22ns | 79ns | base | 6.689 |
| fold_quad | 28ns | 17ns | 64ns | -25.57% | 8.986 |
| fold_sequential | 59ns | 35ns | 133ns | +54.48% | 4.330 |

## Performance model

- Peak throughput: **14.784 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 9.808 | 66.3% |
| fold_quad | 12.075 | 81.7% |
| fold_sequential | 5.953 | 40.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2775ns | 2775ns | base |
| fold_quad | 2573ns | 2573ns | -7.28% |
| fold_sequential | 2636ns | 2636ns | -5.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 26ns | base | --- | [26, 26] | --- | --- | --- | --- |
| fold_quad | 21ns | -4.9ns (-18.8%) | [-9, -5]ns | [21, 21] | YES | 0.0000 | 0.0000 | 3 |
| fold_sequential | 43ns | +16.8ns (+64.4%) | [+14, +17]ns | [35, 43] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 26ns | -18.6% | +35.1% |
| 1 | 2 | 21ns | -18.7% | +65.0% |
| 1 | 3 | 26ns | -33.3% | +35.2% |
| 1 | 4 | 44ns | -60.8% | -20.7% |
| 1 | 5 | 26ns | -18.8% | +65.1% |
| 1 | 6 | 21ns | -0.5% | +102.3% |
| 1 | 7 | 26ns | -18.6% | +35.5% |
| 1 | 8 | 26ns | -33.3% | +64.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.028 | ok |
| fold_quad | 0.125 | ok |
| fold_sequential | 0.061 | ok |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 34.2ns | 38.3ns | 89.3% | HIGH |
| fold_quad | 31.7ns | 28.5ns | 111.2% | HIGH |
| fold_sequential | 32.1ns | 59.1ns | 54.4% | HIGH |

## Distribution (algo ns)

```
fold_paired (n=160, range 22.0-79.0 ns)
     22.0 |
     24.9 |########################################
     27.7 |
     30.6 |
     33.4 |#
     36.3 |
     39.1 |
     42.0 |##########
     44.8 |
     47.7 |
     50.5 |
     53.4 |
     56.2 |
     59.1 |
     61.9 |#
     64.8 |#
     67.6 |#
     70.5 |#
     73.3 |
     76.2 |##
  (27 below, 17 above range)

fold_quad (n=160, range 17.3-64.4 ns)
     17.3 |####################
     19.7 |########################################
     22.0 |
     24.4 |
     26.7 |
     29.1 |
     31.4 |
     33.8 |
     36.1 |
     38.5 |
     40.8 |
     43.2 |
     45.5 |
     47.9 |
     50.2 |
     52.6 |
     54.9 |#
     57.3 |#
     59.7 |#
     62.0 |####
  (24 below, 14 above range)

fold_sequential (n=160, range 35.2-133.2 ns)
     35.2 |########################################
     40.1 |###############################
     45.0 |
     49.9 |
     54.8 |
     59.7 |
     64.6 |
     69.5 |#
     74.4 |
     79.3 |
     84.2 |
     89.1 |
     94.0 |
     98.9 |
    103.8 |
    108.7 |##
    113.6 |#
    118.5 |##
    123.4 |##
    128.3 |##
  (14 below, 14 above range)

```

## Diagnostics

- **fold_paired**: CV=57.6% (high variance, measurements may be unstable)
- **fold_paired**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fold_paired**: bridge=90.4% of algo (FFI overhead may distort results)
- **fold_quad**: CV=65.2% (high variance, measurements may be unstable)
- **fold_quad**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **fold_quad**: bridge=110.8% of algo (FFI overhead may distort results)
- **fold_sequential**: CV=66.3% (high variance, measurements may be unstable)
- **fold_sequential**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **fold_sequential**: bridge=54.4% of algo (FFI overhead may distort results)
