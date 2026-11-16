# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.2% (fastest 591 ns to slowest 592 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (10 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: dispatch_trait** at 590.8 ns median (-0.2% vs baseline)
- Spread: 1.00x (fastest 590.8 ns, slowest 591.7 ns)

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
| dispatch_const_ptr | 2700ns | 2688ns | 2681ns | 2688ns | 2754ns | base |
| dispatch_direct | 2702ns | 2686ns | 2681ns | 2687ns | 2767ns | +0.08% |
| dispatch_trait | 2695ns | 2686ns | 2680ns | 2686ns | 2738ns | -0.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 594ns | 587ns | 610ns | base | 6.892 |
| dispatch_direct | 595ns | 587ns | 611ns | +0.06% | 6.887 |
| dispatch_trait | 594ns | 587ns | 608ns | -0.13% | 6.901 |

## Performance model

- Peak throughput: **6.983 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 6.922 | 99.1% |
| dispatch_direct | 6.922 | 99.1% |
| dispatch_trait | 6.933 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2700ns | 2700ns | base |
| dispatch_direct | 2702ns | 2702ns | +0.08% |
| dispatch_trait | 2695ns | 2695ns | -0.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 592ns | base | --- | [591, 592] | --- | --- | --- | --- |
| dispatch_direct | 592ns | no significant difference | [-0, +0]ns | [591, 592] | no | 1.0000 | 1.0000 | 9 |
| dispatch_trait | 591ns | no significant difference | [-1, +0]ns | [590, 592] | no | 0.4533 | 0.2267 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 590ns | -0.0% | +0.0% |
| 1 | 2 | 592ns | -0.2% | -0.2% |
| 1 | 3 | 591ns | -0.1% | -0.3% |
| 1 | 4 | 591ns | +0.0% | +0.1% |
| 1 | 5 | 591ns | -0.0% | -0.1% |
| 1 | 6 | 612ns | -0.2% | -0.3% |
| 1 | 7 | 590ns | +0.2% | +0.0% |
| 1 | 8 | 591ns | +0.0% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.203 | moderate- |
| dispatch_direct | -0.141 | ok |
| dispatch_trait | -0.169 | ok |

**Consistency summary:**

- **dispatch_direct**: won 2/8, lost 1/8
- **dispatch_trait**: won 5/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 2.7ns | 594.3ns | 0.5% |  |
| dispatch_direct | 2.7ns | 594.7ns | 0.5% |  |
| dispatch_trait | 2.9ns | 593.6ns | 0.5% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 586.6-609.6 ns)
    586.6 |#####################
    587.7 |####################
    588.9 |##############################
    590.0 |#######################
    591.2 |########################################
    592.3 |#########################
    593.5 |###############
    594.6 |###########
    595.8 |###############
    596.9 |#
    598.1 |
    599.2 |
    600.4 |#
    601.5 |
    602.7 |###
    603.8 |###
    605.0 |#
    606.1 |#
    607.3 |###
    608.4 |#
  (12 below, 16 above range)

dispatch_direct (n=160, range 586.8-611.3 ns)
    586.8 |####################
    588.1 |#########################
    589.3 |######################
    590.5 |########################################
    591.7 |######################
    593.0 |###########
    594.2 |#########################
    595.4 |#######
    596.6 |####
    597.9 |##
    599.1 |
    600.3 |
    601.5 |
    602.8 |
    604.0 |#
    605.2 |
    606.4 |##
    607.7 |####
    608.9 |##
    610.1 |####
  (14 below, 12 above range)

dispatch_trait (n=160, range 586.6-608.3 ns)
    586.6 |######################################
    587.6 |####################
    588.7 |########################################
    589.8 |########################################
    590.9 |#################
    592.0 |################################
    593.1 |####################
    594.2 |########################
    595.3 |#######
    596.3 |###
    597.4 |#
    598.5 |#
    599.6 |#
    600.7 |
    601.8 |
    602.9 |
    604.0 |
    605.0 |#
    606.1 |#
    607.2 |
  (11 below, 15 above range)

```
