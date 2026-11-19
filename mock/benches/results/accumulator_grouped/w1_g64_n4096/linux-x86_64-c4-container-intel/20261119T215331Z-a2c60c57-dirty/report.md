# W = 1, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (grouped, sequential) are a dead heat (<1%)

grouped (29.46 us) and sequential (29.64 us) differ by 0.59%, inside the noise, even though the wider field spreads 6.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### per_core shows alternating (throttle bounce) (autocorr -0.56)

per_core's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.85 us) is smaller than the fastest variant's own run-to-run std-dev (4.64 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (grouped)

The baseline grouped is the fastest (29.46 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader grouped vs stability leader sequential (+1% speed for 1.3x steadier)

grouped is fastest (29.46 us, CV 15.8%); sequential gives up 0.6% median for 1.3x lower variance (CV 11.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### per_core is inconsistent: worst-20% is 1.6x its best-20%

per_core's best 20% of batches run at 25.46 us but its worst 20% at 40.09 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (grouped) is the fastest** at 29463.8 ns median
- Spread: 1.06x (fastest 29463.8 ns, slowest 31316.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 1, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 31031ns | 29994ns | 25071ns | 29350ns | 36531ns | base |
| per_core | 33156ns | 31969ns | 25863ns | 30556ns | 40701ns | +6.85% |
| sequential | 30937ns | 30104ns | 26733ns | 29718ns | 34869ns | -0.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 30410ns | 24524ns | 35689ns | base | 0.135 |
| per_core | 32644ns | 25460ns | 40089ns | +7.35% | 0.125 |
| sequential | 30289ns | 26088ns | 34210ns | -0.40% | 0.135 |

## Performance model

- Peak throughput: **0.167 Gops/s** (grouped; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.139 | 83.2% |
| per_core | 0.131 | 78.3% |
| sequential | 0.138 | 82.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 31031ns | 31031ns | base |
| per_core | 33156ns | 33156ns | +6.85% |
| sequential | 30937ns | 30937ns | -0.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 29464ns | base | --- | [26992, 33449] | --- | --- | --- | --- |
| per_core | 31317ns | no significant difference | [-2191, +4048]ns | [27443, 36002] | no | 0.4758 | 0.2379 | 0 |
| sequential | 29638ns | no significant difference | [-4234, +5638]ns | [28065, 31269] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 33076ns | -21.6% | -9.1% |
| 1 | 2 | 34071ns | +8.0% | -19.3% |
| 1 | 3 | 25804ns | +17.7% | +24.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.234 | moderate- |
| per_core | -0.557 | HIGH- (thermal bounce) |
| sequential | -0.541 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 1/3, lost 2/3
- **sequential**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 32328.7ns | 30409.8ns | 106.3% | HIGH |
| per_core | 32542.1ns | 32644.3ns | 99.7% | HIGH |
| sequential | 30966.5ns | 30288.8ns | 102.2% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 24524.3-35688.7 ns)
  24524.3 |#############
  25082.5 |#############
  25640.7 |#############
  26199.0 |#############
  26757.2 |
  27315.4 |########################################
  27873.6 |
  28431.9 |#############
  28990.1 |
  29548.3 |
  30106.5 |#############
  30664.7 |
  31223.0 |
  31781.2 |#############
  32339.4 |
  32897.6 |##########################
  33455.8 |##########################
  34014.1 |
  34572.3 |
  35130.5 |
  (1 below, 3 above range)

per_core (n=18, range 25459.7-40089.4 ns)
  25459.7 |########################################
  26191.2 |####################
  26922.6 |
  27654.1 |####################
  28385.6 |####################
  29117.1 |
  29848.6 |########################################
  30580.1 |
  31311.6 |
  32043.1 |########################################
  32774.6 |
  33506.0 |
  34237.5 |
  34969.0 |####################
  35700.5 |########################################
  36432.0 |
  37163.5 |########################################
  37895.0 |
  38626.5 |
  39357.9 |
  (2 below, 2 above range)

sequential (n=18, range 26087.6-34209.8 ns)
  26087.6 |
  26493.7 |
  26899.8 |#############
  27305.9 |
  27712.0 |########################################
  28118.1 |
  28524.3 |##########################
  28930.4 |
  29336.5 |#############
  29742.6 |#############
  30148.7 |#############
  30554.8 |##########################
  30960.9 |
  31367.0 |#############
  31773.1 |
  32179.2 |
  32585.3 |
  32991.5 |#############
  33397.6 |
  33803.7 |
  (2 below, 3 above range)

```

## Diagnostics

- **grouped**: bridge=110.3% of algo (FFI overhead may distort results)
- **per_core**: CV=20.6% (high variance, measurements may be unstable)
- **per_core**: bridge=98.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=101.3% of algo (FFI overhead may distort results)
