# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bp_balanced) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bp_balanced has the worst median (5.47 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bp_skew_high at 5.30 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (171 ns) is smaller than the fastest variant's own run-to-run std-dev (246 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bp_skew_high vs stability leader bp_skew_low (+2% speed for 1.2x steadier)

bp_skew_high is fastest (5.30 us, CV 4.6%); bp_skew_low gives up 2.3% median for 1.2x lower variance (CV 3.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 3.2% of the fastest

All 3 variants sit between 5.30 us and 5.47 us - a 3.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bp_skew_high's edge over baseline is significant but tiny (-5 ns, 0.10%)

bp_skew_high differs from baseline bp_balanced by -5 ns (0.10%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bp_skew_high** at 5299.0 ns median (-3.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.03x (fastest 5299.0 ns, slowest 5470.2 ns)

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
| bp_balanced | 7649ns | 7696ns | 7204ns | 7637ns | 8129ns | base |
| bp_skew_high | 7537ns | 7466ns | 7188ns | 7481ns | 8057ns | -1.45% |
| bp_skew_low | 7556ns | 7627ns | 7189ns | 7557ns | 7921ns | -1.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 5434ns | 5118ns | 5778ns | base | 0.754 |
| bp_skew_high | 5348ns | 5102ns | 5723ns | -1.58% | 0.766 |
| bp_skew_low | 5363ns | 5103ns | 5622ns | -1.31% | 0.764 |

## Performance model

- Peak throughput: **0.803 Gops/s** (bp_skew_high; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.749 | 93.3% |
| bp_skew_high | 0.773 | 96.3% |
| bp_skew_low | 0.756 | 94.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 7649ns | 7649ns | base |
| bp_skew_high | 7537ns | 7537ns | -1.45% |
| bp_skew_low | 7556ns | 7556ns | -1.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 5470ns | base | --- | [5467, 5472] | --- | --- | --- | --- |
| bp_skew_high | 5299ns | -5.2ns (-0.1%) | [-150, -1]ns | [5236, 5386] | YES | 0.0164 | 0.0164 | 3 |
| bp_skew_low | 5419ns | -4.6ns (-0.1%) | [-64, -1]ns | [5316, 5466] | YES | 0.0164 | 0.0140 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 5123ns | +0.6% | -0.4% |
| 1 | 2 | 5474ns | -0.0% | -3.1% |
| 1 | 3 | 5469ns | -6.6% | +0.0% |
| 1 | 4 | 5178ns | +5.7% | +5.6% |
| 1 | 5 | 5474ns | -5.7% | -0.1% |
| 1 | 6 | 5473ns | -6.7% | -4.9% |
| 1 | 7 | 5473ns | +0.1% | -1.1% |
| 1 | 8 | 5475ns | -0.2% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.190 | ok |
| bp_skew_high | -0.402 | moderate- |
| bp_skew_low | 0.087 | ok |

**Consistency summary:**

- **bp_skew_high**: won 4/8, lost 3/8
- **bp_skew_low**: won 4/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 2.9ns | 5434.2ns | 0.1% |  |
| bp_skew_high | 3.2ns | 5348.2ns | 0.1% |  |
| bp_skew_low | 3.2ns | 5363.2ns | 0.1% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 5118.0-5777.9 ns)
   5118.0 |##
   5151.0 |##
   5184.0 |#
   5217.0 |#
   5250.0 |###
   5283.0 |####
   5315.9 |#
   5348.9 |#
   5381.9 |
   5414.9 |#
   5447.9 |########################################
   5480.9 |######
   5513.9 |#
   5546.9 |
   5579.9 |
   5612.9 |
   5645.9 |
   5678.9 |
   5711.9 |
   5744.9 |
  (23 below, 21 above range)

bp_skew_high (n=160, range 5102.1-5723.4 ns)
   5102.1 |########################################
   5133.1 |########
   5164.2 |#########
   5195.3 |##
   5226.3 |#####
   5257.4 |########
   5288.5 |########
   5319.5 |###
   5350.6 |###
   5381.7 |####
   5412.7 |#
   5443.8 |#############################
   5474.8 |#############
   5505.9 |###
   5537.0 |####
   5568.0 |#
   5599.1 |
   5630.2 |#
   5661.2 |
   5692.3 |#
  (14 below, 16 above range)

bp_skew_low (n=160, range 5102.8-5621.8 ns)
   5102.8 |######################
   5128.7 |######
   5154.7 |
   5180.6 |##
   5206.6 |
   5232.5 |##
   5258.5 |###
   5284.4 |##########
   5310.4 |#####
   5336.3 |
   5362.3 |##
   5388.2 |###
   5414.2 |#####
   5440.1 |####
   5466.1 |########################################
   5492.1 |#####
   5518.0 |#####
   5544.0 |###
   5569.9 |
   5595.9 |
  (13 below, 11 above range)

```
