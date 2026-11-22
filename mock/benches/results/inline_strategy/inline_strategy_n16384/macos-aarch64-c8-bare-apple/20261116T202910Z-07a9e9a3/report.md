# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (59 ns) is smaller than the fastest variant's own run-to-run std-dev (149 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (20.41 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.3% of the fastest

All 3 variants sit between 20.41 us and 20.47 us - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### inline_never's edge over baseline is significant but tiny (49 ns, 0.24%)

inline_never differs from baseline inline_always by 49 ns (0.24%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (inline_always) is the fastest** at 20407.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 20407.7 ns, slowest 20466.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x39f847b2c651de80 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 22575ns | 22527ns | 22500ns | 22533ns | 22776ns | base |
| inline_default | 22560ns | 22518ns | 22500ns | 22526ns | 22720ns | -0.07% |
| inline_never | 22616ns | 22565ns | 22531ns | 22571ns | 22834ns | +0.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 20464ns | 20399ns | 20647ns | base | 0.801 |
| inline_default | 20452ns | 20399ns | 20601ns | -0.06% | 0.801 |
| inline_never | 20521ns | 20448ns | 20720ns | +0.28% | 0.798 |

## Performance model

- Peak throughput: **0.803 Gops/s** (inline_always; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.803 | 100.0% |
| inline_default | 0.803 | 100.0% |
| inline_never | 0.801 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 22575ns | 22575ns | base |
| inline_default | 22560ns | 22560ns | -0.07% |
| inline_never | 22616ns | 22616ns | +0.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 20408ns | base | --- | [20405, 20434] | --- | --- | --- | --- |
| inline_default | 20409ns | no significant difference | [-2, +2]ns | [20406, 20418] | no | 0.4278 | 0.4278 | 1 |
| inline_never | 20467ns | +49.4ns (+0.2%) | [+47, +54]ns | [20457, 20490] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 20446ns | -0.2% | +0.0% |
| 1 | 2 | 20417ns | -0.1% | +0.4% |
| 1 | 3 | 20404ns | +0.0% | +0.4% |
| 1 | 4 | 20446ns | -0.2% | +0.2% |
| 1 | 5 | 20407ns | +0.0% | +0.2% |
| 1 | 6 | 20425ns | -0.0% | +0.1% |
| 1 | 7 | 20406ns | +0.1% | +0.3% |
| 1 | 8 | 20405ns | +0.0% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.328 | moderate- |
| inline_default | 0.369 | moderate+ |
| inline_never | 0.277 | moderate+ |

**Consistency summary:**

- **inline_default**: won 2/8, lost 1/8
- **inline_never**: won 0/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 3.0ns | 20463.6ns | 0.0% |  |
| inline_default | 3.0ns | 20452.1ns | 0.0% |  |
| inline_never | 3.7ns | 20520.7ns | 0.0% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 20399.2-20647.1 ns)
  20399.2 |########################################
  20411.5 |###
  20423.9 |#
  20436.3 |######
  20448.7 |#######
  20461.1 |###
  20473.5 |####
  20485.9 |###
  20498.3 |#
  20510.7 |#
  20523.1 |
  20535.5 |#
  20547.9 |
  20560.3 |
  20572.7 |
  20585.1 |#
  20597.5 |
  20609.9 |
  20622.3 |
  20634.7 |
  (12 below, 7 above range)

inline_default (n=160, range 20399.4-20601.3 ns)
  20399.4 |########################################
  20409.5 |######
  20419.6 |####
  20429.7 |####
  20439.8 |###
  20449.9 |###
  20460.0 |######
  20470.1 |
  20480.2 |###
  20490.3 |##
  20500.3 |
  20510.4 |#
  20520.5 |#
  20530.6 |
  20540.7 |
  20550.8 |
  20560.9 |
  20571.0 |
  20581.1 |
  20591.2 |
  (12 below, 11 above range)

inline_never (n=160, range 20448.1-20720.3 ns)
  20448.1 |########################################
  20461.7 |######
  20475.3 |###
  20488.9 |##############
  20502.5 |###
  20516.1 |###
  20529.7 |######
  20543.4 |###
  20557.0 |
  20570.6 |#
  20584.2 |
  20597.8 |#
  20611.4 |#
  20625.0 |#
  20638.6 |
  20652.2 |#
  20665.8 |
  20679.4 |
  20693.0 |
  20706.7 |
  (14 below, 8 above range)

```
