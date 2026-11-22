# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (7 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_relaxed at 4 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_seqcst beats baseline by 48% (significant)

atomic_seqcst is -3 ns (48%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (atomic_relaxed, atomic_seqcst) are a dead heat (<1%)

atomic_relaxed (4 ns) and atomic_seqcst (4 ns) differ by 0.00%, inside the noise, even though the wider field spreads 86.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### atomic_seqcst shows alternating (throttle bounce) (autocorr -0.75)

atomic_seqcst's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### atomic_relaxed is inconsistent: worst-20% is 2.5x its best-20%

atomic_relaxed's best 20% of batches run at 2 ns but its worst 20% at 5 ns (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: atomic_relaxed** at 3.7 ns median (-46.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.86x (fastest 3.7 ns, slowest 6.9 ns)

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
| atomic_acquire | 2148ns | 2121ns | 2112ns | 2127ns | 2248ns | base |
| atomic_relaxed | 2111ns | 2099ns | 2091ns | 2101ns | 2159ns | -1.75% |
| atomic_seqcst | 2155ns | 2128ns | 2117ns | 2145ns | 2224ns | +0.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 7ns | 5ns | 9ns | base | 9.194 |
| atomic_relaxed | 4ns | 2ns | 5ns | -45.12% | 16.751 |
| atomic_seqcst | 4ns | 2ns | 6ns | -44.99% | 16.713 |

## Performance model

- Peak throughput: **29.091 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 9.275 | 31.9% |
| atomic_relaxed | 17.297 | 59.5% |
| atomic_seqcst | 17.297 | 59.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 2148ns | 2148ns | base |
| atomic_relaxed | 2111ns | 2111ns | -1.75% |
| atomic_seqcst | 2155ns | 2155ns | +0.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 7ns | base | --- | [7, 7] | --- | --- | --- | --- |
| atomic_relaxed | 4ns | -3.0ns (-43.5%) | [-3, -3]ns | [4, 4] | YES | 0.0000 | 0.0000 | 5 |
| atomic_seqcst | 4ns | -3.3ns (-47.8%) | [-4, -3]ns | [3, 4] | YES | 0.0000 | 0.0000 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 7ns | -44.4% | -40.8% |
| 1 | 2 | 6ns | -34.9% | -48.8% |
| 1 | 3 | 7ns | -45.9% | -42.5% |
| 1 | 4 | 7ns | -44.8% | -44.8% |
| 1 | 5 | 7ns | -44.8% | -41.0% |
| 1 | 6 | 8ns | -51.9% | -57.1% |
| 1 | 7 | 8ns | -47.3% | -44.0% |
| 1 | 8 | 7ns | -47.8% | -44.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.168 | ok |
| atomic_relaxed | 0.152 | ok |
| atomic_seqcst | -0.753 | HIGH- (thermal bounce) |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 4.1ns | 7.0ns | 59.3% | HIGH |
| atomic_relaxed | 2.5ns | 3.8ns | 66.5% | HIGH |
| atomic_seqcst | 2.7ns | 3.8ns | 69.8% | HIGH |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 5.1-9.0 ns)
      5.1 |
      5.3 |#######################
      5.5 |
      5.7 |###########################
      5.9 |
      6.0 |########################################
      6.2 |
      6.4 |
      6.6 |###################################
      6.8 |
      7.0 |###############################
      7.2 |
      7.4 |#############################
      7.6 |
      7.8 |###############################
      8.0 |
      8.2 |###############################
      8.4 |
      8.6 |
      8.8 |##################
  (20 below, 12 above range)

atomic_relaxed (n=160, range 2.2-5.5 ns)
      2.2 |
      2.4 |###########################
      2.5 |
      2.7 |
      2.9 |######################
      3.0 |
      3.2 |######################################
      3.4 |
      3.5 |
      3.7 |######################################
      3.8 |
      4.0 |
      4.2 |########################################
      4.3 |
      4.5 |####################################
      4.7 |
      4.8 |
      5.0 |#################
      5.2 |
      5.3 |###############
  (14 below, 10 above range)

atomic_seqcst (n=160, range 2.4-5.6 ns)
      2.4 |##################
      2.5 |
      2.7 |
      2.8 |########################################
      3.0 |
      3.2 |########################
      3.3 |
      3.5 |
      3.6 |#############################
      3.8 |
      4.0 |
      4.1 |###################################
      4.3 |
      4.4 |################################
      4.6 |
      4.8 |
      4.9 |#############
      5.1 |
      5.3 |#######
      5.4 |
  (15 below, 14 above range)

```

## Diagnostics

- **atomic_acquire**: CV=20.1% (high variance, measurements may be unstable)
- **atomic_acquire**: bridge=60.9% of algo (FFI overhead may distort results)
- **atomic_relaxed**: CV=31.1% (high variance, measurements may be unstable)
- **atomic_relaxed**: bridge=67.6% of algo (FFI overhead may distort results)
- **atomic_seqcst**: CV=31.0% (high variance, measurements may be unstable)
- **atomic_seqcst**: bridge=67.6% of algo (FFI overhead may distort results)
