# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (load_aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline load_aligned has the worst median (2.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest load_unaligned at 2.51 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 2.51 us to slowest 2.51 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (11 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: load_unaligned** at 2507.1 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 2507.1 ns, slowest 2507.7 ns)

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
| load_aligned | 4618ns | 4610ns | 4602ns | 4610ns | 4659ns | base |
| load_byte_pack | 4615ns | 4608ns | 4602ns | 4609ns | 4646ns | -0.07% |
| load_unaligned | 4611ns | 4605ns | 4599ns | 4606ns | 4639ns | -0.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 2512ns | 2504ns | 2533ns | base | 6.522 |
| load_byte_pack | 2512ns | 2503ns | 2532ns | -0.01% | 6.523 |
| load_unaligned | 2510ns | 2503ns | 2524ns | -0.08% | 6.528 |

## Performance model

- Peak throughput: **6.545 Gops/s** (load_byte_pack; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 6.533 | 99.8% |
| load_byte_pack | 6.534 | 99.8% |
| load_unaligned | 6.535 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 4618ns | 4618ns | base |
| load_byte_pack | 4615ns | 4615ns | -0.07% |
| load_unaligned | 4611ns | 4611ns | -0.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 2508ns | base | --- | [2507, 2508] | --- | --- | --- | --- |
| load_byte_pack | 2508ns | no significant difference | [-0, +0]ns | [2507, 2508] | no | 0.6852 | 0.6255 | 9 |
| load_unaligned | 2507ns | no significant difference | [-1, +1]ns | [2507, 2508] | no | 0.6852 | 0.6852 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 2508ns | +0.0% | +0.0% |
| 1 | 2 | 2506ns | +0.0% | +0.1% |
| 1 | 3 | 2508ns | -0.0% | -0.0% |
| 1 | 4 | 2508ns | +0.0% | -0.1% |
| 1 | 5 | 2508ns | +0.0% | -0.0% |
| 1 | 6 | 2508ns | -0.0% | -0.0% |
| 1 | 7 | 2507ns | +0.0% | +0.0% |
| 1 | 8 | 2509ns | -0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | -0.387 | moderate- |
| load_byte_pack | -0.251 | moderate- |
| load_unaligned | 0.391 | moderate+ |

**Consistency summary:**

- **load_byte_pack**: won 0/8, lost 0/8
- **load_unaligned**: won 0/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 3.2ns | 2511.9ns | 0.1% |  |
| load_byte_pack | 3.2ns | 2511.6ns | 0.1% |  |
| load_unaligned | 3.1ns | 2509.9ns | 0.1% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 2503.8-2533.1 ns)
   2503.8 |###########
   2505.3 |########################################
   2506.8 |#########################
   2508.2 |#############################
   2509.7 |############
   2511.1 |##
   2512.6 |#
   2514.1 |
   2515.5 |
   2517.0 |
   2518.5 |
   2519.9 |#
   2521.4 |
   2522.8 |#
   2524.3 |
   2525.8 |
   2527.2 |
   2528.7 |
   2530.2 |
   2531.6 |
  (14 below, 11 above range)

load_byte_pack (n=160, range 2503.3-2531.7 ns)
   2503.3 |################
   2504.7 |#######################
   2506.1 |########################################
   2507.5 |###########################
   2508.9 |#####################
   2510.4 |############
   2511.8 |###
   2513.2 |#
   2514.6 |
   2516.0 |
   2517.5 |
   2518.9 |##
   2520.3 |#
   2521.7 |##
   2523.1 |
   2524.6 |##
   2526.0 |
   2527.4 |#
   2528.8 |
   2530.2 |
  (11 below, 10 above range)

load_unaligned (n=160, range 2503.4-2524.0 ns)
   2503.4 |############
   2504.4 |##########################
   2505.5 |########################
   2506.5 |########################################
   2507.5 |########################
   2508.6 |############
   2509.6 |##################
   2510.6 |#############
   2511.6 |##
   2512.7 |
   2513.7 |
   2514.7 |
   2515.8 |#
   2516.8 |
   2517.8 |#
   2518.9 |#
   2519.9 |
   2520.9 |
   2522.0 |#
   2523.0 |#
  (14 below, 11 above range)

```
