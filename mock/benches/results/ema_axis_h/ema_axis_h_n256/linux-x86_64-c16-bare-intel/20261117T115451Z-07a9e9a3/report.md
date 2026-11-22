# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_scalar dominates: 97% faster than the next best (ema_autovec)

ema_scalar (30 ns) leads ema_autovec (59 ns) by 97%, a clear separation rather than a photo finish. CV 114.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_scalar beats baseline by 49% (significant)

ema_scalar is -29 ns (49%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_scalar is fastest but the noisiest (CV 114.6%)

ema_scalar wins on median (30 ns) yet has the highest variance (CV 114.6%), while ema_neon is the steadiest (CV 87.8%, 59 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (29 ns) is smaller than the fastest variant's own run-to-run std-dev (34 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### ema_scalar is inconsistent: worst-20% is 3.8x its best-20%

ema_scalar's best 20% of batches run at 29 ns but its worst 20% at 113 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### ema_neon's edge over baseline is significant but tiny (0 ns, 0.42%)

ema_neon differs from baseline ema_autovec by 0 ns (0.42%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ema_scalar** at 30.0 ns median (-49.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.98x (fastest 30.0 ns, slowest 59.3 ns)

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
| Counter frequency | 2112807538 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_autovec | 2604ns | 1943ns | 1589ns | 1841ns | 5908ns | base |
| ema_neon | 2686ns | 1945ns | 1602ns | 1949ns | 5983ns | +3.17% |
| ema_scalar | 2553ns | 1574ns | 1568ns | 1751ns | 5942ns | -1.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 79ns | 48ns | 181ns | base | 3.222 |
| ema_neon | 81ns | 49ns | 180ns | +2.56% | 3.142 |
| ema_scalar | 49ns | 29ns | 113ns | -38.93% | 5.277 |

## Performance model

- Peak throughput: **8.691 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 4.328 | 49.8% |
| ema_neon | 4.313 | 49.6% |
| ema_scalar | 8.533 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2604ns | 2604ns | base |
| ema_neon | 2686ns | 2686ns | +3.17% |
| ema_scalar | 2553ns | 2553ns | -1.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 59ns | base | --- | [59, 59] | --- | --- | --- | --- |
| ema_neon | 59ns | +0.2ns (+0.4%) | [+0, +0]ns | [59, 59] | YES | 0.0010 | 0.0010 | 11 |
| ema_scalar | 30ns | -29.2ns (-49.4%) | [-29, -27]ns | [30, 30] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 48ns | +22.7% | -24.6% |
| 1 | 2 | 59ns | +0.5% | -49.5% |
| 1 | 3 | 59ns | -18.4% | -49.7% |
| 1 | 4 | 59ns | +0.3% | -49.4% |
| 1 | 5 | 48ns | +22.2% | -20.1% |
| 1 | 6 | 59ns | -0.2% | -49.8% |
| 1 | 7 | 48ns | +22.9% | -38.1% |
| 1 | 8 | 49ns | -0.2% | -39.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.130 | ok |
| ema_neon | -0.225 | moderate- |
| ema_scalar | -0.227 | moderate- |

**Consistency summary:**

- **ema_neon**: won 3/8, lost 5/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 32.1ns | 79.4ns | 40.4% | HIGH |
| ema_neon | 32.9ns | 81.5ns | 40.4% | HIGH |
| ema_scalar | 31.7ns | 48.5ns | 65.3% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 48.4-180.7 ns)
     48.4 |###########################
     55.0 |########################################
     61.6 |#
     68.2 |
     74.8 |
     81.4 |
     88.1 |
     94.7 |
    101.3 |
    107.9 |
    114.5 |
    121.2 |
    127.8 |
    134.4 |
    141.0 |
    147.6 |#
    154.2 |#
    160.9 |##
    167.5 |
    174.1 |#
  (16 below, 16 above range)

ema_neon (n=160, range 48.8-180.0 ns)
     48.8 |
     55.4 |########################################
     61.9 |
     68.5 |
     75.0 |
     81.6 |
     88.2 |
     94.7 |
    101.3 |
    107.8 |
    114.4 |
    120.9 |
    127.5 |
    134.1 |
    140.6 |
    147.2 |#
    153.7 |
    160.3 |#
    166.8 |
    173.4 |#
  (31 below, 17 above range)

ema_scalar (n=160, range 29.5-112.7 ns)
     29.5 |########################################
     33.6 |########
     37.8 |
     42.0 |
     46.1 |
     50.3 |
     54.4 |
     58.6 |
     62.8 |
     66.9 |
     71.1 |
     75.3 |
     79.4 |
     83.6 |
     87.8 |
     91.9 |###
     96.1 |#
    100.3 |#
    104.4 |
    108.6 |##
  (14 below, 11 above range)

```

## Diagnostics

- **ema_autovec**: CV=69.4% (high variance, measurements may be unstable)
- **ema_autovec**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **ema_autovec**: bridge=40.2% of algo (FFI overhead may distort results)
- **ema_neon**: CV=63.9% (high variance, measurements may be unstable)
- **ema_neon**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **ema_neon**: bridge=40.1% of algo (FFI overhead may distort results)
- **ema_scalar**: CV=70.9% (high variance, measurements may be unstable)
- **ema_scalar**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **ema_scalar**: bridge=65.7% of algo (FFI overhead may distort results)
