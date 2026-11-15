# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bp_balanced) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bp_balanced has the worst median (333 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bp_skew_high at 332 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.2% (fastest 332 ns to slowest 333 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### bp_skew_high is fastest but the noisiest (CV 6.5%)

bp_skew_high wins on median (332 ns) yet has the highest variance (CV 6.5%), while bp_skew_low is the steadiest (CV 4.9%, 332 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (21 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bp_skew_high vs stability leader bp_skew_low (+0% speed for 1.3x steadier)

bp_skew_high is fastest (332 ns, CV 6.5%); bp_skew_low gives up 0.0% median for 1.3x lower variance (CV 4.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bp_skew_high** at 332.5 ns median (-0.2% vs baseline)
- Spread: 1.00x (fastest 332.5 ns, slowest 333.1 ns)

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
| bp_balanced | 2651ns | 2735ns | 2436ns | 2688ns | 2757ns | base |
| bp_skew_high | 2628ns | 2720ns | 2420ns | 2653ns | 2757ns | -0.89% |
| bp_skew_low | 2631ns | 2728ns | 2424ns | 2662ns | 2747ns | -0.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 327ns | 299ns | 342ns | base | 0.784 |
| bp_skew_high | 326ns | 297ns | 351ns | -0.03% | 0.784 |
| bp_skew_low | 325ns | 300ns | 342ns | -0.44% | 0.788 |

## Performance model

- Peak throughput: **0.861 Gops/s** (bp_skew_high; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.769 | 89.3% |
| bp_skew_high | 0.770 | 89.5% |
| bp_skew_low | 0.770 | 89.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 2651ns | 2651ns | base |
| bp_skew_high | 2628ns | 2628ns | -0.89% |
| bp_skew_low | 2631ns | 2631ns | -0.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 333ns | base | --- | [331, 336] | --- | --- | --- | --- |
| bp_skew_high | 332ns | no significant difference | [-1, +2]ns | [325, 335] | no | 0.7465 | 0.7465 | 7 |
| bp_skew_low | 332ns | no significant difference | [-2, +0]ns | [328, 335] | no | 0.3972 | 0.1986 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 337ns | -11.7% | -6.8% |
| 1 | 2 | 333ns | -0.7% | +0.8% |
| 1 | 3 | 338ns | -0.4% | -10.3% |
| 1 | 4 | 332ns | -8.3% | -0.1% |
| 1 | 5 | 339ns | -1.9% | -1.9% |
| 1 | 6 | 293ns | +14.3% | +15.1% |
| 1 | 7 | 335ns | +1.3% | +0.7% |
| 1 | 8 | 326ns | +4.2% | +3.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.289 | moderate- |
| bp_skew_high | -0.067 | ok |
| bp_skew_low | -0.136 | ok |

**Consistency summary:**

- **bp_skew_high**: won 5/8, lost 3/8
- **bp_skew_low**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 2.9ns | 326.5ns | 0.9% |  |
| bp_skew_high | 3.1ns | 326.4ns | 0.9% |  |
| bp_skew_low | 3.0ns | 325.1ns | 0.9% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 299.1-342.3 ns)
    299.1 |
    301.3 |#
    303.4 |
    305.6 |#
    307.7 |#
    309.9 |#####
    312.1 |###########
    314.2 |######################
    316.4 |###################
    318.5 |###
    320.7 |#
    322.8 |
    325.0 |#
    327.2 |###
    329.3 |###############
    331.5 |####################################
    333.6 |###########
    335.8 |########################
    338.0 |########################################
    340.1 |########################################
  (20 below, 12 above range)

bp_skew_high (n=160, range 297.4-350.5 ns)
    297.4 |#########
    300.1 |#######
    302.7 |#############
    305.4 |#######
    308.1 |#######
    310.7 |#########
    313.4 |#######
    316.0 |##########################
    318.7 |#####
    321.3 |###
    324.0 |#
    326.6 |#######
    329.3 |#######
    331.9 |#################
    334.6 |########################################
    337.3 |####################################
    339.9 |################################
    342.6 |###########
    345.2 |###########
    347.9 |#
  (15 below, 5 above range)

bp_skew_low (n=160, range 299.8-342.2 ns)
    299.8 |##
    301.9 |##########
    304.0 |################
    306.2 |############
    308.3 |################
    310.4 |######
    312.5 |########
    314.7 |##########
    316.8 |########
    318.9 |##########
    321.0 |##
    323.1 |##
    325.3 |
    327.4 |##########
    329.5 |########
    331.6 |###########################
    333.7 |##################
    335.9 |###############################
    338.0 |########################################
    340.1 |#################################
  (15 below, 13 above range)

```
