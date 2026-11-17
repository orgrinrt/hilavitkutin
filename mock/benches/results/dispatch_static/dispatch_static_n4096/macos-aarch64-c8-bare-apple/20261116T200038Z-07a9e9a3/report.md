# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (dispatch_const_ptr) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline dispatch_const_ptr has the worst median (598 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest dispatch_trait at 592 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5 ns) is smaller than the fastest variant's own run-to-run std-dev (5 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.8% of the fastest

All 3 variants sit between 592 ns and 598 ns - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### dispatch_direct's edge over baseline is significant but tiny (-1 ns, 0.22%)

dispatch_direct differs from baseline dispatch_const_ptr by -1 ns (0.22%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: dispatch_trait** at 592.5 ns median (-0.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.01x (fastest 592.5 ns, slowest 597.5 ns)

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
| dispatch_const_ptr | 2725ns | 2731ns | 2681ns | 2717ns | 2793ns | base |
| dispatch_direct | 2709ns | 2691ns | 2681ns | 2702ns | 2759ns | -0.57% |
| dispatch_trait | 2704ns | 2691ns | 2682ns | 2696ns | 2747ns | -0.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 599ns | 588ns | 614ns | base | 6.842 |
| dispatch_direct | 597ns | 587ns | 615ns | -0.28% | 6.862 |
| dispatch_trait | 593ns | 587ns | 602ns | -0.87% | 6.902 |

## Performance model

- Peak throughput: **6.978 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 6.855 | 98.2% |
| dispatch_direct | 6.901 | 98.9% |
| dispatch_trait | 6.913 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2725ns | 2725ns | base |
| dispatch_direct | 2709ns | 2709ns | -0.57% |
| dispatch_trait | 2704ns | 2704ns | -0.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 598ns | base | --- | [595, 599] | --- | --- | --- | --- |
| dispatch_direct | 594ns | -1.3ns (-0.2%) | [-3, -1]ns | [592, 595] | YES | 0.0051 | 0.0026 | 8 |
| dispatch_trait | 592ns | -5.6ns (-0.9%) | [-8, -1]ns | [592, 593] | YES | 0.0058 | 0.0058 | 7 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 593ns | +1.3% | +1.4% |
| 1 | 2 | 594ns | -0.3% | -0.5% |
| 1 | 3 | 591ns | +0.9% | +1.2% |
| 1 | 4 | 599ns | -1.3% | -1.4% |
| 1 | 5 | 601ns | -1.6% | -1.8% |
| 1 | 6 | 600ns | -0.9% | -1.3% |
| 1 | 7 | 594ns | -0.0% | -0.4% |
| 1 | 8 | 614ns | -2.8% | -3.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.024 | ok |
| dispatch_direct | -0.209 | moderate- |
| dispatch_trait | -0.245 | moderate- |

**Consistency summary:**

- **dispatch_direct**: won 5/8, lost 2/8
- **dispatch_trait**: won 6/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 2.9ns | 598.6ns | 0.5% |  |
| dispatch_direct | 2.9ns | 596.9ns | 0.5% |  |
| dispatch_trait | 2.6ns | 593.4ns | 0.4% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 587.7-613.5 ns)
    587.7 |########################
    589.0 |##############################
    590.3 |########################
    591.6 |########################
    592.9 |########################################
    594.2 |#####################
    595.5 |########################
    596.8 |###########################
    598.0 |#####################
    599.3 |########################
    600.6 |###########################
    601.9 |########################
    603.2 |#####################
    604.5 |#####################
    605.8 |###
    607.1 |##################
    608.4 |############
    609.6 |######
    610.9 |######
    612.2 |######
  (15 below, 11 above range)

dispatch_direct (n=160, range 587.3-615.2 ns)
    587.3 |############
    588.7 |###############################
    590.0 |###############################
    591.4 |########################################
    592.8 |#############################
    594.2 |################
    595.6 |#########################
    597.0 |#########################
    598.4 |##########
    599.8 |########
    601.2 |############
    602.6 |##########
    604.0 |##############
    605.4 |################
    606.8 |####
    608.2 |##
    609.6 |
    611.0 |
    612.4 |
    613.8 |
  (18 below, 3 above range)

dispatch_trait (n=160, range 587.0-602.0 ns)
    587.0 |###############
    587.8 |####################################
    588.5 |#################################
    589.3 |###########################
    590.0 |##################
    590.8 |#################################
    591.5 |########################################
    592.3 |########################################
    593.0 |###############
    593.8 |##################
    594.5 |###########################
    595.3 |########################
    596.0 |######
    596.8 |############
    597.5 |######
    598.3 |##################
    599.0 |#########
    599.8 |######
    600.5 |############
    601.3 |###############
  (11 below, 13 above range)

```
