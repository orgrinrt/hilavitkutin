# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_scalar is an outlier: 7.2x slower than the field

ema_scalar (3 ns) is 7.2x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ema_autovec, ema_neon) are a dead heat (<1%)

ema_autovec (0 ns) and ema_neon (0 ns) differ by 0.00%, inside the noise, even though the wider field spreads 625.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ema_autovec shows alternating (throttle bounce) (autocorr -0.51)

ema_autovec's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ema_autovec)

The baseline ema_autovec is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### ema_neon's comparison is tie-heavy (35% tied pairs)

35% of paired samples for ema_neon are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Wide spread: slowest is 7.2x the fastest

Fastest ema_autovec (0 ns) to slowest ema_scalar (3 ns): 7.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ema_scalar is inconsistent: worst-20% is 2.9x its best-20%

ema_scalar's best 20% of batches run at 2 ns but its worst 20% at 5 ns (2.9x) - a bimodal or bursty profile the median hides.

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
| ema_autovec | 2087ns | 2082ns | 2075ns | 2082ns | 2114ns | base |
| ema_neon | 2103ns | 2086ns | 2074ns | 2092ns | 2166ns | +0.80% |
| ema_scalar | 2096ns | 2084ns | 2077ns | 2086ns | 2144ns | +0.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 0ns | 0ns | 1ns | base | 224.561 |
| ema_neon | 0ns | 0ns | 1ns | +23.90% | 181.239 |
| ema_scalar | 3ns | 2ns | 5ns | +934.21% | 21.713 |

## Performance model

- Peak throughput: **41.124 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 160.000 | 389.1% |
| ema_neon | 160.000 | 389.1% |
| ema_scalar | 22.069 | 53.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2087ns | 2087ns | base |
| ema_neon | 2103ns | 2103ns | +0.80% |
| ema_scalar | 2096ns | 2096ns | +0.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| ema_neon | 0ns | no significant difference | [+0, +0]ns | [0, 0] | no | 0.4926 | 0.4926 | **56** (35%, HIGH) |
| ema_scalar | 3ns | +2.5ns (+625.0%) | [+2, +3]ns | [2, 3] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 0ns | +100.0% | +1450.0% |
| 1 | 2 | 0ns | +0.0% | +575.0% |
| 1 | 3 | 0ns | -100.0% | +525.0% |
| 1 | 4 | 0ns | +0.0% | +0.0% |
| 1 | 5 | 0ns | +0.0% | +525.0% |
| 1 | 6 | 0ns | +100.0% | +1750.0% |
| 1 | 7 | 0ns | +0.0% | +525.0% |
| 1 | 8 | 0ns | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.511 | HIGH- (thermal bounce) |
| ema_neon | -0.161 | ok |
| ema_scalar | -0.310 | moderate- |

**Consistency summary:**

- **ema_neon**: won 1/8, lost 2/8
- **ema_scalar**: won 0/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 2.3ns | 0.3ns | 811.6% | HIGH |
| ema_neon | 2.5ns | 0.4ns | 706.2% | HIGH |
| ema_scalar | 2.3ns | 2.9ns | 79.6% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 0.0-0.8 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.4 |
      0.4 |##############################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.7 |
  (0 below, 24 above range)

ema_neon (n=160, range 0.0-1.0 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.4 |#############################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |################
      0.8 |
      0.9 |
      0.9 |
  (0 below, 11 above range)

ema_scalar (n=160, range 1.6-4.5 ns)
      1.6 |#######################
      1.7 |
      1.9 |
      2.0 |###################
      2.2 |
      2.3 |
      2.5 |########################################
      2.6 |
      2.8 |###########################
      2.9 |
      3.0 |
      3.2 |#######################
      3.3 |
      3.5 |
      3.6 |#######################
      3.8 |
      3.9 |
      4.1 |################
      4.2 |
      4.4 |
  (10 below, 16 above range)

```

## Diagnostics

- **ema_autovec**: CV=115.7% (high variance, measurements may be unstable)
- **ema_autovec**: bridge=525.0% of algo (FFI overhead may distort results)
- **ema_neon**: CV=108.3% (high variance, measurements may be unstable)
- **ema_neon**: bridge=625.0% of algo (FFI overhead may distort results)
- **ema_scalar**: CV=36.5% (high variance, measurements may be unstable)
- **ema_scalar**: bridge=79.3% of algo (FFI overhead may distort results)
