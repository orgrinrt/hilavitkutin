# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 40% faster than the next best (fxpmul_precise)

fxpmul_hot (18 ns) leads fxpmul_precise (25 ns) by 40%, a clear separation rather than a photo finish. CV 11.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_precise shows alternating (throttle bounce) (autocorr -0.70)

fxpmul_precise's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (18 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 17.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.40x (fastest 17.9 ns, slowest 25.0 ns)

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
| fxpmul_hot | 2108ns | 2100ns | 2094ns | 2100ns | 2145ns | base |
| fxpmul_precise | 2124ns | 2106ns | 2098ns | 2110ns | 2193ns | +0.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 18ns | 15ns | 21ns | base | 14.473 |
| fxpmul_precise | 25ns | 19ns | 29ns | +40.06% | 10.334 |

## Performance model

- Peak throughput: **17.393 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 14.302 | 82.2% |
| fxpmul_precise | 10.240 | 58.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2108ns | 2108ns | base |
| fxpmul_precise | 2124ns | 2124ns | +0.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 18ns | base | --- | [17, 18] | --- | --- | --- | --- |
| fxpmul_precise | 25ns | +7.1ns (+39.7%) | [+7, +8]ns | [25, 26] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 17ns | +47.4% |
| 1 | 2 | 18ns | +38.5% |
| 1 | 3 | 18ns | +42.9% |
| 1 | 4 | 18ns | +44.0% |
| 1 | 5 | 18ns | +34.9% |
| 1 | 6 | 18ns | +41.0% |
| 1 | 7 | 18ns | +36.3% |
| 1 | 8 | 17ns | +49.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.037 | ok |
| fxpmul_precise | -0.702 | HIGH- (thermal bounce) |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 7.3ns | 17.7ns | 41.5% | HIGH |
| fxpmul_precise | 6.0ns | 24.8ns | 24.2% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 14.7-20.6 ns)
     14.7 |##############
     15.0 |
     15.3 |##################
     15.6 |#######
     15.9 |
     16.2 |########################################
     16.5 |#####################
     16.8 |
     17.1 |############################
     17.4 |#########################
     17.7 |############################
     18.0 |
     18.3 |#####################################
     18.6 |#########################
     18.9 |
     19.2 |################
     19.5 |##############
     19.7 |#####################
     20.0 |
     20.3 |###########
  (13 below, 15 above range)

fxpmul_precise (n=160, range 19.5-29.0 ns)
     19.5 |########
     20.0 |#################
     20.4 |########
     20.9 |###########
     21.4 |
     21.9 |#####
     22.3 |###########
     22.8 |###############################
     23.3 |######################
     23.8 |############################
     24.3 |############################
     24.7 |############################
     25.2 |#################
     25.7 |#########################
     26.2 |#########################
     26.6 |########################################
     27.1 |##############
     27.6 |#########################
     28.1 |########
     28.6 |#################
  (13 below, 15 above range)

```

## Diagnostics

- **fxpmul_hot**: bridge=39.7% of algo (FFI overhead may distort results)
- **fxpmul_precise**: bridge=24.0% of algo (FFI overhead may distort results)
