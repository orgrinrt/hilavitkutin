# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 41% faster than the next best (fxpmul_precise)

fxpmul_hot (13 ns) leads fxpmul_precise (19 ns) by 41%, a clear separation rather than a photo finish. CV 15.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (13 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_precise is inconsistent: worst-20% is 1.6x its best-20%

fxpmul_precise's best 20% of batches run at 16 ns but its worst 20% at 25 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 13.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.41x (fastest 13.3 ns, slowest 18.8 ns)

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
| fxpmul_hot | 2138ns | 2133ns | 2127ns | 2133ns | 2164ns | base |
| fxpmul_precise | 2143ns | 2135ns | 2127ns | 2138ns | 2175ns | +0.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 13ns | 11ns | 16ns | base | 19.152 |
| fxpmul_precise | 20ns | 16ns | 25ns | +47.81% | 12.957 |

## Performance model

- Peak throughput: **24.187 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 19.248 | 79.6% |
| fxpmul_precise | 13.617 | 56.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2138ns | 2138ns | base |
| fxpmul_precise | 2143ns | 2143ns | +0.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 13ns | base | --- | [13, 14] | --- | --- | --- | --- |
| fxpmul_precise | 19ns | +5.9ns (+44.4%) | [+5, +7]ns | [18, 20] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 14ns | +35.1% |
| 1 | 2 | 13ns | +44.4% |
| 1 | 3 | 15ns | +28.8% |
| 1 | 4 | 14ns | +100.0% |
| 1 | 5 | 13ns | +45.7% |
| 1 | 6 | 14ns | +36.9% |
| 1 | 7 | 13ns | +48.0% |
| 1 | 8 | 14ns | +35.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.147 | ok |
| fxpmul_precise | -0.143 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 3.5ns | 13.4ns | 26.1% | HIGH |
| fxpmul_precise | 6.2ns | 19.8ns | 31.4% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 10.6-16.2 ns)
     10.6 |##################
     10.9 |
     11.1 |########
     11.4 |################
     11.7 |
     12.0 |################
     12.3 |########################################
     12.6 |
     12.8 |###############################
     13.1 |##################
     13.4 |
     13.7 |###############################
     14.0 |#####################
     14.3 |
     14.5 |###############################
     14.8 |##################
     15.1 |
     15.4 |############
     15.7 |##########
     15.9 |####
  (14 below, 12 above range)

fxpmul_precise (n=160, range 16.1-25.1 ns)
     16.1 |######################
     16.6 |###########
     17.0 |####################
     17.5 |########################################
     17.9 |########################################
     18.4 |############################
     18.8 |#################
     19.3 |###############
     19.7 |######################
     20.2 |#############
     20.6 |###########
     21.1 |###############
     21.5 |####
     22.0 |###############
     22.4 |####
     22.9 |#############
     23.3 |####
     23.8 |##
     24.2 |
     24.7 |
  (9 below, 15 above range)

```

## Diagnostics

- **fxpmul_hot**: bridge=24.8% of algo (FFI overhead may distort results)
- **fxpmul_precise**: bridge=33.0% of algo (FFI overhead may distort results)
