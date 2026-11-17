# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### atomic_relaxed dominates: 229% faster than the next best (atomic_acquire)

atomic_relaxed (4.27 us) leads atomic_acquire (14.02 us) by 229%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### atomic_relaxed beats baseline by 70% (significant)

atomic_relaxed is -9.75 us (70%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_seqcst is an outlier: 3.3x slower than the field

atomic_seqcst (14.03 us) is 3.3x the fastest (4.27 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.3x the fastest

Fastest atomic_relaxed (4.27 us) to slowest atomic_seqcst (14.03 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: atomic_relaxed** at 4266.7 ns median (-69.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.29x (fastest 4266.7 ns, slowest 14033.5 ns)

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
| atomic_acquire | 16171ns | 16142ns | 16126ns | 16149ns | 16282ns | base |
| atomic_relaxed | 6406ns | 6364ns | 6351ns | 6367ns | 6579ns | -60.38% |
| atomic_seqcst | 16236ns | 16152ns | 16122ns | 16158ns | 16584ns | +0.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 14049ns | 14012ns | 14148ns | base | 1.166 |
| atomic_relaxed | 4295ns | 4258ns | 4411ns | -69.43% | 3.815 |
| atomic_seqcst | 14107ns | 14011ns | 14413ns | +0.42% | 1.161 |

## Performance model

- Peak throughput: **3.848 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.169 | 30.4% |
| atomic_relaxed | 3.840 | 99.8% |
| atomic_seqcst | 1.167 | 30.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 16171ns | 16171ns | base |
| atomic_relaxed | 6406ns | 6406ns | -60.38% |
| atomic_seqcst | 16236ns | 16236ns | +0.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 14020ns | base | --- | [14018, 14025] | --- | --- | --- | --- |
| atomic_relaxed | 4267ns | -9754.6ns (-69.6%) | [-9757, -9752]ns | [4265, 4269] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 14034ns | no significant difference | [-2, +7]ns | [14019, 14038] | no | 0.2668 | 0.2668 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 14029ns | -69.6% | -0.1% |
| 1 | 2 | 14028ns | -69.6% | -0.1% |
| 1 | 3 | 14028ns | -69.6% | -0.1% |
| 1 | 4 | 14019ns | -69.6% | +3.5% |
| 1 | 5 | 14015ns | -68.4% | +0.0% |
| 1 | 6 | 14026ns | -69.6% | +0.1% |
| 1 | 7 | 14019ns | -69.6% | -0.0% |
| 1 | 8 | 14019ns | -69.6% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | 0.213 | moderate+ |
| atomic_relaxed | -0.174 | ok |
| atomic_seqcst | -0.174 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 0/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 3.0ns | 14048.5ns | 0.0% |  |
| atomic_relaxed | 3.0ns | 4294.6ns | 0.1% |  |
| atomic_seqcst | 3.1ns | 14107.0ns | 0.0% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 14012.0-14147.7 ns)
  14012.0 |########################################
  14018.8 |############
  14025.6 |#####
  14032.3 |#
  14039.1 |####
  14045.9 |###
  14052.7 |###
  14059.5 |####
  14066.3 |#
  14073.1 |###
  14079.8 |
  14086.6 |###
  14093.4 |##
  14100.2 |#
  14107.0 |
  14113.8 |#
  14120.6 |
  14127.4 |
  14134.1 |
  14140.9 |
  (14 below, 8 above range)

atomic_relaxed (n=160, range 4257.7-4410.7 ns)
   4257.7 |########################################
   4265.3 |#################################
   4273.0 |########
   4280.6 |####
   4288.3 |####
   4295.9 |
   4303.6 |
   4311.2 |#
   4318.9 |#
   4326.5 |
   4334.2 |
   4341.8 |
   4349.5 |
   4357.1 |
   4364.8 |
   4372.4 |
   4380.1 |
   4387.8 |
   4395.4 |
   4403.1 |
  (13 below, 15 above range)

atomic_seqcst (n=160, range 14011.3-14413.0 ns)
  14011.3 |########################################
  14031.4 |###############
  14051.5 |######
  14071.5 |######
  14091.6 |####
  14111.7 |#
  14131.8 |#
  14151.9 |
  14172.0 |
  14192.1 |#
  14212.1 |
  14232.2 |
  14252.3 |
  14272.4 |
  14292.5 |
  14312.6 |
  14332.6 |
  14352.7 |
  14372.8 |
  14392.9 |
  (14 below, 16 above range)

```
