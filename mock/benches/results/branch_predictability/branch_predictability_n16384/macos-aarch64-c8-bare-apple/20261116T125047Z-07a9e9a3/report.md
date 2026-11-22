# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 20.51 us to slowest 20.51 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (159 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bp_balanced)

The baseline bp_balanced is the fastest (20.51 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (bp_balanced) is the fastest** at 20506.0 ns median
- Spread: 1.00x (fastest 20506.0 ns, slowest 20506.9 ns)

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
| bp_balanced | 22658ns | 22601ns | 22589ns | 22617ns | 22847ns | base |
| bp_skew_high | 22634ns | 22608ns | 22590ns | 22618ns | 22727ns | -0.10% |
| bp_skew_low | 22633ns | 22610ns | 22588ns | 22620ns | 22718ns | -0.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 20558ns | 20498ns | 20732ns | base | 0.797 |
| bp_skew_high | 20536ns | 20499ns | 20618ns | -0.11% | 0.798 |
| bp_skew_low | 20533ns | 20498ns | 20605ns | -0.12% | 0.798 |

## Performance model

- Peak throughput: **0.799 Gops/s** (bp_balanced; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.799 | 100.0% |
| bp_skew_high | 0.799 | 100.0% |
| bp_skew_low | 0.799 | 100.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 22658ns | 22658ns | base |
| bp_skew_high | 22634ns | 22634ns | -0.10% |
| bp_skew_low | 22633ns | 22633ns | -0.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 20506ns | base | --- | [20504, 20514] | --- | --- | --- | --- |
| bp_skew_high | 20507ns | no significant difference | [-1, +2]ns | [20505, 20520] | no | 0.8103 | 0.8103 | 4 |
| bp_skew_low | 20507ns | no significant difference | [-2, +2]ns | [20505, 20515] | no | 0.8103 | 0.7497 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 20506ns | -0.0% | +0.0% |
| 1 | 2 | 20509ns | +0.1% | -0.0% |
| 1 | 3 | 20504ns | +0.1% | +0.0% |
| 1 | 4 | 20521ns | -0.1% | -0.0% |
| 1 | 5 | 20505ns | +0.0% | +0.0% |
| 1 | 6 | 20507ns | -0.0% | +0.1% |
| 1 | 7 | 20553ns | -0.2% | -0.2% |
| 1 | 8 | 20505ns | -0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.290 | moderate- |
| bp_skew_high | -0.041 | ok |
| bp_skew_low | -0.332 | moderate- |

**Consistency summary:**

- **bp_skew_high**: won 1/8, lost 1/8
- **bp_skew_low**: won 1/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 3.1ns | 20558.2ns | 0.0% |  |
| bp_skew_high | 3.3ns | 20536.2ns | 0.0% |  |
| bp_skew_low | 3.3ns | 20532.9ns | 0.0% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 20497.8-20732.5 ns)
  20497.8 |########################################
  20509.5 |##
  20521.3 |#
  20533.0 |##
  20544.7 |#########
  20556.5 |#####
  20568.2 |
  20579.9 |####
  20591.7 |###
  20603.4 |
  20615.1 |#
  20626.9 |
  20638.6 |
  20650.3 |
  20662.1 |
  20673.8 |#
  20685.5 |
  20697.3 |
  20709.0 |
  20720.7 |
  (16 below, 5 above range)

bp_skew_high (n=160, range 20498.8-20618.3 ns)
  20498.8 |########################################
  20504.8 |#######################
  20510.8 |###
  20516.8 |
  20522.7 |#
  20528.7 |##
  20534.7 |
  20540.7 |#####
  20546.6 |########
  20552.6 |########
  20558.6 |######
  20564.6 |####
  20570.5 |
  20576.5 |#
  20582.5 |###
  20588.5 |#
  20594.4 |#
  20600.4 |
  20606.4 |
  20612.4 |
  (13 below, 12 above range)

bp_skew_low (n=160, range 20498.3-20605.5 ns)
  20498.3 |########################################
  20503.6 |######################################
  20509.0 |#####
  20514.4 |####
  20519.7 |#
  20525.1 |########
  20530.4 |
  20535.8 |#
  20541.2 |######
  20546.5 |########
  20551.9 |########
  20557.3 |#####
  20562.6 |##########
  20568.0 |
  20573.3 |####
  20578.7 |#
  20584.1 |###
  20589.4 |##
  20594.8 |##
  20600.1 |##
  (15 below, 10 above range)

```
