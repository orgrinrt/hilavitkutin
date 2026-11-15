# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 68% faster than the next best (fold_paired)

fold_quad (197 ns) leads fold_paired (331 ns) by 68%, a clear separation rather than a photo finish. CV 5.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 42% (significant)

fold_quad is -141 ns (42%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.1x slower than the field

fold_sequential (617 ns) is 3.1x the fastest (197 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.1x the fastest

Fastest fold_quad (197 ns) to slowest fold_sequential (617 ns): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: fold_quad** at 196.7 ns median (-40.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.14x (fastest 196.7 ns, slowest 617.3 ns)

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
| fold_paired | 2570ns | 2555ns | 2382ns | 2565ns | 2771ns | base |
| fold_quad | 2487ns | 2578ns | 2323ns | 2504ns | 2600ns | -3.21% |
| fold_sequential | 2845ns | 2811ns | 2672ns | 2816ns | 3102ns | +10.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 334ns | 309ns | 364ns | base | 12.255 |
| fold_quad | 195ns | 181ns | 210ns | -41.52% | 20.958 |
| fold_sequential | 627ns | 588ns | 684ns | +87.49% | 6.536 |

## Performance model

- Peak throughput: **22.587 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 12.367 | 54.8% |
| fold_quad | 20.824 | 92.2% |
| fold_sequential | 6.635 | 29.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2570ns | 2570ns | base |
| fold_quad | 2487ns | 2487ns | -3.21% |
| fold_sequential | 2845ns | 2845ns | +10.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 331ns | base | --- | [329, 333] | --- | --- | --- | --- |
| fold_quad | 197ns | -140.7ns (-42.5%) | [-143, -130]ns | [192, 200] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 617ns | +281.6ns (+85.0%) | [+277, +295]ns | [612, 628] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 353ns | -43.2% | +72.3% |
| 1 | 2 | 357ns | -43.6% | +75.9% |
| 1 | 3 | 358ns | -43.5% | +64.8% |
| 1 | 4 | 358ns | -47.8% | +70.6% |
| 1 | 5 | 314ns | -40.7% | +115.8% |
| 1 | 6 | 311ns | -40.7% | +94.8% |
| 1 | 7 | 328ns | -38.6% | +106.7% |
| 1 | 8 | 310ns | -34.5% | +103.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | 0.496 | moderate+ |
| fold_quad | 0.357 | moderate+ |
| fold_sequential | -0.298 | moderate- |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 2.8ns | 334.2ns | 0.8% |  |
| fold_quad | 2.9ns | 195.4ns | 1.5% |  |
| fold_sequential | 2.8ns | 626.7ns | 0.4% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 308.8-364.0 ns)
    308.8 |########################################
    311.6 |###########################
    314.3 |########
    317.1 |####
    319.9 |##########
    322.6 |####
    325.4 |##########
    328.2 |#############################
    330.9 |###############################
    333.7 |################
    336.4 |##
    339.2 |######
    342.0 |
    344.7 |
    347.5 |
    350.2 |######
    353.0 |########
    355.8 |###################################
    358.5 |##################
    361.3 |#####################
  (15 below, 11 above range)

fold_quad (n=160, range 181.3-209.7 ns)
    181.3 |#############################
    182.8 |##############
    184.2 |#############################
    185.6 |################################
    187.0 |########################################
    188.4 |#####################
    189.8 |####################################
    191.3 |#####################
    192.7 |##################
    194.1 |###
    195.5 |#######
    196.9 |##############
    198.4 |#############################
    199.8 |########################################
    201.2 |########################################
    202.6 |########################################
    204.0 |#########################
    205.4 |#####################
    206.9 |#########################
    208.3 |################################
  (11 below, 5 above range)

fold_sequential (n=160, range 588.4-683.7 ns)
    588.4 |########################################
    593.2 |#####
    597.9 |#####
    602.7 |##############
    607.5 |################################
    612.2 |####################
    617.0 |#########
    621.8 |#########
    626.5 |################################
    631.3 |#####################
    636.1 |#######
    640.8 |#
    645.6 |#####
    650.4 |
    655.1 |
    659.9 |
    664.7 |#
    669.4 |
    674.2 |################
    679.0 |#######################
  (13 below, 11 above range)

```
