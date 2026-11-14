# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 20.53 us to slowest 20.55 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (17 ns) is smaller than the fastest variant's own run-to-run std-dev (358 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bp_balanced)

The baseline bp_balanced is the fastest (20.53 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (bp_balanced) is the fastest** at 20534.8 ns median
- Spread: 1.00x (fastest 20534.8 ns, slowest 20551.4 ns)

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
| bp_balanced | 22826ns | 22627ns | 22579ns | 22680ns | 23512ns | base |
| bp_skew_high | 22802ns | 22642ns | 22588ns | 22680ns | 23384ns | -0.11% |
| bp_skew_low | 22782ns | 22637ns | 22584ns | 22659ns | 23346ns | -0.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 20714ns | 20496ns | 21327ns | base | 0.791 |
| bp_skew_high | 20688ns | 20500ns | 21208ns | -0.13% | 0.792 |
| bp_skew_low | 20673ns | 20500ns | 21185ns | -0.20% | 0.793 |

## Performance model

- Peak throughput: **0.799 Gops/s** (bp_balanced; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.798 | 99.8% |
| bp_skew_high | 0.797 | 99.7% |
| bp_skew_low | 0.798 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 22826ns | 22826ns | base |
| bp_skew_high | 22802ns | 22802ns | -0.11% |
| bp_skew_low | 22782ns | 22782ns | -0.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 20535ns | base | --- | [20516, 20551] | --- | --- | --- | --- |
| bp_skew_high | 20551ns | no significant difference | [-8, +7]ns | [20531, 20573] | no | 1.0000 | 0.9370 | 0 |
| bp_skew_low | 20542ns | no significant difference | [-6, +4]ns | [20519, 20552] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 20538ns | +0.0% | +0.3% |
| 1 | 2 | 20589ns | -0.4% | -0.3% |
| 1 | 3 | 20515ns | +0.1% | +0.2% |
| 1 | 4 | 21312ns | -3.7% | -3.4% |
| 1 | 5 | 20638ns | +0.5% | -0.6% |
| 1 | 6 | 20515ns | +0.2% | +0.1% |
| 1 | 7 | 20514ns | +0.3% | +0.2% |
| 1 | 8 | 20495ns | +0.4% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.074 | ok |
| bp_skew_high | -0.199 | ok |
| bp_skew_low | -0.288 | moderate- |

**Consistency summary:**

- **bp_skew_high**: won 2/8, lost 5/8
- **bp_skew_low**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 3.1ns | 20714.0ns | 0.0% |  |
| bp_skew_high | 3.3ns | 20687.6ns | 0.0% |  |
| bp_skew_low | 2.9ns | 20672.8ns | 0.0% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 20495.5-21326.9 ns)
  20495.5 |########################################
  20537.1 |############
  20578.6 |###
  20620.2 |##
  20661.8 |
  20703.4 |
  20744.9 |
  20786.5 |
  20828.1 |#
  20869.6 |######
  20911.2 |#
  20952.8 |#
  20994.3 |
  21035.9 |
  21077.5 |
  21119.1 |
  21160.6 |#
  21202.2 |##
  21243.8 |
  21285.3 |#
  (11 below, 13 above range)

bp_skew_high (n=160, range 20499.9-21208.3 ns)
  20499.9 |########################################
  20535.3 |#############
  20570.7 |###########
  20606.2 |####
  20641.6 |##
  20677.0 |##
  20712.4 |##
  20747.9 |####
  20783.3 |
  20818.7 |
  20854.1 |##
  20889.5 |##
  20925.0 |#
  20960.4 |##
  20995.8 |
  21031.2 |
  21066.7 |
  21102.1 |
  21137.5 |
  21172.9 |
  (12 below, 14 above range)

bp_skew_low (n=160, range 20500.3-21184.9 ns)
  20500.3 |########################################
  20534.5 |#############
  20568.7 |#######
  20603.0 |#
  20637.2 |###
  20671.4 |#
  20705.7 |
  20739.9 |#
  20774.1 |
  20808.3 |
  20842.6 |
  20876.8 |######
  20911.0 |
  20945.3 |#
  20979.5 |
  21013.7 |
  21048.0 |
  21082.2 |
  21116.4 |
  21150.7 |
  (11 below, 15 above range)

```
