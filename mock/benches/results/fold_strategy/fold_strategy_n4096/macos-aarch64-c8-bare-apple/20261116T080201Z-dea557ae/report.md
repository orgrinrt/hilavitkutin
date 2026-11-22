# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 76% faster than the next best (fold_paired)

fold_quad (180 ns) leads fold_paired (317 ns) by 76%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 43% (significant)

fold_quad is -137 ns (43%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.3x slower than the field

fold_sequential (600 ns) is 3.3x the fastest (180 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.3x the fastest

Fastest fold_quad (180 ns) to slowest fold_sequential (600 ns): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: fold_quad** at 180.4 ns median (-43.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.32x (fastest 180.4 ns, slowest 599.6 ns)

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
| fold_paired | 2433ns | 2426ns | 2418ns | 2427ns | 2465ns | base |
| fold_quad | 2285ns | 2281ns | 2275ns | 2282ns | 2303ns | -6.09% |
| fold_sequential | 2733ns | 2725ns | 2718ns | 2726ns | 2770ns | +12.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 317ns | 313ns | 322ns | base | 12.919 |
| fold_quad | 181ns | 176ns | 188ns | -42.94% | 22.641 |
| fold_sequential | 602ns | 597ns | 613ns | +89.78% | 6.808 |

## Performance model

- Peak throughput: **23.314 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 12.917 | 55.4% |
| fold_quad | 22.705 | 97.4% |
| fold_sequential | 6.831 | 29.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2433ns | 2433ns | base |
| fold_quad | 2285ns | 2285ns | -6.09% |
| fold_sequential | 2733ns | 2733ns | +12.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 317ns | base | --- | [317, 318] | --- | --- | --- | --- |
| fold_quad | 180ns | -136.7ns (-43.1%) | [-137, -136]ns | [180, 181] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 600ns | +282.5ns (+89.1%) | [+282, +283]ns | [599, 600] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 317ns | -43.0% | +89.2% |
| 1 | 2 | 318ns | -43.2% | +88.9% |
| 1 | 3 | 317ns | -43.7% | +88.9% |
| 1 | 4 | 316ns | -43.0% | +89.7% |
| 1 | 5 | 318ns | -43.1% | +88.7% |
| 1 | 6 | 318ns | -42.9% | +88.7% |
| 1 | 7 | 316ns | -43.6% | +89.6% |
| 1 | 8 | 318ns | -42.9% | +88.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.410 | moderate- |
| fold_quad | -0.368 | moderate- |
| fold_sequential | 0.050 | ok |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 2.8ns | 317.0ns | 0.9% |  |
| fold_quad | 2.7ns | 180.9ns | 1.5% |  |
| fold_sequential | 2.8ns | 601.7ns | 0.5% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 312.6-321.6 ns)
    312.6 |###############
    313.0 |######
    313.5 |###############
    313.9 |#####################
    314.4 |########################
    314.8 |###############
    315.3 |##################
    315.7 |#####################
    316.2 |###############
    316.6 |###########################
    317.1 |########################################
    317.5 |##############################
    318.0 |###########################
    318.4 |##################
    318.9 |#################################
    319.3 |###############
    319.8 |###############
    320.2 |######
    320.7 |###
    321.1 |#####################
  (17 below, 15 above range)

fold_quad (n=160, range 175.7-187.7 ns)
    175.7 |######################
    176.3 |########
    176.9 |########
    177.5 |#############
    178.1 |######################
    178.7 |############################
    179.3 |#################
    179.9 |########################################
    180.5 |#############
    181.1 |##########################
    181.7 |####
    182.3 |########################################
    182.9 |#############
    183.5 |########
    184.1 |###########
    184.7 |########
    185.3 |#############
    185.9 |##
    186.5 |##
    187.1 |####
  (15 below, 5 above range)

fold_sequential (n=160, range 596.7-612.6 ns)
    596.7 |################################
    597.5 |###########################
    598.3 |########################################
    599.0 |########################################
    599.8 |#############################
    600.6 |##################
    601.4 |###########################
    602.2 |##################
    603.0 |##############
    603.8 |###
    604.6 |#####
    605.4 |#####
    606.2 |#
    607.0 |
    607.8 |
    608.6 |
    609.4 |
    610.2 |
    611.0 |###
    611.8 |
  (10 below, 3 above range)

```
