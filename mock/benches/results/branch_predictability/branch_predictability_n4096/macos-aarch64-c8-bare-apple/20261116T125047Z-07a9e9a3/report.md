# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bp_balanced) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bp_balanced has the worst median (5.10 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bp_skew_low at 5.10 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 5.10 us to slowest 5.10 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### bp_balanced shows alternating (throttle bounce) (autocorr -0.72)

bp_balanced's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (20 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: bp_skew_low** at 5103.7 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 5103.7 ns, slowest 5104.8 ns)

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
| bp_balanced | 7223ns | 7196ns | 7188ns | 7198ns | 7332ns | base |
| bp_skew_high | 7215ns | 7198ns | 7189ns | 7201ns | 7285ns | -0.10% |
| bp_skew_low | 7203ns | 7196ns | 7188ns | 7196ns | 7237ns | -0.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 5128ns | 5098ns | 5225ns | base | 0.799 |
| bp_skew_high | 5116ns | 5098ns | 5167ns | -0.22% | 0.801 |
| bp_skew_low | 5109ns | 5098ns | 5133ns | -0.37% | 0.802 |

## Performance model

- Peak throughput: **0.803 Gops/s** (bp_skew_low; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.802 | 99.9% |
| bp_skew_high | 0.802 | 99.9% |
| bp_skew_low | 0.803 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 7223ns | 7223ns | base |
| bp_skew_high | 7215ns | 7215ns | -0.10% |
| bp_skew_low | 7203ns | 7203ns | -0.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 5105ns | base | --- | [5104, 5106] | --- | --- | --- | --- |
| bp_skew_high | 5105ns | no significant difference | [+0, +1]ns | [5104, 5106] | no | 0.2411 | 0.1205 | 10 |
| bp_skew_low | 5104ns | no significant difference | [-1, +0]ns | [5103, 5105] | no | 0.6852 | 0.6852 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 5105ns | -0.0% | -0.0% |
| 1 | 2 | 5104ns | +0.0% | +0.0% |
| 1 | 3 | 5106ns | +0.0% | -0.0% |
| 1 | 4 | 5104ns | +0.0% | +0.0% |
| 1 | 5 | 5108ns | -0.0% | -0.1% |
| 1 | 6 | 5103ns | +0.0% | +0.0% |
| 1 | 7 | 5105ns | +0.0% | -0.0% |
| 1 | 8 | 5104ns | -0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.725 | HIGH- (thermal bounce) |
| bp_skew_high | 0.213 | moderate+ |
| bp_skew_low | -0.064 | ok |

**Consistency summary:**

- **bp_skew_high**: won 0/8, lost 0/8
- **bp_skew_low**: won 0/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 2.8ns | 5127.7ns | 0.1% |  |
| bp_skew_high | 2.8ns | 5116.1ns | 0.1% |  |
| bp_skew_low | 3.0ns | 5108.7ns | 0.1% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 5098.1-5224.6 ns)
   5098.1 |########################################
   5104.5 |################################
   5110.8 |#
   5117.1 |###
   5123.4 |#
   5129.8 |
   5136.1 |
   5142.4 |#
   5148.7 |####
   5155.1 |
   5161.4 |
   5167.7 |
   5174.0 |
   5180.4 |#
   5186.7 |#
   5193.0 |
   5199.3 |
   5205.7 |
   5212.0 |
   5218.3 |
  (11 below, 9 above range)

bp_skew_high (n=160, range 5098.4-5167.5 ns)
   5098.4 |##############################
   5101.9 |########################################
   5105.3 |################################
   5108.8 |##########
   5112.2 |
   5115.7 |#
   5119.1 |######
   5122.6 |
   5126.0 |
   5129.5 |
   5132.9 |
   5136.4 |
   5139.9 |
   5143.3 |
   5146.8 |
   5150.2 |#####
   5153.7 |
   5157.1 |
   5160.6 |##
   5164.0 |###
  (13 below, 6 above range)

bp_skew_low (n=160, range 5098.0-5132.8 ns)
   5098.0 |###########
   5099.8 |###############
   5101.5 |#################
   5103.2 |########################################
   5105.0 |######################
   5106.7 |#########
   5108.5 |########
   5110.2 |#####
   5111.9 |#
   5113.7 |
   5115.4 |
   5117.2 |
   5118.9 |##
   5120.6 |###
   5122.4 |##
   5124.1 |####
   5125.9 |
   5127.6 |
   5129.3 |
   5131.1 |#
  (14 below, 9 above range)

```
