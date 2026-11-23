# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_scalar dominates: 66% faster than the next best (ema_autovec)

ema_scalar (1.76 us) leads ema_autovec (2.92 us) by 66%, a clear separation rather than a photo finish. CV 19.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_scalar beats baseline by 40% (significant)

ema_scalar is -1.16 us (40%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_scalar is fastest but the noisiest (CV 19.4%)

ema_scalar wins on median (1.76 us) yet has the highest variance (CV 19.4%), while ema_neon is the steadiest (CV 7.9%, 2.94 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ema_neon's edge over baseline is significant but tiny (10 ns, 0.36%)

ema_neon differs from baseline ema_autovec by 10 ns (0.36%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ema_scalar** at 1760.0 ns median (-39.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.67x (fastest 1760.0 ns, slowest 2935.5 ns)

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
| ema_autovec | 4623ns | 4833ns | 3956ns | 4765ns | 4866ns | base |
| ema_neon | 4640ns | 4842ns | 3965ns | 4779ns | 4898ns | +0.36% |
| ema_scalar | 3777ns | 3640ns | 3633ns | 3642ns | 4324ns | -18.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 2796ns | 2394ns | 2938ns | base | 5.860 |
| ema_neon | 2811ns | 2404ns | 2963ns | +0.56% | 5.828 |
| ema_scalar | 1826ns | 1756ns | 2092ns | -34.69% | 8.973 |

## Performance model

- Peak throughput: **9.330 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 5.602 | 60.0% |
| ema_neon | 5.581 | 59.8% |
| ema_scalar | 9.309 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 4623ns | 4623ns | base |
| ema_neon | 4640ns | 4640ns | +0.36% |
| ema_scalar | 3777ns | 3777ns | -18.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 2925ns | base | --- | [2925, 2925] | --- | --- | --- | --- |
| ema_neon | 2936ns | +10.5ns (+0.4%) | [+10, +11]ns | [2935, 2936] | YES | 0.0000 | 0.0000 | 1 |
| ema_scalar | 1760ns | -1164.4ns (-39.8%) | [-1165, -1164]ns | [1760, 1760] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 2925ns | +0.4% | -39.8% |
| 1 | 2 | 2393ns | +0.4% | -26.5% |
| 1 | 3 | 2925ns | -17.7% | -39.9% |
| 1 | 4 | 2924ns | +0.4% | -39.8% |
| 1 | 5 | 2393ns | +22.7% | -26.4% |
| 1 | 6 | 2928ns | +0.3% | -39.9% |
| 1 | 7 | 2925ns | +0.4% | -39.9% |
| 1 | 8 | 2926ns | +0.4% | -39.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.375 | moderate- |
| ema_neon | 0.291 | moderate+ |
| ema_scalar | -0.225 | moderate- |

**Consistency summary:**

- **ema_neon**: won 1/8, lost 7/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 22.6ns | 2795.7ns | 0.8% |  |
| ema_neon | 22.3ns | 2811.2ns | 0.8% |  |
| ema_scalar | 24.9ns | 1825.9ns | 1.4% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 2393.6-2938.4 ns)
   2393.6 |####
   2420.8 |
   2448.1 |
   2475.3 |
   2502.6 |
   2529.8 |
   2557.0 |
   2584.3 |
   2611.5 |
   2638.8 |
   2666.0 |
   2693.3 |
   2720.5 |
   2747.7 |
   2775.0 |
   2802.2 |
   2829.5 |
   2856.7 |
   2884.0 |
   2911.2 |########################################
  (28 below, 9 above range)

ema_neon (n=160, range 2403.8-2963.0 ns)
   2403.8 |####
   2431.8 |
   2459.8 |
   2487.7 |#
   2515.7 |
   2543.6 |
   2571.6 |
   2599.6 |
   2627.5 |
   2655.5 |
   2683.4 |
   2711.4 |
   2739.4 |
   2767.3 |
   2795.3 |
   2823.2 |
   2851.2 |
   2879.1 |
   2907.1 |########
   2935.1 |########################################
  (25 below, 5 above range)

ema_scalar (n=160, range 1756.0-2092.3 ns)
   1756.0 |########################################
   1772.9 |##
   1789.7 |
   1806.5 |
   1823.3 |
   1840.1 |
   1856.9 |
   1873.7 |
   1890.5 |
   1907.4 |
   1924.2 |
   1941.0 |
   1957.8 |
   1974.6 |
   1991.4 |
   2008.2 |
   2025.0 |
   2041.9 |
   2058.7 |
   2075.5 |
  (15 below, 6 above range)

```
