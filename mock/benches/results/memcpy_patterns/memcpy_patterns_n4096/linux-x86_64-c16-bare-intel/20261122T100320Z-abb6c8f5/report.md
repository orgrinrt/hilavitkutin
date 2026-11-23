# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### memcpy_naive dominates: 21% faster than the next best (memcpy_intrinsic)

memcpy_naive (3.83 us) leads memcpy_intrinsic (4.64 us) by 21%, a clear separation rather than a photo finish. CV 9.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### memcpy_naive's edge over baseline is significant but tiny (-3 ns, 0.07%)

memcpy_naive differs from baseline memcpy_intrinsic by -3 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: memcpy_naive** at 3825.2 ns median (-17.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.22x (fastest 3825.2 ns, slowest 4664.1 ns)

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
| Counter frequency | 2112541357 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 5981ns | 6514ns | 5353ns | 5988ns | 6588ns | base |
| memcpy_naive | 5661ns | 5362ns | 5349ns | 5465ns | 6559ns | -5.36% |
| memcpy_unrolled | 6430ns | 6550ns | 5355ns | 6456ns | 7428ns | +7.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 4259ns | 3816ns | 4691ns | base | 0.962 |
| memcpy_naive | 4032ns | 3816ns | 4670ns | -5.33% | 1.016 |
| memcpy_unrolled | 4579ns | 3816ns | 5282ns | +7.51% | 0.894 |

## Performance model

- Peak throughput: **1.073 Gops/s** (memcpy_intrinsic; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.882 | 82.2% |
| memcpy_naive | 1.071 | 99.8% |
| memcpy_unrolled | 0.878 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 5981ns | 5981ns | base |
| memcpy_naive | 5661ns | 5661ns | -5.36% |
| memcpy_unrolled | 6430ns | 6430ns | +7.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 4644ns | base | --- | [3828, 4664] | --- | --- | --- | --- |
| memcpy_naive | 3825ns | -3.0ns (-0.1%) | [-10, -1]ns | [3817, 3826] | YES | 0.0012 | 0.0006 | 2 |
| memcpy_unrolled | 4664ns | no significant difference | [-0, +2]ns | [4664, 4664] | no | 0.6909 | 0.6909 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 3821ns | -0.0% | +22.2% |
| 1 | 2 | 4668ns | -0.1% | +0.2% |
| 1 | 3 | 3816ns | +0.0% | -0.0% |
| 1 | 4 | 3816ns | +0.0% | -0.0% |
| 1 | 5 | 4695ns | -18.7% | -0.6% |
| 1 | 6 | 4664ns | -18.1% | -0.0% |
| 1 | 7 | 3827ns | -0.3% | +21.9% |
| 1 | 8 | 4670ns | -0.0% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.366 | moderate- |
| memcpy_naive | -0.207 | moderate- |
| memcpy_unrolled | 0.287 | moderate+ |

**Consistency summary:**

- **memcpy_naive**: won 3/8, lost 0/8
- **memcpy_unrolled**: won 2/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 22.6ns | 4259.1ns | 0.5% |  |
| memcpy_naive | 21.3ns | 4032.3ns | 0.5% |  |
| memcpy_unrolled | 24.3ns | 4579.1ns | 0.5% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 3815.8-4691.0 ns)
   3815.8 |##################################
   3859.6 |
   3903.4 |
   3947.1 |
   3990.9 |#
   4034.6 |
   4078.4 |
   4122.2 |
   4165.9 |
   4209.7 |
   4253.4 |
   4297.2 |
   4341.0 |
   4384.7 |
   4428.5 |
   4472.2 |
   4516.0 |
   4559.8 |
   4603.5 |
   4647.3 |########################################
  (17 below, 14 above range)

memcpy_naive (n=160, range 3815.9-4670.4 ns)
   3815.9 |########################################
   3858.6 |
   3901.3 |
   3944.1 |
   3986.8 |
   4029.5 |
   4072.2 |
   4115.0 |
   4157.7 |
   4200.4 |
   4243.1 |
   4285.8 |
   4328.6 |
   4371.3 |
   4414.0 |
   4456.7 |
   4499.5 |
   4542.2 |
   4584.9 |
   4627.6 |########
  (10 below, 17 above range)

memcpy_unrolled (n=160, range 3816.4-5281.6 ns)
   3816.4 |####
   3889.7 |
   3962.9 |
   4036.2 |
   4109.5 |
   4182.7 |
   4256.0 |
   4329.2 |
   4402.5 |
   4475.8 |
   4549.0 |
   4622.3 |########################################
   4695.5 |
   4768.8 |
   4842.1 |
   4915.3 |
   4988.6 |
   5061.8 |
   5135.1 |
   5208.4 |
  (26 below, 5 above range)

```
