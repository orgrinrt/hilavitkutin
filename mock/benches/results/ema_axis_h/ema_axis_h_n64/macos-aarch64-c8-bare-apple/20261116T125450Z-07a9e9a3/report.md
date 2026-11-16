# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_neon's comparison is tie-heavy (41% tied pairs)

41% of paired samples for ema_neon are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### ema_scalar is inconsistent: worst-20% is 3.2x its best-20%

ema_scalar's best 20% of batches run at 1 ns but its worst 20% at 4 ns (3.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.0% of the fastest

All 3 variants sit between 0 ns and 2 ns - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: ema_neon** at 0.0 ns median (-100.0% vs baseline)
- 1 variant significantly slower than baseline

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_autovec | 2096ns | 2092ns | 2087ns | 2093ns | 2114ns | base |
| ema_neon | 2096ns | 2092ns | 2084ns | 2091ns | 2123ns | +0.03% |
| ema_scalar | 2097ns | 2094ns | 2089ns | 2094ns | 2113ns | +0.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 0ns | 0ns | 1ns | base | 209.407 |
| ema_neon | 0ns | 0ns | 1ns | -8.18% | 228.062 |
| ema_scalar | 3ns | 1ns | 4ns | +776.69% | 23.886 |

## Performance model

- Peak throughput: **49.709 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 160.000 | 321.9% |
| ema_scalar | 25.600 | 51.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2096ns | 2096ns | base |
| ema_neon | 2096ns | 2096ns | +0.03% |
| ema_scalar | 2097ns | 2097ns | +0.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| ema_neon | 0ns | no significant difference | [+0, +0]ns | [0, 0] | no | 0.6817 | 0.6817 | **65** (41%, HIGH) |
| ema_scalar | 2ns | +2.1ns (+525.0%) | [+2, +2]ns | [2, 3] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 0ns | -100.0% | +525.0% |
| 1 | 2 | 0ns | -100.0% | +525.0% |
| 1 | 3 | 0ns | +0.0% | +475.0% |
| 1 | 4 | 0ns | +0.0% | +575.0% |
| 1 | 5 | 0ns | +0.0% | +0.0% |
| 1 | 6 | 0ns | -100.0% | +625.0% |
| 1 | 7 | 0ns | -100.0% | +625.0% |
| 1 | 8 | 0ns | -100.0% | +1150.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.230 | moderate- |
| ema_neon | 0.292 | moderate+ |
| ema_scalar | 0.094 | ok |

**Consistency summary:**

- **ema_neon**: won 5/8, lost 0/8
- **ema_scalar**: won 0/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 2.7ns | 0.3ns | 878.1% | HIGH |
| ema_neon | 2.8ns | 0.3ns | 999.8% | HIGH |
| ema_scalar | 2.5ns | 2.7ns | 92.7% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 0.0-0.8 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.3 |
      0.4 |################################
      0.4 |
      0.5 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |
      0.8 |##########
  (0 below, 7 above range)

ema_neon (n=160, range 0.0-0.8 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.3 |
      0.4 |######################
      0.4 |
      0.5 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |
      0.8 |############
  (0 below, 4 above range)

ema_scalar (n=160, range 1.3-4.2 ns)
      1.3 |
      1.4 |
      1.6 |######################################
      1.7 |
      1.9 |
      2.0 |####################################
      2.2 |
      2.3 |
      2.4 |##################################
      2.6 |
      2.7 |
      2.9 |###############################
      3.0 |
      3.2 |########################################
      3.3 |
      3.5 |
      3.6 |#############################
      3.7 |
      3.9 |
      4.0 |
  (20 below, 19 above range)

```

## Diagnostics

- **ema_autovec**: CV=112.7% (high variance, measurements may be unstable)
- **ema_autovec**: bridge=625.0% of algo (FFI overhead may distort results)
- **ema_neon**: CV=122.1% (high variance, measurements may be unstable)
- **ema_scalar**: CV=39.2% (high variance, measurements may be unstable)
- **ema_scalar**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **ema_scalar**: bridge=100.0% of algo (FFI overhead may distort results)
