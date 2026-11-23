# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_scalar dominates: 66% faster than the next best (ema_autovec)

ema_scalar (118 ns) leads ema_autovec (196 ns) by 66%, a clear separation rather than a photo finish. CV 87.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_scalar beats baseline by 40% (significant)

ema_scalar is -78 ns (40%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_scalar is fastest but the noisiest (CV 87.9%)

ema_scalar wins on median (118 ns) yet has the highest variance (CV 87.9%), while ema_autovec is the steadiest (CV 77.3%, 196 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (79 ns) is smaller than the fastest variant's own run-to-run std-dev (103 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### ema_scalar is inconsistent: worst-20% is 3.8x its best-20%

ema_scalar's best 20% of batches run at 96 ns but its worst 20% at 363 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### ema_neon's edge over baseline is significant but tiny (1 ns, 0.51%)

ema_neon differs from baseline ema_autovec by 1 ns (0.51%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ema_scalar** at 117.7 ns median (-39.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.67x (fastest 117.7 ns, slowest 196.7 ns)

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
| ema_autovec | 2736ns | 2080ns | 1729ns | 2082ns | 5705ns | base |
| ema_neon | 2776ns | 2076ns | 1699ns | 2035ns | 6077ns | +1.48% |
| ema_scalar | 2764ns | 1992ns | 1631ns | 1999ns | 6192ns | +1.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 256ns | 163ns | 531ns | base | 3.993 |
| ema_neon | 262ns | 161ns | 572ns | +2.23% | 3.906 |
| ema_scalar | 163ns | 96ns | 363ns | -36.57% | 6.296 |

## Performance model

- Peak throughput: **10.640 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 5.235 | 49.2% |
| ema_neon | 5.206 | 48.9% |
| ema_scalar | 8.700 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2736ns | 2736ns | base |
| ema_neon | 2776ns | 2776ns | +1.48% |
| ema_scalar | 2764ns | 2764ns | +1.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 196ns | base | --- | [196, 196] | --- | --- | --- | --- |
| ema_neon | 197ns | +1.0ns (+0.5%) | [+1, +1]ns | [197, 197] | YES | 0.0002 | 0.0002 | 0 |
| ema_scalar | 118ns | -78.0ns (-39.9%) | [-78, -78]ns | [118, 118] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 196ns | +0.6% | -39.7% |
| 1 | 2 | 196ns | +0.4% | -50.8% |
| 1 | 3 | 196ns | +0.6% | -39.8% |
| 1 | 4 | 160ns | +22.7% | -40.0% |
| 1 | 5 | 196ns | -17.7% | -39.9% |
| 1 | 6 | 196ns | -17.9% | -40.0% |
| 1 | 7 | 160ns | +0.5% | -26.4% |
| 1 | 8 | 196ns | +0.6% | -39.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.374 | moderate- |
| ema_neon | 0.392 | moderate+ |
| ema_scalar | -0.379 | moderate- |

**Consistency summary:**

- **ema_neon**: won 2/8, lost 6/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 31.3ns | 256.4ns | 12.2% | HIGH |
| ema_neon | 31.6ns | 262.1ns | 12.0% | HIGH |
| ema_scalar | 33.0ns | 162.7ns | 20.3% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 163.0-531.0 ns)
    163.0 |
    181.4 |########################################
    199.8 |#
    218.2 |
    236.6 |
    255.0 |
    273.4 |
    291.8 |
    310.2 |
    328.6 |
    347.0 |
    365.4 |
    383.8 |
    402.2 |
    420.6 |
    439.0 |
    457.4 |
    475.8 |
    494.2 |
    512.6 |
  (28 below, 21 above range)

ema_neon (n=160, range 160.9-572.3 ns)
    160.9 |##############
    181.5 |########################################
    202.1 |
    222.7 |
    243.2 |
    263.8 |
    284.4 |
    304.9 |
    325.5 |
    346.1 |
    366.6 |
    387.2 |
    407.8 |
    428.4 |
    448.9 |
    469.5 |
    490.1 |
    510.6 |
    531.2 |#
    551.8 |
  (16 below, 19 above range)

ema_scalar (n=160, range 96.2-362.7 ns)
     96.2 |######
    109.6 |########################################
    122.9 |#
    136.2 |
    149.5 |
    162.9 |
    176.2 |
    189.5 |
    202.8 |
    216.1 |
    229.5 |
    242.8 |
    256.1 |
    269.4 |
    282.7 |
    296.1 |
    309.4 |
    322.7 |##
    336.0 |
    349.4 |#
  (16 below, 17 above range)

```

## Diagnostics

- **ema_autovec**: CV=58.9% (high variance, measurements may be unstable)
- **ema_autovec**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **ema_autovec**: bridge=12.2% of algo (FFI overhead may distort results)
- **ema_neon**: CV=61.6% (high variance, measurements may be unstable)
- **ema_neon**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **ema_neon**: bridge=12.0% of algo (FFI overhead may distort results)
- **ema_scalar**: CV=63.6% (high variance, measurements may be unstable)
- **ema_scalar**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **ema_scalar**: bridge=20.3% of algo (FFI overhead may distort results)
