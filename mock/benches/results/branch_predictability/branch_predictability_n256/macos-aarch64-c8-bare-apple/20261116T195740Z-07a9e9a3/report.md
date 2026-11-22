# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bp_balanced) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bp_balanced has the worst median (294 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bp_skew_high at 294 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (5 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.1% of the fastest

All 3 variants sit between 294 ns and 294 ns - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bp_skew_high's edge over baseline is significant but tiny (-1 ns, 0.27%)

bp_skew_high differs from baseline bp_balanced by -1 ns (0.27%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bp_skew_high** at 293.8 ns median (-0.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 293.8 ns, slowest 294.2 ns)

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
| bp_balanced | 2395ns | 2382ns | 2374ns | 2389ns | 2435ns | base |
| bp_skew_high | 2389ns | 2380ns | 2374ns | 2382ns | 2426ns | -0.26% |
| bp_skew_low | 2394ns | 2383ns | 2372ns | 2388ns | 2433ns | -0.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 295ns | 287ns | 303ns | base | 0.869 |
| bp_skew_high | 293ns | 286ns | 300ns | -0.42% | 0.873 |
| bp_skew_low | 294ns | 287ns | 300ns | -0.31% | 0.872 |

## Performance model

- Peak throughput: **0.894 Gops/s** (bp_skew_high; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.870 | 97.4% |
| bp_skew_high | 0.871 | 97.5% |
| bp_skew_low | 0.871 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 2395ns | 2395ns | base |
| bp_skew_high | 2389ns | 2389ns | -0.26% |
| bp_skew_low | 2394ns | 2394ns | -0.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 294ns | base | --- | [294, 295] | --- | --- | --- | --- |
| bp_skew_high | 294ns | -0.8ns (-0.3%) | [-1, -0]ns | [292, 294] | YES | 0.0314 | 0.0157 | 5 |
| bp_skew_low | 294ns | no significant difference | [-1, +0]ns | [293, 295] | no | 0.2917 | 0.2917 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 293ns | +0.4% | +0.0% |
| 1 | 2 | 292ns | +0.2% | +0.2% |
| 1 | 3 | 294ns | -0.1% | -0.4% |
| 1 | 4 | 293ns | +0.1% | +0.4% |
| 1 | 5 | 300ns | -0.8% | -2.1% |
| 1 | 6 | 294ns | -0.4% | +1.5% |
| 1 | 7 | 298ns | -1.7% | -0.8% |
| 1 | 8 | 293ns | -0.2% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.230 | moderate- |
| bp_skew_high | -0.222 | moderate- |
| bp_skew_low | 0.288 | moderate+ |

**Consistency summary:**

- **bp_skew_high**: won 4/8, lost 2/8
- **bp_skew_low**: won 3/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 2.6ns | 294.6ns | 0.9% |  |
| bp_skew_high | 2.7ns | 293.3ns | 0.9% |  |
| bp_skew_low | 2.7ns | 293.7ns | 0.9% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 287.4-303.0 ns)
    287.4 |####################
    288.2 |####################
    289.0 |######################
    289.8 |###############
    290.5 |#################
    291.3 |###############
    292.1 |############
    292.9 |#################
    293.6 |########################################
    294.4 |##############################
    295.2 |##########
    296.0 |##############################
    296.8 |################################
    297.5 |###############
    298.3 |##########
    299.1 |##########
    299.9 |############
    300.6 |############
    301.4 |#######
    302.2 |##
  (11 below, 8 above range)

bp_skew_high (n=160, range 286.5-300.1 ns)
    286.5 |##############
    287.2 |#######
    287.8 |################
    288.5 |#######
    289.2 |####
    289.9 |##################
    290.6 |##############
    291.2 |###########
    291.9 |########################################
    292.6 |###########
    293.3 |##############################
    294.0 |############################
    294.6 |###########
    295.3 |#####################################
    296.0 |#######
    296.7 |##################
    297.3 |##############
    298.0 |#######
    298.7 |####
    299.4 |#######
  (14 below, 13 above range)

bp_skew_low (n=160, range 286.9-300.2 ns)
    286.9 |#####################
    287.6 |########
    288.2 |########
    288.9 |##
    289.6 |#############
    290.2 |################################
    290.9 |#############
    291.5 |################
    292.2 |################
    292.9 |##################################
    293.5 |########################################
    294.2 |#############
    294.9 |#####################################
    295.5 |########
    296.2 |##########################
    296.9 |########################
    297.5 |##########
    298.2 |#############
    298.9 |##
    299.5 |################
  (13 below, 13 above range)

```
