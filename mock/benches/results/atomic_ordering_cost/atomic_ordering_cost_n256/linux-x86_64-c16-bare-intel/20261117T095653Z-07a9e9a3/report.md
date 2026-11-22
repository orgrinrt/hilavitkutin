# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (225 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_relaxed at 172 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_relaxed beats baseline by 37% (significant)

atomic_relaxed is -84 ns (37%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (atomic_relaxed, atomic_seqcst) are a dead heat (<1%)

atomic_relaxed (172 ns) and atomic_seqcst (172 ns) differ by 0.12%, inside the noise, even though the wider field spreads 30.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (53 ns) is smaller than the fastest variant's own run-to-run std-dev (188 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### atomic_seqcst is inconsistent: worst-20% is 5.1x its best-20%

atomic_seqcst's best 20% of batches run at 154 ns but its worst 20% at 788 ns (5.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: atomic_relaxed** at 172.0 ns median (-23.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.31x (fastest 172.0 ns, slowest 225.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x0ddc0ffee055b011 |
| Counter frequency | 2112683200 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| atomic_acquire | 2725ns | 1775ns | 1764ns | 1932ns | 6068ns | base |
| atomic_relaxed | 2973ns | 2052ns | 1679ns | 2197ns | 6596ns | +9.09% |
| atomic_seqcst | 3918ns | 2060ns | 1842ns | 2754ns | 9488ns | +43.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 345ns | 225ns | 764ns | base | 0.741 |
| atomic_relaxed | 253ns | 141ns | 571ns | -26.79% | 1.013 |
| atomic_seqcst | 327ns | 154ns | 788ns | -5.38% | 0.784 |

## Performance model

- Peak throughput: **1.819 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.137 | 62.5% |
| atomic_relaxed | 1.488 | 81.8% |
| atomic_seqcst | 1.487 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 2725ns | 2725ns | base |
| atomic_relaxed | 2973ns | 2973ns | +9.09% |
| atomic_seqcst | 3918ns | 3918ns | +43.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 225ns | base | --- | [225, 252] | --- | --- | --- | --- |
| atomic_relaxed | 172ns | -84.2ns (-37.4%) | [-84, -84]ns | [142, 172] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 172ns | -79.4ns (-35.3%) | [-84, -53]ns | [172, 172] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 225ns | -36.7% | -23.5% |
| 1 | 2 | 225ns | +76.3% | +68.6% |
| 1 | 3 | 225ns | -37.5% | -23.5% |
| 1 | 4 | 225ns | -37.4% | +121.5% |
| 1 | 5 | 225ns | -37.4% | -37.4% |
| 1 | 6 | 275ns | -48.8% | -37.4% |
| 1 | 7 | 225ns | -23.5% | -23.5% |
| 1 | 8 | 275ns | -37.4% | -37.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.211 | moderate- |
| atomic_relaxed | -0.214 | moderate- |
| atomic_seqcst | -0.384 | moderate- |

**Consistency summary:**

- **atomic_relaxed**: won 7/8, lost 1/8
- **atomic_seqcst**: won 6/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 30.9ns | 345.3ns | 9.0% | HIGH |
| atomic_relaxed | 34.2ns | 252.8ns | 13.5% | HIGH |
| atomic_seqcst | 45.7ns | 326.7ns | 14.0% | HIGH |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 225.0-764.4 ns)
    225.0 |########################################
    251.9 |###############
    278.9 |#
    305.9 |
    332.8 |
    359.8 |#
    386.8 |
    413.8 |
    440.7 |
    467.7 |
    494.7 |
    521.7 |
    548.6 |
    575.6 |
    602.6 |
    629.5 |#
    656.5 |#
    683.5 |###
    710.5 |#
    737.4 |##
  (14 below, 11 above range)

atomic_relaxed (n=160, range 140.7-570.8 ns)
    140.7 |########################################
    162.2 |#######################
    183.7 |##
    205.2 |##
    226.7 |
    248.3 |
    269.8 |
    291.3 |##
    312.8 |
    334.3 |#
    355.8 |
    377.3 |##
    398.8 |##
    420.3 |#
    441.8 |##
    463.3 |##
    484.8 |#
    506.3 |#
    527.8 |
    549.3 |#
  (19 below, 14 above range)

atomic_seqcst (n=160, range 154.2-787.6 ns)
    154.2 |########################################
    185.9 |#
    217.6 |#
    249.3 |##
    280.9 |
    312.6 |#
    344.3 |##
    375.9 |#
    407.6 |##
    439.3 |###
    470.9 |###
    502.6 |#
    534.3 |###
    565.9 |#
    597.6 |#
    629.3 |#
    660.9 |
    692.6 |
    724.3 |
    756.0 |
  (18 below, 9 above range)

```

## Diagnostics

- **atomic_acquire**: CV=64.6% (high variance, measurements may be unstable)
- **atomic_acquire**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **atomic_relaxed**: CV=74.2% (high variance, measurements may be unstable)
- **atomic_relaxed**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **atomic_relaxed**: bridge=14.0% of algo (FFI overhead may distort results)
- **atomic_seqcst**: CV=94.2% (high variance, measurements may be unstable)
- **atomic_seqcst**: worst_20/best_20 = 5.1x (possible bimodal distribution)
- **atomic_seqcst**: bridge=14.1% of algo (FFI overhead may distort results)
