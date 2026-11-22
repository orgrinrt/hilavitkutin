# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Highlights

Baseline for all deltas below: **wide_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide_blob dominates: 90% faster than the next best (wide_decomposed)

wide_blob (17.18 us) leads wide_decomposed (32.58 us) by 90%, a clear separation rather than a photo finish. CV 0.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (wide_blob)

The baseline wide_blob is the fastest (17.18 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (wide_blob) is the fastest** at 17176.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.90x (fastest 17176.2 ns, slowest 32580.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Intra-resource member gather at M=64: contiguous blob vs scattered columns |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide_blob | 17242ns | 17227ns | 17201ns | 17235ns | 17304ns | base |
| wide_decomposed | 32636ns | 32626ns | 32576ns | 32627ns | 32723ns | +89.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 17189ns | 17150ns | 17250ns | base | 0.015 |
| wide_decomposed | 32590ns | 32530ns | 32678ns | +89.60% | 0.008 |

## Performance model

- Peak throughput: **0.015 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 0.015 | 99.8% |
| wide_decomposed | 0.008 | 52.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 17242ns | 17242ns | base |
| wide_decomposed | 32636ns | 32636ns | +89.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 17176ns | base | --- | [17171, 17186] | --- | --- | --- | --- |
| wide_decomposed | 32581ns | +15401.4ns (+89.7%) | [+15384, +15412]ns | [32570, 32590] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide_blob | wide_decomposed |
|---|---|---|---|
| 1 | 1 | 17173ns | +89.8% |
| 1 | 2 | 17171ns | +89.8% |
| 1 | 3 | 17172ns | +89.8% |
| 1 | 4 | 17176ns | +89.6% |
| 1 | 5 | 17171ns | +89.8% |
| 2 | 1 | 17195ns | +89.4% |
| 2 | 2 | 17175ns | +89.7% |
| 2 | 3 | 17182ns | +89.5% |
| 2 | 4 | 17180ns | +89.5% |
| 2 | 5 | 17176ns | +89.8% |
| 3 | 1 | 17173ns | +89.7% |
| 3 | 2 | 17207ns | +89.3% |
| 3 | 3 | 17183ns | +89.7% |
| 3 | 4 | 17169ns | +89.8% |
| 3 | 5 | 17179ns | +89.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | -0.088 | ok |
| wide_decomposed | 0.130 | ok |

**Consistency summary:**

- **wide_decomposed**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 134.5ns | 17189.3ns | 0.8% |  |
| wide_decomposed | 335.8ns | 32590.2ns | 1.0% |  |

## Distribution (algo ns)

```
wide_blob (n=120, range 17149.8-17250.1 ns)
  17149.8 |#########################
  17154.8 |########
  17159.8 |####################
  17164.9 |########################################
  17169.9 |############################
  17174.9 |###############################
  17179.9 |##
  17184.9 |#####
  17189.9 |##############
  17194.9 |########
  17200.0 |##
  17205.0 |########
  17210.0 |######################
  17215.0 |########
  17220.0 |#################
  17225.0 |#####
  17230.0 |###########
  17235.0 |##
  17240.1 |########
  17245.1 |#####
  (15 below, 7 above range)

wide_decomposed (n=120, range 32529.9-32677.8 ns)
  32529.9 |##################
  32537.3 |################################
  32544.7 |#######
  32552.1 |#####################
  32559.5 |########################################
  32566.9 |################################
  32574.3 |#############################
  32581.7 |################################
  32589.1 |####################################
  32596.5 |#############################
  32603.9 |#######
  32611.3 |##########
  32618.7 |##############
  32626.1 |##############
  32633.5 |###
  32640.9 |##################
  32648.3 |#######
  32655.7 |##############
  32663.1 |#######
  32670.4 |
  (11 below, 5 above range)

```
