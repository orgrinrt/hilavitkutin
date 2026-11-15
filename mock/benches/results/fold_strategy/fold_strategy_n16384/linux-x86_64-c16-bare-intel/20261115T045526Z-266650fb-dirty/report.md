# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 98% faster than the next best (fold_paired)

fold_quad (580 ns) leads fold_paired (1.15 us) by 98%, a clear separation rather than a photo finish. CV 70.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 49% (significant)

fold_quad is -568 ns (49%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.9x slower than the field

fold_sequential (2.29 us) is 3.9x the fastest (580 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.9x the fastest

Fastest fold_quad (580 ns) to slowest fold_sequential (2.29 us): 3.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fold_quad is inconsistent: worst-20% is 2.8x its best-20%

fold_quad's best 20% of batches run at 524 ns but its worst 20% at 1.48 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fold_quad** at 580.2 ns median (-49.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.94x (fastest 580.2 ns, slowest 2287.4 ns)

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
| fold_paired | 3241ns | 3027ns | 2698ns | 3028ns | 4424ns | base |
| fold_quad | 3244ns | 2460ns | 2218ns | 2622ns | 6133ns | +0.08% |
| fold_sequential | 4894ns | 4166ns | 3694ns | 4168ns | 8270ns | +50.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 1228ns | 1025ns | 1669ns | base | 13.342 |
| fold_quad | 776ns | 524ns | 1481ns | -36.82% | 21.116 |
| fold_sequential | 2684ns | 2029ns | 4529ns | +118.56% | 6.104 |

## Performance model

- Peak throughput: **31.263 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 14.264 | 45.6% |
| fold_quad | 28.239 | 90.3% |
| fold_sequential | 7.163 | 22.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 3241ns | 3241ns | base |
| fold_quad | 3244ns | 3244ns | +0.08% |
| fold_sequential | 4894ns | 4894ns | +50.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 1149ns | base | --- | [1149, 1149] | --- | --- | --- | --- |
| fold_quad | 580ns | -568.3ns (-49.5%) | [-568, -568]ns | [580, 580] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 2287ns | +1138.7ns (+99.1%) | [+1139, +1139]ns | [2287, 2287] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 1149ns | -49.5% | +99.2% |
| 1 | 2 | 940ns | -49.5% | +99.2% |
| 1 | 3 | 1149ns | -49.5% | +99.1% |
| 1 | 4 | 1149ns | -49.5% | +99.1% |
| 1 | 5 | 1149ns | -49.5% | +99.2% |
| 1 | 6 | 1149ns | -10.7% | +99.1% |
| 1 | 7 | 1149ns | -49.5% | +99.2% |
| 1 | 8 | 1149ns | -49.5% | +99.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.161 | ok |
| fold_quad | -0.083 | ok |
| fold_sequential | -0.161 | ok |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 25.4ns | 1228.0ns | 2.1% |  |
| fold_quad | 31.2ns | 775.9ns | 4.0% |  |
| fold_sequential | 27.9ns | 2684.0ns | 1.0% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 1025.5-1668.9 ns)
   1025.5 |
   1057.6 |
   1089.8 |
   1122.0 |########################################
   1154.2 |######
   1186.3 |
   1218.5 |
   1250.7 |
   1282.8 |
   1315.0 |
   1347.2 |
   1379.4 |
   1411.5 |
   1443.7 |
   1475.9 |
   1508.0 |
   1540.2 |
   1572.4 |
   1604.6 |
   1636.7 |
  (19 below, 9 above range)

fold_quad (n=160, range 524.1-1480.8 ns)
    524.1 |
    571.9 |########################################
    619.7 |
    667.6 |
    715.4 |
    763.3 |
    811.1 |
    858.9 |
    906.8 |
    954.6 |
   1002.5 |#######
   1050.3 |
   1098.1 |
   1146.0 |
   1193.8 |
   1241.6 |
   1289.5 |
   1337.3 |
   1385.2 |
   1433.0 |
  (17 below, 18 above range)

fold_sequential (n=160, range 2028.7-4529.1 ns)
   2028.7 |
   2153.7 |
   2278.8 |########################################
   2403.8 |
   2528.8 |
   2653.8 |
   2778.9 |
   2903.9 |
   3028.9 |
   3153.9 |
   3278.9 |
   3404.0 |
   3529.0 |
   3654.0 |
   3779.0 |
   3904.0 |
   4029.1 |
   4154.1 |
   4279.1 |
   4404.1 |
  (20 below, 4 above range)

```

## Diagnostics

- **fold_paired**: CV=36.2% (high variance, measurements may be unstable)
- **fold_quad**: CV=52.9% (high variance, measurements may be unstable)
- **fold_sequential**: CV=100.8% (high variance, measurements may be unstable)
