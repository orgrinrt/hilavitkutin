# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (memcpy_intrinsic) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline memcpy_intrinsic has the worst median (376 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest memcpy_naive at 375 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (271 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### memcpy_unrolled is inconsistent: worst-20% is 3.0x its best-20%

memcpy_unrolled's best 20% of batches run at 308 ns but its worst 20% at 937 ns (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.1% of the fastest

All 3 variants sit between 375 ns and 376 ns - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_naive's edge over baseline is significant but tiny (-0 ns, 0.11%)

memcpy_naive differs from baseline memcpy_intrinsic by -0 ns (0.11%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: memcpy_naive** at 375.3 ns median (-0.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 375.3 ns, slowest 375.6 ns)

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
| Counter frequency | 2112541357 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 2983ns | 2259ns | 2062ns | 2259ns | 6077ns | base |
| memcpy_naive | 2865ns | 2253ns | 2033ns | 2255ns | 5526ns | -3.97% |
| memcpy_unrolled | 2857ns | 2252ns | 1843ns | 2254ns | 5683ns | -4.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 499ns | 347ns | 1016ns | base | 0.513 |
| memcpy_naive | 476ns | 339ns | 914ns | -4.73% | 0.538 |
| memcpy_unrolled | 474ns | 308ns | 937ns | -5.02% | 0.540 |

## Performance model

- Peak throughput: **0.832 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.682 | 81.9% |
| memcpy_naive | 0.682 | 82.0% |
| memcpy_unrolled | 0.682 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 2983ns | 2983ns | base |
| memcpy_naive | 2865ns | 2865ns | -3.97% |
| memcpy_unrolled | 2857ns | 2857ns | -4.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 376ns | base | --- | [376, 376] | --- | --- | --- | --- |
| memcpy_naive | 375ns | -0.4ns (-0.1%) | [-0, -0]ns | [375, 375] | YES | 0.0000 | 0.0000 | 7 |
| memcpy_unrolled | 376ns | no significant difference | [-0, +0]ns | [375, 376] | no (adj: YES) | 0.0106 | 0.0106 | 12 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 316ns | +18.7% | +18.8% |
| 1 | 2 | 376ns | -0.0% | +0.0% |
| 1 | 3 | 376ns | -18.2% | -18.2% |
| 1 | 4 | 376ns | -0.1% | +0.0% |
| 1 | 5 | 376ns | -0.1% | -0.0% |
| 1 | 6 | 376ns | -0.1% | -0.0% |
| 1 | 7 | 376ns | -0.0% | +0.0% |
| 1 | 8 | 386ns | -2.9% | -20.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.010 | ok |
| memcpy_naive | -0.162 | ok |
| memcpy_unrolled | -0.209 | moderate- |

**Consistency summary:**

- **memcpy_naive**: won 2/8, lost 1/8
- **memcpy_unrolled**: won 2/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 33.0ns | 499.3ns | 6.6% | HIGH |
| memcpy_naive | 31.2ns | 475.7ns | 6.6% | HIGH |
| memcpy_unrolled | 31.4ns | 474.3ns | 6.6% | HIGH |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 347.5-1016.3 ns)
    347.5 |########################################
    380.9 |########
    414.3 |
    447.8 |
    481.2 |
    514.7 |
    548.1 |
    581.6 |
    615.0 |
    648.5 |
    681.9 |
    715.3 |
    748.8 |
    782.2 |
    815.7 |
    849.1 |
    882.6 |
    916.0 |
    949.5 |
    982.9 |
  (15 below, 19 above range)

memcpy_naive (n=160, range 338.9-913.8 ns)
    338.9 |
    367.6 |########################################
    396.4 |
    425.1 |
    453.9 |
    482.6 |
    511.3 |
    540.1 |
    568.8 |
    597.6 |
    626.3 |
    655.1 |
    683.8 |
    712.6 |
    741.3 |
    770.0 |
    798.8 |
    827.5 |
    856.3 |
    885.0 |
  (17 below, 19 above range)

memcpy_unrolled (n=160, range 307.6-937.3 ns)
    307.6 |
    339.1 |
    370.6 |########################################
    402.1 |
    433.5 |
    465.0 |
    496.5 |
    528.0 |
    559.5 |
    591.0 |
    622.5 |
    654.0 |
    685.4 |
    716.9 |
    748.4 |
    779.9 |
    811.4 |
    842.9 |
    874.4 |
    905.8 |
  (31 below, 19 above range)

```

## Diagnostics

- **memcpy_intrinsic**: CV=57.8% (high variance, measurements may be unstable)
- **memcpy_naive**: CV=56.9% (high variance, measurements may be unstable)
- **memcpy_unrolled**: CV=58.9% (high variance, measurements may be unstable)
- **memcpy_unrolled**: worst_20/best_20 = 3.0x (possible bimodal distribution)
