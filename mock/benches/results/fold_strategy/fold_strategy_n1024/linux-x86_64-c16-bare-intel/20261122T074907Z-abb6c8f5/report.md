# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 76% faster than the next best (fold_paired)

fold_quad (45 ns) leads fold_paired (79 ns) by 76%, a clear separation rather than a photo finish. CV 90.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 43% (significant)

fold_quad is -34 ns (43%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.3x slower than the field

fold_sequential (149 ns) is 3.3x the fastest (45 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### fold_quad is fastest but the noisiest (CV 90.4%)

fold_quad wins on median (45 ns) yet has the highest variance (CV 90.4%), while fold_sequential is the steadiest (CV 85.4%, 149 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3.3x the fastest

Fastest fold_quad (45 ns) to slowest fold_sequential (149 ns): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fold_quad is inconsistent: worst-20% is 3.8x its best-20%

fold_quad's best 20% of batches run at 37 ns but its worst 20% at 140 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fold_quad** at 45.0 ns median (-43.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.32x (fastest 45.0 ns, slowest 149.4 ns)

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
| Counter frequency | 2112775296 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fold_paired | 2823ns | 1958ns | 1677ns | 2127ns | 6056ns | base |
| fold_quad | 2778ns | 1924ns | 1571ns | 2090ns | 6051ns | -1.59% |
| fold_sequential | 2721ns | 2024ns | 1655ns | 1979ns | 6013ns | -3.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 114ns | 68ns | 244ns | base | 8.973 |
| fold_quad | 65ns | 37ns | 140ns | -43.15% | 15.784 |
| fold_sequential | 200ns | 122ns | 441ns | +75.54% | 5.111 |

## Performance model

- Peak throughput: **27.777 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 12.897 | 46.4% |
| fold_quad | 22.730 | 81.8% |
| fold_sequential | 6.854 | 24.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2823ns | 2823ns | base |
| fold_quad | 2778ns | 2778ns | -1.59% |
| fold_sequential | 2721ns | 2721ns | -3.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 79ns | base | --- | [79, 79] | --- | --- | --- | --- |
| fold_quad | 45ns | -34.4ns (-43.3%) | [-42, -34]ns | [45, 45] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 149ns | +69.2ns (+87.1%) | [+57, +70]ns | [122, 150] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 65ns | -43.2% | +88.2% |
| 1 | 2 | 79ns | -43.2% | +88.4% |
| 1 | 3 | 80ns | -53.5% | +53.8% |
| 1 | 4 | 65ns | -30.7% | +88.0% |
| 1 | 5 | 79ns | -53.5% | +54.2% |
| 1 | 6 | 79ns | -43.1% | +88.4% |
| 1 | 7 | 79ns | -43.2% | +54.0% |
| 1 | 8 | 79ns | -43.2% | +88.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.210 | moderate- |
| fold_quad | -0.471 | moderate- |
| fold_sequential | -0.476 | moderate- |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 34.2ns | 114.1ns | 30.0% | HIGH |
| fold_quad | 34.1ns | 64.9ns | 52.5% | HIGH |
| fold_sequential | 31.8ns | 200.3ns | 15.9% | HIGH |

## Distribution (algo ns)

```
fold_paired (n=160, range 68.1-243.8 ns)
     68.1 |
     76.9 |########################################
     85.7 |#
     94.4 |
    103.2 |
    112.0 |
    120.8 |
    129.6 |
    138.4 |
    147.1 |
    155.9 |
    164.7 |
    173.5 |
    182.3 |##
    191.1 |
    199.8 |#
    208.6 |
    217.4 |#
    226.2 |#
    235.0 |#
  (25 below, 14 above range)

fold_quad (n=160, range 36.9-140.0 ns)
     36.9 |###############
     42.0 |########################################
     47.2 |
     52.3 |
     57.5 |
     62.7 |
     67.8 |
     73.0 |
     78.1 |
     83.3 |
     88.4 |
     93.6 |
     98.8 |
    103.9 |#
    109.1 |
    114.2 |##
    119.4 |
    124.5 |####
    129.7 |#
    134.9 |##
  (11 below, 13 above range)

fold_sequential (n=160, range 122.3-440.8 ns)
    122.3 |########################################
    138.2 |################################
    154.1 |
    170.0 |
    186.0 |
    201.9 |
    217.8 |
    233.7 |##
    249.7 |
    265.6 |
    281.5 |
    297.5 |##
    313.4 |
    329.3 |#
    345.2 |
    361.2 |##
    377.1 |##
    393.0 |##
    408.9 |##
    424.9 |
  (14 below, 14 above range)

```

## Diagnostics

- **fold_paired**: CV=60.7% (high variance, measurements may be unstable)
- **fold_paired**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fold_paired**: bridge=30.1% of algo (FFI overhead may distort results)
- **fold_quad**: CV=62.7% (high variance, measurements may be unstable)
- **fold_quad**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **fold_quad**: bridge=52.8% of algo (FFI overhead may distort results)
- **fold_sequential**: CV=63.7% (high variance, measurements may be unstable)
- **fold_sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fold_sequential**: bridge=15.9% of algo (FFI overhead may distort results)
