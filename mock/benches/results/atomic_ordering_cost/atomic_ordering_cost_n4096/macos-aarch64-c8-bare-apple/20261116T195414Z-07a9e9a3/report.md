# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### atomic_relaxed dominates: 235% faster than the next best (atomic_acquire)

atomic_relaxed (1.03 us) leads atomic_acquire (3.46 us) by 235%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### atomic_relaxed beats baseline by 70% (significant)

atomic_relaxed is -2.43 us (70%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_seqcst is an outlier: 3.4x slower than the field

atomic_seqcst (3.47 us) is 3.4x the fastest (1.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.4x the fastest

Fastest atomic_relaxed (1.03 us) to slowest atomic_seqcst (3.47 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### atomic_seqcst's edge over baseline is significant but tiny (4 ns, 0.11%)

atomic_seqcst differs from baseline atomic_acquire by 4 ns (0.11%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: atomic_relaxed** at 1033.3 ns median (-70.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.36x (fastest 1033.3 ns, slowest 3468.8 ns)

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
| atomic_acquire | 5594ns | 5581ns | 5572ns | 5583ns | 5647ns | base |
| atomic_relaxed | 3138ns | 3130ns | 3122ns | 3131ns | 3177ns | -43.90% |
| atomic_seqcst | 5602ns | 5581ns | 5570ns | 5587ns | 5682ns | +0.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 3474ns | 3459ns | 3515ns | base | 1.179 |
| atomic_relaxed | 1037ns | 1028ns | 1056ns | -70.16% | 3.950 |
| atomic_seqcst | 3481ns | 3463ns | 3530ns | +0.18% | 1.177 |

## Performance model

- Peak throughput: **3.983 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.182 | 29.7% |
| atomic_relaxed | 3.964 | 99.5% |
| atomic_seqcst | 1.181 | 29.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 5594ns | 5594ns | base |
| atomic_relaxed | 3138ns | 3138ns | -43.90% |
| atomic_seqcst | 5602ns | 5602ns | +0.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 3465ns | base | --- | [3464, 3466] | --- | --- | --- | --- |
| atomic_relaxed | 1033ns | -2431.4ns (-70.2%) | [-2432, -2431]ns | [1032, 1034] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 3469ns | +3.9ns (+0.1%) | [+3, +5]ns | [3468, 3470] | YES | 0.0000 | 0.0000 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 3465ns | -70.2% | +0.1% |
| 1 | 2 | 3464ns | -70.2% | +0.1% |
| 1 | 3 | 3462ns | -70.2% | +0.3% |
| 1 | 4 | 3464ns | -70.1% | +0.1% |
| 1 | 5 | 3483ns | -70.4% | -0.4% |
| 1 | 6 | 3464ns | -70.2% | +0.2% |
| 1 | 7 | 3464ns | -70.2% | +0.2% |
| 1 | 8 | 3467ns | -70.2% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.158 | ok |
| atomic_relaxed | -0.463 | moderate- |
| atomic_seqcst | 0.020 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 1/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 3.1ns | 3474.2ns | 0.1% |  |
| atomic_relaxed | 2.8ns | 1036.9ns | 0.3% |  |
| atomic_seqcst | 2.9ns | 3480.7ns | 0.1% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 3458.8-3515.3 ns)
   3458.8 |#############################
   3461.6 |########################################
   3464.4 |####################################
   3467.2 |############
   3470.1 |####
   3472.9 |#
   3475.7 |##
   3478.6 |###
   3481.4 |#
   3484.2 |####
   3487.0 |###
   3489.9 |###
   3492.7 |
   3495.5 |#
   3498.4 |
   3501.2 |####
   3504.0 |#
   3506.8 |#
   3509.7 |##
   3512.5 |
  (14 below, 12 above range)

atomic_relaxed (n=160, range 1028.4-1055.5 ns)
   1028.4 |##########################
   1029.8 |################
   1031.1 |#################################
   1032.5 |########################################
   1033.8 |#######################
   1035.2 |###########################
   1036.5 |############
   1037.9 |#
   1039.3 |###
   1040.6 |
   1042.0 |####
   1043.3 |###
   1044.7 |####
   1046.0 |###
   1047.4 |#
   1048.8 |#
   1050.1 |#
   1051.5 |
   1052.8 |######
   1054.2 |###
  (14 below, 7 above range)

atomic_seqcst (n=160, range 3463.0-3529.9 ns)
   3463.0 |################################
   3466.3 |########################################
   3469.7 |####################
   3473.0 |#
   3476.4 |
   3479.7 |
   3483.1 |
   3486.4 |#
   3489.8 |
   3493.1 |#
   3496.5 |
   3499.8 |#
   3503.2 |
   3506.5 |
   3509.8 |#
   3513.2 |#
   3516.5 |
   3519.9 |
   3523.2 |
   3526.6 |####
  (15 below, 20 above range)

```
