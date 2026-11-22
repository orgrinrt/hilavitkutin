# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.4% (fastest 295 ns to slowest 296 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (6 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bp_balanced)

The baseline bp_balanced is the fastest (295 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (bp_balanced) is the fastest** at 294.6 ns median
- Spread: 1.00x (fastest 294.6 ns, slowest 295.8 ns)

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
| bp_balanced | 2386ns | 2370ns | 2362ns | 2380ns | 2427ns | base |
| bp_skew_high | 2386ns | 2373ns | 2361ns | 2379ns | 2430ns | -0.00% |
| bp_skew_low | 2397ns | 2403ns | 2363ns | 2392ns | 2447ns | +0.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 295ns | 287ns | 302ns | base | 0.869 |
| bp_skew_high | 295ns | 288ns | 304ns | +0.16% | 0.867 |
| bp_skew_low | 296ns | 288ns | 305ns | +0.51% | 0.864 |

## Performance model

- Peak throughput: **0.892 Gops/s** (bp_balanced; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.869 | 97.5% |
| bp_skew_high | 0.869 | 97.5% |
| bp_skew_low | 0.865 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 2386ns | 2386ns | base |
| bp_skew_high | 2386ns | 2386ns | -0.00% |
| bp_skew_low | 2397ns | 2397ns | +0.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 295ns | base | --- | [294, 295] | --- | --- | --- | --- |
| bp_skew_high | 295ns | no significant difference | [-0, +1]ns | [294, 295] | no | 0.5233 | 0.5233 | 3 |
| bp_skew_low | 296ns | no significant difference | [+0, +1]ns | [295, 297] | no (adj: YES) | 0.0367 | 0.0184 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 299ns | -1.0% | -0.4% |
| 1 | 2 | 294ns | +0.6% | +0.1% |
| 1 | 3 | 299ns | -0.4% | +0.1% |
| 1 | 4 | 296ns | -0.9% | +0.8% |
| 1 | 5 | 294ns | -0.2% | +0.7% |
| 1 | 6 | 292ns | +0.9% | +0.4% |
| 1 | 7 | 293ns | -0.2% | -0.4% |
| 1 | 8 | 294ns | -0.4% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | 0.133 | ok |
| bp_skew_high | 0.265 | moderate+ |
| bp_skew_low | 0.389 | moderate+ |

**Consistency summary:**

- **bp_skew_high**: won 6/8, lost 2/8
- **bp_skew_low**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 2.8ns | 294.6ns | 0.9% |  |
| bp_skew_high | 2.9ns | 295.1ns | 1.0% |  |
| bp_skew_low | 2.8ns | 296.1ns | 1.0% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 287.1-301.9 ns)
    287.1 |####
    287.9 |#################
    288.6 |###########
    289.4 |###########
    290.1 |###########
    290.8 |########
    291.6 |###########
    292.3 |#################
    293.1 |######
    293.8 |############################
    294.5 |########################################
    295.3 |###############
    296.0 |####################
    296.8 |#################
    297.5 |####################
    298.2 |#################
    299.0 |####################
    299.7 |########
    300.4 |####
    301.2 |######
  (14 below, 11 above range)

bp_skew_high (n=160, range 288.2-303.6 ns)
    288.2 |##########################
    289.0 |################
    289.8 |########################
    290.5 |################
    291.3 |##################
    292.1 |##################
    292.9 |################################
    293.6 |#####################
    294.4 |########################################
    295.2 |########################
    295.9 |################################
    296.7 |#############
    297.5 |#############################
    298.3 |########################
    299.0 |##################
    299.8 |########
    300.6 |#############
    301.3 |##########
    302.1 |
    302.9 |########
  (9 below, 3 above range)

bp_skew_low (n=160, range 288.5-305.1 ns)
    288.5 |################
    289.3 |#######################
    290.1 |##########################
    291.0 |#######################
    291.8 |##########################
    292.6 |##############################
    293.5 |#################################
    294.3 |####################
    295.1 |########################################
    296.0 |#################################
    296.8 |##########################
    297.6 |##########################
    298.5 |################
    299.3 |##########################
    300.1 |####################################
    300.9 |####################
    301.8 |######
    302.6 |###
    303.4 |################
    304.3 |###
  (13 below, 10 above range)

```
