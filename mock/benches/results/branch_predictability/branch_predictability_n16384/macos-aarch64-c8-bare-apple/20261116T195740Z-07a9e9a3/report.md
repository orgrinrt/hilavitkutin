# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 20.51 us to slowest 20.51 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6 ns) is smaller than the fastest variant's own run-to-run std-dev (103 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bp_balanced)

The baseline bp_balanced is the fastest (20.51 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (bp_balanced) is the fastest** at 20507.7 ns median
- Spread: 1.00x (fastest 20507.7 ns, slowest 20513.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x1b41e7c35fa28d06 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bp_balanced | 22650ns | 22616ns | 22591ns | 22625ns | 22783ns | base |
| bp_skew_high | 22638ns | 22622ns | 22592ns | 22625ns | 22724ns | -0.05% |
| bp_skew_low | 22652ns | 22622ns | 22592ns | 22625ns | 22796ns | +0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 20548ns | 20498ns | 20673ns | base | 0.797 |
| bp_skew_high | 20536ns | 20498ns | 20612ns | -0.06% | 0.798 |
| bp_skew_low | 20549ns | 20498ns | 20674ns | +0.01% | 0.797 |

## Performance model

- Peak throughput: **0.799 Gops/s** (bp_skew_low; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.799 | 100.0% |
| bp_skew_high | 0.799 | 99.9% |
| bp_skew_low | 0.799 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 22650ns | 22650ns | base |
| bp_skew_high | 22638ns | 22638ns | -0.05% |
| bp_skew_low | 22652ns | 22652ns | +0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 20508ns | base | --- | [20505, 20532] | --- | --- | --- | --- |
| bp_skew_high | 20514ns | no significant difference | [-2, +2]ns | [20508, 20529] | no | 0.8740 | 0.8740 | 1 |
| bp_skew_low | 20511ns | no significant difference | [-3, +0]ns | [20506, 20533] | no | 0.4029 | 0.2015 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 20521ns | +0.1% | -0.0% |
| 1 | 2 | 20505ns | +0.0% | +0.2% |
| 1 | 3 | 20525ns | -0.1% | -0.1% |
| 1 | 4 | 20546ns | -0.2% | -0.2% |
| 1 | 5 | 20506ns | +0.1% | +0.2% |
| 1 | 6 | 20513ns | -0.0% | +0.1% |
| 1 | 7 | 20511ns | +0.2% | +0.1% |
| 1 | 8 | 20506ns | +0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.074 | ok |
| bp_skew_high | -0.289 | moderate- |
| bp_skew_low | -0.264 | moderate- |

**Consistency summary:**

- **bp_skew_high**: won 1/8, lost 1/8
- **bp_skew_low**: won 1/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 3.3ns | 20547.9ns | 0.0% |  |
| bp_skew_high | 3.3ns | 20536.1ns | 0.0% |  |
| bp_skew_low | 3.2ns | 20549.3ns | 0.0% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 20498.0-20672.7 ns)
  20498.0 |########################################
  20506.8 |######
  20515.5 |#
  20524.2 |#
  20533.0 |####
  20541.7 |#####
  20550.4 |####
  20559.2 |#####
  20567.9 |#####
  20576.6 |###
  20585.4 |#
  20594.1 |
  20602.8 |
  20611.6 |
  20620.3 |
  20629.0 |
  20637.8 |
  20646.5 |
  20655.2 |#
  20664.0 |#
  (12 below, 10 above range)

bp_skew_high (n=160, range 20497.8-20612.3 ns)
  20497.8 |########################################
  20503.5 |#################################
  20509.2 |########
  20514.9 |#########
  20520.7 |##
  20526.4 |###
  20532.1 |#######
  20537.8 |####
  20543.6 |#######
  20549.3 |########
  20555.0 |########
  20560.7 |###
  20566.5 |#######
  20572.2 |#
  20577.9 |###
  20583.6 |##
  20589.4 |###
  20595.1 |#
  20600.8 |#
  20606.5 |##
  (13 below, 14 above range)

bp_skew_low (n=160, range 20497.6-20674.0 ns)
  20497.6 |########################################
  20506.4 |########
  20515.2 |###
  20524.0 |##
  20532.9 |#####
  20541.7 |######
  20550.5 |####
  20559.3 |#####
  20568.1 |#
  20577.0 |###
  20585.8 |####
  20594.6 |##
  20603.4 |#
  20612.3 |
  20621.1 |#
  20629.9 |##
  20638.7 |
  20647.5 |#
  20656.4 |
  20665.2 |##
  (16 below, 6 above range)

```
