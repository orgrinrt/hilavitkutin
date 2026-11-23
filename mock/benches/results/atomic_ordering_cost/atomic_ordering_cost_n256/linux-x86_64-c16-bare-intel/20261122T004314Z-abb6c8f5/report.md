# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (240 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_relaxed at 143 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_relaxed dominates: 21% faster than the next best (atomic_seqcst)

atomic_relaxed (143 ns) leads atomic_seqcst (172 ns) by 21%, a clear separation rather than a photo finish. CV 105.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### atomic_relaxed beats baseline by 35% (significant)

atomic_relaxed is -84 ns (35%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_relaxed is fastest but the noisiest (CV 105.9%)

atomic_relaxed wins on median (143 ns) yet has the highest variance (CV 105.9%), while atomic_seqcst is the steadiest (CV 78.7%, 172 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (97 ns) is smaller than the fastest variant's own run-to-run std-dev (151 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### atomic_relaxed is inconsistent: worst-20% is 3.7x its best-20%

atomic_relaxed's best 20% of batches run at 141 ns but its worst 20% at 519 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: atomic_relaxed** at 142.7 ns median (-40.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.68x (fastest 142.7 ns, slowest 239.6 ns)

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
| atomic_acquire | 2603ns | 1782ns | 1761ns | 1957ns | 5381ns | base |
| atomic_relaxed | 2687ns | 1689ns | 1678ns | 1847ns | 6216ns | +3.25% |
| atomic_seqcst | 2707ns | 2053ns | 1714ns | 2055ns | 5654ns | +4.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 331ns | 225ns | 679ns | base | 0.774 |
| atomic_relaxed | 225ns | 141ns | 519ns | -31.92% | 1.137 |
| atomic_seqcst | 227ns | 144ns | 471ns | -31.47% | 1.129 |

## Performance model

- Peak throughput: **1.819 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.069 | 58.7% |
| atomic_relaxed | 1.794 | 98.6% |
| atomic_seqcst | 1.488 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 2603ns | 2603ns | base |
| atomic_relaxed | 2687ns | 2687ns | +3.25% |
| atomic_seqcst | 2707ns | 2707ns | +4.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 240ns | base | --- | [225, 275] | --- | --- | --- | --- |
| atomic_relaxed | 143ns | -84.3ns (-35.2%) | [-102, -84]ns | [142, 157] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 172ns | -84.1ns (-35.1%) | [-103, -53]ns | [172, 172] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 275ns | -48.8% | -37.4% |
| 1 | 2 | 225ns | -37.4% | -37.4% |
| 1 | 3 | 225ns | -37.4% | -23.5% |
| 1 | 4 | 225ns | -37.4% | -22.7% |
| 1 | 5 | 275ns | -37.4% | -37.4% |
| 1 | 6 | 225ns | -23.5% | -23.4% |
| 1 | 7 | 275ns | -48.2% | -48.8% |
| 1 | 8 | 225ns | -36.6% | -23.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.477 | moderate- |
| atomic_relaxed | 0.306 | moderate+ |
| atomic_seqcst | -0.344 | moderate- |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 29.2ns | 330.8ns | 8.8% | HIGH |
| atomic_relaxed | 31.7ns | 225.2ns | 14.1% | HIGH |
| atomic_seqcst | 31.7ns | 226.7ns | 14.0% | HIGH |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 224.9-679.4 ns)
    224.9 |########################################
    247.6 |
    270.4 |############################
    293.1 |
    315.8 |
    338.5 |
    361.3 |
    384.0 |
    406.7 |
    429.4 |
    452.2 |
    474.9 |#
    497.6 |
    520.4 |
    543.1 |
    565.8 |
    588.5 |
    611.3 |#
    634.0 |
    656.7 |
  (8 below, 19 above range)

atomic_relaxed (n=160, range 140.7-519.3 ns)
    140.7 |########################################
    159.7 |#################
    178.6 |
    197.5 |
    216.4 |
    235.4 |
    254.3 |
    273.2 |
    292.2 |
    311.1 |
    330.0 |
    348.9 |
    367.9 |
    386.8 |
    405.7 |
    424.7 |#
    443.6 |#
    462.5 |##
    481.5 |#
    500.4 |##
  (20 below, 14 above range)

atomic_seqcst (n=160, range 143.7-470.6 ns)
    143.7 |
    160.1 |########################################
    176.4 |##
    192.7 |
    209.1 |
    225.4 |
    241.8 |
    258.1 |
    274.5 |
    290.8 |
    307.1 |
    323.5 |
    339.8 |
    356.2 |
    372.5 |
    388.9 |
    405.2 |
    421.5 |
    437.9 |
    454.2 |
  (29 below, 21 above range)

```

## Diagnostics

- **atomic_acquire**: CV=63.6% (high variance, measurements may be unstable)
- **atomic_acquire**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **atomic_relaxed**: CV=67.1% (high variance, measurements may be unstable)
- **atomic_relaxed**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **atomic_relaxed**: bridge=13.9% of algo (FFI overhead may distort results)
- **atomic_seqcst**: CV=59.7% (high variance, measurements may be unstable)
- **atomic_seqcst**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **atomic_seqcst**: bridge=14.0% of algo (FFI overhead may distort results)
