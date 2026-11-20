# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (76 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_seqcst at 52 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_relaxed beats baseline by 32% (significant)

atomic_relaxed is -24 ns (32%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_seqcst is fastest but the noisiest (CV 89.8%)

atomic_seqcst wins on median (52 ns) yet has the highest variance (CV 89.8%), while atomic_relaxed is the steadiest (CV 85.1%, 52 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (atomic_seqcst, atomic_relaxed) are a dead heat (<1%)

atomic_seqcst (52 ns) and atomic_relaxed (52 ns) differ by 0.10%, inside the noise, even though the wider field spreads 46.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (24 ns) is smaller than the fastest variant's own run-to-run std-dev (46 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader atomic_seqcst vs stability leader atomic_relaxed (+0% speed for 1.1x steadier)

atomic_seqcst is fastest (52 ns, CV 89.8%); atomic_relaxed gives up 0.1% median for 1.1x lower variance (CV 85.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### atomic_seqcst is inconsistent: worst-20% is 3.8x its best-20%

atomic_seqcst's best 20% of batches run at 42 ns but its worst 20% at 160 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: atomic_seqcst** at 51.5 ns median (-31.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.47x (fastest 51.5 ns, slowest 75.6 ns)

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
| atomic_acquire | 2687ns | 1957ns | 1602ns | 2018ns | 5778ns | base |
| atomic_relaxed | 2890ns | 1936ns | 1581ns | 2378ns | 5737ns | +7.57% |
| atomic_seqcst | 2703ns | 1934ns | 1584ns | 1949ns | 6087ns | +0.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 104ns | 62ns | 226ns | base | 0.613 |
| atomic_relaxed | 77ns | 42ns | 152ns | -26.36% | 0.833 |
| atomic_seqcst | 72ns | 42ns | 160ns | -31.38% | 0.893 |

## Performance model

- Peak throughput: **1.518 Gops/s** (atomic_seqcst; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 0.847 | 55.8% |
| atomic_relaxed | 1.242 | 81.8% |
| atomic_seqcst | 1.243 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 2687ns | 2687ns | base |
| atomic_relaxed | 2890ns | 2890ns | +7.57% |
| atomic_seqcst | 2703ns | 2703ns | +0.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 76ns | base | --- | [76, 76] | --- | --- | --- | --- |
| atomic_relaxed | 52ns | -24.1ns (-31.9%) | [-33, -10]ns | [52, 52] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 52ns | -24.0ns (-31.7%) | [-24, -24]ns | [52, 52] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 62ns | -16.6% | -16.7% |
| 1 | 2 | 62ns | -16.6% | -31.8% |
| 1 | 3 | 76ns | -44.2% | -31.7% |
| 1 | 4 | 76ns | -31.9% | -31.7% |
| 1 | 5 | 62ns | -16.7% | -16.6% |
| 1 | 6 | 76ns | -44.2% | -31.8% |
| 1 | 7 | 62ns | +76.7% | -16.6% |
| 1 | 8 | 76ns | -44.0% | -44.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.373 | moderate- |
| atomic_relaxed | -0.354 | moderate- |
| atomic_seqcst | -0.205 | moderate- |

**Consistency summary:**

- **atomic_relaxed**: won 7/8, lost 1/8
- **atomic_seqcst**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 33.9ns | 104.4ns | 32.5% | HIGH |
| atomic_relaxed | 35.7ns | 76.9ns | 46.5% | HIGH |
| atomic_seqcst | 33.5ns | 71.6ns | 46.8% | HIGH |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 61.8-226.5 ns)
     61.8 |##############################
     70.0 |########################################
     78.3 |
     86.5 |
     94.7 |
    103.0 |
    111.2 |
    119.4 |
    127.7 |
    135.9 |
    144.1 |
    152.4 |
    160.6 |
    168.8 |
    177.1 |#
    185.3 |#
    193.5 |###
    201.8 |####
    210.0 |
    218.2 |##
  (4 below, 13 above range)

atomic_relaxed (n=160, range 42.2-151.6 ns)
     42.2 |#################
     47.6 |########################################
     53.1 |
     58.6 |#
     64.0 |
     69.5 |#
     75.0 |
     80.5 |#
     85.9 |
     91.4 |#
     96.9 |
    102.3 |
    107.8 |#
    113.3 |####
    118.8 |#####
    124.2 |
    129.7 |####
    135.2 |
    140.6 |##
    146.1 |
  (11 below, 14 above range)

atomic_seqcst (n=160, range 42.2-160.4 ns)
     42.2 |#######
     48.1 |########################################
     54.0 |
     59.9 |
     65.8 |
     71.7 |
     77.6 |
     83.6 |
     89.5 |
     95.4 |
    101.3 |
    107.2 |
    113.1 |
    119.0 |
    125.0 |
    130.9 |
    136.8 |
    142.7 |
    148.6 |##
    154.5 |
  (15 below, 18 above range)

```

## Diagnostics

- **atomic_acquire**: CV=62.6% (high variance, measurements may be unstable)
- **atomic_acquire**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **atomic_acquire**: bridge=32.1% of algo (FFI overhead may distort results)
- **atomic_relaxed**: CV=57.1% (high variance, measurements may be unstable)
- **atomic_relaxed**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **atomic_relaxed**: bridge=46.7% of algo (FFI overhead may distort results)
- **atomic_seqcst**: CV=64.6% (high variance, measurements may be unstable)
- **atomic_seqcst**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **atomic_seqcst**: bridge=46.6% of algo (FFI overhead may distort results)
