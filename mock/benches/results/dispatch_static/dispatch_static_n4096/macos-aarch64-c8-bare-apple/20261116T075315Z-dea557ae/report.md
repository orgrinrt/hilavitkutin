# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (dispatch_const_ptr) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline dispatch_const_ptr has the worst median (597 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest dispatch_direct at 591 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6 ns) is smaller than the fastest variant's own run-to-run std-dev (12 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.0% of the fastest

All 3 variants sit between 591 ns and 597 ns - a 1.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### dispatch_direct's edge over baseline is significant but tiny (-3 ns, 0.43%)

dispatch_direct differs from baseline dispatch_const_ptr by -3 ns (0.43%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: dispatch_direct** at 591.2 ns median (-1.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 591.2 ns, slowest 597.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Static dispatch shapes: direct vs sealed-trait vs const fn pointer |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x7eedfacedeadbeef |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2706ns | 2718ns | 2668ns | 2706ns | 2746ns | base |
| dispatch_direct | 2693ns | 2678ns | 2668ns | 2683ns | 2749ns | -0.49% |
| dispatch_trait | 2697ns | 2680ns | 2667ns | 2693ns | 2738ns | -0.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 597ns | 588ns | 606ns | base | 6.860 |
| dispatch_direct | 594ns | 586ns | 610ns | -0.46% | 6.891 |
| dispatch_trait | 596ns | 588ns | 608ns | -0.17% | 6.871 |

## Performance model

- Peak throughput: **6.984 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 6.858 | 98.2% |
| dispatch_direct | 6.928 | 99.2% |
| dispatch_trait | 6.893 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2706ns | 2706ns | base |
| dispatch_direct | 2693ns | 2693ns | -0.49% |
| dispatch_trait | 2697ns | 2697ns | -0.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 597ns | base | --- | [596, 599] | --- | --- | --- | --- |
| dispatch_direct | 591ns | -2.5ns (-0.4%) | [-4, -2]ns | [590, 592] | YES | 0.0000 | 0.0000 | 10 |
| dispatch_trait | 594ns | no significant difference | [-1, +0]ns | [593, 596] | no | 0.2267 | 0.2267 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 591ns | +0.0% | +0.5% |
| 1 | 2 | 601ns | -0.6% | -1.7% |
| 1 | 3 | 590ns | +0.0% | +0.3% |
| 1 | 4 | 591ns | -0.1% | -0.0% |
| 1 | 5 | 601ns | -1.7% | +0.1% |
| 1 | 6 | 602ns | -0.6% | -0.6% |
| 1 | 7 | 602ns | -1.8% | -0.7% |
| 1 | 8 | 602ns | -1.8% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.159 | ok |
| dispatch_direct | -0.346 | moderate- |
| dispatch_trait | 0.287 | moderate+ |

**Consistency summary:**

- **dispatch_direct**: won 5/8, lost 0/8
- **dispatch_trait**: won 4/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 2.9ns | 597.1ns | 0.5% |  |
| dispatch_direct | 3.0ns | 594.4ns | 0.5% |  |
| dispatch_trait | 3.0ns | 596.1ns | 0.5% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 588.1-606.1 ns)
    588.1 |####################
    589.0 |################
    589.9 |##############################
    590.8 |#######################
    591.7 |########################################
    592.6 |###
    593.5 |##########################
    594.4 |#############
    595.3 |#############
    596.2 |################
    597.1 |#######################
    598.0 |#################################
    598.9 |######
    599.8 |##########################
    600.7 |#######################
    601.6 |##############################
    602.5 |########################################
    603.4 |####################
    604.3 |################
    605.2 |#############
  (14 below, 15 above range)

dispatch_direct (n=160, range 586.5-609.7 ns)
    586.5 |#################
    587.6 |########################################
    588.8 |###############
    590.0 |####################################
    591.1 |###########################
    592.3 |#############
    593.5 |###############
    594.6 |##########
    595.8 |#################
    596.9 |##########
    598.1 |#####
    599.3 |#####
    600.4 |#####
    601.6 |#####
    602.8 |###
    603.9 |#
    605.1 |#####
    606.3 |###
    607.4 |#####
    608.6 |#
  (13 below, 5 above range)

dispatch_trait (n=160, range 588.2-608.2 ns)
    588.2 |##############################
    589.2 |####################
    590.2 |########################################
    591.2 |##########
    592.2 |##################
    593.2 |####################
    594.2 |############
    595.2 |############
    596.2 |############
    597.2 |################
    598.2 |############
    599.2 |##############
    600.2 |########################
    601.2 |####
    602.2 |##########
    603.2 |####
    604.2 |##
    605.2 |############
    606.2 |####
    607.2 |########
  (13 below, 5 above range)

```
