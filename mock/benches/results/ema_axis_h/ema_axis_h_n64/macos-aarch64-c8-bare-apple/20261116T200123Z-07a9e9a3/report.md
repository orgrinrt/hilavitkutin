# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_scalar is an outlier: 7.2x slower than the field

ema_scalar (3 ns) is 7.2x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_autovec is fastest but the noisiest (CV 96.7%)

ema_autovec wins on median (0 ns) yet has the highest variance (CV 96.7%), while ema_scalar is the steadiest (CV 34.6%, 3 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (ema_autovec, ema_neon) are a dead heat (<1%)

ema_autovec (0 ns) and ema_neon (0 ns) differ by 0.00%, inside the noise, even though the wider field spreads 625.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ema_neon shows alternating (throttle bounce) (autocorr -0.67)

ema_neon's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ema_autovec)

The baseline ema_autovec is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### ema_neon's comparison is tie-heavy (40% tied pairs)

40% of paired samples for ema_neon are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Wide spread: slowest is 7.2x the fastest

Fastest ema_autovec (0 ns) to slowest ema_scalar (3 ns): 7.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ema_scalar is inconsistent: worst-20% is 3.5x its best-20%

ema_scalar's best 20% of batches run at 1 ns but its worst 20% at 4 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (ema_autovec) is the fastest** at 0.4 ns median
- 1 variant significantly slower than baseline
- Spread: 7.25x (fastest 0.4 ns, slowest 2.9 ns)

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
| ema_autovec | 2124ns | 2095ns | 2086ns | 2101ns | 2230ns | base |
| ema_neon | 2115ns | 2095ns | 2089ns | 2101ns | 2184ns | -0.41% |
| ema_scalar | 2116ns | 2098ns | 2092ns | 2103ns | 2179ns | -0.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 0ns | 0ns | 1ns | base | 196.923 |
| ema_neon | 0ns | 0ns | 1ns | -2.88% | 202.772 |
| ema_scalar | 3ns | 1ns | 4ns | +713.08% | 24.219 |

## Performance model

- Peak throughput: **56.110 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 160.000 | 285.2% |
| ema_neon | 160.000 | 285.2% |
| ema_scalar | 22.069 | 39.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2124ns | 2124ns | base |
| ema_neon | 2115ns | 2115ns | -0.41% |
| ema_scalar | 2116ns | 2116ns | -0.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| ema_neon | 0ns | no significant difference | [+0, +0]ns | [0, 0] | no | 1.0000 | 1.0000 | **64** (40%, HIGH) |
| ema_scalar | 3ns | +2.5ns (+625.0%) | [+2, +2]ns | [2, 3] | YES | 0.0000 | 0.0000 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 0ns | +0.0% | +525.0% |
| 1 | 2 | 0ns | +0.0% | +0.0% |
| 1 | 3 | 0ns | +0.0% | +475.0% |
| 1 | 4 | 0ns | -100.0% | +575.0% |
| 1 | 5 | 0ns | +100.0% | +1350.0% |
| 1 | 6 | 0ns | -50.0% | +525.0% |
| 1 | 7 | 0ns | +0.0% | +0.0% |
| 1 | 8 | 0ns | -100.0% | +625.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.598 | HIGH- (thermal bounce) |
| ema_neon | -0.667 | HIGH- (thermal bounce) |
| ema_scalar | 0.111 | ok |

**Consistency summary:**

- **ema_neon**: won 3/8, lost 1/8
- **ema_scalar**: won 0/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 2.9ns | 0.3ns | 889.0% | HIGH |
| ema_neon | 2.8ns | 0.3ns | 890.7% | HIGH |
| ema_scalar | 2.2ns | 2.6ns | 84.2% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 0.0-1.0 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.4 |###########################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |#############
      0.8 |
      0.9 |
      0.9 |
  (0 below, 7 above range)

ema_neon (n=160, range 0.0-0.9 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.4 |###########################
      0.4 |
      0.5 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |#############
      0.8 |
      0.9 |
  (0 below, 7 above range)

ema_scalar (n=160, range 1.1-4.0 ns)
      1.1 |################
      1.3 |
      1.4 |
      1.6 |#############
      1.7 |
      1.9 |
      2.0 |###########################
      2.1 |
      2.3 |
      2.4 |###################################
      2.6 |
      2.7 |
      2.8 |########################################
      3.0 |
      3.1 |
      3.3 |#################################
      3.4 |
      3.6 |#################
      3.7 |
      3.8 |
  (11 below, 15 above range)

```

## Diagnostics

- **ema_autovec**: CV=119.0% (high variance, measurements may be unstable)
- **ema_autovec**: bridge=725.0% of algo (FFI overhead may distort results)
- **ema_neon**: CV=113.5% (high variance, measurements may be unstable)
- **ema_neon**: bridge=725.0% of algo (FFI overhead may distort results)
- **ema_scalar**: CV=38.0% (high variance, measurements may be unstable)
- **ema_scalar**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **ema_scalar**: bridge=72.4% of algo (FFI overhead may distort results)
