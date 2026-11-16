# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (bc_index, bc_iter) are a dead heat (<1%)

bc_index (358 ns) and bc_iter (359 ns) differ by 0.53%, inside the noise, even though the wider field spreads 77.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (bc_index)

The baseline bc_index is the fastest (358 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### bc_iter's edge over baseline is significant but tiny (1 ns, 0.36%)

bc_iter differs from baseline bc_index by 1 ns (0.36%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (bc_index) is the fastest** at 357.5 ns median
- 2 variants significantly slower than baseline
- Spread: 1.77x (fastest 357.5 ns, slowest 633.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Bounds-check elision: iter() vs explicit index vs get_unchecked |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5a3ec07491bf2d68 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 2458ns | 2459ns | 2419ns | 2459ns | 2494ns | base |
| bc_iter | 2467ns | 2457ns | 2430ns | 2458ns | 2533ns | +0.38% |
| bc_unchecked | 2747ns | 2742ns | 2692ns | 2742ns | 2817ns | +11.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 358ns | 350ns | 370ns | base | 45.716 |
| bc_iter | 361ns | 350ns | 378ns | +0.79% | 45.356 |
| bc_unchecked | 635ns | 621ns | 655ns | +77.19% | 25.800 |

## Performance model

- Peak throughput: **46.811 Gops/s** (bc_index; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 45.829 | 97.9% |
| bc_iter | 45.587 | 97.4% |
| bc_unchecked | 25.879 | 55.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2458ns | 2458ns | base |
| bc_iter | 2467ns | 2467ns | +0.38% |
| bc_unchecked | 2747ns | 2747ns | +11.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 358ns | base | --- | [356, 359] | --- | --- | --- | --- |
| bc_iter | 359ns | +1.3ns (+0.4%) | [+0, +2]ns | [358, 360] | YES | 0.0049 | 0.0049 | 4 |
| bc_unchecked | 633ns | +275.4ns (+77.0%) | [+274, +276]ns | [632, 635] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 358ns | +3.0% | +77.1% |
| 1 | 2 | 358ns | +0.2% | +76.4% |
| 1 | 3 | 359ns | +0.1% | +76.2% |
| 1 | 4 | 358ns | +0.3% | +77.3% |
| 1 | 5 | 355ns | -0.7% | +76.6% |
| 1 | 6 | 354ns | +1.3% | +82.6% |
| 1 | 7 | 359ns | -0.4% | +75.0% |
| 1 | 8 | 359ns | -0.3% | +74.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | 0.226 | moderate+ |
| bc_iter | -0.025 | ok |
| bc_unchecked | -0.467 | moderate- |

**Consistency summary:**

- **bc_iter**: won 3/8, lost 5/8
- **bc_unchecked**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 2.6ns | 358.4ns | 0.7% |  |
| bc_iter | 2.8ns | 361.2ns | 0.8% |  |
| bc_unchecked | 2.7ns | 635.0ns | 0.4% |  |

## Distribution (algo ns)

```
bc_index (n=160, range 350.0-370.0 ns)
    350.0 |#######################
    351.0 |#######
    352.0 |###################################
    353.0 |##################
    354.0 |##################
    355.0 |##############
    356.0 |##############################
    357.0 |################################
    358.0 |################
    359.0 |########################################
    360.0 |#####################
    361.0 |################
    362.0 |##################
    363.0 |##################
    364.0 |
    365.0 |################
    366.0 |
    367.0 |####
    368.0 |
    369.0 |##
  (12 below, 5 above range)

bc_iter (n=160, range 350.4-378.1 ns)
    350.4 |##############
    351.8 |################
    353.2 |#######################
    354.5 |#########################
    355.9 |#########################
    357.3 |########################################
    358.7 |###################################
    360.1 |#####################
    361.5 |###################################
    362.8 |#########################
    364.2 |###########
    365.6 |####
    367.0 |#######################
    368.4 |#########
    369.8 |####
    371.2 |####
    372.5 |
    373.9 |
    375.3 |##
    376.7 |##
  (14 below, 7 above range)

bc_unchecked (n=160, range 621.5-654.6 ns)
    621.5 |############
    623.1 |##############
    624.8 |########
    626.4 |##################
    628.1 |#####################
    629.7 |###############################
    631.4 |###############################
    633.1 |#############################
    634.7 |########################################
    636.4 |#####################
    638.0 |##############
    639.7 |############
    641.3 |########
    643.0 |########
    644.6 |########
    646.3 |######
    647.9 |
    649.6 |######
    651.3 |
    652.9 |##
  (14 below, 5 above range)

```
