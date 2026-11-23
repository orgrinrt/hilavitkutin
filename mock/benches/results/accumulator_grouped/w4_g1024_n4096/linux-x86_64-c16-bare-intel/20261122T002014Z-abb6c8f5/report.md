# W = 4, G = 1024

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is fastest but the noisiest (CV 32.8%)

sequential wins on median (22.25 us) yet has the highest variance (CV 32.8%), while grouped is the steadiest (CV 9.8%, 23.62 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential shows alternating (throttle bounce) (autocorr -0.64)

sequential's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.43 us) is smaller than the fastest variant's own run-to-run std-dev (7.30 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader sequential vs stability leader grouped (+6% speed for 3.3x steadier)

sequential is fastest (22.25 us, CV 32.8%); grouped gives up 6.2% median for 3.3x lower variance (CV 9.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### per_core is inconsistent: worst-20% is 2.0x its best-20%

per_core's best 20% of batches run at 15.74 us but its worst 20% at 30.92 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 22252.9 ns median (-5.8% vs baseline)
- Spread: 1.06x (fastest 22252.9 ns, slowest 23680.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 1024 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112752809 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 25037ns | 24516ns | 21970ns | 24233ns | 27777ns | base |
| per_core | 25021ns | 24583ns | 16343ns | 22795ns | 32699ns | -0.07% |
| sequential | 26462ns | 23891ns | 19475ns | 22919ns | 35269ns | +5.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 23962ns | 21244ns | 26606ns | base | 0.171 |
| per_core | 23734ns | 15740ns | 30924ns | -0.95% | 0.173 |
| sequential | 25279ns | 18864ns | 33697ns | +5.49% | 0.162 |

## Performance model

- Peak throughput: **0.260 Gops/s** (per_core; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.173 | 66.6% |
| per_core | 0.173 | 66.5% |
| sequential | 0.184 | 70.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 25037ns | 25037ns | base |
| per_core | 25021ns | 25021ns | -0.07% |
| sequential | 26462ns | 26462ns | +5.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 23622ns | base | --- | [22241, 25203] | --- | --- | --- | --- |
| per_core | 23680ns | no significant difference | [-4678, +5468]ns | [18783, 28344] | no | 0.8145 | 0.4807 | 0 |
| sequential | 22253ns | no significant difference | [-3158, +3766]ns | [20405, 28201] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 22165ns | +37.7% | -3.5% |
| 1 | 2 | 23782ns | -5.5% | +25.6% |
| 1 | 3 | 25517ns | -35.5% | -25.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.000 | ok |
| per_core | -0.005 | ok |
| sequential | -0.639 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 24865.6ns | 23962.3ns | 103.8% | HIGH |
| per_core | 23720.1ns | 23733.7ns | 99.9% | HIGH |
| sequential | 26546.0ns | 25279.0ns | 105.0% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 21244.3-26606.1 ns)
  21244.3 |
  21512.4 |
  21780.5 |
  22048.6 |########################################
  22316.7 |########
  22584.7 |
  22852.8 |
  23120.9 |
  23389.0 |########
  23657.1 |########
  23925.2 |########
  24193.3 |
  24461.4 |
  24729.5 |
  24997.5 |########################
  25265.6 |
  25533.7 |########
  25801.8 |
  26069.9 |########
  26338.0 |
  (2 below, 2 above range)

per_core (n=18, range 15740.4-30923.8 ns)
  15740.4 |####################
  16499.6 |########################################
  17258.8 |
  18017.9 |
  18777.1 |
  19536.3 |
  20295.4 |########################################
  21054.6 |####################
  21813.8 |
  22572.9 |
  23332.1 |########################################
  24091.3 |####################
  24850.5 |
  25609.6 |####################
  26368.8 |####################
  27128.0 |
  27887.1 |
  28646.3 |
  29405.5 |####################
  30164.6 |
  (2 below, 4 above range)

sequential (n=18, range 18864.5-33697.1 ns)
  18864.5 |##########################
  19606.1 |#############
  20347.8 |########################################
  21089.4 |#############
  21831.0 |
  22572.6 |#############
  23314.3 |#############
  24055.9 |
  24797.5 |
  25539.2 |
  26280.8 |#############
  27022.4 |#############
  27764.0 |
  28505.7 |
  29247.3 |##########################
  29988.9 |
  30730.6 |
  31472.2 |
  32213.8 |
  32955.4 |#############
  (2 below, 2 above range)

```

## Diagnostics

- **grouped**: bridge=102.3% of algo (FFI overhead may distort results)
- **per_core**: CV=25.5% (high variance, measurements may be unstable)
- **per_core**: bridge=96.8% of algo (FFI overhead may distort results)
- **sequential**: CV=28.9% (high variance, measurements may be unstable)
- **sequential**: bridge=101.0% of algo (FFI overhead may distort results)
