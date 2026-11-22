# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 21% faster than the next best (fxpmul_precise)

fxpmul_hot (394 ns) leads fxpmul_precise (477 ns) by 21%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (394 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 394.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.21x (fastest 394.2 ns, slowest 476.7 ns)

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
| fxpmul_hot | 2494ns | 2497ns | 2467ns | 2497ns | 2511ns | base |
| fxpmul_precise | 2585ns | 2583ns | 2576ns | 2584ns | 2600ns | +3.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 394ns | 387ns | 400ns | base | 10.394 |
| fxpmul_precise | 477ns | 471ns | 482ns | +20.99% | 8.591 |

## Performance model

- Peak throughput: **10.573 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 10.391 | 98.3% |
| fxpmul_precise | 8.592 | 81.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2494ns | 2494ns | base |
| fxpmul_precise | 2585ns | 2585ns | +3.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 394ns | base | --- | [394, 395] | --- | --- | --- | --- |
| fxpmul_precise | 477ns | +82.1ns (+20.8%) | [+82, +82]ns | [476, 478] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 395ns | +20.8% |
| 1 | 2 | 394ns | +20.7% |
| 1 | 3 | 395ns | +20.7% |
| 1 | 4 | 395ns | +20.6% |
| 1 | 5 | 395ns | +21.0% |
| 1 | 6 | 395ns | +21.0% |
| 1 | 7 | 389ns | +22.5% |
| 1 | 8 | 395ns | +20.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.214 | moderate- |
| fxpmul_precise | 0.132 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 3.1ns | 394.1ns | 0.8% |  |
| fxpmul_precise | 3.3ns | 476.8ns | 0.7% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 387.4-400.2 ns)
    387.4 |##
    388.0 |
    388.7 |########
    389.3 |########
    390.0 |################
    390.6 |##########################
    391.2 |#############
    391.9 |#####################
    392.5 |#############
    393.1 |################################
    393.8 |##################################
    394.4 |########################################
    395.1 |########################
    395.7 |##################################
    396.3 |#####
    397.0 |#####################
    397.6 |#####
    398.3 |################
    398.9 |##########
    399.5 |##################
  (16 below, 12 above range)

fxpmul_precise (n=160, range 471.4-481.9 ns)
    471.4 |
    471.9 |########
    472.4 |################
    473.0 |########
    473.5 |#############
    474.0 |########
    474.5 |##########################
    475.1 |##################
    475.6 |########################################
    476.1 |#####################
    476.7 |########################################
    477.2 |########################
    477.7 |#############################
    478.2 |########################
    478.8 |#####################
    479.3 |##################
    479.8 |################
    480.4 |##################
    480.9 |##
    481.4 |##########
  (13 below, 10 above range)

```
