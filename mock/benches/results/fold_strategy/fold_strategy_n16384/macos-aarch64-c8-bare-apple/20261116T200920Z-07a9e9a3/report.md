# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 93% faster than the next best (fold_paired)

fold_quad (659 ns) leads fold_paired (1.27 us) by 93%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 48% (significant)

fold_quad is -611 ns (48%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.8x slower than the field

fold_sequential (2.51 us) is 3.8x the fastest (659 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.8x the fastest

Fastest fold_quad (659 ns) to slowest fold_sequential (2.51 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: fold_quad** at 658.5 ns median (-48.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.81x (fastest 658.5 ns, slowest 2508.3 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fold_paired | 3369ns | 3357ns | 3350ns | 3357ns | 3422ns | base |
| fold_quad | 2760ns | 2735ns | 2727ns | 2741ns | 2847ns | -18.09% |
| fold_sequential | 4664ns | 4610ns | 4603ns | 4619ns | 4861ns | +38.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 1275ns | 1265ns | 1302ns | base | 12.848 |
| fold_quad | 663ns | 653ns | 682ns | -48.04% | 24.726 |
| fold_sequential | 2536ns | 2504ns | 2643ns | +98.86% | 6.461 |

## Performance model

- Peak throughput: **25.084 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 12.905 | 51.4% |
| fold_quad | 24.879 | 99.2% |
| fold_sequential | 6.532 | 26.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 3369ns | 3369ns | base |
| fold_quad | 2760ns | 2760ns | -18.09% |
| fold_sequential | 4664ns | 4664ns | +38.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 1270ns | base | --- | [1269, 1270] | --- | --- | --- | --- |
| fold_quad | 659ns | -611.2ns (-48.1%) | [-612, -610]ns | [658, 660] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 2508ns | +1238.8ns (+97.6%) | [+1238, +1240]ns | [2507, 2510] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 1268ns | -48.2% | +112.3% |
| 1 | 2 | 1272ns | -48.1% | +97.1% |
| 1 | 3 | 1269ns | -48.2% | +97.6% |
| 1 | 4 | 1269ns | -47.5% | +97.5% |
| 1 | 5 | 1269ns | -48.2% | +97.5% |
| 1 | 6 | 1271ns | -48.2% | +97.3% |
| 1 | 7 | 1270ns | -46.4% | +102.0% |
| 1 | 8 | 1270ns | -48.2% | +97.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.444 | moderate- |
| fold_quad | -0.327 | moderate- |
| fold_sequential | -0.085 | ok |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 3.1ns | 1275.3ns | 0.2% |  |
| fold_quad | 3.0ns | 662.6ns | 0.5% |  |
| fold_sequential | 3.1ns | 2536.0ns | 0.1% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 1265.4-1302.1 ns)
   1265.4 |#########################
   1267.3 |#################################
   1269.1 |########################################
   1270.9 |#################
   1272.8 |###############
   1274.6 |
   1276.4 |
   1278.3 |##
   1280.1 |#
   1281.9 |#
   1283.8 |#
   1285.6 |#
   1287.4 |#
   1289.3 |##
   1291.1 |
   1292.9 |##
   1294.8 |
   1296.6 |##
   1298.4 |#
   1300.3 |##
  (15 below, 7 above range)

fold_quad (n=160, range 653.2-682.1 ns)
    653.2 |################
    654.6 |########################################
    656.0 |###########################
    657.5 |########################################
    658.9 |################
    660.4 |#######################
    661.8 |################
    663.3 |##############
    664.7 |#######
    666.2 |
    667.6 |
    669.1 |###
    670.5 |###
    672.0 |
    673.4 |
    674.8 |#####
    676.3 |#########
    677.7 |###
    679.2 |#####
    680.6 |#####
  (16 below, 13 above range)

fold_sequential (n=160, range 2504.0-2642.5 ns)
   2504.0 |########################################
   2510.9 |####
   2517.9 |##
   2524.8 |
   2531.7 |
   2538.6 |#
   2545.6 |
   2552.5 |#
   2559.4 |
   2566.3 |
   2573.3 |
   2580.2 |
   2587.1 |
   2594.1 |##
   2601.0 |
   2607.9 |
   2614.8 |
   2621.8 |
   2628.7 |
   2635.6 |
  (14 below, 15 above range)

```
