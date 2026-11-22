# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 23% faster than the next best (fold_paired)

fold_quad (21 ns) leads fold_paired (26 ns) by 23%, a clear separation rather than a photo finish. CV 101.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_sequential is an outlier: 2.0x slower than the field

fold_sequential (43 ns) is 2.0x the fastest (21 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### fold_quad is fastest but the noisiest (CV 101.4%)

fold_quad wins on median (21 ns) yet has the highest variance (CV 101.4%), while fold_paired is the steadiest (CV 87.6%, 26 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fold_quad is inconsistent: worst-20% is 4.1x its best-20%

fold_quad's best 20% of batches run at 17 ns but its worst 20% at 70 ns (4.1x) - a bimodal or bursty profile the median hides.

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
| Counter frequency | 2112849597 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fold_paired | 2701ns | 1910ns | 1561ns | 2052ns | 5791ns | base |
| fold_quad | 2705ns | 1905ns | 1558ns | 1959ns | 6089ns | +0.13% |
| fold_sequential | 2722ns | 1926ns | 1575ns | 1983ns | 6085ns | +0.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 37ns | 21ns | 78ns | base | 6.983 |
| fold_quad | 31ns | 17ns | 70ns | -16.59% | 8.372 |
| fold_sequential | 61ns | 35ns | 134ns | +65.13% | 4.229 |

## Performance model

- Peak throughput: **14.774 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 9.808 | 66.4% |
| fold_quad | 12.075 | 81.7% |
| fold_sequential | 5.940 | 40.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2701ns | 2701ns | base |
| fold_quad | 2705ns | 2705ns | +0.13% |
| fold_sequential | 2722ns | 2722ns | +0.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 26ns | base | --- | [23, 26] | --- | --- | --- | --- |
| fold_quad | 21ns | -4.5ns (-17.0%) | [-5, -4]ns | [21, 21] | YES | 0.0000 | 0.0000 | 6 |
| fold_sequential | 43ns | +21.5ns (+82.4%) | [+17, +22]ns | [43, 43] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 21ns | -0.9% | +101.4% |
| 1 | 2 | 21ns | -0.9% | +65.0% |
| 1 | 3 | 21ns | -0.5% | +101.4% |
| 1 | 4 | 26ns | -33.6% | +34.7% |
| 1 | 5 | 26ns | -33.6% | +34.7% |
| 1 | 6 | 21ns | -18.7% | +65.0% |
| 1 | 7 | 26ns | -18.6% | +65.1% |
| 1 | 8 | 21ns | -18.9% | +100.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.132 | ok |
| fold_quad | 0.116 | ok |
| fold_sequential | -0.125 | ok |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 33.3ns | 36.7ns | 90.8% | HIGH |
| fold_quad | 33.4ns | 30.6ns | 109.1% | HIGH |
| fold_sequential | 33.3ns | 60.5ns | 54.9% | HIGH |

## Distribution (algo ns)

```
fold_paired (n=160, range 21.3-78.0 ns)
     21.3 |#################################
     24.1 |########################################
     27.0 |
     29.8 |
     32.7 |
     35.5 |
     38.3 |
     41.2 |
     44.0 |
     46.8 |#
     49.7 |
     52.5 |
     55.3 |
     58.2 |###
     61.0 |###
     63.9 |
     66.7 |####
     69.5 |##
     72.4 |####
     75.2 |
  (29 below, 16 above range)

fold_quad (n=160, range 17.3-70.2 ns)
     17.3 |####################
     20.0 |########################################
     22.6 |
     25.3 |
     27.9 |
     30.6 |
     33.2 |
     35.8 |
     38.5 |
     41.1 |
     43.8 |
     46.4 |
     49.1 |
     51.7 |#
     54.3 |##
     57.0 |#
     59.6 |#
     62.3 |#
     64.9 |#
     67.6 |##
  (23 below, 13 above range)

fold_sequential (n=160, range 35.2-134.1 ns)
     35.2 |####################
     40.2 |########################################
     45.1 |##
     50.0 |
     55.0 |
     59.9 |
     64.9 |
     69.8 |
     74.8 |
     79.7 |
     84.6 |
     89.6 |
     94.5 |#
     99.5 |#
    104.4 |
    109.4 |#
    114.3 |###
    119.2 |
    124.2 |#
    129.1 |#
  (24 below, 16 above range)

```

## Diagnostics

- **fold_paired**: CV=62.4% (high variance, measurements may be unstable)
- **fold_paired**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **fold_paired**: bridge=90.4% of algo (FFI overhead may distort results)
- **fold_quad**: CV=70.3% (high variance, measurements may be unstable)
- **fold_quad**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **fold_quad**: bridge=111.3% of algo (FFI overhead may distort results)
- **fold_sequential**: CV=64.5% (high variance, measurements may be unstable)
- **fold_sequential**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **fold_sequential**: bridge=54.8% of algo (FFI overhead may distort results)
