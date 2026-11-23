# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 93% faster than the next best (fold_paired)

fold_quad (152 ns) leads fold_paired (294 ns) by 93%, a clear separation rather than a photo finish. CV 85.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 48% (significant)

fold_quad is -141 ns (48%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.8x slower than the field

fold_sequential (578 ns) is 3.8x the fastest (152 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### fold_quad is fastest but the noisiest (CV 85.3%)

fold_quad wins on median (152 ns) yet has the highest variance (CV 85.3%), while fold_sequential is the steadiest (CV 66.7%, 578 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3.8x the fastest

Fastest fold_quad (152 ns) to slowest fold_sequential (578 ns): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fold_quad is inconsistent: worst-20% is 3.6x its best-20%

fold_quad's best 20% of batches run at 124 ns but its worst 20% at 444 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fold_quad** at 152.1 ns median (-48.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.80x (fastest 152.1 ns, slowest 577.5 ns)

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
| fold_paired | 2509ns | 2169ns | 1775ns | 2087ns | 4508ns | base |
| fold_quad | 2638ns | 2024ns | 1657ns | 1858ns | 5957ns | +5.16% |
| fold_sequential | 2983ns | 2454ns | 2010ns | 2447ns | 5565ns | +18.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 338ns | 240ns | 603ns | base | 12.114 |
| fold_quad | 197ns | 124ns | 444ns | -41.66% | 20.764 |
| fold_sequential | 700ns | 472ns | 1299ns | +106.94% | 5.854 |

## Performance model

- Peak throughput: **32.901 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 13.946 | 42.4% |
| fold_quad | 26.930 | 81.8% |
| fold_sequential | 7.093 | 21.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2509ns | 2509ns | base |
| fold_quad | 2638ns | 2638ns | +5.16% |
| fold_sequential | 2983ns | 2983ns | +18.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 294ns | base | --- | [294, 294] | --- | --- | --- | --- |
| fold_quad | 152ns | -141.4ns (-48.1%) | [-142, -122]ns | [125, 152] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 578ns | +283.8ns (+96.6%) | [+284, +284]ns | [577, 578] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 240ns | -36.7% | +140.3% |
| 1 | 2 | 294ns | -57.5% | +60.9% |
| 1 | 3 | 240ns | -48.2% | +96.6% |
| 1 | 4 | 294ns | -57.5% | +96.6% |
| 1 | 5 | 294ns | -48.1% | +96.6% |
| 1 | 6 | 294ns | -48.2% | +96.6% |
| 1 | 7 | 240ns | -48.2% | +140.3% |
| 1 | 8 | 294ns | -57.6% | +96.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.475 | moderate- |
| fold_quad | 0.061 | ok |
| fold_sequential | 0.292 | moderate+ |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 27.1ns | 338.1ns | 8.0% | HIGH |
| fold_quad | 30.7ns | 197.3ns | 15.6% | HIGH |
| fold_sequential | 28.6ns | 699.7ns | 4.1% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 240.2-603.4 ns)
    240.2 |#############
    258.4 |
    276.5 |########################################
    294.7 |#
    312.9 |
    331.0 |
    349.2 |
    367.3 |
    385.5 |
    403.7 |
    421.8 |
    440.0 |
    458.1 |
    476.3 |
    494.5 |
    512.6 |
    530.8 |
    548.9 |
    567.1 |
    585.3 |
  (24 below, 13 above range)

fold_quad (n=160, range 124.5-444.0 ns)
    124.5 |########################################
    140.5 |#########################
    156.4 |#
    172.4 |
    188.4 |
    204.4 |
    220.3 |
    236.3 |
    252.3 |
    268.3 |#
    284.2 |
    300.2 |
    316.2 |
    332.2 |
    348.1 |
    364.1 |
    380.1 |#
    396.1 |
    412.0 |#
    428.0 |
  (3 below, 18 above range)

fold_sequential (n=160, range 472.5-1298.8 ns)
    472.5 |#######
    513.8 |
    555.1 |########################################
    596.4 |
    637.7 |
    679.1 |
    720.4 |
    761.7 |
    803.0 |
    844.3 |
    885.6 |
    927.0 |
    968.3 |
   1009.6 |
   1050.9 |
   1092.2 |
   1133.5 |
   1174.9 |
   1216.2 |#
   1257.5 |
  (15 below, 16 above range)

```

## Diagnostics

- **fold_paired**: CV=59.0% (high variance, measurements may be unstable)
- **fold_quad**: CV=65.7% (high variance, measurements may be unstable)
- **fold_quad**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fold_quad**: bridge=15.5% of algo (FFI overhead may distort results)
- **fold_sequential**: CV=55.1% (high variance, measurements may be unstable)
