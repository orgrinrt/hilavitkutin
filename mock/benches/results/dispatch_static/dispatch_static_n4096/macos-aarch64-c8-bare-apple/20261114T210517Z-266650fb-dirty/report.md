# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_trait shows alternating (throttle bounce) (autocorr -0.71)

dispatch_trait's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (19 ns) is smaller than the fastest variant's own run-to-run std-dev (37 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (630 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader dispatch_const_ptr vs stability leader dispatch_trait (+0% speed for 1.1x steadier)

dispatch_const_ptr is fastest (630 ns, CV 5.8%); dispatch_trait gives up 0.5% median for 1.1x lower variance (CV 5.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 2.9% of the fastest

All 3 variants sit between 630 ns and 648 ns - a 2.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### dispatch_direct's edge over baseline is significant but tiny (7 ns, 1.06%)

dispatch_direct differs from baseline dispatch_const_ptr by 7 ns (1.06%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 629.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.03x (fastest 629.6 ns, slowest 648.1 ns)

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
| dispatch_const_ptr | 2844ns | 2862ns | 2671ns | 2802ns | 3146ns | base |
| dispatch_direct | 2953ns | 3020ns | 2701ns | 2972ns | 3145ns | +3.80% |
| dispatch_trait | 2892ns | 2870ns | 2689ns | 2888ns | 3104ns | +1.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 626ns | 588ns | 684ns | base | 6.548 |
| dispatch_direct | 649ns | 594ns | 694ns | +3.72% | 6.313 |
| dispatch_trait | 638ns | 594ns | 689ns | +2.01% | 6.419 |

## Performance model

- Peak throughput: **6.966 Gops/s** (dispatch_const_ptr; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 6.506 | 93.4% |
| dispatch_direct | 6.320 | 90.7% |
| dispatch_trait | 6.474 | 92.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2844ns | 2844ns | base |
| dispatch_direct | 2953ns | 2953ns | +3.80% |
| dispatch_trait | 2892ns | 2892ns | +1.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 630ns | base | --- | [599, 632] | --- | --- | --- | --- |
| dispatch_direct | 648ns | +6.6ns (+1.1%) | [+2, +27]ns | [634, 678] | YES | 0.0021 | 0.0010 | 2 |
| dispatch_trait | 633ns | no significant difference | [-1, +30]ns | [631, 635] | no | 0.2326 | 0.2326 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 590ns | +15.0% | +11.3% |
| 1 | 2 | 636ns | +7.0% | -7.1% |
| 1 | 3 | 591ns | +15.6% | +15.4% |
| 1 | 4 | 680ns | -11.0% | -8.6% |
| 1 | 5 | 590ns | +3.5% | +7.9% |
| 1 | 6 | 635ns | +6.8% | +0.0% |
| 1 | 7 | 634ns | -5.0% | -3.2% |
| 1 | 8 | 594ns | +14.6% | +6.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.665 | HIGH- (thermal bounce) |
| dispatch_direct | -0.160 | ok |
| dispatch_trait | -0.708 | HIGH- (thermal bounce) |

**Consistency summary:**

- **dispatch_direct**: won 2/8, lost 6/8
- **dispatch_trait**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 2.9ns | 625.6ns | 0.5% |  |
| dispatch_direct | 3.1ns | 648.8ns | 0.5% |  |
| dispatch_trait | 3.0ns | 638.2ns | 0.5% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 588.0-684.1 ns)
    588.0 |########################################
    592.8 |###############
    597.6 |###
    602.4 |###
    607.2 |#
    612.0 |
    616.8 |#
    621.6 |#
    626.4 |##############
    631.2 |##########################
    636.0 |######
    640.8 |##
    645.6 |
    650.4 |
    655.2 |
    660.0 |
    664.8 |
    669.6 |
    674.5 |##########
    679.3 |##############
  (14 below, 13 above range)

dispatch_direct (n=160, range 594.3-693.5 ns)
    594.3 |#########
    599.2 |###
    604.2 |#####
    609.2 |############
    614.1 |############
    619.1 |#
    624.0 |#
    629.0 |################
    634.0 |##############
    638.9 |#
    643.9 |##
    648.9 |
    653.8 |
    658.8 |#
    663.7 |
    668.7 |#
    673.7 |##############
    678.6 |########################################
    683.6 |################################
    688.5 |#######
  (18 below, 4 above range)

dispatch_trait (n=160, range 593.9-688.8 ns)
    593.9 |##########
    598.6 |#####
    603.4 |##
    608.1 |#########
    612.9 |########
    617.6 |######
    622.3 |#####
    627.1 |##########################
    631.8 |########################################
    636.6 |##########
    641.3 |
    646.1 |#
    650.8 |#
    655.5 |#
    660.3 |
    665.0 |
    669.8 |#
    674.5 |################
    679.3 |########################
    684.0 |#############
  (18 below, 4 above range)

```
