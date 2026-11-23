# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (59 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_scalar at 36 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_scalar dominates: 64% faster than the next best (ema_neon)

ema_scalar (36 ns) leads ema_neon (59 ns) by 64%, a clear separation rather than a photo finish. CV 90.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_scalar beats baseline by 49% (significant)

ema_scalar is -29 ns (49%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_scalar is fastest but the noisiest (CV 90.2%)

ema_scalar wins on median (36 ns) yet has the highest variance (CV 90.2%), while ema_autovec is the steadiest (CV 81.8%, 59 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (23 ns) is smaller than the fastest variant's own run-to-run std-dev (33 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### ema_scalar is inconsistent: worst-20% is 3.8x its best-20%

ema_scalar's best 20% of batches run at 30 ns but its worst 20% at 112 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### ema_neon's edge over baseline is significant but tiny (-1 ns, 1.94%)

ema_neon differs from baseline ema_autovec by -1 ns (1.94%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ema_scalar** at 36.1 ns median (-39.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.64x (fastest 36.1 ns, slowest 59.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | EMA streaming pass: NEON vs scalar vs autovec under realistic workload |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112833612 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_autovec | 2631ns | 1938ns | 1781ns | 1941ns | 5553ns | base |
| ema_neon | 2648ns | 1935ns | 1583ns | 1971ns | 5744ns | +0.64% |
| ema_scalar | 2706ns | 1913ns | 1565ns | 1995ns | 5978ns | +2.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 80ns | 54ns | 167ns | base | 3.207 |
| ema_neon | 81ns | 48ns | 176ns | +1.75% | 3.152 |
| ema_scalar | 51ns | 30ns | 112ns | -36.06% | 5.015 |

## Performance model

- Peak throughput: **8.647 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 4.317 | 49.9% |
| ema_neon | 4.324 | 50.0% |
| ema_scalar | 7.091 | 82.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2631ns | 2631ns | base |
| ema_neon | 2648ns | 2648ns | +0.64% |
| ema_scalar | 2706ns | 2706ns | +2.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 59ns | base | --- | [59, 59] | --- | --- | --- | --- |
| ema_neon | 59ns | -1.2ns (-1.9%) | [-10, -0]ns | [49, 59] | YES | 0.0024 | 0.0024 | 10 |
| ema_scalar | 36ns | -29.1ns (-49.1%) | [-29, -23]ns | [36, 36] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 59ns | +0.0% | -49.8% |
| 1 | 2 | 59ns | -17.9% | -49.5% |
| 1 | 3 | 59ns | -18.0% | -49.6% |
| 1 | 4 | 59ns | -18.0% | -38.3% |
| 1 | 5 | 49ns | +0.3% | -24.9% |
| 1 | 6 | 59ns | -17.8% | -38.9% |
| 1 | 7 | 59ns | +0.2% | -49.7% |
| 1 | 8 | 59ns | +0.3% | -38.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.163 | ok |
| ema_neon | 0.259 | moderate+ |
| ema_scalar | 0.142 | ok |

**Consistency summary:**

- **ema_neon**: won 4/8, lost 3/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 32.2ns | 79.8ns | 40.3% | HIGH |
| ema_neon | 32.2ns | 81.2ns | 39.7% | HIGH |
| ema_scalar | 33.6ns | 51.0ns | 65.8% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 54.4-166.8 ns)
     54.4 |########################################
     60.0 |
     65.7 |
     71.3 |
     76.9 |
     82.5 |
     88.1 |
     93.8 |
     99.4 |
    105.0 |
    110.6 |
    116.2 |
    121.9 |
    127.5 |
    133.1 |
    138.7 |
    144.3 |
    150.0 |#
    155.6 |
    161.2 |
  (14 below, 18 above range)

ema_neon (n=160, range 48.5-176.3 ns)
     48.5 |########################################
     54.9 |################################
     61.2 |
     67.6 |
     74.0 |
     80.4 |
     86.8 |
     93.2 |
     99.6 |
    106.0 |
    112.4 |
    118.8 |
    125.2 |#
    131.5 |##
    137.9 |###
    144.3 |#
    150.7 |###
    157.1 |
    163.5 |#
    169.9 |##
  (14 below, 13 above range)

ema_scalar (n=160, range 29.6-112.3 ns)
     29.6 |###########################
     33.7 |########################################
     37.9 |
     42.0 |
     46.1 |
     50.3 |
     54.4 |
     58.5 |
     62.7 |
     66.8 |#
     71.0 |
     75.1 |
     79.2 |
     83.4 |
     87.5 |
     91.6 |###
     95.8 |
     99.9 |#
    104.0 |#
    108.2 |####
  (16 below, 14 above range)

```

## Diagnostics

- **ema_autovec**: CV=60.8% (high variance, measurements may be unstable)
- **ema_autovec**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **ema_autovec**: bridge=40.3% of algo (FFI overhead may distort results)
- **ema_neon**: CV=64.1% (high variance, measurements may be unstable)
- **ema_neon**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **ema_neon**: bridge=39.9% of algo (FFI overhead may distort results)
- **ema_scalar**: CV=63.8% (high variance, measurements may be unstable)
- **ema_scalar**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **ema_scalar**: bridge=66.1% of algo (FFI overhead may distort results)
