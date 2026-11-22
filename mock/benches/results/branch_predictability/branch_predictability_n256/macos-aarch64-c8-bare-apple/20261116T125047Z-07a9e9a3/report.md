# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bp_balanced) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bp_balanced has the worst median (294 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bp_skew_high at 292 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (5 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.6% of the fastest

All 3 variants sit between 292 ns and 294 ns - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bp_skew_high's edge over baseline is significant but tiny (-1 ns, 0.31%)

bp_skew_high differs from baseline bp_balanced by -1 ns (0.31%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bp_skew_high** at 292.5 ns median (-0.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.01x (fastest 292.5 ns, slowest 294.2 ns)

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
| bp_balanced | 2397ns | 2382ns | 2375ns | 2383ns | 2461ns | base |
| bp_skew_high | 2382ns | 2380ns | 2374ns | 2380ns | 2394ns | -0.63% |
| bp_skew_low | 2384ns | 2381ns | 2375ns | 2381ns | 2402ns | -0.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 294ns | 288ns | 301ns | base | 0.870 |
| bp_skew_high | 293ns | 286ns | 299ns | -0.51% | 0.875 |
| bp_skew_low | 293ns | 287ns | 300ns | -0.40% | 0.874 |

## Performance model

- Peak throughput: **0.894 Gops/s** (bp_skew_high; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.870 | 97.4% |
| bp_skew_high | 0.875 | 97.9% |
| bp_skew_low | 0.875 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 2397ns | 2397ns | base |
| bp_skew_high | 2382ns | 2382ns | -0.63% |
| bp_skew_low | 2384ns | 2384ns | -0.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 294ns | base | --- | [293, 295] | --- | --- | --- | --- |
| bp_skew_high | 292ns | -0.9ns (-0.3%) | [-2, -0]ns | [292, 293] | YES | 0.0002 | 0.0002 | 9 |
| bp_skew_low | 292ns | -1.3ns (-0.4%) | [-2, -1]ns | [292, 293] | YES | 0.0002 | 0.0002 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 292ns | +0.3% | +0.4% |
| 1 | 2 | 293ns | -0.1% | -0.1% |
| 1 | 3 | 294ns | -0.6% | -0.4% |
| 1 | 4 | 294ns | -0.0% | -0.4% |
| 1 | 5 | 293ns | -0.3% | -0.0% |
| 1 | 6 | 294ns | -0.2% | -0.7% |
| 1 | 7 | 294ns | -0.3% | -0.6% |
| 1 | 8 | 302ns | -3.2% | -3.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | 0.081 | ok |
| bp_skew_high | -0.170 | ok |
| bp_skew_low | -0.364 | moderate- |

**Consistency summary:**

- **bp_skew_high**: won 6/8, lost 1/8
- **bp_skew_low**: won 5/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 2.4ns | 294.1ns | 0.8% |  |
| bp_skew_high | 2.6ns | 292.6ns | 0.9% |  |
| bp_skew_low | 2.7ns | 292.9ns | 0.9% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 287.5-301.4 ns)
    287.5 |######
    288.2 |#####################
    288.9 |#####################
    289.6 |########################
    290.3 |#################################
    291.0 |######
    291.7 |########################################
    292.4 |###############
    293.1 |###############
    293.8 |###########################
    294.5 |########################################
    295.2 |########################################
    295.8 |#########
    296.5 |####################################
    297.2 |##############################
    297.9 |###
    298.6 |###############
    299.3 |#########
    300.0 |#########
    300.7 |######
  (13 below, 13 above range)

bp_skew_high (n=160, range 286.4-298.9 ns)
    286.4 |#######
    287.1 |###########
    287.7 |##############
    288.3 |#######
    288.9 |###########
    289.6 |#########################
    290.2 |################
    290.8 |#########
    291.4 |##############
    292.1 |########################################
    292.7 |########################################
    293.3 |##############
    293.9 |###########
    294.6 |##############################
    295.2 |##################
    295.8 |################
    296.4 |#######
    297.1 |#########
    297.7 |##############
    298.3 |#######
  (14 below, 7 above range)

bp_skew_low (n=160, range 286.6-300.2 ns)
    286.6 |#####################
    287.3 |########
    288.0 |########
    288.6 |##################
    289.3 |################
    290.0 |######
    290.7 |################
    291.4 |##################
    292.0 |########################################
    292.7 |################
    293.4 |##########
    294.1 |###############################
    294.8 |#############################
    295.4 |##############
    296.1 |#######################
    296.8 |########
    297.5 |####
    298.2 |####
    298.9 |##
    299.5 |##
  (11 below, 5 above range)

```
