# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (275 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (4.57 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader inline_always vs stability leader inline_never (+0% speed for 45.5x steadier)

inline_always is fastest (4.57 us, CV 6.0%); inline_never gives up 0.0% median for 45.5x lower variance (CV 0.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.0% of the fastest

All 3 variants sit between 4.57 us and 4.57 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### inline_default's edge over baseline is significant but tiny (0 ns, 0.01%)

inline_default differs from baseline inline_always by 0 ns (0.01%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (inline_always) is the fastest** at 4567.4 ns median
- 2 variants significantly slower than baseline
- Spread: 1.00x (fastest 4567.4 ns, slowest 4567.8 ns)

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
| Counter frequency | 2112795995 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 6310ns | 6453ns | 5716ns | 6452ns | 6480ns | base |
| inline_default | 6493ns | 6456ns | 6443ns | 6454ns | 6661ns | +2.90% |
| inline_never | 6460ns | 6457ns | 6445ns | 6456ns | 6485ns | +2.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 4468ns | 4050ns | 4580ns | base | 0.917 |
| inline_default | 4608ns | 4567ns | 4756ns | +3.13% | 0.889 |
| inline_never | 4573ns | 4567ns | 4580ns | +2.35% | 0.896 |

## Performance model

- Peak throughput: **1.011 Gops/s** (inline_always; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.897 | 88.7% |
| inline_default | 0.897 | 88.7% |
| inline_never | 0.897 | 88.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 6310ns | 6310ns | base |
| inline_default | 6493ns | 6493ns | +2.90% |
| inline_never | 6460ns | 6460ns | +2.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 4567ns | base | --- | [4567, 4568] | --- | --- | --- | --- |
| inline_default | 4568ns | +0.3ns (+0.0%) | [+0, +1]ns | [4567, 4578] | YES | 0.0061 | 0.0061 | 5 |
| inline_never | 4568ns | +0.5ns (+0.0%) | [+0, +1]ns | [4568, 4578] | YES | 0.0000 | 0.0000 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 4568ns | +0.2% | +0.0% |
| 1 | 2 | 3737ns | +22.2% | +22.4% |
| 1 | 3 | 4578ns | -0.0% | -0.1% |
| 1 | 4 | 4572ns | -0.0% | -0.1% |
| 1 | 5 | 4567ns | +0.3% | +0.1% |
| 1 | 6 | 4568ns | +0.0% | +0.0% |
| 1 | 7 | 4567ns | +0.0% | +0.2% |
| 1 | 8 | 4578ns | -0.2% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.166 | ok |
| inline_default | -0.195 | ok |
| inline_never | -0.039 | ok |

**Consistency summary:**

- **inline_default**: won 1/8, lost 3/8
- **inline_never**: won 2/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 23.0ns | 4468.2ns | 0.5% |  |
| inline_default | 23.5ns | 4608.2ns | 0.5% |  |
| inline_never | 23.8ns | 4573.1ns | 0.5% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 4049.7-4579.6 ns)
   4049.7 |
   4076.2 |
   4102.7 |
   4129.1 |
   4155.6 |
   4182.1 |
   4208.6 |
   4235.1 |
   4261.6 |
   4288.1 |
   4314.6 |
   4341.1 |
   4367.6 |
   4394.1 |
   4420.6 |
   4447.1 |
   4473.6 |
   4500.1 |
   4526.6 |
   4553.1 |########################################
  (20 below, 8 above range)

inline_default (n=160, range 4567.1-4756.4 ns)
   4567.1 |#######################################
   4576.5 |########################################
   4586.0 |#
   4595.5 |
   4604.9 |
   4614.4 |
   4623.9 |
   4633.3 |
   4642.8 |
   4652.3 |
   4661.7 |
   4671.2 |
   4680.7 |
   4690.1 |
   4699.6 |
   4709.1 |
   4718.5 |
   4728.0 |
   4737.5 |
   4746.9 |
  (11 below, 1 above range)

inline_never (n=160, range 4567.4-4580.4 ns)
   4567.4 |########################################
   4568.0 |
   4568.7 |
   4569.3 |
   4570.0 |
   4570.6 |
   4571.3 |
   4571.9 |
   4572.6 |
   4573.2 |
   4573.9 |
   4574.5 |
   4575.2 |
   4575.8 |
   4576.5 |
   4577.1 |
   4577.8 |########
   4578.4 |###############
   4579.1 |##########
   4579.7 |###
  (12 below, 7 above range)

```
