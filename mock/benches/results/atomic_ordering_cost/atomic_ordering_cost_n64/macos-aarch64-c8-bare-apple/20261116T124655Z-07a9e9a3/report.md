# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (7 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_relaxed at 4 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_relaxed beats baseline by 48% (significant)

atomic_relaxed is -3 ns (48%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_relaxed is fastest but the noisiest (CV 32.2%)

atomic_relaxed wins on median (4 ns) yet has the highest variance (CV 32.2%), while atomic_acquire is the steadiest (CV 22.6%, 7 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### atomic_relaxed is inconsistent: worst-20% is 2.5x its best-20%

atomic_relaxed's best 20% of batches run at 2 ns but its worst 20% at 6 ns (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: atomic_relaxed** at 3.7 ns median (-47.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.92x (fastest 3.7 ns, slowest 7.1 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| atomic_acquire | 2167ns | 2159ns | 2153ns | 2160ns | 2203ns | base |
| atomic_relaxed | 2137ns | 2134ns | 2128ns | 2134ns | 2155ns | -1.37% |
| atomic_seqcst | 2180ns | 2160ns | 2154ns | 2160ns | 2264ns | +0.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 7ns | 5ns | 10ns | base | 8.821 |
| atomic_relaxed | 4ns | 2ns | 6ns | -46.49% | 16.484 |
| atomic_seqcst | 4ns | 2ns | 6ns | -44.21% | 15.810 |

## Performance model

- Peak throughput: **27.676 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 9.014 | 32.6% |
| atomic_relaxed | 17.297 | 62.5% |
| atomic_seqcst | 16.203 | 58.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 2167ns | 2167ns | base |
| atomic_relaxed | 2137ns | 2137ns | -1.37% |
| atomic_seqcst | 2180ns | 2180ns | +0.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 7ns | base | --- | [7, 8] | --- | --- | --- | --- |
| atomic_relaxed | 4ns | -3.4ns (-47.9%) | [-4, -3]ns | [4, 4] | YES | 0.0000 | 0.0000 | 1 |
| atomic_seqcst | 4ns | -3.3ns (-46.5%) | [-3, -3]ns | [4, 4] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 8ns | -45.5% | -51.9% |
| 1 | 2 | 8ns | -44.0% | -50.7% |
| 1 | 3 | 7ns | -46.4% | -33.3% |
| 1 | 4 | 6ns | -42.6% | -42.6% |
| 1 | 5 | 7ns | -53.5% | -40.8% |
| 1 | 6 | 7ns | -47.9% | -35.2% |
| 1 | 7 | 7ns | -49.3% | -49.3% |
| 1 | 8 | 7ns | -52.1% | -49.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | 0.334 | moderate+ |
| atomic_relaxed | 0.357 | moderate+ |
| atomic_seqcst | -0.243 | moderate- |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 4.1ns | 7.3ns | 56.1% | HIGH |
| atomic_relaxed | 2.7ns | 3.9ns | 68.6% | HIGH |
| atomic_seqcst | 2.5ns | 4.0ns | 62.3% | HIGH |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 5.1-9.6 ns)
      5.1 |
      5.3 |############
      5.5 |
      5.7 |#######################
      6.0 |
      6.2 |###########################
      6.4 |
      6.6 |##############
      6.9 |
      7.1 |########################################
      7.3 |################################
      7.5 |
      7.8 |#######################
      8.0 |
      8.2 |####################
      8.4 |
      8.7 |#######################
      8.9 |
      9.1 |############
      9.3 |
  (18 below, 15 above range)

atomic_relaxed (n=160, range 2.3-5.7 ns)
      2.3 |
      2.5 |###########################
      2.6 |
      2.8 |#################
      3.0 |
      3.2 |############################
      3.3 |
      3.5 |
      3.7 |########################################
      3.8 |
      4.0 |
      4.2 |#########################
      4.3 |
      4.5 |########################
      4.7 |
      4.8 |############
      5.0 |
      5.2 |
      5.3 |############
      5.5 |
  (12 below, 16 above range)

atomic_seqcst (n=160, range 2.5-5.8 ns)
      2.5 |#######################
      2.7 |
      2.8 |#####################
      3.0 |
      3.2 |##########################
      3.3 |
      3.5 |
      3.7 |########################################
      3.8 |
      4.0 |
      4.2 |################################
      4.3 |
      4.5 |##############################
      4.7 |
      4.8 |
      5.0 |########################
      5.1 |
      5.3 |######
      5.5 |
      5.6 |##########
  (8 below, 12 above range)

```

## Diagnostics

- **atomic_acquire**: CV=22.1% (high variance, measurements may be unstable)
- **atomic_acquire**: bridge=59.2% of algo (FFI overhead may distort results)
- **atomic_relaxed**: CV=30.7% (high variance, measurements may be unstable)
- **atomic_relaxed**: bridge=67.6% of algo (FFI overhead may distort results)
- **atomic_seqcst**: CV=29.6% (high variance, measurements may be unstable)
- **atomic_seqcst**: bridge=63.3% of algo (FFI overhead may distort results)
