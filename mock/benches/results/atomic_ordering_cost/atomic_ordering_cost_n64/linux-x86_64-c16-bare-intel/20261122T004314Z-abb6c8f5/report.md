# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (76 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_relaxed at 52 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_seqcst beats baseline by 32% (significant)

atomic_seqcst is -24 ns (32%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (atomic_relaxed, atomic_seqcst) are a dead heat (<1%)

atomic_relaxed (52 ns) and atomic_seqcst (52 ns) differ by 0.00%, inside the noise, even though the wider field spreads 46.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### atomic_acquire shows warm-up / thermal drift (autocorr +0.63)

atomic_acquire's per-pass series has lag-1 autocorrelation +0.63, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (24 ns) is smaller than the fastest variant's own run-to-run std-dev (47 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader atomic_relaxed vs stability leader atomic_seqcst (+0% speed for 1.1x steadier)

atomic_relaxed is fastest (52 ns, CV 91.0%); atomic_seqcst gives up 0.0% median for 1.1x lower variance (CV 81.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### atomic_acquire is inconsistent: worst-20% is 3.8x its best-20%

atomic_acquire's best 20% of batches run at 62 ns but its worst 20% at 237 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: atomic_relaxed** at 51.5 ns median (-31.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.47x (fastest 51.5 ns, slowest 75.5 ns)

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
| Counter frequency | 2112895079 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| atomic_acquire | 2738ns | 1955ns | 1600ns | 1973ns | 6173ns | base |
| atomic_relaxed | 2741ns | 1931ns | 1578ns | 2012ns | 6091ns | +0.11% |
| atomic_seqcst | 2510ns | 1931ns | 1580ns | 1830ns | 5479ns | -8.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 106ns | 62ns | 237ns | base | 0.607 |
| atomic_relaxed | 73ns | 42ns | 161ns | -31.10% | 0.880 |
| atomic_seqcst | 67ns | 42ns | 145ns | -36.64% | 0.957 |

## Performance model

- Peak throughput: **1.520 Gops/s** (atomic_seqcst; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 0.848 | 55.8% |
| atomic_relaxed | 1.243 | 81.8% |
| atomic_seqcst | 1.243 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 2738ns | 2738ns | base |
| atomic_relaxed | 2741ns | 2741ns | +0.11% |
| atomic_seqcst | 2510ns | 2510ns | -8.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 76ns | base | --- | [76, 76] | --- | --- | --- | --- |
| atomic_relaxed | 52ns | -24.0ns (-31.8%) | [-24, -24]ns | [52, 52] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 52ns | -24.1ns (-31.9%) | [-25, -24]ns | [51, 52] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 62ns | -31.8% | -31.8% |
| 1 | 2 | 62ns | -31.8% | -31.8% |
| 1 | 3 | 62ns | -16.7% | -16.7% |
| 1 | 4 | 62ns | -31.8% | -16.6% |
| 1 | 5 | 76ns | -31.8% | -42.1% |
| 1 | 6 | 76ns | -31.7% | -31.9% |
| 1 | 7 | 76ns | -44.2% | -31.9% |
| 1 | 8 | 76ns | -31.7% | -44.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | 0.625 | HIGH+ (drift/warm-up) |
| atomic_relaxed | -0.374 | moderate- |
| atomic_seqcst | -0.056 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 33.9ns | 105.5ns | 32.2% | HIGH |
| atomic_relaxed | 34.1ns | 72.7ns | 46.9% | HIGH |
| atomic_seqcst | 31.0ns | 66.9ns | 46.4% | HIGH |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 61.8-237.2 ns)
     61.8 |####################################
     70.6 |########################################
     79.3 |
     88.1 |
     96.9 |
    105.6 |
    114.4 |
    123.2 |
    132.0 |
    140.7 |
    149.5 |
    158.3 |
    167.0 |
    175.8 |
    184.6 |#
    193.4 |##
    202.1 |##
    210.9 |
    219.7 |###
    228.5 |#
  (3 below, 16 above range)

atomic_relaxed (n=160, range 42.1-160.6 ns)
     42.1 |#######################
     48.1 |########################################
     54.0 |
     59.9 |
     65.8 |
     71.8 |
     77.7 |
     83.6 |
     89.5 |
     95.5 |#
    101.4 |
    107.3 |#
    113.2 |
    119.2 |#
    125.1 |
    131.0 |
    136.9 |##
    142.9 |#
    148.8 |###
    154.7 |#
  (21 below, 15 above range)

atomic_seqcst (n=160, range 42.1-145.3 ns)
     42.1 |#####################
     47.3 |########################################
     52.4 |#
     57.6 |
     62.7 |
     67.9 |
     73.1 |
     78.2 |
     83.4 |
     88.5 |
     93.7 |#
     98.9 |
    104.0 |
    109.2 |
    114.3 |
    119.5 |
    124.7 |
    129.8 |#
    135.0 |#
    140.1 |#
  (25 below, 17 above range)

```

## Diagnostics

- **atomic_acquire**: CV=65.8% (high variance, measurements may be unstable)
- **atomic_acquire**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **atomic_acquire**: autocorrelation=0.63 (measurement drift or warm-up artifact)
- **atomic_acquire**: bridge=32.2% of algo (FFI overhead may distort results)
- **atomic_relaxed**: CV=64.4% (high variance, measurements may be unstable)
- **atomic_relaxed**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **atomic_relaxed**: bridge=46.6% of algo (FFI overhead may distort results)
- **atomic_seqcst**: CV=62.9% (high variance, measurements may be unstable)
- **atomic_seqcst**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **atomic_seqcst**: bridge=46.6% of algo (FFI overhead may distort results)
