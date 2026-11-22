# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (3 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (590 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.1% of the fastest

All 3 variants sit between 590 ns and 590 ns - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### load_byte_pack's edge over baseline is significant but tiny (1 ns, 0.15%)

load_byte_pack differs from baseline load_aligned by 1 ns (0.15%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (load_aligned) is the fastest** at 589.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 589.6 ns, slowest 590.0 ns)

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
| load_aligned | 2694ns | 2689ns | 2682ns | 2689ns | 2722ns | base |
| load_byte_pack | 2708ns | 2691ns | 2684ns | 2693ns | 2776ns | +0.51% |
| load_unaligned | 2694ns | 2688ns | 2682ns | 2689ns | 2721ns | -0.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 589ns | 585ns | 593ns | base | 6.949 |
| load_byte_pack | 594ns | 586ns | 610ns | +0.69% | 6.901 |
| load_unaligned | 590ns | 586ns | 596ns | +0.10% | 6.942 |

## Performance model

- Peak throughput: **6.996 Gops/s** (load_aligned; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 6.947 | 99.3% |
| load_byte_pack | 6.942 | 99.2% |
| load_unaligned | 6.947 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2694ns | 2694ns | base |
| load_byte_pack | 2708ns | 2708ns | +0.51% |
| load_unaligned | 2694ns | 2694ns | -0.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 590ns | base | --- | [589, 590] | --- | --- | --- | --- |
| load_byte_pack | 590ns | +0.9ns (+0.2%) | [+0, +2]ns | [590, 591] | YES | 0.0077 | 0.0039 | 12 |
| load_unaligned | 590ns | no significant difference | [-0, +1]ns | [589, 590] | no | 0.5124 | 0.5124 | 11 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 588ns | +0.4% | -0.1% |
| 1 | 2 | 590ns | +3.7% | +0.0% |
| 1 | 3 | 590ns | -0.1% | +0.0% |
| 1 | 4 | 589ns | +0.1% | +0.0% |
| 1 | 5 | 589ns | +0.1% | +0.0% |
| 1 | 6 | 590ns | +0.1% | -0.1% |
| 1 | 7 | 590ns | -0.1% | +0.0% |
| 1 | 8 | 590ns | +0.0% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | -0.167 | ok |
| load_byte_pack | -0.158 | ok |
| load_unaligned | -0.230 | moderate- |

**Consistency summary:**

- **load_byte_pack**: won 1/8, lost 3/8
- **load_unaligned**: won 0/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 2.8ns | 589.4ns | 0.5% |  |
| load_byte_pack | 2.9ns | 593.5ns | 0.5% |  |
| load_unaligned | 2.7ns | 590.1ns | 0.5% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 585.4-593.3 ns)
    585.4 |##############
    585.8 |#####
    586.2 |
    586.6 |#####
    587.0 |#################
    587.4 |#########################
    587.8 |######################
    588.2 |######################
    588.6 |############################
    589.0 |###############################
    589.4 |########################################
    589.7 |#####################################
    590.1 |#################
    590.5 |####################
    590.9 |###############################
    591.3 |#################
    591.7 |####################
    592.1 |
    592.5 |###########
    592.9 |#################
  (14 below, 11 above range)

load_byte_pack (n=160, range 586.2-609.7 ns)
    586.2 |#################
    587.4 |##########################
    588.6 |########################################
    589.8 |############################
    590.9 |####################
    592.1 |#############
    593.3 |#############
    594.5 |####
    595.6 |#
    596.8 |##
    598.0 |
    599.1 |
    600.3 |#
    601.5 |#
    602.7 |
    603.8 |
    605.0 |##
    606.2 |
    607.3 |#
    608.5 |#####
  (11 below, 17 above range)

load_unaligned (n=160, range 585.6-596.1 ns)
    585.6 |######
    586.1 |########
    586.6 |########################################
    587.2 |########
    587.7 |######################
    588.2 |###########
    588.7 |########################################
    589.3 |######################
    589.8 |#################
    590.3 |#####################################
    590.8 |###############
    591.4 |###########
    591.9 |###############
    592.4 |####################
    592.9 |#################
    593.4 |######
    594.0 |######
    594.5 |####
    595.0 |
    595.5 |
  (14 below, 5 above range)

```
