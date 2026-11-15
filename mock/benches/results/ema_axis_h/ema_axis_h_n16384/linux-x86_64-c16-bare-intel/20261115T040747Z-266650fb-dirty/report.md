# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_scalar dominates: 66% faster than the next best (ema_autovec)

ema_scalar (1.76 us) leads ema_autovec (2.93 us) by 66%, a clear separation rather than a photo finish. CV 24.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_scalar beats baseline by 40% (significant)

ema_scalar is -1.17 us (40%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_scalar is fastest but the noisiest (CV 24.2%)

ema_scalar wins on median (1.76 us) yet has the highest variance (CV 24.2%), while ema_neon is the steadiest (CV 7.9%, 2.94 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: ema_scalar** at 1760.2 ns median (-39.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.67x (fastest 1760.2 ns, slowest 2935.4 ns)

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
| ema_autovec | 5010ns | 4835ns | 4823ns | 4836ns | 5716ns | base |
| ema_neon | 4644ns | 4848ns | 3966ns | 4787ns | 4896ns | -7.29% |
| ema_scalar | 3796ns | 3641ns | 3629ns | 3642ns | 4428ns | -24.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 3012ns | 2924ns | 3343ns | base | 5.440 |
| ema_neon | 2810ns | 2403ns | 2955ns | -6.70% | 5.831 |
| ema_scalar | 1833ns | 1755ns | 2130ns | -39.13% | 8.938 |

## Performance model

- Peak throughput: **9.337 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 5.599 | 60.0% |
| ema_neon | 5.582 | 59.8% |
| ema_scalar | 9.308 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 5010ns | 5010ns | base |
| ema_neon | 4644ns | 4644ns | -7.29% |
| ema_scalar | 3796ns | 3796ns | -24.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 2926ns | base | --- | [2926, 2936] | --- | --- | --- | --- |
| ema_neon | 2935ns | no significant difference | [-1, +10]ns | [2935, 2936] | no | 0.2356 | 0.2356 | 0 |
| ema_scalar | 1760ns | -1168.3ns (-39.9%) | [-1172, -1166]ns | [1760, 1760] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 2926ns | +0.3% | -39.8% |
| 1 | 2 | 2935ns | -18.2% | -40.0% |
| 1 | 3 | 2925ns | +0.4% | -39.8% |
| 1 | 4 | 3769ns | -36.3% | -53.3% |
| 1 | 5 | 2925ns | +0.4% | -39.8% |
| 1 | 6 | 2926ns | +0.3% | -39.8% |
| 1 | 7 | 2930ns | +0.2% | -39.9% |
| 1 | 8 | 2926ns | +0.3% | -39.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.168 | ok |
| ema_neon | -0.376 | moderate- |
| ema_scalar | 0.086 | ok |

**Consistency summary:**

- **ema_neon**: won 2/8, lost 6/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 24.8ns | 3011.7ns | 0.8% |  |
| ema_neon | 22.2ns | 2810.0ns | 0.8% |  |
| ema_scalar | 25.0ns | 1833.1ns | 1.4% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 2924.4-3342.7 ns)
   2924.4 |########################################
   2945.3 |###
   2966.2 |
   2987.1 |
   3008.0 |
   3029.0 |
   3049.9 |
   3070.8 |
   3091.7 |
   3112.6 |
   3133.5 |
   3154.4 |
   3175.4 |
   3196.3 |
   3217.2 |
   3238.1 |
   3259.0 |
   3279.9 |
   3300.8 |
   3321.8 |
  (7 below, 14 above range)

ema_neon (n=160, range 2403.0-2954.8 ns)
   2403.0 |###
   2430.6 |
   2458.1 |
   2485.7 |
   2513.3 |
   2540.9 |
   2568.5 |
   2596.1 |
   2623.7 |
   2651.3 |
   2678.9 |
   2706.5 |
   2734.1 |
   2761.6 |
   2789.2 |
   2816.8 |
   2844.4 |
   2872.0 |
   2899.6 |
   2927.2 |########################################
  (28 below, 2 above range)

ema_scalar (n=160, range 1754.8-2130.2 ns)
   1754.8 |########################################
   1773.5 |###
   1792.3 |
   1811.1 |
   1829.8 |
   1848.6 |
   1867.4 |
   1886.2 |
   1904.9 |
   1923.7 |
   1942.5 |
   1961.2 |
   1980.0 |
   1998.8 |
   2017.6 |
   2036.3 |
   2055.1 |
   2073.9 |
   2092.6 |
   2111.4 |
  (17 below, 5 above range)

```

## Diagnostics

- **ema_scalar**: CV=23.2% (high variance, measurements may be unstable)
