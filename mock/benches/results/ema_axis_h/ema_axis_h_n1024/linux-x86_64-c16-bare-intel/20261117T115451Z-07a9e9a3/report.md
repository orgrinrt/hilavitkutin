# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_scalar dominates: 66% faster than the next best (ema_autovec)

ema_scalar (118 ns) leads ema_autovec (196 ns) by 66%, a clear separation rather than a photo finish. CV 88.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_scalar beats baseline by 40% (significant)

ema_scalar is -78 ns (40%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec shows alternating (throttle bounce) (autocorr -0.89)

ema_autovec's per-pass series has lag-1 autocorrelation -0.89, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (79 ns) is smaller than the fastest variant's own run-to-run std-dev (104 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### ema_autovec is inconsistent: worst-20% is 4.8x its best-20%

ema_autovec's best 20% of batches run at 160 ns but its worst 20% at 768 ns (4.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### ema_neon's edge over baseline is significant but tiny (1 ns, 0.36%)

ema_neon differs from baseline ema_autovec by 1 ns (0.36%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ema_scalar** at 117.7 ns median (-39.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.67x (fastest 117.7 ns, slowest 196.6 ns)

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
| ema_autovec | 3259ns | 2082ns | 1703ns | 2135ns | 8187ns | base |
| ema_neon | 2782ns | 2083ns | 1703ns | 2010ns | 6177ns | -14.63% |
| ema_scalar | 2721ns | 2002ns | 1639ns | 2003ns | 5954ns | -16.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 306ns | 160ns | 768ns | base | 3.347 |
| ema_neon | 262ns | 161ns | 580ns | -14.39% | 3.909 |
| ema_scalar | 160ns | 96ns | 349ns | -47.81% | 6.412 |

## Performance model

- Peak throughput: **10.626 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 5.232 | 49.2% |
| ema_neon | 5.209 | 49.0% |
| ema_scalar | 8.700 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 3259ns | 3259ns | base |
| ema_neon | 2782ns | 2782ns | -14.63% |
| ema_scalar | 2721ns | 2721ns | -16.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 196ns | base | --- | [196, 196] | --- | --- | --- | --- |
| ema_neon | 197ns | +0.7ns (+0.4%) | [+0, +1]ns | [196, 197] | YES | 0.0000 | 0.0000 | 4 |
| ema_scalar | 118ns | -78.0ns (-39.9%) | [-78, -78]ns | [118, 118] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 163ns | +20.6% | -27.8% |
| 1 | 2 | 196ns | +0.3% | -39.9% |
| 1 | 3 | 160ns | +0.4% | -39.9% |
| 1 | 4 | 196ns | +0.1% | -40.0% |
| 1 | 5 | 160ns | +0.5% | -26.4% |
| 1 | 6 | 196ns | -17.8% | -39.9% |
| 1 | 7 | 161ns | +0.2% | -39.8% |
| 1 | 8 | 196ns | +0.4% | -39.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.888 | HIGH- (thermal bounce) |
| ema_neon | -0.124 | ok |
| ema_scalar | -0.372 | moderate- |

**Consistency summary:**

- **ema_neon**: won 1/8, lost 7/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 37.1ns | 306.0ns | 12.1% | HIGH |
| ema_neon | 33.5ns | 261.9ns | 12.8% | HIGH |
| ema_scalar | 32.7ns | 159.7ns | 20.5% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 160.0-768.4 ns)
    160.0 |##############################
    190.4 |########################################
    220.8 |
    251.2 |
    281.7 |
    312.1 |
    342.5 |#
    372.9 |#
    403.3 |
    433.8 |
    464.2 |###
    494.6 |###
    525.0 |###
    555.4 |
    585.9 |#
    616.3 |
    646.7 |
    677.1 |#
    707.5 |
    738.0 |#
  (9 below, 6 above range)

ema_neon (n=160, range 160.8-580.0 ns)
    160.8 |#########################
    181.8 |########################################
    202.8 |##
    223.7 |
    244.7 |
    265.6 |
    286.6 |
    307.5 |
    328.5 |
    349.4 |
    370.4 |
    391.4 |
    412.3 |
    433.3 |
    454.2 |#
    475.2 |
    496.1 |#
    517.1 |
    538.0 |###
    559.0 |###
  (15 below, 14 above range)

ema_scalar (n=160, range 96.4-348.6 ns)
     96.4 |######
    109.0 |########################################
    121.6 |#
    134.2 |
    146.8 |
    159.4 |
    172.0 |
    184.6 |
    197.2 |
    209.9 |
    222.5 |
    235.1 |
    247.7 |
    260.3 |
    272.9 |
    285.5 |
    298.1 |
    310.7 |
    323.4 |#
    336.0 |
  (17 below, 21 above range)

```

## Diagnostics

- **ema_autovec**: CV=102.1% (high variance, measurements may be unstable)
- **ema_autovec**: worst_20/best_20 = 4.8x (possible bimodal distribution)
- **ema_autovec**: bridge=12.2% of algo (FFI overhead may distort results)
- **ema_neon**: CV=62.6% (high variance, measurements may be unstable)
- **ema_neon**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **ema_neon**: bridge=12.1% of algo (FFI overhead may distort results)
- **ema_scalar**: CV=64.9% (high variance, measurements may be unstable)
- **ema_scalar**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **ema_scalar**: bridge=20.3% of algo (FFI overhead may distort results)
