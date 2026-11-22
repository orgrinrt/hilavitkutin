# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 5.11 us to slowest 5.11 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (44 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bp_balanced)

The baseline bp_balanced is the fastest (5.11 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (bp_balanced) is the fastest** at 5105.4 ns median
- Spread: 1.00x (fastest 5105.4 ns, slowest 5106.2 ns)

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
| bp_balanced | 7212ns | 7190ns | 7178ns | 7193ns | 7303ns | base |
| bp_skew_high | 7212ns | 7189ns | 7177ns | 7193ns | 7305ns | +0.00% |
| bp_skew_low | 7221ns | 7191ns | 7175ns | 7197ns | 7339ns | +0.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 5122ns | 5099ns | 5190ns | base | 0.800 |
| bp_skew_high | 5122ns | 5099ns | 5187ns | -0.00% | 0.800 |
| bp_skew_low | 5127ns | 5098ns | 5211ns | +0.10% | 0.799 |

## Performance model

- Peak throughput: **0.803 Gops/s** (bp_skew_low; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.802 | 99.9% |
| bp_skew_high | 0.802 | 99.9% |
| bp_skew_low | 0.802 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 7212ns | 7212ns | base |
| bp_skew_high | 7212ns | 7212ns | +0.00% |
| bp_skew_low | 7221ns | 7221ns | +0.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 5105ns | base | --- | [5105, 5106] | --- | --- | --- | --- |
| bp_skew_high | 5106ns | no significant difference | [-1, +2]ns | [5105, 5107] | no | 0.5206 | 0.5206 | 5 |
| bp_skew_low | 5106ns | no significant difference | [-1, +2]ns | [5105, 5107] | no | 0.5206 | 0.2979 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 5105ns | +0.0% | +0.0% |
| 1 | 2 | 5105ns | +0.0% | +0.0% |
| 1 | 3 | 5106ns | +0.0% | -0.0% |
| 1 | 4 | 5104ns | +0.0% | +0.1% |
| 1 | 5 | 5104ns | +0.0% | +0.0% |
| 1 | 6 | 5106ns | -0.0% | -0.0% |
| 1 | 7 | 5106ns | +0.0% | +0.0% |
| 1 | 8 | 5106ns | -0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.123 | ok |
| bp_skew_high | -0.136 | ok |
| bp_skew_low | -0.033 | ok |

**Consistency summary:**

- **bp_skew_high**: won 0/8, lost 0/8
- **bp_skew_low**: won 0/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 2.8ns | 5122.0ns | 0.1% |  |
| bp_skew_high | 2.8ns | 5121.9ns | 0.1% |  |
| bp_skew_low | 2.8ns | 5127.2ns | 0.1% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 5099.2-5190.2 ns)
   5099.2 |###################################
   5103.7 |########################################
   5108.3 |########
   5112.8 |
   5117.4 |###
   5121.9 |####
   5126.5 |
   5131.0 |#
   5135.6 |
   5140.1 |##
   5144.7 |
   5149.2 |
   5153.8 |
   5158.3 |##
   5162.9 |
   5167.4 |
   5172.0 |#
   5176.5 |
   5181.1 |
   5185.6 |
  (12 below, 13 above range)

bp_skew_high (n=160, range 5099.2-5186.8 ns)
   5099.2 |################################
   5103.6 |########################################
   5108.0 |###########
   5112.4 |###
   5116.7 |
   5121.1 |######
   5125.5 |
   5129.9 |
   5134.3 |#
   5138.7 |
   5143.0 |
   5147.4 |##
   5151.8 |
   5156.2 |#
   5160.6 |
   5164.9 |##
   5169.3 |
   5173.7 |#
   5178.1 |
   5182.5 |##
  (13 below, 13 above range)

bp_skew_low (n=160, range 5098.5-5211.1 ns)
   5098.5 |#######################
   5104.1 |########################################
   5109.7 |#
   5115.4 |##
   5121.0 |##
   5126.6 |#
   5132.3 |##
   5137.9 |#
   5143.5 |
   5149.2 |####
   5154.8 |
   5160.4 |#
   5166.1 |
   5171.7 |
   5177.3 |##
   5183.0 |##
   5188.6 |#
   5194.2 |
   5199.9 |
   5205.5 |
  (19 below, 9 above range)

```
