# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 76% faster than the next best (fold_paired)

fold_quad (45 ns) leads fold_paired (79 ns) by 76%, a clear separation rather than a photo finish. CV 82.6%.

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

### fold_paired is inconsistent: worst-20% is 3.6x its best-20%

fold_paired's best 20% of batches run at 65 ns but its worst 20% at 232 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fold_quad** at 45.2 ns median (-43.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.31x (fastest 45.2 ns, slowest 149.6 ns)

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
| Counter frequency | 2112849597 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fold_paired | 2619ns | 1964ns | 1606ns | 1915ns | 5745ns | base |
| fold_quad | 2594ns | 1930ns | 1598ns | 1931ns | 5579ns | -0.95% |
| fold_sequential | 2770ns | 2035ns | 1836ns | 2036ns | 5905ns | +5.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 106ns | 65ns | 232ns | base | 9.672 |
| fold_quad | 61ns | 37ns | 130ns | -42.70% | 16.880 |
| fold_sequential | 203ns | 135ns | 429ns | +91.43% | 5.052 |

## Performance model

- Peak throughput: **27.430 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 12.897 | 47.0% |
| fold_quad | 22.655 | 82.6% |
| fold_sequential | 6.845 | 25.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2619ns | 2619ns | base |
| fold_quad | 2594ns | 2594ns | -0.95% |
| fold_sequential | 2770ns | 2770ns | +5.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 79ns | base | --- | [79, 79] | --- | --- | --- | --- |
| fold_quad | 45ns | -34.2ns (-43.1%) | [-34, -34]ns | [45, 45] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 150ns | +70.4ns (+88.7%) | [+70, +84]ns | [150, 150] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 80ns | -43.1% | +88.1% |
| 1 | 2 | 79ns | -43.0% | +88.5% |
| 1 | 3 | 80ns | -53.6% | +88.2% |
| 1 | 4 | 65ns | -30.8% | +129.0% |
| 1 | 5 | 80ns | -43.1% | +88.2% |
| 1 | 6 | 65ns | -30.5% | +130.5% |
| 1 | 7 | 65ns | -30.6% | +129.6% |
| 1 | 8 | 80ns | -53.5% | +54.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.138 | ok |
| fold_quad | -0.211 | moderate- |
| fold_sequential | -0.015 | ok |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 32.0ns | 105.9ns | 30.2% | HIGH |
| fold_quad | 32.1ns | 60.7ns | 53.0% | HIGH |
| fold_sequential | 32.2ns | 202.7ns | 15.9% | HIGH |

## Distribution (algo ns)

```
fold_paired (n=160, range 65.0-232.2 ns)
     65.0 |##################
     73.3 |########################################
     81.7 |
     90.1 |
     98.4 |
    106.8 |
    115.2 |
    123.5 |
    131.9 |
    140.2 |
    148.6 |
    157.0 |
    165.3 |
    173.7 |
    182.0 |
    190.4 |
    198.8 |
    207.1 |
    215.5 |
    223.9 |
  (9 below, 19 above range)

fold_quad (n=160, range 37.3-130.5 ns)
     37.3 |
     42.0 |########################################
     46.6 |
     51.3 |
     56.0 |
     60.6 |
     65.3 |
     69.9 |
     74.6 |
     79.2 |
     83.9 |
     88.5 |
     93.2 |
     97.9 |
    102.5 |
    107.2 |
    111.8 |
    116.5 |
    121.1 |##
    125.8 |
  (30 below, 16 above range)

fold_sequential (n=160, range 135.0-429.4 ns)
    135.0 |########################################
    149.8 |##############
    164.5 |
    179.2 |
    193.9 |
    208.6 |
    223.3 |
    238.1 |
    252.8 |
    267.5 |
    282.2 |
    296.9 |
    311.6 |
    326.4 |
    341.1 |
    355.8 |
    370.5 |
    385.2 |
    399.9 |
    414.7 |
  (17 below, 21 above range)

```

## Diagnostics

- **fold_paired**: CV=66.0% (high variance, measurements may be unstable)
- **fold_paired**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fold_paired**: bridge=30.1% of algo (FFI overhead may distort results)
- **fold_quad**: CV=61.5% (high variance, measurements may be unstable)
- **fold_quad**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **fold_quad**: bridge=52.7% of algo (FFI overhead may distort results)
- **fold_sequential**: CV=64.2% (high variance, measurements may be unstable)
- **fold_sequential**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **fold_sequential**: bridge=16.0% of algo (FFI overhead may distort results)
