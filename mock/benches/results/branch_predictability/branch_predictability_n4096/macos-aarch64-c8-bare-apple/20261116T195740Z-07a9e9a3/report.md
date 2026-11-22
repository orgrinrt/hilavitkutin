# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (25 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bp_balanced)

The baseline bp_balanced is the fastest (5.10 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.0% of the fastest

All 3 variants sit between 5.10 us and 5.11 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bp_skew_high's edge over baseline is significant but tiny (1 ns, 0.02%)

bp_skew_high differs from baseline bp_balanced by 1 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (bp_balanced) is the fastest** at 5103.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 5103.7 ns, slowest 5105.8 ns)

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
| bp_balanced | 7208ns | 7198ns | 7189ns | 7198ns | 7259ns | base |
| bp_skew_high | 7232ns | 7199ns | 7190ns | 7203ns | 7362ns | +0.33% |
| bp_skew_low | 7239ns | 7201ns | 7190ns | 7207ns | 7384ns | +0.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 5112ns | 5098ns | 5147ns | base | 0.801 |
| bp_skew_high | 5129ns | 5098ns | 5224ns | +0.34% | 0.799 |
| bp_skew_low | 5134ns | 5098ns | 5244ns | +0.43% | 0.798 |

## Performance model

- Peak throughput: **0.803 Gops/s** (bp_balanced; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.803 | 99.9% |
| bp_skew_high | 0.802 | 99.8% |
| bp_skew_low | 0.802 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 7208ns | 7208ns | base |
| bp_skew_high | 7232ns | 7232ns | +0.33% |
| bp_skew_low | 7239ns | 7239ns | +0.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 5104ns | base | --- | [5103, 5105] | --- | --- | --- | --- |
| bp_skew_high | 5106ns | +1.2ns (+0.0%) | [+0, +2]ns | [5105, 5107] | YES | 0.0027 | 0.0014 | 10 |
| bp_skew_low | 5106ns | no significant difference | [+0, +2]ns | [5105, 5107] | no | 0.0551 | 0.0551 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 5103ns | +0.1% | +0.0% |
| 1 | 2 | 5104ns | +0.0% | +0.0% |
| 1 | 3 | 5104ns | +0.0% | +0.0% |
| 1 | 4 | 5105ns | +0.0% | +0.0% |
| 1 | 5 | 5103ns | +0.0% | +0.0% |
| 1 | 6 | 5105ns | +0.0% | -0.0% |
| 1 | 7 | 5105ns | +3.1% | -0.0% |
| 1 | 8 | 5103ns | +0.1% | +3.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.424 | moderate- |
| bp_skew_high | -0.149 | ok |
| bp_skew_low | -0.017 | ok |

**Consistency summary:**

- **bp_skew_high**: won 0/8, lost 1/8
- **bp_skew_low**: won 0/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 3.0ns | 5111.5ns | 0.1% |  |
| bp_skew_high | 2.8ns | 5128.8ns | 0.1% |  |
| bp_skew_low | 2.9ns | 5133.5ns | 0.1% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 5098.1-5147.3 ns)
   5098.1 |#####################################
   5100.6 |######################################
   5103.0 |########################################
   5105.5 |################################
   5108.0 |###########
   5110.4 |##
   5112.9 |#
   5115.3 |#
   5117.8 |##
   5120.2 |
   5122.7 |
   5125.2 |
   5127.6 |#
   5130.1 |##
   5132.5 |
   5135.0 |#
   5137.4 |
   5139.9 |
   5142.4 |#
   5144.8 |###
  (10 below, 12 above range)

bp_skew_high (n=160, range 5098.5-5223.6 ns)
   5098.5 |########################################
   5104.7 |######################################
   5111.0 |#######
   5117.2 |##
   5123.5 |
   5129.8 |
   5136.0 |###
   5142.3 |####
   5148.5 |#
   5154.8 |
   5161.1 |
   5167.3 |
   5173.6 |
   5179.8 |
   5186.1 |##
   5192.3 |#
   5198.6 |
   5204.9 |
   5211.1 |
   5217.4 |
  (16 below, 14 above range)

bp_skew_low (n=160, range 5098.5-5244.0 ns)
   5098.5 |########################################
   5105.7 |#####################
   5113.0 |#
   5120.3 |##
   5127.6 |
   5134.8 |
   5142.1 |####
   5149.4 |#
   5156.7 |
   5163.9 |
   5171.2 |
   5178.5 |
   5185.8 |##
   5193.0 |
   5200.3 |
   5207.6 |
   5214.9 |
   5222.1 |
   5229.4 |
   5236.7 |#
  (14 below, 17 above range)

```
