# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.8% (fastest 2.61 us to slowest 2.69 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (2.61 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (load_aligned) is the fastest** at 2611.0 ns median
- Spread: 1.03x (fastest 2611.0 ns, slowest 2685.0 ns)

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
| load_aligned | 4796ns | 4781ns | 4599ns | 4804ns | 4966ns | base |
| load_byte_pack | 4854ns | 4863ns | 4675ns | 4853ns | 5038ns | +1.22% |
| load_unaligned | 4845ns | 4921ns | 4604ns | 4857ns | 5051ns | +1.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 2612ns | 2507ns | 2695ns | base | 6.272 |
| load_byte_pack | 2645ns | 2548ns | 2741ns | +1.25% | 6.194 |
| load_unaligned | 2640ns | 2507ns | 2753ns | +1.05% | 6.207 |

## Performance model

- Peak throughput: **6.536 Gops/s** (load_aligned; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 6.275 | 96.0% |
| load_byte_pack | 6.182 | 94.6% |
| load_unaligned | 6.102 | 93.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 4796ns | 4796ns | base |
| load_byte_pack | 4854ns | 4854ns | +1.22% |
| load_unaligned | 4845ns | 4845ns | +1.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 2611ns | base | --- | [2596, 2641] | --- | --- | --- | --- |
| load_byte_pack | 2650ns | no significant difference | [-1, +42]ns | [2608, 2685] | no | 0.2326 | 0.2326 | 2 |
| load_unaligned | 2685ns | no significant difference | [-0, +70]ns | [2637, 2687] | no | 0.1374 | 0.0687 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 2604ns | -0.3% | +3.2% |
| 1 | 2 | 2594ns | +3.4% | -1.3% |
| 1 | 3 | 2508ns | +5.4% | +7.2% |
| 1 | 4 | 2692ns | -0.1% | +0.0% |
| 1 | 5 | 2686ns | -5.2% | -6.6% |
| 1 | 6 | 2681ns | +0.1% | -6.4% |
| 1 | 7 | 2509ns | +7.1% | +7.1% |
| 1 | 8 | 2687ns | -2.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | -0.231 | moderate- |
| load_byte_pack | -0.485 | moderate- |
| load_unaligned | -0.026 | ok |

**Consistency summary:**

- **load_byte_pack**: won 3/8, lost 4/8
- **load_unaligned**: won 3/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 3.3ns | 2612.4ns | 0.1% |  |
| load_byte_pack | 3.2ns | 2645.1ns | 0.1% |  |
| load_unaligned | 3.1ns | 2639.7ns | 0.1% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 2506.9-2694.9 ns)
   2506.9 |####################
   2516.3 |###
   2525.7 |
   2535.1 |
   2544.5 |
   2553.9 |#
   2563.3 |
   2572.7 |
   2582.1 |####
   2591.5 |##################
   2600.9 |######
   2610.3 |#########
   2619.7 |#
   2629.1 |#
   2638.5 |
   2647.9 |#
   2657.3 |
   2666.7 |#
   2676.1 |#######
   2685.5 |########################################
  (15 below, 9 above range)

load_byte_pack (n=160, range 2548.1-2740.8 ns)
   2548.1 |#
   2557.8 |
   2567.4 |
   2577.0 |
   2586.7 |##################
   2596.3 |############
   2605.9 |##
   2615.6 |
   2625.2 |##
   2634.8 |##
   2644.5 |#
   2654.1 |
   2663.7 |
   2673.3 |##
   2683.0 |########################################
   2692.6 |##
   2702.2 |
   2711.9 |#
   2721.5 |
   2731.1 |
  (16 below, 8 above range)

load_unaligned (n=160, range 2506.9-2753.5 ns)
   2506.9 |################
   2519.3 |
   2531.6 |#
   2543.9 |#
   2556.2 |
   2568.6 |
   2580.9 |#
   2593.2 |########
   2605.5 |####
   2617.9 |##
   2630.2 |#
   2642.5 |
   2654.9 |
   2667.2 |
   2679.5 |########################################
   2691.8 |##########
   2704.2 |
   2716.5 |
   2728.8 |##
   2741.1 |
  (14 below, 10 above range)

```
