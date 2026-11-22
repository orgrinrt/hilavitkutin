# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### memcpy_naive is fastest but the noisiest (CV 10.0%)

memcpy_naive wins on median (4.25 us) yet has the highest variance (CV 10.0%), while memcpy_unrolled is the steadiest (CV 7.9%, 4.66 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (414 ns) is smaller than the fastest variant's own run-to-run std-dev (425 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader memcpy_naive vs stability leader memcpy_unrolled (+10% speed for 1.3x steadier)

memcpy_naive is fastest (4.25 us, CV 10.0%); memcpy_unrolled gives up 9.7% median for 1.3x lower variance (CV 7.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: memcpy_naive** at 4250.1 ns median (-2.3% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.10x (fastest 4250.1 ns, slowest 4664.0 ns)

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
| Counter frequency | 2112829592 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 5971ns | 6110ns | 5356ns | 5967ns | 6597ns | base |
| memcpy_naive | 5965ns | 6008ns | 5357ns | 5964ns | 6577ns | -0.10% |
| memcpy_unrolled | 6273ns | 6555ns | 5364ns | 6458ns | 6629ns | +5.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 4248ns | 3816ns | 4686ns | base | 0.964 |
| memcpy_naive | 4245ns | 3816ns | 4677ns | -0.06% | 0.965 |
| memcpy_unrolled | 4460ns | 3818ns | 4693ns | +5.00% | 0.918 |

## Performance model

- Peak throughput: **1.073 Gops/s** (memcpy_intrinsic; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.941 | 87.7% |
| memcpy_naive | 0.964 | 89.8% |
| memcpy_unrolled | 0.878 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 5971ns | 5971ns | base |
| memcpy_naive | 5965ns | 5965ns | -0.10% |
| memcpy_unrolled | 6273ns | 6273ns | +5.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 4352ns | base | --- | [3827, 4663] | --- | --- | --- | --- |
| memcpy_naive | 4250ns | no significant difference | [+0, +1]ns | [3827, 4664] | no | 0.0669 | 0.0669 | 2 |
| memcpy_unrolled | 4664ns | +377.7ns (+8.7%) | [+4, +838]ns | [4664, 4664] | YES | 0.0049 | 0.0025 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 4665ns | +0.1% | +0.2% |
| 1 | 2 | 4664ns | -18.2% | -18.2% |
| 1 | 3 | 3816ns | -0.0% | +22.2% |
| 1 | 4 | 4665ns | +0.2% | +0.2% |
| 1 | 5 | 3816ns | +22.2% | +22.3% |
| 1 | 6 | 3816ns | +0.1% | +22.3% |
| 1 | 7 | 4669ns | -0.1% | -18.2% |
| 1 | 8 | 3821ns | -0.1% | +22.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.376 | moderate- |
| memcpy_naive | -0.371 | moderate- |
| memcpy_unrolled | -0.370 | moderate- |

**Consistency summary:**

- **memcpy_naive**: won 3/8, lost 4/8
- **memcpy_unrolled**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 22.6ns | 4248.1ns | 0.5% |  |
| memcpy_naive | 22.4ns | 4245.4ns | 0.5% |  |
| memcpy_unrolled | 23.8ns | 4460.5ns | 0.5% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 3815.9-4686.0 ns)
   3815.9 |#################################
   3859.4 |
   3902.9 |
   3946.4 |
   3989.9 |
   4033.4 |
   4076.9 |
   4120.4 |
   4163.9 |
   4207.4 |
   4250.9 |
   4294.4 |
   4337.9 |
   4381.4 |
   4424.9 |
   4468.4 |
   4511.9 |
   4555.4 |
   4598.9 |
   4642.5 |########################################
  (14 below, 2 above range)

memcpy_naive (n=160, range 3815.9-4677.2 ns)
   3815.9 |####################################
   3858.9 |
   3902.0 |
   3945.1 |
   3988.1 |
   4031.2 |
   4074.3 |
   4117.3 |
   4160.4 |
   4203.5 |
   4246.5 |
   4289.6 |
   4332.7 |
   4375.7 |
   4418.8 |
   4461.9 |
   4504.9 |
   4548.0 |
   4591.1 |
   4634.1 |########################################
  (14 below, 8 above range)

memcpy_unrolled (n=160, range 3818.3-4693.1 ns)
   3818.3 |#####
   3862.1 |
   3905.8 |
   3949.5 |
   3993.3 |
   4037.0 |
   4080.7 |
   4124.5 |
   4168.2 |
   4212.0 |
   4255.7 |
   4299.4 |
   4343.2 |
   4386.9 |
   4430.7 |
   4474.4 |
   4518.1 |
   4561.9 |
   4605.6 |
   4649.3 |########################################
  (25 below, 4 above range)

```
