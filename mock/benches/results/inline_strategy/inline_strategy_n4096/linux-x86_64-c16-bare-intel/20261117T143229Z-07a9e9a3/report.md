# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (760 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (3.75 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### inline_never is inconsistent: worst-20% is 2.4x its best-20%

inline_never's best 20% of batches run at 3.74 us but its worst 20% at 8.94 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.1% of the fastest

All 3 variants sit between 3.75 us and 3.75 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### inline_never's edge over baseline is significant but tiny (1 ns, 0.01%)

inline_never differs from baseline inline_always by 1 ns (0.01%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (inline_always) is the fastest** at 3746.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 3746.1 ns, slowest 3748.5 ns)

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
| Counter frequency | 2112795803 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 5650ns | 5289ns | 5277ns | 5308ns | 7048ns | base |
| inline_default | 6883ns | 5294ns | 5278ns | 5535ns | 12529ns | +21.82% |
| inline_never | 6913ns | 5292ns | 5277ns | 5634ns | 12387ns | +22.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 3995ns | 3737ns | 4978ns | base | 1.025 |
| inline_default | 4856ns | 3737ns | 8806ns | +21.57% | 0.843 |
| inline_never | 4927ns | 3737ns | 8939ns | +23.34% | 0.831 |

## Performance model

- Peak throughput: **1.096 Gops/s** (inline_always; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 1.093 | 99.8% |
| inline_default | 1.093 | 99.7% |
| inline_never | 1.093 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 5650ns | 5650ns | base |
| inline_default | 6883ns | 6883ns | +21.82% |
| inline_never | 6913ns | 6913ns | +22.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 3746ns | base | --- | [3737, 3747] | --- | --- | --- | --- |
| inline_default | 3747ns | no significant difference | [-0, +10]ns | [3746, 3752] | no | 0.2979 | 0.2979 | 4 |
| inline_never | 3748ns | +0.5ns (+0.0%) | [+0, +10]ns | [3747, 3775] | YES | 0.0000 | 0.0000 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 4568ns | -18.2% | +0.0% |
| 1 | 2 | 3737ns | +0.2% | +0.0% |
| 1 | 3 | 3737ns | +22.2% | +0.3% |
| 1 | 4 | 3746ns | -0.2% | +22.2% |
| 1 | 5 | 3737ns | +0.4% | +10.0% |
| 1 | 6 | 3899ns | -4.1% | -4.1% |
| 1 | 7 | 3737ns | +2.0% | +0.0% |
| 1 | 8 | 3741ns | +22.1% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.063 | ok |
| inline_default | -0.183 | ok |
| inline_never | -0.060 | ok |

**Consistency summary:**

- **inline_default**: won 3/8, lost 5/8
- **inline_never**: won 1/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 20.8ns | 3994.7ns | 0.5% |  |
| inline_default | 25.1ns | 4856.5ns | 0.5% |  |
| inline_never | 24.9ns | 4926.9ns | 0.5% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 3736.8-4978.4 ns)
   3736.8 |########################################
   3798.8 |#
   3860.9 |
   3923.0 |#
   3985.1 |
   4047.2 |
   4109.2 |
   4171.3 |
   4233.4 |
   4295.5 |
   4357.6 |
   4419.7 |
   4481.7 |
   4543.8 |#######
   4605.9 |
   4668.0 |
   4730.1 |
   4792.1 |
   4854.2 |
   4916.3 |
  (14 below, 5 above range)

inline_default (n=160, range 3736.9-8805.8 ns)
   3736.9 |########################################
   3990.3 |#
   4243.8 |#
   4497.2 |################
   4750.7 |
   5004.1 |
   5257.6 |
   5511.0 |
   5764.4 |
   6017.9 |
   6271.3 |
   6524.8 |
   6778.2 |
   7031.7 |
   7285.1 |
   7538.6 |
   7792.0 |
   8045.5 |
   8298.9 |
   8552.4 |
  (10 below, 6 above range)

inline_never (n=160, range 3737.1-8939.1 ns)
   3737.1 |########################################
   3997.2 |
   4257.3 |#
   4517.4 |####################
   4777.5 |#
   5037.6 |
   5297.7 |
   5557.8 |
   5817.9 |
   6078.0 |
   6338.1 |#
   6598.2 |
   6858.3 |
   7118.4 |
   7378.5 |
   7638.6 |
   7898.7 |#
   8158.8 |
   8418.9 |
   8679.0 |
  (22 below, 5 above range)

```

## Diagnostics

- **inline_default**: CV=108.0% (high variance, measurements may be unstable)
- **inline_never**: CV=102.5% (high variance, measurements may be unstable)
