# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 93% faster than the next best (fold_paired)

fold_quad (658 ns) leads fold_paired (1.27 us) by 93%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 48% (significant)

fold_quad is -612 ns (48%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.8x slower than the field

fold_sequential (2.51 us) is 3.8x the fastest (658 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.8x the fastest

Fastest fold_quad (658 ns) to slowest fold_sequential (2.51 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: fold_quad** at 657.5 ns median (-48.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.81x (fastest 657.5 ns, slowest 2507.1 ns)

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
| fold_paired | 3358ns | 3353ns | 3347ns | 3353ns | 3385ns | base |
| fold_quad | 2750ns | 2734ns | 2729ns | 2735ns | 2819ns | -18.11% |
| fold_sequential | 4642ns | 4609ns | 4602ns | 4610ns | 4777ns | +38.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 1272ns | 1266ns | 1284ns | base | 12.884 |
| fold_quad | 661ns | 652ns | 679ns | -48.05% | 24.798 |
| fold_sequential | 2529ns | 2503ns | 2621ns | +98.89% | 6.478 |

## Performance model

- Peak throughput: **25.141 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 12.905 | 51.3% |
| fold_quad | 24.919 | 99.1% |
| fold_sequential | 6.535 | 26.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 3358ns | 3358ns | base |
| fold_quad | 2750ns | 2750ns | -18.11% |
| fold_sequential | 4642ns | 4642ns | +38.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 1270ns | base | --- | [1269, 1270] | --- | --- | --- | --- |
| fold_quad | 658ns | -612.5ns (-48.2%) | [-613, -612]ns | [656, 658] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 2507ns | +1237.2ns (+97.5%) | [+1237, +1238]ns | [2506, 2508] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 1269ns | -48.2% | +97.5% |
| 1 | 2 | 1270ns | -48.2% | +97.4% |
| 1 | 3 | 1270ns | -46.1% | +97.4% |
| 1 | 4 | 1270ns | -48.3% | +97.3% |
| 1 | 5 | 1269ns | -48.2% | +97.5% |
| 1 | 6 | 1269ns | -48.3% | +97.6% |
| 1 | 7 | 1269ns | -48.3% | +97.6% |
| 1 | 8 | 1269ns | -48.3% | +97.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | 0.493 | moderate+ |
| fold_quad | -0.108 | ok |
| fold_sequential | -0.219 | moderate- |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 3.0ns | 1271.7ns | 0.2% |  |
| fold_quad | 3.2ns | 660.7ns | 0.5% |  |
| fold_sequential | 3.0ns | 2529.2ns | 0.1% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 1265.6-1283.9 ns)
   1265.6 |##########
   1266.5 |##################
   1267.5 |########################################
   1268.4 |#####################
   1269.3 |######################
   1270.2 |##############
   1271.1 |##############
   1272.0 |#########################
   1272.9 |#####
   1273.9 |#######
   1274.8 |####
   1275.7 |####
   1276.6 |#
   1277.5 |##
   1278.4 |
   1279.3 |##
   1280.3 |#
   1281.2 |
   1282.1 |#
   1283.0 |
  (16 below, 5 above range)

fold_quad (n=160, range 651.7-679.0 ns)
    651.7 |#################################
    653.1 |###############################
    654.4 |########################################
    655.8 |#################################
    657.2 |#################################
    658.5 |#######################
    659.9 |#################################
    661.3 |##########
    662.6 |##############
    664.0 |######
    665.4 |######
    666.7 |##
    668.1 |##
    669.5 |
    670.8 |
    672.2 |##
    673.5 |##
    674.9 |
    676.3 |##
    677.6 |####
  (11 below, 15 above range)

fold_sequential (n=160, range 2503.2-2621.5 ns)
   2503.2 |########################################
   2509.1 |##########
   2515.0 |
   2520.9 |#
   2526.8 |
   2532.7 |
   2538.7 |
   2544.6 |
   2550.5 |
   2556.4 |
   2562.3 |
   2568.2 |
   2574.1 |
   2580.1 |
   2586.0 |
   2591.9 |
   2597.8 |
   2603.7 |
   2609.6 |
   2615.5 |
  (11 below, 10 above range)

```
