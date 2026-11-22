# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 93% faster than the next best (fold_paired)

fold_quad (671 ns) leads fold_paired (1.29 us) by 93%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 48% (significant)

fold_quad is -623 ns (48%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.8x slower than the field

fold_sequential (2.55 us) is 3.8x the fastest (671 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.8x the fastest

Fastest fold_quad (671 ns) to slowest fold_sequential (2.55 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: fold_quad** at 670.8 ns median (-48.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.81x (fastest 670.8 ns, slowest 2554.6 ns)

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
| fold_paired | 3414ns | 3408ns | 3399ns | 3408ns | 3444ns | base |
| fold_quad | 2782ns | 2776ns | 2736ns | 2777ns | 2842ns | -18.50% |
| fold_sequential | 4675ns | 4682ns | 4599ns | 4682ns | 4729ns | +36.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 1296ns | 1290ns | 1306ns | base | 12.643 |
| fold_quad | 673ns | 662ns | 689ns | -48.05% | 24.337 |
| fold_sequential | 2551ns | 2510ns | 2582ns | +96.84% | 6.423 |

## Performance model

- Peak throughput: **24.759 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 12.656 | 51.1% |
| fold_quad | 24.425 | 98.6% |
| fold_sequential | 6.414 | 25.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 3414ns | 3414ns | base |
| fold_quad | 2782ns | 2782ns | -18.50% |
| fold_sequential | 4675ns | 4675ns | +36.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 1295ns | base | --- | [1294, 1295] | --- | --- | --- | --- |
| fold_quad | 671ns | -623.3ns (-48.2%) | [-624, -622]ns | [670, 672] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 2555ns | +1260.0ns (+97.3%) | [+1259, +1260]ns | [2554, 2555] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 1295ns | -46.8% | +97.3% |
| 1 | 2 | 1293ns | -48.1% | +97.6% |
| 1 | 3 | 1295ns | -47.9% | +97.2% |
| 1 | 4 | 1296ns | -48.3% | +96.8% |
| 1 | 5 | 1295ns | -48.3% | +93.8% |
| 1 | 6 | 1293ns | -49.1% | +97.7% |
| 1 | 7 | 1295ns | -48.1% | +97.3% |
| 1 | 8 | 1295ns | -48.3% | +97.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.199 | ok |
| fold_quad | 0.026 | ok |
| fold_sequential | -0.124 | ok |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 2.8ns | 1295.9ns | 0.2% |  |
| fold_quad | 2.5ns | 673.2ns | 0.4% |  |
| fold_sequential | 2.6ns | 2550.8ns | 0.1% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 1289.8-1305.6 ns)
   1289.8 |############
   1290.6 |##############
   1291.4 |#####################
   1292.1 |###################################
   1292.9 |##############
   1293.7 |#########################
   1294.5 |########################################
   1295.3 |###############################
   1296.1 |#########################
   1296.9 |##############
   1297.7 |########
   1298.5 |##########
   1299.3 |########
   1300.0 |######
   1300.8 |##
   1301.6 |####
   1302.4 |##
   1303.2 |##
   1304.0 |
   1304.8 |
  (14 below, 13 above range)

fold_quad (n=160, range 661.7-689.5 ns)
    661.7 |#
    663.1 |##
    664.5 |#############
    665.9 |##########
    667.3 |###################
    668.7 |########################################
    670.1 |##############
    671.4 |###################
    672.8 |###########
    674.2 |#################
    675.6 |#######
    677.0 |########
    678.4 |####
    679.8 |#####
    681.2 |####
    682.6 |##
    683.9 |####
    685.3 |#####
    686.7 |
    688.1 |####
  (15 below, 10 above range)

fold_sequential (n=160, range 2509.7-2582.1 ns)
   2509.7 |#
   2513.3 |
   2516.9 |##
   2520.6 |
   2524.2 |
   2527.8 |
   2531.4 |
   2535.0 |
   2538.7 |#
   2542.3 |#
   2545.9 |
   2549.5 |###############
   2553.1 |########################################
   2556.7 |###############
   2560.4 |#####
   2564.0 |#
   2567.6 |
   2571.2 |##
   2574.8 |
   2578.5 |
  (25 below, 11 above range)

```
