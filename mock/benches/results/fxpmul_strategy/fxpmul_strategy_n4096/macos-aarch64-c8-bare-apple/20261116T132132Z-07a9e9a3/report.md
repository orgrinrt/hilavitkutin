# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 21% faster than the next best (fxpmul_precise)

fxpmul_hot (387 ns) leads fxpmul_precise (468 ns) by 21%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (387 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 387.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.21x (fastest 387.1 ns, slowest 467.5 ns)

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
| fxpmul_hot | 2472ns | 2463ns | 2457ns | 2465ns | 2509ns | base |
| fxpmul_precise | 2549ns | 2546ns | 2541ns | 2546ns | 2566ns | +3.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 387ns | 381ns | 395ns | base | 10.571 |
| fxpmul_precise | 468ns | 461ns | 475ns | +20.72% | 8.756 |

## Performance model

- Peak throughput: **10.751 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 10.581 | 98.4% |
| fxpmul_precise | 8.761 | 81.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2472ns | 2472ns | base |
| fxpmul_precise | 2549ns | 2549ns | +3.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 387ns | base | --- | [386, 388] | --- | --- | --- | --- |
| fxpmul_precise | 468ns | +80.0ns (+20.7%) | [+80, +81]ns | [467, 468] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 394ns | +18.6% |
| 1 | 2 | 388ns | +20.5% |
| 1 | 3 | 386ns | +20.9% |
| 1 | 4 | 386ns | +21.2% |
| 1 | 5 | 386ns | +21.1% |
| 1 | 6 | 387ns | +20.7% |
| 1 | 7 | 387ns | +21.1% |
| 1 | 8 | 387ns | +21.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | 0.233 | moderate+ |
| fxpmul_precise | 0.220 | moderate+ |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 2.8ns | 387.5ns | 0.7% |  |
| fxpmul_precise | 2.9ns | 467.8ns | 0.6% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 381.0-394.8 ns)
    381.0 |######
    381.7 |###########
    382.4 |###########
    383.1 |#############
    383.7 |######################
    384.4 |#################
    385.1 |########################################
    385.8 |########
    386.5 |############################
    387.2 |#################
    387.9 |###############################
    388.6 |#################
    389.3 |######
    389.9 |######
    390.6 |###########
    391.3 |###########
    392.0 |####################
    392.7 |####
    393.4 |##
    394.1 |######
  (11 below, 16 above range)

fxpmul_precise (n=160, range 461.2-475.2 ns)
    461.2 |##
    461.9 |########
    462.6 |########
    463.3 |######
    464.0 |################
    464.7 |##################################
    465.4 |############
    466.1 |########
    466.8 |########################################
    467.5 |##############
    468.2 |################################
    468.9 |########
    469.6 |################################
    470.3 |########################
    471.0 |######
    471.7 |##############
    472.4 |####
    473.1 |####
    473.8 |##
    474.5 |######
  (16 below, 4 above range)

```
