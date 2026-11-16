# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 77% faster than the next best (fold_paired)

fold_quad (176 ns) leads fold_paired (310 ns) by 77%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 43% (significant)

fold_quad is -134 ns (43%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.4x slower than the field

fold_sequential (589 ns) is 3.4x the fastest (176 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.4x the fastest

Fastest fold_quad (176 ns) to slowest fold_sequential (589 ns): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: fold_quad** at 175.6 ns median (-43.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.36x (fastest 175.6 ns, slowest 589.2 ns)

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
| fold_paired | 2396ns | 2390ns | 2385ns | 2391ns | 2424ns | base |
| fold_quad | 2254ns | 2250ns | 2245ns | 2250ns | 2276ns | -5.94% |
| fold_sequential | 2690ns | 2685ns | 2680ns | 2685ns | 2715ns | +12.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 310ns | 306ns | 315ns | base | 13.194 |
| fold_quad | 176ns | 172ns | 180ns | -43.34% | 23.289 |
| fold_sequential | 590ns | 586ns | 598ns | +90.12% | 6.940 |

## Performance model

- Peak throughput: **23.848 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 13.213 | 55.4% |
| fold_quad | 23.326 | 97.8% |
| fold_sequential | 6.952 | 29.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2396ns | 2396ns | base |
| fold_quad | 2254ns | 2254ns | -5.94% |
| fold_sequential | 2690ns | 2690ns | +12.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 310ns | base | --- | [310, 311] | --- | --- | --- | --- |
| fold_quad | 176ns | -134.2ns (-43.3%) | [-135, -134]ns | [175, 176] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 589ns | +279.2ns (+90.1%) | [+278, +280]ns | [589, 590] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 310ns | -43.3% | +90.2% |
| 1 | 2 | 310ns | -43.6% | +89.8% |
| 1 | 3 | 311ns | -43.4% | +89.6% |
| 1 | 4 | 310ns | -43.4% | +90.2% |
| 1 | 5 | 311ns | -43.3% | +89.5% |
| 1 | 6 | 310ns | -43.5% | +89.8% |
| 1 | 7 | 310ns | -43.2% | +90.1% |
| 1 | 8 | 310ns | -43.0% | +90.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.025 | ok |
| fold_quad | 0.032 | ok |
| fold_sequential | -0.245 | moderate- |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 2.6ns | 310.4ns | 0.8% |  |
| fold_quad | 2.5ns | 175.9ns | 1.4% |  |
| fold_sequential | 2.6ns | 590.2ns | 0.4% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 306.4-315.1 ns)
    306.4 |###########
    306.8 |#######################
    307.3 |#####################
    307.7 |##################
    308.1 |#########
    308.6 |##############
    309.0 |##############
    309.5 |##################
    309.9 |##############################
    310.3 |#####################
    310.8 |################
    311.2 |################
    311.6 |###########
    312.1 |########################################
    312.5 |####
    312.9 |###########
    313.4 |###########
    313.8 |##################
    314.3 |#########
    314.7 |#######
  (12 below, 7 above range)

fold_quad (n=160, range 171.8-180.3 ns)
    171.8 |#####################
    172.2 |#########################
    172.6 |##################
    173.0 |################################
    173.5 |##############
    173.9 |#############################
    174.3 |#############################
    174.7 |#############################
    175.2 |########################################
    175.6 |#########################
    176.0 |#############################
    176.4 |########################################
    176.9 |#######
    177.3 |##############
    177.7 |##############
    178.2 |################################
    178.6 |##################
    179.0 |#########################
    179.4 |#####################
    179.9 |##############
  (14 below, 13 above range)

fold_sequential (n=160, range 585.6-597.8 ns)
    585.6 |######################
    586.3 |############
    586.9 |##############
    587.5 |####################################
    588.1 |####################
    588.7 |########################################
    589.3 |################################
    589.9 |########################
    590.5 |############
    591.1 |################
    591.7 |##############
    592.3 |##########
    592.9 |####
    593.5 |########
    594.1 |##########
    594.7 |######
    595.3 |
    595.9 |
    596.5 |##
    597.1 |##
  (12 below, 6 above range)

```
