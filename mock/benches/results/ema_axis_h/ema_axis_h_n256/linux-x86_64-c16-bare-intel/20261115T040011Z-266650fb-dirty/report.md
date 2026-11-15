# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_scalar dominates: 62% faster than the next best (ema_autovec)

ema_scalar (36 ns) leads ema_autovec (59 ns) by 62%, a clear separation rather than a photo finish. CV 89.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_scalar beats baseline by 39% (significant)

ema_scalar is -23 ns (39%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (23 ns) is smaller than the fastest variant's own run-to-run std-dev (33 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### ema_neon's comparison is tie-heavy (16% tied pairs)

16% of paired samples for ema_neon are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### ema_scalar is inconsistent: worst-20% is 3.5x its best-20%

ema_scalar's best 20% of batches run at 33 ns but its worst 20% at 114 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_scalar** at 36.5 ns median (-38.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.62x (fastest 36.5 ns, slowest 59.3 ns)

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
| Counter frequency | 2112781281 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_autovec | 2809ns | 1939ns | 1932ns | 1995ns | 6126ns | base |
| ema_neon | 2895ns | 1942ns | 1780ns | 2166ns | 6198ns | +3.08% |
| ema_scalar | 2843ns | 1916ns | 1724ns | 2120ns | 6128ns | +1.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 86ns | 59ns | 186ns | base | 2.989 |
| ema_neon | 89ns | 54ns | 189ns | +3.50% | 2.888 |
| ema_scalar | 53ns | 33ns | 114ns | -38.40% | 4.852 |

## Performance model

- Peak throughput: **7.867 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 4.317 | 54.9% |
| ema_neon | 4.317 | 54.9% |
| ema_scalar | 7.014 | 89.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2809ns | 2809ns | base |
| ema_neon | 2895ns | 2895ns | +3.08% |
| ema_scalar | 2843ns | 2843ns | +1.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 59ns | base | --- | [59, 59] | --- | --- | --- | --- |
| ema_neon | 59ns | no significant difference | [+0, +0]ns | [59, 59] | no | 0.6660 | 0.6660 | **26** (16%, HIGH) |
| ema_scalar | 36ns | -23.1ns (-39.0%) | [-23, -23]ns | [36, 37] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 59ns | +0.2% | -49.9% |
| 1 | 2 | 59ns | -18.1% | -38.8% |
| 1 | 3 | 59ns | +0.2% | -38.2% |
| 1 | 4 | 59ns | +0.0% | -39.0% |
| 1 | 5 | 59ns | +0.1% | -38.3% |
| 1 | 6 | 59ns | -0.1% | -14.1% |
| 1 | 7 | 59ns | +0.1% | -38.7% |
| 1 | 8 | 59ns | +0.2% | -38.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | 0.363 | moderate+ |
| ema_neon | -0.156 | ok |
| ema_scalar | -0.051 | ok |

**Consistency summary:**

- **ema_neon**: won 1/8, lost 3/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 34.3ns | 85.7ns | 40.1% | HIGH |
| ema_neon | 35.3ns | 88.7ns | 39.8% | HIGH |
| ema_scalar | 35.7ns | 52.8ns | 67.7% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 59.1-186.5 ns)
     59.1 |########################################
     65.5 |
     71.8 |
     78.2 |
     84.6 |
     90.9 |
     97.3 |
    103.7 |
    110.0 |
    116.4 |
    122.8 |
    129.2 |
    135.5 |
    141.9 |
    148.3 |
    154.6 |
    161.0 |#
    167.4 |
    173.7 |#
    180.1 |
  (9 below, 17 above range)

ema_neon (n=160, range 54.4-188.9 ns)
     54.4 |########################################
     61.1 |
     67.9 |
     74.6 |
     81.3 |
     88.0 |
     94.8 |
    101.5 |
    108.2 |
    114.9 |
    121.7 |
    128.4 |
    135.1 |
    141.8 |
    148.6 |
    155.3 |
    162.0 |##
    168.7 |#
    175.4 |##
    182.2 |
  (14 below, 16 above range)

ema_scalar (n=160, range 32.5-113.6 ns)
     32.5 |########################################
     36.6 |#########
     40.6 |
     44.7 |
     48.7 |#########
     52.8 |
     56.8 |
     60.9 |
     64.9 |
     69.0 |
     73.0 |
     77.1 |
     81.2 |
     85.2 |
     89.3 |
     93.3 |
     97.4 |
    101.4 |##
    105.5 |#
    109.5 |
  (17 below, 20 above range)

```

## Diagnostics

- **ema_autovec**: CV=61.0% (high variance, measurements may be unstable)
- **ema_autovec**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **ema_autovec**: bridge=40.1% of algo (FFI overhead may distort results)
- **ema_neon**: CV=60.8% (high variance, measurements may be unstable)
- **ema_neon**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **ema_neon**: bridge=40.1% of algo (FFI overhead may distort results)
- **ema_scalar**: CV=61.7% (high variance, measurements may be unstable)
- **ema_scalar**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **ema_scalar**: bridge=65.8% of algo (FFI overhead may distort results)
