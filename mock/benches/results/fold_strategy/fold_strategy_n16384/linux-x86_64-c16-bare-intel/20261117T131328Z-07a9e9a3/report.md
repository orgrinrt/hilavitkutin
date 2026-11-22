# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 98% faster than the next best (fold_paired)

fold_quad (580 ns) leads fold_paired (1.15 us) by 98%, a clear separation rather than a photo finish. CV 68.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 49% (significant)

fold_quad is -568 ns (49%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.9x slower than the field

fold_sequential (2.29 us) is 3.9x the fastest (580 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### fold_quad is fastest but the noisiest (CV 68.8%)

fold_quad wins on median (580 ns) yet has the highest variance (CV 68.8%), while fold_sequential is the steadiest (CV 14.6%, 2.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3.9x the fastest

Fastest fold_quad (580 ns) to slowest fold_sequential (2.29 us): 3.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fold_quad is inconsistent: worst-20% is 2.9x its best-20%

fold_quad's best 20% of batches run at 475 ns but its worst 20% at 1.38 us (2.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fold_quad** at 580.2 ns median (-49.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.94x (fastest 580.2 ns, slowest 2287.3 ns)

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
| fold_paired | 3111ns | 3033ns | 2486ns | 2982ns | 4123ns | base |
| fold_quad | 3022ns | 2465ns | 2017ns | 2394ns | 5912ns | -2.86% |
| fold_sequential | 3941ns | 4172ns | 3414ns | 3966ns | 4391ns | +26.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 1174ns | 941ns | 1545ns | base | 13.952 |
| fold_quad | 710ns | 475ns | 1384ns | -39.56% | 23.086 |
| fold_sequential | 2157ns | 1871ns | 2400ns | +83.70% | 7.595 |

## Performance model

- Peak throughput: **34.518 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 14.266 | 41.3% |
| fold_quad | 28.239 | 81.8% |
| fold_sequential | 7.163 | 20.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 3111ns | 3111ns | base |
| fold_quad | 3022ns | 3022ns | -2.86% |
| fold_sequential | 3941ns | 3941ns | +26.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 1148ns | base | --- | [1148, 1148] | --- | --- | --- | --- |
| fold_quad | 580ns | -568.3ns (-49.5%) | [-568, -568]ns | [580, 580] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 2287ns | +1138.7ns (+99.1%) | [+945, +1139]ns | [2287, 2287] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 1149ns | -49.5% | +99.2% |
| 1 | 2 | 940ns | -49.5% | +143.4% |
| 1 | 3 | 1149ns | -49.5% | +62.9% |
| 1 | 4 | 1148ns | -49.5% | +63.0% |
| 1 | 5 | 1118ns | -57.6% | +104.5% |
| 1 | 6 | 1148ns | -49.5% | +63.0% |
| 1 | 7 | 940ns | -38.3% | +143.4% |
| 1 | 8 | 1149ns | -58.7% | +99.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.447 | moderate- |
| fold_quad | -0.475 | moderate- |
| fold_sequential | -0.141 | ok |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 24.4ns | 1174.3ns | 2.1% |  |
| fold_quad | 29.1ns | 709.7ns | 4.1% |  |
| fold_sequential | 22.6ns | 2157.2ns | 1.0% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 940.5-1545.3 ns)
    940.5 |##
    970.8 |#
   1001.0 |
   1031.2 |
   1061.5 |
   1091.7 |##
   1121.9 |########################################
   1152.2 |####
   1182.4 |
   1212.7 |
   1242.9 |
   1273.1 |
   1303.4 |
   1333.6 |
   1363.8 |
   1394.1 |
   1424.3 |
   1454.6 |
   1484.8 |
   1515.0 |
  (29 below, 9 above range)

fold_quad (n=160, range 474.7-1384.0 ns)
    474.7 |#################
    520.1 |
    565.6 |########################################
    611.1 |
    656.5 |
    702.0 |
    747.5 |
    792.9 |
    838.4 |
    883.9 |
    929.3 |
    974.8 |
   1020.3 |
   1065.7 |
   1111.2 |
   1156.7 |
   1202.1 |
   1247.6 |
   1293.1 |
   1338.6 |
  (11 below, 20 above range)

fold_sequential (n=160, range 1871.4-2399.6 ns)
   1871.4 |#######################
   1897.9 |
   1924.3 |
   1950.7 |
   1977.1 |
   2003.5 |
   2029.9 |
   2056.3 |
   2082.7 |
   2109.1 |
   2135.5 |
   2162.0 |
   2188.4 |
   2214.8 |
   2241.2 |
   2267.6 |########################################
   2294.0 |##############
   2320.4 |
   2346.8 |
   2373.2 |
  (16 below, 1 above range)

```

## Diagnostics

- **fold_paired**: CV=30.1% (high variance, measurements may be unstable)
- **fold_quad**: CV=56.2% (high variance, measurements may be unstable)
