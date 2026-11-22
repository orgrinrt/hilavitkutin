# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### atomic_relaxed dominates: 236% faster than the next best (atomic_acquire)

atomic_relaxed (1.05 us) leads atomic_acquire (3.53 us) by 236%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### atomic_relaxed beats baseline by 70% (significant)

atomic_relaxed is -2.48 us (70%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_seqcst is an outlier: 3.4x slower than the field

atomic_seqcst (3.53 us) is 3.4x the fastest (1.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.4x the fastest

Fastest atomic_relaxed (1.05 us) to slowest atomic_seqcst (3.53 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### atomic_seqcst's edge over baseline is significant but tiny (4 ns, 0.11%)

atomic_seqcst differs from baseline atomic_acquire by 4 ns (0.11%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: atomic_relaxed** at 1051.2 ns median (-70.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.36x (fastest 1051.2 ns, slowest 3534.8 ns)

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
| atomic_acquire | 5689ns | 5675ns | 5641ns | 5678ns | 5771ns | base |
| atomic_relaxed | 3166ns | 3172ns | 3112ns | 3171ns | 3204ns | -44.35% |
| atomic_seqcst | 5680ns | 5677ns | 5611ns | 5678ns | 5752ns | -0.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 3540ns | 3510ns | 3590ns | base | 1.157 |
| atomic_relaxed | 1050ns | 1031ns | 1066ns | -70.34% | 3.902 |
| atomic_seqcst | 3536ns | 3491ns | 3583ns | -0.12% | 1.158 |

## Performance model

- Peak throughput: **3.974 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.160 | 29.2% |
| atomic_relaxed | 3.896 | 98.0% |
| atomic_seqcst | 1.159 | 29.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 5689ns | 5689ns | base |
| atomic_relaxed | 3166ns | 3166ns | -44.35% |
| atomic_seqcst | 5680ns | 5680ns | -0.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 3532ns | base | --- | [3531, 3533] | --- | --- | --- | --- |
| atomic_relaxed | 1051ns | -2482.1ns (-70.3%) | [-2488, -2480]ns | [1050, 1052] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 3535ns | +3.8ns (+0.1%) | [+2, +5]ns | [3534, 3535] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 3533ns | -70.2% | +0.2% |
| 1 | 2 | 3532ns | -70.2% | +0.1% |
| 1 | 3 | 3530ns | -70.2% | +0.2% |
| 1 | 4 | 3535ns | -70.2% | -1.9% |
| 1 | 5 | 3530ns | -70.8% | +0.1% |
| 1 | 6 | 3531ns | -70.2% | +0.1% |
| 1 | 7 | 3545ns | -70.3% | -0.3% |
| 1 | 8 | 3531ns | -70.8% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.351 | moderate- |
| atomic_relaxed | -0.233 | moderate- |
| atomic_seqcst | -0.158 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 2/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 3.2ns | 3539.7ns | 0.1% |  |
| atomic_relaxed | 2.8ns | 1049.8ns | 0.3% |  |
| atomic_seqcst | 3.0ns | 3535.7ns | 0.1% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 3510.4-3589.8 ns)
   3510.4 |
   3514.3 |
   3518.3 |
   3522.3 |#################
   3526.2 |##############################
   3530.2 |########################################
   3534.2 |###########
   3538.2 |#####
   3542.1 |#######
   3546.1 |###
   3550.1 |##
   3554.0 |
   3558.0 |
   3562.0 |
   3566.0 |
   3569.9 |
   3573.9 |###
   3577.9 |##
   3581.9 |#
   3585.8 |#
  (8 below, 10 above range)

atomic_relaxed (n=160, range 1030.7-1066.5 ns)
   1030.7 |###########
   1032.5 |#####
   1034.3 |#
   1036.1 |#
   1037.9 |#
   1039.6 |#
   1041.4 |
   1043.2 |
   1045.0 |##
   1046.8 |########
   1048.6 |######################
   1050.4 |########################################
   1052.2 |##################
   1054.0 |############
   1055.8 |#####
   1057.5 |##
   1059.3 |
   1061.1 |###
   1062.9 |#
   1064.7 |#
  (15 below, 12 above range)

atomic_seqcst (n=160, range 3491.0-3583.3 ns)
   3491.0 |
   3495.6 |
   3500.2 |
   3504.8 |
   3509.5 |
   3514.1 |
   3518.7 |
   3523.3 |##
   3527.9 |##################
   3532.5 |########################################
   3537.1 |############
   3541.7 |###
   3546.4 |#
   3551.0 |#
   3555.6 |#
   3560.2 |
   3564.8 |
   3569.4 |
   3574.0 |
   3578.7 |
  (20 below, 11 above range)

```
