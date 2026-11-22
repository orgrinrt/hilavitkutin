# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 77% faster than the next best (fold_paired)

fold_quad (176 ns) leads fold_paired (311 ns) by 77%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 43% (significant)

fold_quad is -135 ns (43%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.4x slower than the field

fold_sequential (589 ns) is 3.4x the fastest (176 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.4x the fastest

Fastest fold_quad (176 ns) to slowest fold_sequential (589 ns): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: fold_quad** at 175.8 ns median (-43.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.35x (fastest 175.8 ns, slowest 589.2 ns)

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
| fold_paired | 2407ns | 2393ns | 2386ns | 2398ns | 2453ns | base |
| fold_quad | 2272ns | 2256ns | 2246ns | 2266ns | 2317ns | -5.58% |
| fold_sequential | 2698ns | 2688ns | 2682ns | 2689ns | 2741ns | +12.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 312ns | 307ns | 322ns | base | 13.115 |
| fold_quad | 174ns | 161ns | 183ns | -44.25% | 23.526 |
| fold_sequential | 592ns | 585ns | 607ns | +89.55% | 6.919 |

## Performance model

- Peak throughput: **25.490 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 13.179 | 51.7% |
| fold_quad | 23.299 | 91.4% |
| fold_sequential | 6.952 | 27.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2407ns | 2407ns | base |
| fold_quad | 2272ns | 2272ns | -5.58% |
| fold_sequential | 2698ns | 2698ns | +12.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 311ns | base | --- | [310, 312] | --- | --- | --- | --- |
| fold_quad | 176ns | -135.0ns (-43.4%) | [-136, -134]ns | [175, 177] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 589ns | +278.7ns (+89.7%) | [+278, +279]ns | [589, 590] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 312ns | -44.0% | +88.6% |
| 1 | 2 | 311ns | -43.3% | +89.2% |
| 1 | 3 | 311ns | -50.2% | +89.9% |
| 1 | 4 | 311ns | -43.4% | +89.1% |
| 1 | 5 | 311ns | -43.6% | +89.8% |
| 1 | 6 | 312ns | -43.8% | +88.5% |
| 1 | 7 | 310ns | -42.3% | +89.8% |
| 1 | 8 | 311ns | -43.0% | +89.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.114 | ok |
| fold_quad | -0.144 | ok |
| fold_sequential | -0.429 | moderate- |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 2.6ns | 312.3ns | 0.8% |  |
| fold_quad | 2.5ns | 174.1ns | 1.4% |  |
| fold_sequential | 2.8ns | 592.0ns | 0.5% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 306.7-321.5 ns)
    306.7 |#####
    307.5 |#####################################
    308.2 |################################
    308.9 |################################
    309.7 |########################################
    310.4 |####################
    311.2 |##############################
    311.9 |################################
    312.6 |###########################
    313.4 |##
    314.1 |###############
    314.9 |############
    315.6 |#######
    316.4 |#######
    317.1 |##
    317.8 |##########
    318.6 |##########
    319.3 |#######
    320.1 |#####
    320.8 |#####
  (16 below, 7 above range)

fold_quad (n=160, range 160.7-182.6 ns)
    160.7 |##
    161.8 |
    162.9 |
    164.0 |
    165.1 |
    166.2 |
    167.3 |##
    168.3 |####
    169.4 |########
    170.5 |####
    171.6 |#########################
    172.7 |###########################
    173.8 |#################################
    174.9 |#################################
    176.0 |##########
    177.1 |########################################
    178.2 |########################################
    179.3 |##########
    180.4 |##############
    181.5 |##############
  (19 below, 12 above range)

fold_sequential (n=160, range 585.4-606.6 ns)
    585.4 |##################
    586.4 |####################
    587.5 |#################################
    588.5 |########################################
    589.6 |###############
    590.7 |###############
    591.7 |##########
    592.8 |#########
    593.8 |#####
    594.9 |###
    596.0 |###
    597.0 |##
    598.1 |#
    599.1 |
    600.2 |#
    601.3 |
    602.3 |###
    603.4 |##
    604.4 |#
    605.5 |
  (10 below, 4 above range)

```
