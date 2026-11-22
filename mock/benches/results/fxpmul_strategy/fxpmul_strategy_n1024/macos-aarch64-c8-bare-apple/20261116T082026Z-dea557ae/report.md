# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 16% faster than the next best (fxpmul_precise)

fxpmul_hot (95 ns) leads fxpmul_precise (110 ns) by 16%, a clear separation rather than a photo finish. CV 4.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (95 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 95.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.16x (fastest 95.0 ns, slowest 110.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate) |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5555666677770007 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 2221ns | 2203ns | 2191ns | 2206ns | 2292ns | base |
| fxpmul_precise | 2237ns | 2223ns | 2213ns | 2226ns | 2297ns | +0.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 95ns | 88ns | 101ns | base | 10.802 |
| fxpmul_precise | 110ns | 105ns | 116ns | +16.53% | 9.270 |

## Performance model

- Peak throughput: **11.605 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 10.779 | 92.9% |
| fxpmul_precise | 9.275 | 79.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2221ns | 2221ns | base |
| fxpmul_precise | 2237ns | 2237ns | +0.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 95ns | base | --- | [95, 96] | --- | --- | --- | --- |
| fxpmul_precise | 110ns | +15.4ns (+16.2%) | [+15, +16]ns | [110, 111] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 96ns | +18.1% |
| 1 | 2 | 98ns | +13.4% |
| 1 | 3 | 93ns | +16.4% |
| 1 | 4 | 95ns | +17.1% |
| 1 | 5 | 93ns | +18.4% |
| 1 | 6 | 95ns | +15.2% |
| 1 | 7 | 95ns | +16.5% |
| 1 | 8 | 95ns | +15.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.005 | ok |
| fxpmul_precise | 0.010 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 3.1ns | 94.8ns | 3.3% |  |
| fxpmul_precise | 2.8ns | 110.5ns | 2.5% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 88.2-100.5 ns)
     88.2 |######
     88.9 |########
     89.5 |#################
     90.1 |
     90.7 |########
     91.3 |####
     91.9 |########################
     92.5 |###########
     93.2 |###############
     93.8 |##########################
     94.4 |########################################
     95.0 |###############
     95.6 |##########################
     96.2 |#############
     96.8 |#################
     97.5 |#############
     98.1 |###########
     98.7 |########################
     99.3 |###########
     99.9 |#############
  (11 below, 9 above range)

fxpmul_precise (n=160, range 104.9-115.8 ns)
    104.9 |########
    105.4 |######
    106.0 |####
    106.5 |########
    107.1 |################
    107.6 |##########
    108.2 |######################
    108.7 |##############
    109.3 |################
    109.8 |####################
    110.4 |######################
    110.9 |##################
    111.4 |########
    112.0 |########################################
    112.5 |##############
    113.1 |##########
    113.6 |########
    114.2 |######
    114.7 |##############
    115.3 |##########
  (14 below, 9 above range)

```
