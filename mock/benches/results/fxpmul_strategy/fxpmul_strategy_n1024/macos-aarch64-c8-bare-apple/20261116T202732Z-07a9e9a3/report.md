# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 17% faster than the next best (fxpmul_precise)

fxpmul_hot (92 ns) leads fxpmul_precise (108 ns) by 17%, a clear separation rather than a photo finish. CV 7.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_hot is fastest but the noisiest (CV 7.0%)

fxpmul_hot wins on median (92 ns) yet has the highest variance (CV 7.0%), while fxpmul_precise is the steadiest (CV 3.6%, 108 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fxpmul_precise shows warm-up / thermal drift (autocorr +0.52)

fxpmul_precise's per-pass series has lag-1 autocorrelation +0.52, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (92 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 91.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.17x (fastest 91.9 ns, slowest 107.9 ns)

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
| fxpmul_hot | 2190ns | 2170ns | 2162ns | 2179ns | 2251ns | base |
| fxpmul_precise | 2216ns | 2198ns | 2184ns | 2206ns | 2276ns | +1.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 92ns | 85ns | 99ns | base | 11.129 |
| fxpmul_precise | 108ns | 102ns | 113ns | +17.20% | 9.495 |

## Performance model

- Peak throughput: **12.031 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 11.143 | 92.6% |
| fxpmul_precise | 9.490 | 78.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2190ns | 2190ns | base |
| fxpmul_precise | 2216ns | 2216ns | +1.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 92ns | base | --- | [91, 93] | --- | --- | --- | --- |
| fxpmul_precise | 108ns | +15.9ns (+17.3%) | [+15, +17]ns | [107, 109] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 92ns | +18.0% |
| 1 | 2 | 91ns | +20.0% |
| 1 | 3 | 94ns | +14.7% |
| 1 | 4 | 92ns | +14.6% |
| 1 | 5 | 92ns | +15.3% |
| 1 | 6 | 91ns | +18.2% |
| 1 | 7 | 90ns | +20.0% |
| 1 | 8 | 94ns | +16.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.244 | moderate- |
| fxpmul_precise | 0.525 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 3.1ns | 92.0ns | 3.4% |  |
| fxpmul_precise | 3.0ns | 107.8ns | 2.7% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 85.1-99.4 ns)
     85.1 |##################
     85.8 |##
     86.5 |################
     87.2 |########################
     88.0 |################
     88.7 |##################
     89.4 |##################
     90.1 |##################################
     90.8 |########################
     91.5 |##################
     92.2 |########################################
     92.9 |#####
     93.7 |#############################
     94.4 |#############################
     95.1 |########
     95.8 |########################
     96.5 |##################
     97.2 |#####################
     97.9 |########
     98.6 |#####
  (12 below, 5 above range)

fxpmul_precise (n=160, range 102.4-113.2 ns)
    102.4 |######
    103.0 |######
    103.5 |######
    104.0 |#########
    104.6 |##############################
    105.1 |###############
    105.7 |##############################
    106.2 |########################################
    106.7 |#################################
    107.3 |#####################
    107.8 |#################################
    108.3 |########################
    108.9 |#################################
    109.4 |###############
    110.0 |########################################
    110.5 |##############################
    111.0 |######
    111.6 |#########
    112.1 |###
    112.6 |
  (14 below, 17 above range)

```

## Diagnostics

- **fxpmul_precise**: autocorrelation=0.52 (measurement drift or warm-up artifact)
