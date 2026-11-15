# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_scalar dominates: 66% faster than the next best (ema_autovec)

ema_scalar (118 ns) leads ema_autovec (196 ns) by 66%, a clear separation rather than a photo finish. CV 88.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_scalar beats baseline by 40% (significant)

ema_scalar is -78 ns (40%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_scalar is fastest but the noisiest (CV 88.0%)

ema_scalar wins on median (118 ns) yet has the highest variance (CV 88.0%), while ema_neon is the steadiest (CV 84.5%, 197 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (79 ns) is smaller than the fastest variant's own run-to-run std-dev (104 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### ema_autovec is inconsistent: worst-20% is 3.4x its best-20%

ema_autovec's best 20% of batches run at 178 ns but its worst 20% at 608 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### ema_neon's edge over baseline is significant but tiny (1 ns, 0.46%)

ema_neon differs from baseline ema_autovec by 1 ns (0.46%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ema_scalar** at 117.8 ns median (-39.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.67x (fastest 117.8 ns, slowest 196.8 ns)

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
| ema_autovec | 2943ns | 2079ns | 1883ns | 2134ns | 6428ns | base |
| ema_neon | 2905ns | 2077ns | 1885ns | 2091ns | 6366ns | -1.29% |
| ema_scalar | 2870ns | 1997ns | 1989ns | 2027ns | 6280ns | -2.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 279ns | 178ns | 608ns | base | 3.668 |
| ema_neon | 274ns | 179ns | 595ns | -1.94% | 3.741 |
| ema_scalar | 169ns | 117ns | 366ns | -39.61% | 6.074 |

## Performance model

- Peak throughput: **8.715 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 5.224 | 59.9% |
| ema_neon | 5.203 | 59.7% |
| ema_scalar | 8.693 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2943ns | 2943ns | base |
| ema_neon | 2905ns | 2905ns | -1.29% |
| ema_scalar | 2870ns | 2870ns | -2.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 196ns | base | --- | [196, 196] | --- | --- | --- | --- |
| ema_neon | 197ns | +0.9ns (+0.5%) | [+1, +1]ns | [197, 197] | YES | 0.0001 | 0.0001 | 1 |
| ema_scalar | 118ns | -78.2ns (-39.9%) | [-78, -78]ns | [118, 118] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 196ns | -17.7% | -39.7% |
| 1 | 2 | 160ns | +23.0% | -26.5% |
| 1 | 3 | 196ns | +0.6% | -39.8% |
| 1 | 4 | 196ns | +0.3% | -39.9% |
| 1 | 5 | 196ns | +0.4% | -39.8% |
| 1 | 6 | 208ns | -5.2% | -43.2% |
| 1 | 7 | 196ns | +0.5% | -39.8% |
| 1 | 8 | 196ns | +0.4% | -40.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.048 | ok |
| ema_neon | -0.019 | ok |
| ema_scalar | 0.203 | moderate+ |

**Consistency summary:**

- **ema_neon**: won 2/8, lost 6/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 33.5ns | 279.2ns | 12.0% | HIGH |
| ema_neon | 33.3ns | 273.7ns | 12.2% | HIGH |
| ema_scalar | 34.0ns | 168.6ns | 20.2% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 177.7-608.2 ns)
    177.7 |########################################
    199.2 |#########
    220.7 |
    242.3 |
    263.8 |
    285.3 |
    306.8 |
    328.4 |
    349.9 |
    371.4 |
    392.9 |
    414.4 |
    436.0 |
    457.5 |
    479.0 |
    500.5 |
    522.1 |#
    543.6 |#
    565.1 |
    586.6 |##
  (16 below, 17 above range)

ema_neon (n=160, range 178.8-595.3 ns)
    178.8 |########################################
    199.6 |#
    220.4 |
    241.2 |
    262.1 |
    282.9 |
    303.7 |
    324.5 |
    345.4 |
    366.2 |
    387.0 |
    407.8 |
    428.7 |
    449.5 |
    470.3 |
    491.1 |
    512.0 |
    532.8 |#
    553.6 |
    574.4 |#
  (16 below, 17 above range)

ema_scalar (n=160, range 117.5-366.2 ns)
    117.5 |########################################
    129.9 |#
    142.4 |
    154.8 |
    167.2 |
    179.7 |
    192.1 |
    204.5 |
    217.0 |
    229.4 |
    241.8 |
    254.3 |
    266.7 |
    279.1 |
    291.6 |
    304.0 |
    316.4 |
    328.9 |
    341.3 |
    353.7 |
  (10 below, 22 above range)

```

## Diagnostics

- **ema_autovec**: CV=60.3% (high variance, measurements may be unstable)
- **ema_autovec**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **ema_autovec**: bridge=12.1% of algo (FFI overhead may distort results)
- **ema_neon**: CV=60.8% (high variance, measurements may be unstable)
- **ema_neon**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **ema_neon**: bridge=12.0% of algo (FFI overhead may distort results)
- **ema_scalar**: CV=61.5% (high variance, measurements may be unstable)
- **ema_scalar**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **ema_scalar**: bridge=20.2% of algo (FFI overhead may distort results)
