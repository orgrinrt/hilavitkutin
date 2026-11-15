# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 25% faster than the next best (fxpmul_precise)

fxpmul_hot (401 ns) leads fxpmul_precise (501 ns) by 25%, a clear separation rather than a photo finish. CV 5.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (401 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 401.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.25x (fastest 401.0 ns, slowest 500.8 ns)

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
| fxpmul_hot | 2581ns | 2564ns | 2450ns | 2554ns | 2794ns | base |
| fxpmul_precise | 2737ns | 2717ns | 2538ns | 2733ns | 2948ns | +6.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 406ns | 384ns | 442ns | base | 10.079 |
| fxpmul_precise | 504ns | 466ns | 545ns | +23.94% | 8.132 |

## Performance model

- Peak throughput: **10.674 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 10.214 | 95.7% |
| fxpmul_precise | 8.179 | 76.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2581ns | 2581ns | base |
| fxpmul_precise | 2737ns | 2737ns | +6.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 401ns | base | --- | [393, 410] | --- | --- | --- | --- |
| fxpmul_precise | 501ns | +109.6ns (+27.3%) | [+99, +114]ns | [498, 502] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 416ns | +12.3% |
| 1 | 2 | 412ns | +28.4% |
| 1 | 3 | 391ns | +28.4% |
| 1 | 4 | 386ns | +36.6% |
| 1 | 5 | 445ns | +5.2% |
| 1 | 6 | 390ns | +28.4% |
| 1 | 7 | 414ns | +16.8% |
| 1 | 8 | 389ns | +38.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.479 | moderate- |
| fxpmul_precise | -0.448 | moderate- |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 3.2ns | 406.4ns | 0.8% |  |
| fxpmul_precise | 3.3ns | 503.7ns | 0.7% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 383.7-442.5 ns)
    383.7 |########################################
    386.7 |####################################
    389.6 |##############################
    392.6 |##########
    395.5 |##########
    398.4 |########
    401.4 |####
    404.3 |########
    407.2 |##############
    410.2 |####################
    413.1 |##################
    416.1 |##########################
    419.0 |############
    421.9 |########
    424.9 |
    427.8 |
    430.7 |
    433.7 |##
    436.6 |
    439.6 |######
  (14 below, 20 above range)

fxpmul_precise (n=160, range 466.1-545.2 ns)
    466.1 |##################################
    470.1 |##################
    474.0 |#
    478.0 |#######
    481.9 |#########
    485.9 |#####
    489.8 |#########
    493.8 |############
    497.7 |########################################
    501.7 |###########################
    505.6 |###
    509.6 |
    513.6 |
    517.5 |
    521.5 |#
    525.4 |#######
    529.4 |###
    533.3 |############
    537.3 |########################################
    541.2 |####################
  (13 below, 7 above range)

```
