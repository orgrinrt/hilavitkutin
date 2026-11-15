# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (memcpy_intrinsic) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline memcpy_intrinsic has the worst median (376 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest memcpy_naive at 375 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (278 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### memcpy_naive's comparison is tie-heavy (10% tied pairs)

10% of paired samples for memcpy_naive are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader memcpy_naive vs stability leader memcpy_intrinsic (+0% speed for 1.0x steadier)

memcpy_naive is fastest (375 ns, CV 74.0%); memcpy_intrinsic gives up 0.1% median for 1.0x lower variance (CV 72.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### memcpy_unrolled is inconsistent: worst-20% is 3.0x its best-20%

memcpy_unrolled's best 20% of batches run at 308 ns but its worst 20% at 924 ns (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.1% of the fastest

All 3 variants sit between 375 ns and 376 ns - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_naive's edge over baseline is significant but tiny (-0 ns, 0.05%)

memcpy_naive differs from baseline memcpy_intrinsic by -0 ns (0.05%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: memcpy_naive** at 375.4 ns median (-0.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.00x (fastest 375.4 ns, slowest 375.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4646464650504040 |
| Counter frequency | 2112632730 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 2936ns | 2257ns | 2247ns | 2259ns | 5658ns | base |
| memcpy_naive | 2848ns | 2256ns | 2248ns | 2257ns | 5218ns | -3.02% |
| memcpy_unrolled | 2830ns | 2252ns | 1843ns | 2251ns | 5553ns | -3.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 489ns | 375ns | 936ns | base | 0.524 |
| memcpy_naive | 474ns | 375ns | 870ns | -3.00% | 0.540 |
| memcpy_unrolled | 472ns | 308ns | 924ns | -3.55% | 0.543 |

## Performance model

- Peak throughput: **0.832 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.682 | 81.9% |
| memcpy_naive | 0.682 | 81.9% |
| memcpy_unrolled | 0.682 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 2936ns | 2936ns | base |
| memcpy_naive | 2848ns | 2848ns | -3.02% |
| memcpy_unrolled | 2830ns | 2830ns | -3.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 376ns | base | --- | [376, 376] | --- | --- | --- | --- |
| memcpy_naive | 375ns | -0.2ns (-0.1%) | [-0, -0]ns | [375, 375] | YES | 0.0003 | 0.0002 | 16 |
| memcpy_unrolled | 376ns | -0.2ns (-0.1%) | [-1, -0]ns | [375, 376] | YES | 0.0031 | 0.0031 | 11 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 375ns | -0.0% | -18.2% |
| 1 | 2 | 376ns | -0.1% | -0.0% |
| 1 | 3 | 376ns | -0.0% | -18.2% |
| 1 | 4 | 376ns | -0.1% | -0.1% |
| 1 | 5 | 376ns | -0.0% | +0.0% |
| 1 | 6 | 376ns | -0.1% | +0.0% |
| 1 | 7 | 375ns | -0.0% | +0.0% |
| 1 | 8 | 386ns | -2.8% | -2.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.034 | ok |
| memcpy_naive | 0.317 | moderate+ |
| memcpy_unrolled | -0.208 | moderate- |

**Consistency summary:**

- **memcpy_naive**: won 1/8, lost 0/8
- **memcpy_unrolled**: won 3/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 32.1ns | 488.9ns | 6.6% | HIGH |
| memcpy_naive | 31.0ns | 474.3ns | 6.5% | HIGH |
| memcpy_unrolled | 31.0ns | 471.5ns | 6.6% | HIGH |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 375.3-935.9 ns)
    375.3 |########################################
    403.3 |
    431.3 |
    459.4 |
    487.4 |
    515.4 |
    543.5 |
    571.5 |
    599.5 |
    627.5 |
    655.6 |
    683.6 |
    711.6 |
    739.7 |
    767.7 |
    795.7 |
    823.8 |
    851.8 |
    879.8 |
    907.8 |
  (7 below, 20 above range)

memcpy_naive (n=160, range 375.1-869.9 ns)
    375.1 |########################################
    399.9 |
    424.6 |
    449.3 |
    474.1 |
    498.8 |
    523.6 |
    548.3 |
    573.0 |
    597.8 |
    622.5 |
    647.3 |
    672.0 |
    696.8 |
    721.5 |
    746.2 |
    771.0 |
    795.7 |
    820.5 |
    845.2 |
  (20 below, 17 above range)

memcpy_unrolled (n=160, range 307.6-924.2 ns)
    307.6 |
    338.4 |
    369.2 |########################################
    400.1 |
    430.9 |
    461.7 |
    492.6 |
    523.4 |
    554.2 |
    585.1 |
    615.9 |
    646.7 |
    677.6 |
    708.4 |
    739.2 |
    770.1 |
    800.9 |
    831.7 |
    862.6 |
    893.4 |
  (31 below, 18 above range)

```

## Diagnostics

- **memcpy_intrinsic**: CV=55.8% (high variance, measurements may be unstable)
- **memcpy_naive**: CV=58.6% (high variance, measurements may be unstable)
- **memcpy_unrolled**: CV=62.0% (high variance, measurements may be unstable)
- **memcpy_unrolled**: worst_20/best_20 = 3.0x (possible bimodal distribution)
