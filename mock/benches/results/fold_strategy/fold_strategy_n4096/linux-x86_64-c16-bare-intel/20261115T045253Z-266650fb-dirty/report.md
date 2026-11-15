# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 93% faster than the next best (fold_paired)

fold_quad (152 ns) leads fold_paired (294 ns) by 93%, a clear separation rather than a photo finish. CV 87.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 48% (significant)

fold_quad is -142 ns (48%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.8x slower than the field

fold_sequential (578 ns) is 3.8x the fastest (152 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### fold_quad is fastest but the noisiest (CV 87.4%)

fold_quad wins on median (152 ns) yet has the highest variance (CV 87.4%), while fold_sequential is the steadiest (CV 66.0%, 578 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3.8x the fastest

Fastest fold_quad (152 ns) to slowest fold_sequential (578 ns): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fold_paired is inconsistent: worst-20% is 3.4x its best-20%

fold_paired's best 20% of batches run at 245 ns but its worst 20% at 837 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fold_quad** at 152.2 ns median (-48.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.79x (fastest 152.2 ns, slowest 577.5 ns)

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
| fold_paired | 2910ns | 2170ns | 1806ns | 2172ns | 6230ns | base |
| fold_quad | 2947ns | 2032ns | 1901ns | 2159ns | 6354ns | +1.25% |
| fold_sequential | 3014ns | 2454ns | 2200ns | 2455ns | 5506ns | +3.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 393ns | 245ns | 837ns | base | 10.431 |
| fold_quad | 220ns | 143ns | 473ns | -43.93% | 18.603 |
| fold_sequential | 707ns | 518ns | 1283ns | +79.97% | 5.796 |

## Performance model

- Peak throughput: **28.720 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 13.946 | 48.6% |
| fold_quad | 26.912 | 93.7% |
| fold_sequential | 7.093 | 24.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2910ns | 2910ns | base |
| fold_quad | 2947ns | 2947ns | +1.25% |
| fold_sequential | 3014ns | 3014ns | +3.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 294ns | base | --- | [294, 294] | --- | --- | --- | --- |
| fold_quad | 152ns | -141.5ns (-48.2%) | [-142, -141]ns | [152, 152] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 578ns | +283.9ns (+96.7%) | [+284, +284]ns | [578, 578] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 294ns | -48.2% | +96.7% |
| 1 | 2 | 294ns | -57.6% | +96.6% |
| 1 | 3 | 294ns | -48.2% | +96.6% |
| 1 | 4 | 240ns | -36.7% | +140.3% |
| 1 | 5 | 294ns | -48.2% | +96.6% |
| 1 | 6 | 240ns | -36.7% | +140.3% |
| 1 | 7 | 294ns | -48.2% | +96.6% |
| 1 | 8 | 294ns | -48.1% | +60.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.376 | moderate- |
| fold_quad | -0.160 | ok |
| fold_sequential | -0.018 | ok |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 31.5ns | 392.7ns | 8.0% | HIGH |
| fold_quad | 33.9ns | 220.2ns | 15.4% | HIGH |
| fold_sequential | 28.8ns | 706.7ns | 4.1% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 245.3-837.1 ns)
    245.3 |#
    274.9 |########################################
    304.5 |
    334.0 |
    363.6 |
    393.2 |
    422.8 |
    452.4 |
    482.0 |
    511.6 |
    541.2 |
    570.8 |
    600.3 |
    629.9 |
    659.5 |
    689.1 |
    718.7 |
    748.3 |
    777.9 |
    807.5 |
  (27 below, 21 above range)

fold_quad (n=160, range 142.6-473.2 ns)
    142.6 |########################################
    159.1 |
    175.7 |
    192.2 |
    208.7 |
    225.3 |
    241.8 |
    258.3 |
    274.9 |
    291.4 |
    307.9 |
    324.5 |
    341.0 |
    357.5 |
    374.1 |
    390.6 |
    407.1 |
    423.6 |
    440.2 |#
    456.7 |
  (11 below, 17 above range)

fold_sequential (n=160, range 518.2-1282.8 ns)
    518.2 |
    556.4 |########################################
    594.7 |
    632.9 |
    671.1 |
    709.4 |
    747.6 |
    785.8 |
    824.0 |
    862.3 |
    900.5 |
    938.7 |
    977.0 |
   1015.2 |
   1053.4 |
   1091.7 |
   1129.9 |
   1168.1 |
   1206.4 |
   1244.6 |
  (18 below, 18 above range)

```

## Diagnostics

- **fold_paired**: CV=61.5% (high variance, measurements may be unstable)
- **fold_paired**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **fold_quad**: CV=60.4% (high variance, measurements may be unstable)
- **fold_quad**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **fold_quad**: bridge=15.4% of algo (FFI overhead may distort results)
- **fold_sequential**: CV=53.9% (high variance, measurements may be unstable)
