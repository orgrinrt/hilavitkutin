# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### acc_atomic_relaxed dominates: 22% faster than the next best (acc_plain)

acc_atomic_relaxed (3.75 us) leads acc_plain (4.57 us) by 22%, a clear separation rather than a photo finish. CV 11.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### acc_atomic_relaxed is fastest but the noisiest (CV 11.2%)

acc_atomic_relaxed wins on median (3.75 us) yet has the highest variance (CV 11.2%), while acc_atomic_seqcst is the steadiest (CV 8.8%, 4.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (acc_atomic_relaxed)

The baseline acc_atomic_relaxed is the fastest (3.75 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### acc_atomic_seqcst's edge over baseline is significant but tiny (0 ns, 0.01%)

acc_atomic_seqcst differs from baseline acc_atomic_relaxed by 0 ns (0.01%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (acc_atomic_relaxed) is the fastest** at 3746.6 ns median
- 2 variants significantly slower than baseline
- Spread: 1.22x (fastest 3746.6 ns, slowest 4567.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4823d76519bef0a3 |
| Counter frequency | 2112846992 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 5741ns | 5292ns | 5279ns | 5635ns | 6518ns | base |
| acc_atomic_seqcst | 6033ns | 6454ns | 5282ns | 6122ns | 6517ns | +5.09% |
| acc_plain | 6033ns | 6455ns | 5281ns | 6125ns | 6510ns | +5.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 4062ns | 3737ns | 4619ns | base | 1.008 |
| acc_atomic_seqcst | 4262ns | 3737ns | 4583ns | +4.91% | 0.961 |
| acc_plain | 4268ns | 3737ns | 4603ns | +5.08% | 0.960 |

## Performance model

- Peak throughput: **1.096 Gops/s** (acc_atomic_relaxed; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 1.093 | 99.7% |
| acc_atomic_seqcst | 0.897 | 81.8% |
| acc_plain | 0.897 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 5741ns | 5741ns | base |
| acc_atomic_seqcst | 6033ns | 6033ns | +5.09% |
| acc_plain | 6033ns | 6033ns | +5.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 3747ns | base | --- | [3746, 3748] | --- | --- | --- | --- |
| acc_atomic_seqcst | 4567ns | +0.4ns (+0.0%) | [+0, +2]ns | [4567, 4567] | YES | 0.0002 | 0.0001 | 2 |
| acc_plain | 4567ns | +10.8ns (+0.3%) | [+0, +14]ns | [4567, 4567] | YES | 0.0157 | 0.0157 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 3737ns | +0.0% | +22.5% |
| 1 | 2 | 3741ns | -0.1% | +22.3% |
| 1 | 3 | 4578ns | -0.1% | +0.0% |
| 1 | 4 | 4573ns | +0.0% | +0.0% |
| 1 | 5 | 3737ns | +0.1% | +0.1% |
| 1 | 6 | 4578ns | -0.2% | -18.4% |
| 1 | 7 | 3737ns | +22.5% | +22.2% |
| 1 | 8 | 3737ns | +22.2% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.143 | ok |
| acc_atomic_seqcst | 0.063 | ok |
| acc_plain | 0.068 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 3/8, lost 3/8
- **acc_plain**: won 1/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 20.9ns | 4061.9ns | 0.5% |  |
| acc_atomic_seqcst | 22.0ns | 4261.5ns | 0.5% |  |
| acc_plain | 21.9ns | 4268.3ns | 0.5% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 3736.8-4618.9 ns)
   3736.8 |########################################
   3780.9 |
   3825.0 |
   3869.1 |
   3913.2 |
   3957.3 |
   4001.4 |
   4045.5 |
   4089.6 |
   4133.7 |
   4177.8 |
   4221.9 |
   4266.0 |
   4310.2 |
   4354.3 |
   4398.4 |
   4442.5 |
   4486.6 |
   4530.7 |############
   4574.8 |###############
  (20 below, 5 above range)

acc_atomic_seqcst (n=160, range 3736.9-4582.9 ns)
   3736.9 |#################
   3779.2 |
   3821.5 |
   3863.8 |
   3906.1 |
   3948.4 |
   3990.7 |
   4033.0 |
   4075.3 |
   4117.6 |
   4159.9 |
   4202.2 |
   4244.5 |
   4286.8 |
   4329.1 |
   4371.4 |
   4413.7 |
   4456.0 |
   4498.3 |
   4540.6 |########################################
  (19 below, 5 above range)

acc_plain (n=160, range 3736.9-4602.5 ns)
   3736.9 |#################
   3780.2 |
   3823.4 |
   3866.7 |
   3910.0 |
   3953.3 |
   3996.6 |
   4039.8 |
   4083.1 |
   4126.4 |
   4169.7 |
   4213.0 |
   4256.3 |
   4299.5 |
   4342.8 |
   4386.1 |
   4429.4 |
   4472.7 |
   4515.9 |
   4559.2 |########################################
  (14 below, 2 above range)

```
