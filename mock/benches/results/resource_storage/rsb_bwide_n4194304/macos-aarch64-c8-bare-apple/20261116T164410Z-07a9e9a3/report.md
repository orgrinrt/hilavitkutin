# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Highlights

Baseline for all deltas below: **wide_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide_blob dominates: 198% faster than the next best (wide_decomposed)

wide_blob (46 ns) leads wide_decomposed (139 ns) by 198%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (wide_blob)

The baseline wide_blob is the fastest (46 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (wide_blob) is the fastest** at 46.5 ns median
- 1 variant significantly slower than baseline
- Spread: 2.98x (fastest 46.5 ns, slowest 138.7 ns)

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
| wide_blob | 95ns | 94ns | 93ns | 95ns | 97ns | base |
| wide_decomposed | 181ns | 181ns | 179ns | 181ns | 184ns | +90.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 47ns | 46ns | 47ns | base | 89927.724 |
| wide_decomposed | 139ns | 138ns | 140ns | +197.71% | 30206.359 |

## Performance model

- Peak throughput: **91246.642 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 90200.086 | 98.9% |
| wide_decomposed | 30240.115 | 33.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 95ns | 95ns | base |
| wide_decomposed | 181ns | 181ns | +90.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 46ns | base | --- | [46, 47] | --- | --- | --- | --- |
| wide_decomposed | 139ns | +92.0ns (+197.8%) | [+92, +92]ns | [138, 139] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide_blob | wide_decomposed |
|---|---|---|---|
| 1 | 1 | 47ns | +196.8% |
| 1 | 2 | 47ns | +198.7% |
| 1 | 3 | 47ns | +196.5% |
| 1 | 4 | 46ns | +197.8% |
| 1 | 5 | 47ns | +197.2% |
| 2 | 1 | 46ns | +198.1% |
| 2 | 2 | 46ns | +200.3% |
| 2 | 3 | 46ns | +197.2% |
| 2 | 4 | 47ns | +196.0% |
| 2 | 5 | 47ns | +197.7% |
| 3 | 1 | 47ns | +195.7% |
| 3 | 2 | 47ns | +196.9% |
| 3 | 3 | 47ns | +197.4% |
| 3 | 4 | 46ns | +200.6% |
| 3 | 5 | 47ns | +201.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | -0.065 | ok |
| wide_decomposed | 0.072 | ok |

**Consistency summary:**

- **wide_decomposed**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 128.3ns | 46.6ns | 275.1% | HIGH |
| wide_decomposed | 294.6ns | 138.9ns | 212.2% | HIGH |

## Distribution (algo ns)

```
wide_blob (n=120, range 46.0-47.4 ns)
     46.0 |##################
     46.0 |
     46.1 |
     46.2 |##################
     46.3 |#######################
     46.3 |
     46.4 |
     46.5 |########################################
     46.6 |
     46.6 |
     46.7 |#####################
     46.8 |#####################
     46.8 |
     46.9 |
     47.0 |#############################
     47.1 |
     47.1 |##########
     47.2 |
     47.3 |#########
     47.4 |
  (7 below, 7 above range)

wide_decomposed (n=120, range 137.7-140.5 ns)
    137.7 |#############################
    137.8 |
    137.9 |################################
    138.1 |#############################
    138.2 |########################################
    138.4 |################################
    138.5 |
    138.6 |################################
    138.8 |########################################
    138.9 |##############
    139.1 |##################
    139.2 |#####################
    139.3 |
    139.5 |##################
    139.6 |#########################
    139.8 |#######
    139.9 |##############
    140.0 |
    140.2 |#######
    140.3 |
  (10 below, 10 above range)

```

## Diagnostics

- **wide_blob**: bridge=275.3% of algo (FFI overhead may distort results)
- **wide_decomposed**: bridge=211.8% of algo (FFI overhead may distort results)
