# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 23% faster than the next best (fold_paired)

fold_quad (21 ns) leads fold_paired (26 ns) by 23%, a clear separation rather than a photo finish. CV 88.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_sequential is an outlier: 2.0x slower than the field

fold_sequential (43 ns) is 2.0x the fastest (21 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### fold_paired shows alternating (throttle bounce) (autocorr -0.68)

fold_paired's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### fold_paired is inconsistent: worst-20% is 3.8x its best-20%

fold_paired's best 20% of batches run at 21 ns but its worst 20% at 81 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fold_quad** at 21.2 ns median (-18.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.03x (fastest 21.2 ns, slowest 43.1 ns)

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
| fold_paired | 2661ns | 1902ns | 1556ns | 1924ns | 5977ns | base |
| fold_quad | 2538ns | 1899ns | 1551ns | 1852ns | 5584ns | -4.62% |
| fold_sequential | 2648ns | 1920ns | 1569ns | 2005ns | 5657ns | -0.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 36ns | 21ns | 81ns | base | 7.075 |
| fold_quad | 28ns | 17ns | 61ns | -22.21% | 9.094 |
| fold_sequential | 59ns | 35ns | 125ns | +63.08% | 4.338 |

## Performance model

- Peak throughput: **14.744 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 9.808 | 66.5% |
| fold_quad | 12.075 | 81.9% |
| fold_sequential | 5.940 | 40.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2661ns | 2661ns | base |
| fold_quad | 2538ns | 2538ns | -4.62% |
| fold_sequential | 2648ns | 2648ns | -0.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 26ns | base | --- | [26, 26] | --- | --- | --- | --- |
| fold_quad | 21ns | -4.9ns (-18.8%) | [-5, -5]ns | [21, 21] | YES | 0.0000 | 0.0000 | 6 |
| fold_sequential | 43ns | +17.0ns (+65.1%) | [+17, +17]ns | [43, 43] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 26ns | -33.3% | +65.1% |
| 1 | 2 | 21ns | -0.9% | +65.0% |
| 1 | 3 | 26ns | -33.3% | +35.2% |
| 1 | 4 | 21ns | -18.7% | +101.4% |
| 1 | 5 | 26ns | -18.5% | +64.8% |
| 1 | 6 | 21ns | -0.9% | +65.0% |
| 1 | 7 | 26ns | -18.9% | +65.2% |
| 1 | 8 | 26ns | -18.8% | +65.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.677 | HIGH- (thermal bounce) |
| fold_quad | 0.054 | ok |
| fold_sequential | -0.142 | ok |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 32.8ns | 36.2ns | 90.6% | HIGH |
| fold_quad | 31.4ns | 28.1ns | 111.5% | HIGH |
| fold_sequential | 32.3ns | 59.0ns | 54.8% | HIGH |

## Distribution (algo ns)

```
fold_paired (n=160, range 21.3-80.5 ns)
     21.3 |#############
     24.3 |########################################
     27.3 |#
     30.2 |
     33.2 |
     36.1 |
     39.1 |
     42.1 |
     45.0 |#
     48.0 |
     50.9 |
     53.9 |
     56.8 |
     59.8 |
     62.8 |
     65.7 |#
     68.7 |
     71.6 |###
     74.6 |
     77.6 |###
  (16 below, 14 above range)

fold_quad (n=160, range 17.4-61.4 ns)
     17.4 |#################
     19.6 |########################################
     21.8 |
     24.0 |
     26.2 |
     28.4 |
     30.6 |
     32.8 |
     35.0 |
     37.2 |
     39.4 |
     41.6 |
     43.8 |
     46.0 |
     48.2 |
     50.4 |
     52.6 |
     54.8 |
     57.0 |
     59.2 |
  (10 below, 24 above range)

fold_sequential (n=160, range 35.2-125.1 ns)
     35.2 |##########
     39.7 |########################################
     44.2 |
     48.7 |
     53.2 |
     57.7 |
     62.2 |
     66.7 |
     71.2 |
     75.7 |
     80.2 |
     84.7 |
     89.2 |
     93.7 |##
     98.2 |
    102.7 |##
    107.2 |#
    111.6 |
    116.1 |#
    120.6 |
  (19 below, 14 above range)

```

## Diagnostics

- **fold_paired**: CV=64.1% (high variance, measurements may be unstable)
- **fold_paired**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **fold_paired**: bridge=90.4% of algo (FFI overhead may distort results)
- **fold_quad**: CV=66.6% (high variance, measurements may be unstable)
- **fold_quad**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **fold_quad**: bridge=111.3% of algo (FFI overhead may distort results)
- **fold_sequential**: CV=60.4% (high variance, measurements may be unstable)
- **fold_sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fold_sequential**: bridge=54.8% of algo (FFI overhead may distort results)
