# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fpt_match_16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fpt_match_16 has the worst median (34 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest fpt_table_16 at 30 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### fpt_table_16 dominates: 13% faster than the next best (fpt_match_16)

fpt_table_16 (30 ns) leads fpt_match_16 (34 ns) by 13%, a clear separation rather than a photo finish. CV 97.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fpt_table_16 is fastest but the noisiest (CV 97.2%)

fpt_table_16 wins on median (30 ns) yet has the highest variance (CV 97.2%), while fpt_match_16 is the steadiest (CV 96.9%, 34 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4 ns) is smaller than the fastest variant's own run-to-run std-dev (29 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### fpt_match_16 is inconsistent: worst-20% is 4.0x its best-20%

fpt_match_16's best 20% of batches run at 28 ns but its worst 20% at 112 ns (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fpt_table_16** at 29.9 ns median (-11.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.13x (fastest 29.9 ns, slowest 33.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fn-pointer table vs match dispatch for 16-variant hot loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x478ab261e0d9f352 |
| Counter frequency | 2112822230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 2691ns | 1912ns | 1563ns | 1930ns | 6100ns | base |
| fpt_table_16 | 2604ns | 1903ns | 1558ns | 1823ns | 5994ns | -3.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 49ns | 28ns | 112ns | base | 5.201 |
| fpt_table_16 | 42ns | 24ns | 97ns | -15.07% | 6.124 |

## Performance model

- Peak throughput: **10.481 Gops/s** (fpt_table_16; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 7.608 | 72.6% |
| fpt_table_16 | 8.562 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2691ns | 2691ns | base |
| fpt_table_16 | 2604ns | 2604ns | -3.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 34ns | base | --- | [33, 35] | --- | --- | --- | --- |
| fpt_table_16 | 30ns | -4.1ns (-12.0%) | [-4, -4]ns | [29, 30] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 28ns | -12.3% |
| 1 | 2 | 28ns | -13.9% |
| 1 | 3 | 28ns | -10.1% |
| 1 | 4 | 34ns | -10.2% |
| 1 | 5 | 30ns | +0.5% |
| 1 | 6 | 33ns | -25.6% |
| 1 | 7 | 36ns | -16.2% |
| 1 | 8 | 36ns | -14.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | 0.466 | moderate+ |
| fpt_table_16 | 0.174 | ok |

**Consistency summary:**

- **fpt_table_16**: won 7/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 34.0ns | 49.2ns | 69.0% | HIGH |
| fpt_table_16 | 33.4ns | 41.8ns | 79.9% | HIGH |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 27.9-111.7 ns)
     27.9 |###################################
     32.1 |########################################
     36.3 |######
     40.5 |
     44.7 |
     48.8 |#
     53.0 |
     57.2 |
     61.4 |
     65.6 |
     69.8 |
     74.0 |#
     78.2 |
     82.4 |
     86.6 |
     90.8 |
     95.0 |#
     99.2 |###
    103.4 |#######
    107.5 |###
  (13 below, 13 above range)

fpt_table_16 (n=160, range 24.4-97.4 ns)
     24.4 |##############################
     28.1 |########################################
     31.7 |
     35.4 |
     39.0 |
     42.7 |#
     46.3 |
     50.0 |
     53.6 |
     57.3 |
     60.9 |
     64.6 |
     68.2 |
     71.9 |
     75.5 |
     79.2 |
     82.8 |
     86.5 |#
     90.1 |
     93.8 |#
  (17 below, 20 above range)

```

## Diagnostics

- **fpt_match_16**: CV=66.2% (high variance, measurements may be unstable)
- **fpt_match_16**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **fpt_match_16**: bridge=71.9% of algo (FFI overhead may distort results)
- **fpt_table_16**: CV=69.5% (high variance, measurements may be unstable)
- **fpt_table_16**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **fpt_table_16**: bridge=81.3% of algo (FFI overhead may distort results)
