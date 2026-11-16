# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### atomic_relaxed dominates: 235% faster than the next best (atomic_acquire)

atomic_relaxed (1.05 us) leads atomic_acquire (3.53 us) by 235%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### atomic_relaxed beats baseline by 70% (significant)

atomic_relaxed is -2.48 us (70%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_seqcst is an outlier: 3.4x slower than the field

atomic_seqcst (3.53 us) is 3.4x the fastest (1.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### atomic_relaxed shows warm-up / thermal drift (autocorr +0.62)

atomic_relaxed's per-pass series has lag-1 autocorrelation +0.62, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.4x the fastest

Fastest atomic_relaxed (1.05 us) to slowest atomic_seqcst (3.53 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### atomic_seqcst's edge over baseline is significant but tiny (4 ns, 0.11%)

atomic_seqcst differs from baseline atomic_acquire by 4 ns (0.11%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: atomic_relaxed** at 1052.1 ns median (-70.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.36x (fastest 1052.1 ns, slowest 3533.8 ns)

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
| atomic_acquire | 5692ns | 5686ns | 5679ns | 5687ns | 5723ns | base |
| atomic_relaxed | 3191ns | 3186ns | 3177ns | 3186ns | 3221ns | -43.94% |
| atomic_seqcst | 5693ns | 5684ns | 5677ns | 5684ns | 5733ns | +0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 3533ns | 3524ns | 3553ns | base | 1.159 |
| atomic_relaxed | 1054ns | 1048ns | 1065ns | -70.17% | 3.887 |
| atomic_seqcst | 3539ns | 3529ns | 3564ns | +0.16% | 1.157 |

## Performance model

- Peak throughput: **3.907 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.160 | 29.7% |
| atomic_relaxed | 3.893 | 99.7% |
| atomic_seqcst | 1.159 | 29.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 5692ns | 5692ns | base |
| atomic_relaxed | 3191ns | 3191ns | -43.94% |
| atomic_seqcst | 5693ns | 5693ns | +0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 3530ns | base | --- | [3529, 3530] | --- | --- | --- | --- |
| atomic_relaxed | 1052ns | -2477.5ns (-70.2%) | [-2478, -2477]ns | [1051, 1052] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 3534ns | +3.8ns (+0.1%) | [+3, +5]ns | [3533, 3534] | YES | 0.0000 | 0.0000 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 3529ns | -70.2% | +0.1% |
| 1 | 2 | 3530ns | -70.2% | +0.1% |
| 1 | 3 | 3531ns | -70.2% | +0.1% |
| 1 | 4 | 3530ns | -70.2% | +0.1% |
| 1 | 5 | 3530ns | -70.2% | +0.2% |
| 1 | 6 | 3530ns | -70.2% | +0.1% |
| 1 | 7 | 3529ns | -70.2% | +0.1% |
| 1 | 8 | 3530ns | -70.2% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | 0.055 | ok |
| atomic_relaxed | 0.624 | HIGH+ (drift/warm-up) |
| atomic_seqcst | -0.269 | moderate- |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 0/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 3.0ns | 3533.2ns | 0.1% |  |
| atomic_relaxed | 2.8ns | 1053.9ns | 0.3% |  |
| atomic_seqcst | 2.9ns | 3538.7ns | 0.1% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 3524.2-3552.8 ns)
   3524.2 |###############
   3525.6 |##########################
   3527.0 |##########################
   3528.5 |########################################
   3529.9 |########################################
   3531.3 |#######################
   3532.8 |###############
   3534.2 |######
   3535.6 |
   3537.1 |
   3538.5 |#
   3539.9 |
   3541.4 |#
   3542.8 |###
   3544.2 |###
   3545.6 |
   3547.1 |###
   3548.5 |
   3549.9 |#
   3551.4 |#
  (14 below, 11 above range)

atomic_relaxed (n=160, range 1048.4-1065.3 ns)
   1048.4 |################
   1049.3 |#######################
   1050.1 |###############################
   1051.0 |###############################
   1051.8 |########################################
   1052.6 |#######################
   1053.5 |####################
   1054.3 |########
   1055.2 |################
   1056.0 |#####
   1056.9 |
   1057.7 |#####
   1058.5 |
   1059.4 |
   1060.2 |
   1061.1 |
   1061.9 |
   1062.8 |
   1063.6 |
   1064.4 |
  (17 below, 10 above range)

atomic_seqcst (n=160, range 3528.8-3563.9 ns)
   3528.8 |###############
   3530.6 |###########################
   3532.3 |##############################
   3534.1 |########################################
   3535.8 |#######
   3537.6 |######
   3539.4 |
   3541.1 |
   3542.9 |
   3544.6 |#
   3546.4 |##
   3548.1 |##
   3549.9 |#
   3551.6 |##
   3553.4 |
   3555.1 |#
   3556.9 |
   3558.6 |#
   3560.4 |
   3562.1 |
  (15 below, 10 above range)

```

## Diagnostics

- **atomic_relaxed**: autocorrelation=0.62 (measurement drift or warm-up artifact)
