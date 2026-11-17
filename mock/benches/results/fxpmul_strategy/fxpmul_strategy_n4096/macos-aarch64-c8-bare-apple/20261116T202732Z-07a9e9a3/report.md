# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 21% faster than the next best (fxpmul_precise)

fxpmul_hot (387 ns) leads fxpmul_precise (468 ns) by 21%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (387 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 387.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.21x (fastest 387.1 ns, slowest 467.9 ns)

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
| fxpmul_hot | 2474ns | 2465ns | 2459ns | 2466ns | 2512ns | base |
| fxpmul_precise | 2561ns | 2548ns | 2542ns | 2550ns | 2611ns | +3.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 387ns | 381ns | 394ns | base | 10.576 |
| fxpmul_precise | 470ns | 462ns | 484ns | +21.42% | 8.711 |

## Performance model

- Peak throughput: **10.743 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 10.581 | 98.5% |
| fxpmul_precise | 8.754 | 81.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2474ns | 2474ns | base |
| fxpmul_precise | 2561ns | 2561ns | +3.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 387ns | base | --- | [386, 388] | --- | --- | --- | --- |
| fxpmul_precise | 468ns | +81.6ns (+21.1%) | [+81, +82]ns | [468, 469] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 392ns | +19.2% |
| 1 | 2 | 387ns | +21.7% |
| 1 | 3 | 386ns | +21.3% |
| 1 | 4 | 388ns | +20.5% |
| 1 | 5 | 386ns | +21.0% |
| 1 | 6 | 387ns | +21.3% |
| 1 | 7 | 387ns | +21.1% |
| 1 | 8 | 386ns | +21.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.056 | ok |
| fxpmul_precise | -0.139 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 2.9ns | 387.3ns | 0.8% |  |
| fxpmul_precise | 2.8ns | 470.2ns | 0.6% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 381.3-393.7 ns)
    381.3 |
    381.9 |#########
    382.5 |##
    383.1 |#########
    383.8 |################################
    384.4 |################
    385.0 |#####################
    385.6 |########################################
    386.2 |################
    386.9 |################
    387.5 |##############################
    388.1 |###########
    388.7 |#########################
    389.3 |#########
    390.0 |###########
    390.6 |####
    391.2 |##################
    391.8 |###########
    392.4 |################
    393.0 |#######
  (15 below, 12 above range)

fxpmul_precise (n=160, range 462.2-484.2 ns)
    462.2 |################
    463.3 |##########
    464.4 |####################
    465.5 |###########################
    466.6 |##################################
    467.7 |########################################
    468.8 |##############
    469.9 |#############################
    471.0 |################
    472.1 |################
    473.2 |##########
    474.3 |#####
    475.4 |
    476.5 |#
    477.6 |#
    478.7 |###
    479.8 |#
    480.9 |#
    482.0 |#
    483.1 |
  (14 below, 6 above range)

```
