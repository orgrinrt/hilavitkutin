# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### acc_atomic_seqcst is fastest but the noisiest (CV 82.3%)

acc_atomic_seqcst wins on median (292 ns) yet has the highest variance (CV 82.3%), while acc_atomic_relaxed is the steadiest (CV 78.7%, 292 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (240 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### acc_atomic_seqcst's comparison is tie-heavy (20% tied pairs)

20% of paired samples for acc_atomic_seqcst are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader acc_atomic_seqcst vs stability leader acc_atomic_relaxed (+0% speed for 1.0x steadier)

acc_atomic_seqcst is fastest (292 ns, CV 82.3%); acc_atomic_relaxed gives up 0.0% median for 1.0x lower variance (CV 78.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### acc_atomic_seqcst is inconsistent: worst-20% is 3.5x its best-20%

acc_atomic_seqcst's best 20% of batches run at 239 ns but its worst 20% at 844 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.1% of the fastest

All 3 variants sit between 292 ns and 292 ns - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### acc_plain's edge over baseline is significant but tiny (0 ns, 0.10%)

acc_plain differs from baseline acc_atomic_relaxed by 0 ns (0.10%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: acc_atomic_seqcst** at 292.1 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 292.1 ns, slowest 292.4 ns)

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
| Counter frequency | 2112779839 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 2784ns | 2167ns | 1772ns | 2047ns | 6010ns | base |
| acc_atomic_seqcst | 2837ns | 2163ns | 1774ns | 2041ns | 6291ns | +1.91% |
| acc_plain | 2811ns | 2167ns | 1775ns | 2046ns | 6144ns | +0.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 374ns | 239ns | 803ns | base | 0.685 |
| acc_atomic_seqcst | 381ns | 239ns | 844ns | +1.99% | 0.671 |
| acc_plain | 377ns | 239ns | 819ns | +0.81% | 0.679 |

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
| acc_atomic_relaxed | 2784ns | 2784ns | base |
| acc_atomic_seqcst | 2837ns | 2837ns | +1.91% |
| acc_plain | 2811ns | 2811ns | +0.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 292ns | base | --- | [292, 292] | --- | --- | --- | --- |
| acc_atomic_seqcst | 292ns | no significant difference | [-0, +0]ns | [239, 292] | no | 0.2504 | 0.2504 | **32** (20%, HIGH) |
| acc_plain | 292ns | +0.3ns (+0.1%) | [+0, +0]ns | [240, 292] | YES | 0.0000 | 0.0000 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 239ns | +22.2% | +0.1% |
| 1 | 2 | 239ns | +0.0% | +0.1% |
| 1 | 3 | 239ns | +0.0% | +22.4% |
| 1 | 4 | 292ns | -18.2% | +0.1% |
| 1 | 5 | 239ns | +0.0% | +0.2% |
| 1 | 6 | 292ns | -18.2% | -18.1% |
| 1 | 7 | 292ns | +0.0% | -18.0% |
| 1 | 8 | 292ns | +0.0% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | 0.124 | ok |
| acc_atomic_seqcst | 0.258 | moderate+ |
| acc_plain | 0.058 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 2/8, lost 1/8
- **acc_plain**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 30.4ns | 373.9ns | 8.1% | HIGH |
| acc_atomic_seqcst | 31.3ns | 381.3ns | 8.2% | HIGH |
| acc_plain | 30.5ns | 376.9ns | 8.1% | HIGH |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 239.1-803.3 ns)
    239.1 |################################
    267.3 |########################################
    295.5 |##
    323.7 |
    351.9 |
    380.1 |
    408.3 |#
    436.6 |
    464.8 |
    493.0 |
    521.2 |
    549.4 |
    577.6 |
    605.8 |
    634.1 |
    662.3 |
    690.5 |
    718.7 |
    746.9 |
    775.1 |
  (14 below, 24 above range)

acc_atomic_seqcst (n=160, range 239.1-843.9 ns)
    239.1 |########################################
    269.3 |#############################
    299.5 |
    329.8 |
    360.0 |
    390.3 |
    420.5 |#
    450.8 |
    481.0 |
    511.3 |
    541.5 |
    571.7 |
    602.0 |
    632.2 |
    662.5 |
    692.7 |
    723.0 |#
    753.2 |
    783.5 |#####
    813.7 |#
  (15 below, 14 above range)

acc_plain (n=160, range 239.3-818.8 ns)
    239.3 |########################################
    268.3 |#####################################
    297.3 |
    326.2 |
    355.2 |
    384.2 |#
    413.2 |
    442.1 |
    471.1 |
    500.1 |
    529.1 |
    558.0 |
    587.0 |
    616.0 |
    644.9 |#
    673.9 |###
    702.9 |#
    731.9 |###
    760.8 |
    789.8 |###
  (22 below, 13 above range)

```

## Diagnostics

- **acc_atomic_relaxed**: CV=61.5% (high variance, measurements may be unstable)
- **acc_atomic_relaxed**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **acc_atomic_seqcst**: CV=63.0% (high variance, measurements may be unstable)
- **acc_atomic_seqcst**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **acc_plain**: CV=61.1% (high variance, measurements may be unstable)
- **acc_plain**: worst_20/best_20 = 3.4x (possible bimodal distribution)
