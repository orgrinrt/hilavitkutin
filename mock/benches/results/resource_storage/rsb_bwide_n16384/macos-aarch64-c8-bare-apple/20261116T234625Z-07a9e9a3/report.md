# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Highlights

Baseline for all deltas below: **wide_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide_blob dominates: 90% faster than the next best (wide_decomposed)

wide_blob (4.11 us) leads wide_decomposed (7.81 us) by 90%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (wide_blob)

The baseline wide_blob is the fastest (4.11 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (wide_blob) is the fastest** at 4110.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.90x (fastest 4110.1 ns, slowest 7814.8 ns)

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
| wide_blob | 4237ns | 4161ns | 4132ns | 4209ns | 4423ns | base |
| wide_decomposed | 7935ns | 7859ns | 7833ns | 7889ns | 8173ns | +87.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 4184ns | 4085ns | 4366ns | base | 3.916 |
| wide_decomposed | 7890ns | 7788ns | 8127ns | +88.56% | 2.077 |

## Performance model

- Peak throughput: **4.011 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 3.986 | 99.4% |
| wide_decomposed | 2.097 | 52.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 4237ns | 4237ns | base |
| wide_decomposed | 7935ns | 7935ns | +87.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 4110ns | base | --- | [4094, 4214] | --- | --- | --- | --- |
| wide_decomposed | 7815ns | +3703.2ns (+90.1%) | [+3663, +3708]ns | [7802, 7842] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide_blob | wide_decomposed |
|---|---|---|---|
| 1 | 1 | 4085ns | +90.9% |
| 1 | 2 | 4237ns | +87.6% |
| 1 | 3 | 4255ns | +83.7% |
| 1 | 4 | 4097ns | +90.3% |
| 1 | 5 | 4159ns | +90.7% |
| 2 | 1 | 4085ns | +91.2% |
| 2 | 2 | 4317ns | +80.7% |
| 2 | 3 | 4087ns | +99.1% |
| 2 | 4 | 4085ns | +97.7% |
| 2 | 5 | 4086ns | +96.1% |
| 3 | 1 | 4245ns | +83.6% |
| 3 | 2 | 4242ns | +84.0% |
| 3 | 3 | 4324ns | +80.2% |
| 3 | 4 | 4277ns | +82.2% |
| 3 | 5 | 4086ns | +95.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | 0.004 | ok |
| wide_decomposed | 0.236 | moderate+ |

**Consistency summary:**

- **wide_decomposed**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 133.9ns | 4184.2ns | 3.2% |  |
| wide_decomposed | 310.7ns | 7889.6ns | 3.9% |  |

## Distribution (algo ns)

```
wide_blob (n=120, range 4084.9-4365.9 ns)
   4084.9 |########################################
   4099.0 |##########
   4113.0 |
   4127.1 |###
   4141.1 |##
   4155.2 |##
   4169.2 |
   4183.3 |
   4197.3 |##
   4211.4 |#
   4225.4 |#########
   4239.5 |#######
   4253.5 |#######
   4267.6 |
   4281.6 |###
   4295.7 |####
   4309.7 |###
   4323.8 |##
   4337.8 |###
   4351.9 |####
  (13 below, 7 above range)

wide_decomposed (n=120, range 7788.3-8127.2 ns)
   7788.3 |########################################
   7805.2 |################
   7822.2 |####
   7839.1 |#####
   7856.1 |###
   7873.0 |##
   7890.0 |#
   7906.9 |#
   7923.9 |##
   7940.8 |##########
   7957.7 |###
   7974.7 |#
   7991.6 |
   8008.6 |#
   8025.5 |#
   8042.5 |#
   8059.4 |####
   8076.4 |###
   8093.3 |#
   8110.3 |#
  (13 below, 11 above range)

```
