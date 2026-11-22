# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (load_aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline load_aligned has the worst median (2.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest load_unaligned at 2.51 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 2.51 us to slowest 2.51 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (16 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: load_unaligned** at 2507.1 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 2507.1 ns, slowest 2508.6 ns)

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
| load_aligned | 4638ns | 4611ns | 4603ns | 4613ns | 4748ns | base |
| load_byte_pack | 4615ns | 4608ns | 4601ns | 4609ns | 4647ns | -0.51% |
| load_unaligned | 4614ns | 4609ns | 4603ns | 4609ns | 4643ns | -0.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 2523ns | 2504ns | 2583ns | base | 6.494 |
| load_byte_pack | 2512ns | 2504ns | 2532ns | -0.44% | 6.523 |
| load_unaligned | 2511ns | 2503ns | 2529ns | -0.47% | 6.525 |

## Performance model

- Peak throughput: **6.545 Gops/s** (load_unaligned; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 6.531 | 99.8% |
| load_byte_pack | 6.533 | 99.8% |
| load_unaligned | 6.535 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 4638ns | 4638ns | base |
| load_byte_pack | 4615ns | 4615ns | -0.51% |
| load_unaligned | 4614ns | 4614ns | -0.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 2509ns | base | --- | [2508, 2510] | --- | --- | --- | --- |
| load_byte_pack | 2508ns | no significant difference | [-1, +0]ns | [2507, 2508] | no | 0.5152 | 0.5152 | 9 |
| load_unaligned | 2507ns | no significant difference | [-2, +0]ns | [2507, 2508] | no (adj: YES) | 0.0162 | 0.0081 | 13 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 2507ns | +0.1% | -0.0% |
| 1 | 2 | 2509ns | -0.1% | -0.1% |
| 1 | 3 | 2508ns | -0.1% | -0.0% |
| 1 | 4 | 2509ns | -0.1% | -0.1% |
| 1 | 5 | 2601ns | -3.7% | -3.6% |
| 1 | 6 | 2507ns | +0.0% | -0.0% |
| 1 | 7 | 2507ns | +0.1% | +0.0% |
| 1 | 8 | 2509ns | -0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | -0.160 | ok |
| load_byte_pack | -0.116 | ok |
| load_unaligned | -0.203 | moderate- |

**Consistency summary:**

- **load_byte_pack**: won 1/8, lost 1/8
- **load_unaligned**: won 1/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 3.1ns | 2522.9ns | 0.1% |  |
| load_byte_pack | 3.3ns | 2511.9ns | 0.1% |  |
| load_unaligned | 3.3ns | 2511.0ns | 0.1% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 2504.1-2582.8 ns)
   2504.1 |########################################
   2508.0 |#############################
   2511.9 |###
   2515.9 |
   2519.8 |#
   2523.8 |##
   2527.7 |#
   2531.6 |
   2535.6 |
   2539.5 |
   2543.4 |
   2547.4 |
   2551.3 |
   2555.3 |
   2559.2 |
   2563.1 |
   2567.1 |
   2571.0 |
   2575.0 |
   2578.9 |
  (12 below, 22 above range)

load_byte_pack (n=160, range 2503.7-2532.0 ns)
   2503.7 |#####################
   2505.1 |#########################
   2506.5 |########################################
   2507.9 |####################
   2509.3 |##############
   2510.8 |#############
   2512.2 |######
   2513.6 |##
   2515.0 |
   2516.4 |#
   2517.8 |#
   2519.3 |
   2520.7 |
   2522.1 |####
   2523.5 |##
   2524.9 |#
   2526.3 |#
   2527.8 |##
   2529.2 |
   2530.6 |
  (12 below, 7 above range)

load_unaligned (n=160, range 2503.3-2529.5 ns)
   2503.3 |##############
   2504.6 |##################################
   2506.0 |########################################
   2507.3 |##############################
   2508.6 |####################
   2509.9 |################
   2511.2 |#######
   2512.5 |#####
   2513.8 |
   2515.1 |
   2516.4 |
   2517.7 |
   2519.0 |
   2520.3 |##
   2521.6 |
   2522.9 |
   2524.2 |##
   2525.5 |
   2526.8 |#
   2528.1 |
  (13 below, 10 above range)

```
