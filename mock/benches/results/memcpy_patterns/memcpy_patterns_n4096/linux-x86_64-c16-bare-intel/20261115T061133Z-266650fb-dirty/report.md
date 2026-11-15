# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### memcpy_intrinsic is fastest but the noisiest (CV 10.0%)

memcpy_intrinsic wins on median (4.25 us) yet has the highest variance (CV 10.0%), while memcpy_naive is the steadiest (CV 0.4%, 4.66 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (419 ns) is smaller than the fastest variant's own run-to-run std-dev (425 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (memcpy_intrinsic)

The baseline memcpy_intrinsic is the fastest (4.25 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader memcpy_intrinsic vs stability leader memcpy_naive (+10% speed for 25.8x steadier)

memcpy_intrinsic is fastest (4.25 us, CV 10.0%); memcpy_naive gives up 9.9% median for 25.8x lower variance (CV 0.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### memcpy_unrolled's edge over baseline is significant but tiny (10 ns, 0.23%)

memcpy_unrolled differs from baseline memcpy_intrinsic by 10 ns (0.23%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (memcpy_intrinsic) is the fastest** at 4245.6 ns median
- 2 variants significantly slower than baseline
- Spread: 1.10x (fastest 4245.6 ns, slowest 4664.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4646464650504040 |
| Counter frequency | 2112632730 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 5957ns | 5996ns | 5349ns | 5956ns | 6567ns | base |
| memcpy_naive | 6559ns | 6553ns | 6540ns | 6552ns | 6598ns | +10.11% |
| memcpy_unrolled | 6277ns | 6549ns | 5356ns | 6452ns | 6674ns | +5.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 4245ns | 3815ns | 4677ns | base | 0.965 |
| memcpy_naive | 4672ns | 4663ns | 4688ns | +10.06% | 0.877 |
| memcpy_unrolled | 4476ns | 3818ns | 4761ns | +5.44% | 0.915 |

## Performance model

- Peak throughput: **1.074 Gops/s** (memcpy_intrinsic; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.965 | 89.9% |
| memcpy_naive | 0.878 | 81.8% |
| memcpy_unrolled | 0.878 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 5957ns | 5957ns | base |
| memcpy_naive | 6559ns | 6559ns | +10.11% |
| memcpy_unrolled | 6277ns | 6277ns | +5.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 4246ns | base | --- | [3826, 4664] | --- | --- | --- | --- |
| memcpy_naive | 4665ns | +501.1ns (+11.8%) | [+12, +840]ns | [4664, 4674] | YES | 0.0000 | 0.0000 | 1 |
| memcpy_unrolled | 4665ns | +10.0ns (+0.2%) | [+2, +12]ns | [4665, 4675] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 4675ns | -0.2% | +0.0% |
| 1 | 2 | 4665ns | +0.1% | +0.6% |
| 1 | 3 | 4669ns | -0.1% | +0.0% |
| 1 | 4 | 3815ns | +22.3% | +22.6% |
| 1 | 5 | 3817ns | +22.2% | +0.0% |
| 1 | 6 | 4664ns | +0.2% | +0.2% |
| 1 | 7 | 3816ns | +22.2% | -0.0% |
| 1 | 8 | 3816ns | +22.5% | +22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | 0.127 | ok |
| memcpy_naive | -0.293 | moderate- |
| memcpy_unrolled | -0.363 | moderate- |

**Consistency summary:**

- **memcpy_naive**: won 2/8, lost 5/8
- **memcpy_unrolled**: won 0/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 22.5ns | 4244.7ns | 0.5% |  |
| memcpy_naive | 24.7ns | 4671.6ns | 0.5% |  |
| memcpy_unrolled | 23.7ns | 4475.5ns | 0.5% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 3815.2-4676.6 ns)
   3815.2 |#####################################
   3858.3 |
   3901.4 |
   3944.4 |
   3987.5 |
   4030.6 |
   4073.6 |
   4116.7 |
   4159.8 |
   4202.9 |
   4245.9 |
   4289.0 |
   4332.1 |
   4375.1 |
   4418.2 |
   4461.3 |
   4504.3 |
   4547.4 |
   4590.5 |
   4633.5 |########################################
  (14 below, 9 above range)

memcpy_naive (n=160, range 4663.3-4687.7 ns)
   4663.3 |########################################
   4664.5 |#####
   4665.8 |
   4667.0 |
   4668.2 |
   4669.4 |
   4670.6 |
   4671.9 |
   4673.1 |######
   4674.3 |##############
   4675.5 |###############
   4676.8 |####
   4678.0 |
   4679.2 |
   4680.4 |
   4681.6 |
   4682.9 |
   4684.1 |
   4685.3 |
   4686.5 |
  (10 below, 3 above range)

memcpy_unrolled (n=160, range 3818.0-4760.9 ns)
   3818.0 |########
   3865.1 |
   3912.3 |
   3959.4 |
   4006.6 |
   4053.7 |
   4100.8 |
   4148.0 |
   4195.1 |
   4242.3 |
   4289.4 |
   4336.6 |
   4383.7 |
   4430.9 |
   4478.0 |
   4525.2 |
   4572.3 |
   4619.5 |##########################
   4666.6 |########################################
   4713.7 |
  (25 below, 2 above range)

```
