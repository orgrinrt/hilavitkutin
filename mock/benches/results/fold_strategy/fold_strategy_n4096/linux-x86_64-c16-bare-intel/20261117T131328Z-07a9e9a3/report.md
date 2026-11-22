# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 93% faster than the next best (fold_paired)

fold_quad (152 ns) leads fold_paired (294 ns) by 93%, a clear separation rather than a photo finish. CV 83.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 58% (significant)

fold_quad is -169 ns (58%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.8x slower than the field

fold_sequential (577 ns) is 3.8x the fastest (152 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.8x the fastest

Fastest fold_quad (152 ns) to slowest fold_sequential (577 ns): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fold_paired is inconsistent: worst-20% is 3.7x its best-20%

fold_paired's best 20% of batches run at 240 ns but its worst 20% at 881 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fold_quad** at 152.2 ns median (-48.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.79x (fastest 152.2 ns, slowest 577.4 ns)

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
| fold_paired | 3105ns | 2180ns | 1785ns | 2382ns | 6593ns | base |
| fold_quad | 2715ns | 2037ns | 1666ns | 1959ns | 6030ns | -12.57% |
| fold_sequential | 3054ns | 2464ns | 2012ns | 2528ns | 5675ns | -1.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 417ns | 240ns | 881ns | base | 9.826 |
| fold_quad | 202ns | 124ns | 444ns | -51.64% | 20.320 |
| fold_sequential | 719ns | 472ns | 1334ns | +72.37% | 5.701 |

## Performance model

- Peak throughput: **32.907 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 13.946 | 42.4% |
| fold_quad | 26.912 | 81.8% |
| fold_sequential | 7.094 | 21.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 3105ns | 3105ns | base |
| fold_quad | 2715ns | 2715ns | -12.57% |
| fold_sequential | 3054ns | 3054ns | -1.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 294ns | base | --- | [294, 294] | --- | --- | --- | --- |
| fold_quad | 152ns | -169.0ns (-57.5%) | [-169, -142]ns | [152, 152] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 577ns | +283.7ns (+96.6%) | [+179, +286]ns | [577, 578] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 294ns | -57.6% | +96.6% |
| 1 | 2 | 294ns | -57.6% | +60.9% |
| 1 | 3 | 240ns | -48.2% | +294.7% |
| 1 | 4 | 294ns | -48.1% | +60.9% |
| 1 | 5 | 240ns | -36.6% | +140.3% |
| 1 | 6 | 703ns | -78.3% | -17.9% |
| 1 | 7 | 294ns | -57.6% | +101.3% |
| 1 | 8 | 294ns | -48.2% | +60.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.215 | moderate- |
| fold_quad | 0.129 | ok |
| fold_sequential | -0.475 | moderate- |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 33.5ns | 416.8ns | 8.0% | HIGH |
| fold_quad | 31.3ns | 201.6ns | 15.5% | HIGH |
| fold_sequential | 29.2ns | 718.5ns | 4.1% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 240.3-880.6 ns)
    240.3 |###########
    272.3 |########################################
    304.3 |
    336.3 |
    368.3 |
    400.4 |
    432.4 |
    464.4 |
    496.4 |
    528.4 |
    560.4 |
    592.5 |
    624.5 |
    656.5 |
    688.5 |#
    720.5 |
    752.5 |#
    784.6 |#
    816.6 |
    848.6 |
  (10 below, 17 above range)

fold_quad (n=160, range 124.5-443.7 ns)
    124.5 |###############################
    140.4 |########################################
    156.4 |#
    172.4 |
    188.3 |
    204.3 |
    220.2 |
    236.2 |
    252.2 |
    268.1 |
    284.1 |
    300.0 |#
    316.0 |
    332.0 |
    347.9 |
    363.9 |#####
    379.9 |
    395.8 |
    411.8 |#
    427.7 |
  (9 below, 16 above range)

fold_sequential (n=160, range 472.4-1333.7 ns)
    472.4 |###################
    515.5 |##
    558.6 |########################################
    601.6 |##
    644.7 |###
    687.7 |
    730.8 |
    773.9 |
    816.9 |
    860.0 |
    903.1 |
    946.1 |#############
    989.2 |
   1032.3 |
   1075.3 |
   1118.4 |
   1161.5 |
   1204.5 |
   1247.6 |
   1290.7 |##
  (20 below, 15 above range)

```

## Diagnostics

- **fold_paired**: CV=59.6% (high variance, measurements may be unstable)
- **fold_paired**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **fold_quad**: CV=63.2% (high variance, measurements may be unstable)
- **fold_quad**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fold_quad**: bridge=15.5% of algo (FFI overhead may distort results)
- **fold_sequential**: CV=50.7% (high variance, measurements may be unstable)
