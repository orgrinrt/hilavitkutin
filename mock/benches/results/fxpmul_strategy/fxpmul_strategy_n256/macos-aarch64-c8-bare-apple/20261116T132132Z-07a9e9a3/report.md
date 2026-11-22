# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 41% faster than the next best (fxpmul_precise)

fxpmul_hot (18 ns) leads fxpmul_precise (25 ns) by 41%, a clear separation rather than a photo finish. CV 13.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_hot is fastest but the noisiest (CV 13.5%)

fxpmul_hot wins on median (18 ns) yet has the highest variance (CV 13.5%), while fxpmul_precise is the steadiest (CV 12.5%, 25 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (18 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 17.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.41x (fastest 17.9 ns, slowest 25.2 ns)

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
| fxpmul_hot | 2102ns | 2098ns | 2093ns | 2098ns | 2121ns | base |
| fxpmul_precise | 2107ns | 2103ns | 2098ns | 2103ns | 2126ns | +0.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 18ns | 14ns | 21ns | base | 14.509 |
| fxpmul_precise | 25ns | 21ns | 29ns | +41.69% | 10.240 |

## Performance model

- Peak throughput: **17.941 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 14.302 | 79.7% |
| fxpmul_precise | 10.159 | 56.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2102ns | 2102ns | base |
| fxpmul_precise | 2107ns | 2107ns | +0.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 18ns | base | --- | [17, 18] | --- | --- | --- | --- |
| fxpmul_precise | 25ns | +7.2ns (+40.5%) | [+7, +8]ns | [24, 26] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 17ns | +46.8% |
| 1 | 2 | 18ns | +36.3% |
| 1 | 3 | 18ns | +49.7% |
| 1 | 4 | 17ns | +58.4% |
| 1 | 5 | 18ns | +38.0% |
| 1 | 6 | 18ns | +38.3% |
| 1 | 7 | 19ns | +36.9% |
| 1 | 8 | 18ns | +35.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | 0.276 | moderate+ |
| fxpmul_precise | -0.151 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 7.6ns | 17.6ns | 42.8% | HIGH |
| fxpmul_precise | 5.9ns | 25.0ns | 23.5% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 14.3-21.0 ns)
     14.3 |########
     14.6 |
     14.9 |########
     15.3 |#############
     15.6 |###############
     15.9 |########################
     16.3 |
     16.6 |#####################################
     16.9 |###########
     17.3 |###############
     17.6 |########################################
     17.9 |
     18.3 |#############
     18.6 |####################
     19.0 |####################
     19.3 |##########################
     19.6 |
     20.0 |#############
     20.3 |###############
     20.6 |######
  (17 below, 12 above range)

fxpmul_precise (n=160, range 20.5-29.2 ns)
     20.5 |##########
     21.0 |#######################
     21.4 |######
     21.8 |#############
     22.3 |##########################
     22.7 |#######################
     23.1 |##########################
     23.6 |##############################
     24.0 |##########################
     24.4 |##########################
     24.9 |#############
     25.3 |####################################
     25.7 |################
     26.2 |#######################
     26.6 |##############################
     27.1 |####################################
     27.5 |########################################
     27.9 |################
     28.4 |####################
     28.8 |##########
  (12 below, 11 above range)

```

## Diagnostics

- **fxpmul_hot**: bridge=41.9% of algo (FFI overhead may distort results)
- **fxpmul_precise**: bridge=23.0% of algo (FFI overhead may distort results)
