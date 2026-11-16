# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (load_aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline load_aligned has the worst median (2.56 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest load_byte_pack at 2.56 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 2.56 us to slowest 2.56 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### load_byte_pack shows alternating (throttle bounce) (autocorr -0.53)

load_byte_pack's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (64 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: load_byte_pack** at 2556.7 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 2556.7 ns, slowest 2559.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x2c8af49106b7ed53 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 4743ns | 4699ns | 4682ns | 4711ns | 4900ns | base |
| load_byte_pack | 4726ns | 4696ns | 4683ns | 4703ns | 4838ns | -0.36% |
| load_unaligned | 4720ns | 4696ns | 4681ns | 4706ns | 4802ns | -0.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 2582ns | 2553ns | 2663ns | base | 6.346 |
| load_byte_pack | 2577ns | 2552ns | 2648ns | -0.21% | 6.359 |
| load_unaligned | 2573ns | 2553ns | 2626ns | -0.34% | 6.367 |

## Performance model

- Peak throughput: **6.419 Gops/s** (load_byte_pack; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 6.401 | 99.7% |
| load_byte_pack | 6.408 | 99.8% |
| load_unaligned | 6.403 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 4743ns | 4743ns | base |
| load_byte_pack | 4726ns | 4726ns | -0.36% |
| load_unaligned | 4720ns | 4720ns | -0.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 2560ns | base | --- | [2558, 2562] | --- | --- | --- | --- |
| load_byte_pack | 2557ns | no significant difference | [-3, +0]ns | [2556, 2558] | no | 0.0728 | 0.0364 | 5 |
| load_unaligned | 2559ns | no significant difference | [-2, +1]ns | [2557, 2561] | no | 0.3785 | 0.3785 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 2557ns | -0.0% | +0.0% |
| 1 | 2 | 2557ns | +0.1% | +0.6% |
| 1 | 3 | 2556ns | -0.0% | +0.1% |
| 1 | 4 | 2606ns | -1.5% | -1.8% |
| 1 | 5 | 2560ns | -0.1% | -0.1% |
| 1 | 6 | 2559ns | -0.1% | +0.0% |
| 1 | 7 | 2558ns | -0.1% | -0.0% |
| 1 | 8 | 2603ns | -1.7% | -1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | -0.204 | moderate- |
| load_byte_pack | -0.530 | HIGH- (thermal bounce) |
| load_unaligned | -0.263 | moderate- |

**Consistency summary:**

- **load_byte_pack**: won 3/8, lost 1/8
- **load_unaligned**: won 3/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 2.7ns | 2581.9ns | 0.1% |  |
| load_byte_pack | 2.9ns | 2576.5ns | 0.1% |  |
| load_unaligned | 2.6ns | 2573.2ns | 0.1% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 2553.0-2663.2 ns)
   2553.0 |########################################
   2558.5 |################
   2564.0 |######
   2569.6 |####
   2575.1 |##
   2580.6 |###
   2586.1 |
   2591.6 |##
   2597.1 |##
   2602.6 |###
   2608.1 |##
   2613.6 |
   2619.1 |##
   2624.6 |##
   2630.2 |
   2635.7 |
   2641.2 |
   2646.7 |
   2652.2 |
   2657.7 |
  (12 below, 11 above range)

load_byte_pack (n=160, range 2552.2-2647.6 ns)
   2552.2 |########################################
   2557.0 |##############
   2561.8 |##
   2566.5 |##
   2571.3 |##
   2576.1 |#
   2580.8 |#
   2585.6 |#
   2590.4 |##
   2595.1 |###
   2599.9 |#
   2604.7 |#
   2609.4 |#
   2614.2 |
   2619.0 |
   2623.7 |
   2628.5 |
   2633.3 |
   2638.0 |
   2642.8 |
  (13 below, 10 above range)

load_unaligned (n=160, range 2553.1-2625.8 ns)
   2553.1 |########################################
   2556.7 |########################
   2560.4 |#########
   2564.0 |########
   2567.6 |###
   2571.3 |##
   2574.9 |##
   2578.5 |#
   2582.2 |##
   2585.8 |##
   2589.4 |##
   2593.1 |####
   2596.7 |
   2600.4 |
   2604.0 |
   2607.6 |#####
   2611.3 |
   2614.9 |
   2618.5 |
   2622.2 |
  (17 below, 13 above range)

```
