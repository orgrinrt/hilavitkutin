# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### acc_atomic_seqcst shows alternating (throttle bounce) (autocorr -0.63)

acc_atomic_seqcst's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (234 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (acc_atomic_relaxed)

The baseline acc_atomic_relaxed is the fastest (292 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### acc_atomic_seqcst's comparison is tie-heavy (15% tied pairs)

15% of paired samples for acc_atomic_seqcst are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### acc_plain is inconsistent: worst-20% is 3.5x its best-20%

acc_plain's best 20% of batches run at 239 ns but its worst 20% at 833 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.1% of the fastest

All 3 variants sit between 292 ns and 293 ns - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### acc_plain's edge over baseline is significant but tiny (0 ns, 0.17%)

acc_plain differs from baseline acc_atomic_relaxed by 0 ns (0.17%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (acc_atomic_relaxed) is the fastest** at 292.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 292.2 ns, slowest 292.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4823d76519bef0a3 |
| Counter frequency | 2112780760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 2823ns | 2170ns | 1775ns | 2067ns | 6139ns | base |
| acc_atomic_seqcst | 2823ns | 2172ns | 1776ns | 2063ns | 6151ns | +0.02% |
| acc_plain | 3029ns | 2174ns | 1779ns | 2376ns | 6241ns | +7.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 378ns | 239ns | 816ns | base | 0.677 |
| acc_atomic_seqcst | 377ns | 239ns | 814ns | -0.40% | 0.680 |
| acc_plain | 408ns | 239ns | 833ns | +7.80% | 0.628 |

## Performance model

- Peak throughput: **1.071 Gops/s** (acc_atomic_relaxed; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.876 | 81.8% |
| acc_atomic_seqcst | 0.876 | 81.8% |
| acc_plain | 0.875 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 2823ns | 2823ns | base |
| acc_atomic_seqcst | 2823ns | 2823ns | +0.02% |
| acc_plain | 3029ns | 3029ns | +7.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 292ns | base | --- | [292, 292] | --- | --- | --- | --- |
| acc_atomic_seqcst | 292ns | no significant difference | [+0, +0]ns | [292, 292] | no | 0.7971 | 0.7971 | **24** (15%, HIGH) |
| acc_plain | 293ns | +0.5ns (+0.2%) | [+0, +1]ns | [293, 293] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 239ns | +0.0% | +101.3% |
| 1 | 2 | 292ns | +0.0% | -18.1% |
| 1 | 3 | 239ns | +22.3% | +22.4% |
| 1 | 4 | 239ns | -0.0% | +22.3% |
| 1 | 5 | 292ns | +0.0% | +0.2% |
| 1 | 6 | 239ns | +0.0% | +0.1% |
| 1 | 7 | 292ns | +0.1% | +0.1% |
| 1 | 8 | 292ns | -18.1% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.375 | moderate- |
| acc_atomic_seqcst | -0.627 | HIGH- (thermal bounce) |
| acc_plain | -0.224 | moderate- |

**Consistency summary:**

- **acc_atomic_seqcst**: won 1/8, lost 2/8
- **acc_plain**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 30.8ns | 378.1ns | 8.2% | HIGH |
| acc_atomic_seqcst | 30.8ns | 376.7ns | 8.2% | HIGH |
| acc_plain | 32.9ns | 407.7ns | 8.1% | HIGH |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 239.0-816.2 ns)
    239.0 |##########################
    267.9 |########################################
    296.8 |######
    325.6 |
    354.5 |
    383.3 |
    412.2 |
    441.1 |
    469.9 |
    498.8 |
    527.6 |
    556.5 |
    585.4 |
    614.2 |
    643.1 |
    671.9 |
    700.8 |##
    729.7 |
    758.5 |##
    787.4 |##
  (20 below, 17 above range)

acc_atomic_seqcst (n=160, range 239.1-813.7 ns)
    239.1 |##############################
    267.8 |########################################
    296.5 |###
    325.2 |
    354.0 |
    382.7 |#
    411.4 |
    440.2 |
    468.9 |
    497.6 |
    526.4 |
    555.1 |
    583.8 |
    612.5 |
    641.3 |
    670.0 |
    698.7 |#
    727.5 |#
    756.2 |
    784.9 |##
  (16 below, 17 above range)

acc_plain (n=160, range 239.4-832.6 ns)
    239.4 |#####
    269.1 |########################################
    298.7 |#
    328.4 |
    358.1 |
    387.7 |
    417.4 |
    447.0 |
    476.7 |#########
    506.3 |
    536.0 |
    565.7 |
    595.3 |
    625.0 |
    654.6 |
    684.3 |
    714.0 |
    743.6 |
    773.3 |
    802.9 |#
  (22 below, 21 above range)

```

## Diagnostics

- **acc_atomic_relaxed**: CV=61.8% (high variance, measurements may be unstable)
- **acc_atomic_relaxed**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **acc_atomic_seqcst**: CV=62.1% (high variance, measurements may be unstable)
- **acc_atomic_seqcst**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **acc_plain**: CV=57.6% (high variance, measurements may be unstable)
- **acc_plain**: worst_20/best_20 = 3.5x (possible bimodal distribution)
