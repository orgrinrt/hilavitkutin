# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (7 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_relaxed at 4 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_relaxed beats baseline by 41% (significant)

atomic_relaxed is -3 ns (41%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (atomic_relaxed, atomic_seqcst) are a dead heat (<1%)

atomic_relaxed (4 ns) and atomic_seqcst (4 ns) differ by 0.00%, inside the noise, even though the wider field spreads 69.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### atomic_relaxed is inconsistent: worst-20% is 2.3x its best-20%

atomic_relaxed's best 20% of batches run at 3 ns but its worst 20% at 6 ns (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: atomic_relaxed** at 4.2 ns median (-40.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.69x (fastest 4.2 ns, slowest 7.1 ns)

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
| atomic_acquire | 2153ns | 2148ns | 2135ns | 2148ns | 2182ns | base |
| atomic_relaxed | 2124ns | 2125ns | 2091ns | 2125ns | 2157ns | -1.31% |
| atomic_seqcst | 2152ns | 2149ns | 2141ns | 2149ns | 2170ns | -0.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 7ns | 5ns | 10ns | base | 8.945 |
| atomic_relaxed | 4ns | 3ns | 6ns | -41.98% | 15.417 |
| atomic_seqcst | 4ns | 3ns | 6ns | -41.60% | 15.316 |

## Performance model

- Peak throughput: **25.160 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 9.014 | 35.8% |
| atomic_relaxed | 15.238 | 60.6% |
| atomic_seqcst | 15.238 | 60.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 2153ns | 2153ns | base |
| atomic_relaxed | 2124ns | 2124ns | -1.31% |
| atomic_seqcst | 2152ns | 2152ns | -0.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 7ns | base | --- | [7, 8] | --- | --- | --- | --- |
| atomic_relaxed | 4ns | -2.9ns (-40.8%) | [-3, -2]ns | [4, 4] | YES | 0.0000 | 0.0000 | 3 |
| atomic_seqcst | 4ns | -2.9ns (-40.8%) | [-3, -3]ns | [4, 4] | YES | 0.0000 | 0.0000 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 8ns | -47.3% | -38.7% |
| 1 | 2 | 8ns | -44.0% | -50.7% |
| 1 | 3 | 7ns | -44.4% | -47.9% |
| 1 | 4 | 6ns | -34.9% | -34.9% |
| 1 | 5 | 8ns | -46.8% | -53.2% |
| 1 | 6 | 7ns | -42.8% | -42.8% |
| 1 | 7 | 7ns | -34.3% | -37.3% |
| 1 | 8 | 7ns | -42.5% | -42.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.329 | moderate- |
| atomic_relaxed | -0.397 | moderate- |
| atomic_seqcst | -0.200 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 2.7ns | 7.2ns | 37.6% | HIGH |
| atomic_relaxed | 2.5ns | 4.2ns | 61.0% | HIGH |
| atomic_seqcst | 2.5ns | 4.2ns | 60.4% | HIGH |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 4.8-9.7 ns)
      4.8 |#################
      5.1 |
      5.3 |##########################
      5.6 |#####################################
      5.8 |
      6.1 |#################
      6.3 |
      6.5 |###############################
      6.8 |
      7.0 |########################################
      7.3 |#################################
      7.5 |
      7.8 |##########################
      8.0 |
      8.2 |##########################
      8.5 |
      8.7 |#################
      9.0 |#############
      9.2 |
      9.4 |########
  (11 below, 15 above range)

atomic_relaxed (n=160, range 2.5-5.8 ns)
      2.5 |
      2.7 |
      2.9 |###################
      3.0 |
      3.2 |#######################
      3.4 |
      3.5 |
      3.7 |##########################
      3.8 |
      4.0 |
      4.2 |########################################
      4.3 |
      4.5 |###############################
      4.7 |
      4.8 |
      5.0 |####################
      5.1 |
      5.3 |#############
      5.5 |
      5.6 |
  (16 below, 17 above range)

atomic_seqcst (n=160, range 2.7-6.1 ns)
      2.7 |
      2.8 |################################
      3.0 |
      3.2 |######################
      3.4 |
      3.5 |
      3.7 |#####################################
      3.9 |
      4.0 |
      4.2 |########################################
      4.4 |
      4.5 |###########
      4.7 |
      4.9 |######################
      5.0 |
      5.2 |
      5.4 |####################
      5.5 |
      5.7 |#######
      5.9 |
  (11 below, 13 above range)

```

## Diagnostics

- **atomic_acquire**: CV=24.0% (high variance, measurements may be unstable)
- **atomic_acquire**: bridge=35.2% of algo (FFI overhead may distort results)
- **atomic_relaxed**: CV=28.2% (high variance, measurements may be unstable)
- **atomic_relaxed**: bridge=59.5% of algo (FFI overhead may distort results)
- **atomic_seqcst**: CV=29.5% (high variance, measurements may be unstable)
- **atomic_seqcst**: bridge=59.5% of algo (FFI overhead may distort results)
