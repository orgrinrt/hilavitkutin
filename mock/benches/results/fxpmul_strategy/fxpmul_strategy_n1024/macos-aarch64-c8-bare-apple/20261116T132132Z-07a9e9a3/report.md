# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 17% faster than the next best (fxpmul_precise)

fxpmul_hot (91 ns) leads fxpmul_precise (107 ns) by 17%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (91 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 91.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.17x (fastest 91.2 ns, slowest 106.7 ns)

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
| fxpmul_hot | 2167ns | 2164ns | 2160ns | 2164ns | 2185ns | base |
| fxpmul_precise | 2189ns | 2185ns | 2181ns | 2185ns | 2206ns | +0.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 91ns | 85ns | 96ns | base | 11.265 |
| fxpmul_precise | 106ns | 101ns | 111ns | +17.06% | 9.623 |

## Performance model

- Peak throughput: **12.032 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 11.228 | 93.3% |
| fxpmul_precise | 9.597 | 79.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2167ns | 2167ns | base |
| fxpmul_precise | 2189ns | 2189ns | +0.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 91ns | base | --- | [90, 92] | --- | --- | --- | --- |
| fxpmul_precise | 107ns | +15.4ns (+16.9%) | [+15, +16]ns | [106, 108] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 91ns | +17.5% |
| 1 | 2 | 93ns | +15.7% |
| 1 | 3 | 92ns | +15.6% |
| 1 | 4 | 90ns | +17.9% |
| 1 | 5 | 91ns | +17.7% |
| 1 | 6 | 91ns | +17.3% |
| 1 | 7 | 92ns | +15.8% |
| 1 | 8 | 92ns | +16.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.031 | ok |
| fxpmul_precise | -0.144 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 2.9ns | 90.9ns | 3.2% |  |
| fxpmul_precise | 2.9ns | 106.4ns | 2.7% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 85.1-95.8 ns)
     85.1 |######
     85.6 |####
     86.2 |#############
     86.7 |###########
     87.3 |######
     87.8 |######################
     88.3 |#############
     88.9 |####################
     89.4 |##
     89.9 |############################
     90.5 |###########
     91.0 |###########
     91.5 |#############
     92.1 |########################
     92.6 |##########################
     93.2 |########
     93.7 |########################################
     94.2 |#############
     94.8 |#############
     95.3 |###########
  (13 below, 11 above range)

fxpmul_precise (n=160, range 101.1-110.9 ns)
    101.1 |##
    101.6 |####
    102.1 |################
    102.5 |######
    103.0 |########
    103.5 |########
    104.0 |########
    104.5 |##############
    105.0 |######################
    105.5 |##########
    106.0 |################
    106.5 |##################
    107.0 |####################
    107.5 |########################################
    107.9 |##############
    108.4 |########
    108.9 |##############
    109.4 |########
    109.9 |##########
    110.4 |##########################
  (15 below, 9 above range)

```
