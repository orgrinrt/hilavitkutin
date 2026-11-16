# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 5.27 us to slowest 5.27 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### memcpy_unrolled shows alternating (throttle bounce) (autocorr -0.67)

memcpy_unrolled's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (35 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: memcpy_unrolled** at 5265.2 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 5265.2 ns, slowest 5265.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4646464650504040 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 7384ns | 7367ns | 7358ns | 7368ns | 7460ns | base |
| memcpy_naive | 7373ns | 7366ns | 7357ns | 7367ns | 7409ns | -0.15% |
| memcpy_unrolled | 7377ns | 7367ns | 7357ns | 7368ns | 7427ns | -0.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 5277ns | 5258ns | 5331ns | base | 0.776 |
| memcpy_naive | 5272ns | 5259ns | 5302ns | -0.10% | 0.777 |
| memcpy_unrolled | 5274ns | 5258ns | 5316ns | -0.06% | 0.777 |

## Performance model

- Peak throughput: **0.779 Gops/s** (memcpy_intrinsic; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.778 | 99.9% |
| memcpy_naive | 0.778 | 99.8% |
| memcpy_unrolled | 0.778 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 7384ns | 7384ns | base |
| memcpy_naive | 7373ns | 7373ns | -0.15% |
| memcpy_unrolled | 7377ns | 7377ns | -0.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 5265ns | base | --- | [5264, 5267] | --- | --- | --- | --- |
| memcpy_naive | 5266ns | no significant difference | [-1, +2]ns | [5265, 5267] | no | 0.7497 | 0.7497 | 3 |
| memcpy_unrolled | 5265ns | no significant difference | [-1, +1]ns | [5265, 5266] | no | 0.7497 | 0.5728 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 5264ns | +0.0% | +0.1% |
| 1 | 2 | 5269ns | -0.1% | -0.2% |
| 1 | 3 | 5264ns | +0.0% | +0.1% |
| 1 | 4 | 5262ns | +0.1% | +0.1% |
| 1 | 5 | 5265ns | +0.0% | -0.1% |
| 1 | 6 | 5270ns | -0.1% | -0.0% |
| 1 | 7 | 5265ns | +0.1% | -0.0% |
| 1 | 8 | 5267ns | -0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.180 | ok |
| memcpy_naive | -0.128 | ok |
| memcpy_unrolled | -0.670 | HIGH- (thermal bounce) |

**Consistency summary:**

- **memcpy_naive**: won 0/8, lost 0/8
- **memcpy_unrolled**: won 1/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 3.4ns | 5277.2ns | 0.1% |  |
| memcpy_naive | 3.5ns | 5271.7ns | 0.1% |  |
| memcpy_unrolled | 3.5ns | 5274.2ns | 0.1% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 5257.7-5331.5 ns)
   5257.7 |#############################
   5261.4 |########################################
   5265.1 |############################
   5268.8 |####################
   5272.5 |#########
   5276.2 |####
   5279.8 |#
   5283.5 |
   5287.2 |##
   5290.9 |#
   5294.6 |
   5298.3 |#
   5302.0 |##
   5305.7 |###
   5309.4 |
   5313.0 |
   5316.7 |
   5320.4 |#
   5324.1 |#
   5327.8 |
  (13 below, 10 above range)

memcpy_naive (n=160, range 5258.5-5302.4 ns)
   5258.5 |##########################
   5260.7 |########################################
   5262.9 |#################################
   5265.1 |################################
   5267.3 |#####################
   5269.5 |########################
   5271.7 |#######
   5273.9 |###
   5276.1 |#
   5278.3 |####
   5280.5 |
   5282.7 |#
   5284.9 |#
   5287.1 |#
   5289.2 |#
   5291.4 |
   5293.6 |
   5295.8 |
   5298.0 |
   5300.2 |
  (12 below, 17 above range)

memcpy_unrolled (n=160, range 5257.7-5316.2 ns)
   5257.7 |###################
   5260.7 |#########################
   5263.6 |########################################
   5266.5 |####################
   5269.4 |##########
   5272.4 |########
   5275.3 |####
   5278.2 |##
   5281.1 |#
   5284.0 |#
   5287.0 |##
   5289.9 |
   5292.8 |#
   5295.7 |##
   5298.6 |#
   5301.6 |
   5304.5 |#
   5307.4 |
   5310.3 |#
   5313.3 |##
  (13 below, 8 above range)

```
