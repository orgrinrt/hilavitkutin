# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Highlights

Baseline for all deltas below: **wide_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide_blob dominates: 91% faster than the next best (wide_decomposed)

wide_blob (4.16 us) leads wide_decomposed (7.96 us) by 91%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (wide_blob)

The baseline wide_blob is the fastest (4.16 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (wide_blob) is the fastest** at 4163.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.91x (fastest 4163.8 ns, slowest 7955.1 ns)

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
| wide_blob | 4226ns | 4216ns | 4211ns | 4219ns | 4264ns | base |
| wide_decomposed | 8023ns | 8000ns | 7982ns | 8000ns | 8133ns | +89.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 4176ns | 4163ns | 4214ns | base | 3.923 |
| wide_decomposed | 7977ns | 7936ns | 8086ns | +91.00% | 2.054 |

## Performance model

- Peak throughput: **3.936 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 3.935 | 100.0% |
| wide_decomposed | 2.060 | 52.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 4226ns | 4226ns | base |
| wide_decomposed | 8023ns | 8023ns | +89.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 4164ns | base | --- | [4164, 4167] | --- | --- | --- | --- |
| wide_decomposed | 7955ns | +3780.2ns (+90.8%) | [+3775, +3793]ns | [7944, 7962] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide_blob | wide_decomposed |
|---|---|---|---|
| 1 | 1 | 4179ns | +90.4% |
| 1 | 2 | 4163ns | +90.7% |
| 1 | 3 | 4164ns | +90.9% |
| 1 | 4 | 4170ns | +90.6% |
| 1 | 5 | 4166ns | +91.0% |
| 2 | 1 | 4173ns | +90.8% |
| 2 | 2 | 4164ns | +90.8% |
| 2 | 3 | 4167ns | +97.2% |
| 2 | 4 | 4164ns | +91.1% |
| 2 | 5 | 4163ns | +91.2% |
| 3 | 1 | 4163ns | +91.0% |
| 3 | 2 | 4186ns | +89.7% |
| 3 | 3 | 4173ns | +90.4% |
| 3 | 4 | 4166ns | +91.2% |
| 3 | 5 | 4163ns | +90.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | -0.035 | ok |
| wide_decomposed | -0.067 | ok |

**Consistency summary:**

- **wide_decomposed**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 134.1ns | 4176.5ns | 3.2% |  |
| wide_decomposed | 314.1ns | 7976.9ns | 3.9% |  |

## Distribution (algo ns)

```
wide_blob (n=120, range 4162.8-4213.6 ns)
   4162.8 |########################################
   4165.3 |#
   4167.8 |####
   4170.4 |##
   4172.9 |
   4175.5 |
   4178.0 |#
   4180.6 |####
   4183.1 |####
   4185.7 |#
   4188.2 |#
   4190.7 |
   4193.3 |#
   4195.8 |
   4198.4 |
   4200.9 |#
   4203.5 |
   4206.0 |##
   4208.5 |##
   4211.1 |
  (9 below, 9 above range)

wide_decomposed (n=120, range 7936.2-8085.6 ns)
   7936.2 |########################################
   7943.6 |#####
   7951.1 |##########
   7958.6 |#################
   7966.1 |########
   7973.5 |####
   7981.0 |####
   7988.5 |##
   7995.9 |#######
   8003.4 |
   8010.9 |#
   8018.4 |
   8025.8 |#
   8033.3 |
   8040.8 |#
   8048.3 |#
   8055.7 |
   8063.2 |
   8070.7 |
   8078.2 |#
  (12 below, 8 above range)

```
