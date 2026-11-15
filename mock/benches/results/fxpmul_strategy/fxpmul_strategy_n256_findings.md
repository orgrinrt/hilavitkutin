# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 43% faster than the next best (fxpmul_precise)

fxpmul_hot (19 ns) leads fxpmul_precise (27 ns) by 43%, a clear separation rather than a photo finish. CV 17.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_hot is fastest but the noisiest (CV 17.5%)

fxpmul_hot wins on median (19 ns) yet has the highest variance (CV 17.5%), while fxpmul_precise is the steadiest (CV 15.5%, 27 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (19 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_hot is inconsistent: worst-20% is 1.7x its best-20%

fxpmul_hot's best 20% of batches run at 13 ns but its worst 20% at 22 ns (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 18.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.43x (fastest 18.8 ns, slowest 26.9 ns)

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
| fxpmul_hot | 2264ns | 2236ns | 2105ns | 2255ns | 2448ns | base |
| fxpmul_precise | 2298ns | 2380ns | 2096ns | 2320ns | 2435ns | +1.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 18ns | 13ns | 22ns | base | 14.017 |
| fxpmul_precise | 26ns | 20ns | 32ns | +44.62% | 9.693 |

## Performance model

- Peak throughput: **19.226 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 13.617 | 70.8% |
| fxpmul_precise | 9.517 | 49.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2264ns | 2264ns | base |
| fxpmul_precise | 2298ns | 2298ns | +1.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 19ns | base | --- | [18, 20] | --- | --- | --- | --- |
| fxpmul_precise | 27ns | +8.3ns (+44.1%) | [+8, +9]ns | [26, 28] | YES | 0.0000 | 0.0000 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 18ns | +44.5% |
| 1 | 2 | 20ns | +3.0% |
| 1 | 3 | 14ns | +96.5% |
| 1 | 4 | 20ns | +38.7% |
| 1 | 5 | 20ns | +43.9% |
| 1 | 6 | 15ns | +62.7% |
| 1 | 7 | 19ns | +49.2% |
| 1 | 8 | 20ns | +38.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.449 | moderate- |
| fxpmul_precise | -0.146 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 7.0ns | 18.3ns | 38.2% | HIGH |
| fxpmul_precise | 6.5ns | 26.4ns | 24.7% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 13.3-22.4 ns)
     13.3 |
     13.8 |##########
     14.2 |###############
     14.7 |###############
     15.1 |############
     15.6 |#################
     16.0 |#####
     16.5 |##########
     16.9 |############
     17.4 |####################
     17.9 |###################################
     18.3 |
     18.8 |########################################
     19.2 |#########################
     19.7 |##############################
     20.1 |#################
     20.6 |###########################
     21.0 |#################
     21.5 |##########
     21.9 |############
  (15 below, 12 above range)

fxpmul_precise (n=160, range 20.3-32.0 ns)
     20.3 |#####################
     20.9 |###############
     21.5 |######
     22.1 |#####################
     22.7 |######
     23.3 |#################################
     23.8 |###############
     24.4 |###########################
     25.0 |#########
     25.6 |############
     26.2 |####################################
     26.8 |###############
     27.3 |########################################
     27.9 |#################################
     28.5 |########################
     29.1 |#################################
     29.7 |########################
     30.3 |##################
     30.8 |######
     31.4 |###
  (13 below, 15 above range)

```

## Diagnostics

- **fxpmul_hot**: bridge=39.9% of algo (FFI overhead may distort results)
- **fxpmul_precise**: bridge=23.0% of algo (FFI overhead may distort results)
