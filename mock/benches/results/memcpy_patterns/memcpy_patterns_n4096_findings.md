# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.5% (fastest 5.50 us to slowest 5.63 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (137 ns) is smaller than the fastest variant's own run-to-run std-dev (206 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader memcpy_unrolled vs stability leader memcpy_naive (+2% speed for 1.2x steadier)

memcpy_unrolled is fastest (5.50 us, CV 3.8%); memcpy_naive gives up 2.5% median for 1.2x lower variance (CV 3.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: memcpy_unrolled** at 5497.7 ns median (-2.3% vs baseline)
- Spread: 1.02x (fastest 5497.7 ns, slowest 5634.6 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 7727ns | 7860ns | 7376ns | 7748ns | 8016ns | base |
| memcpy_naive | 7719ns | 7872ns | 7371ns | 7750ns | 7974ns | -0.11% |
| memcpy_unrolled | 7697ns | 7692ns | 7369ns | 7688ns | 8051ns | -0.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 5528ns | 5273ns | 5736ns | base | 0.741 |
| memcpy_naive | 5522ns | 5271ns | 5702ns | -0.10% | 0.742 |
| memcpy_unrolled | 5506ns | 5271ns | 5768ns | -0.41% | 0.744 |

## Performance model

- Peak throughput: **0.777 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.728 | 93.7% |
| memcpy_naive | 0.727 | 93.5% |
| memcpy_unrolled | 0.745 | 95.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 7727ns | 7727ns | base |
| memcpy_naive | 7719ns | 7719ns | -0.11% |
| memcpy_unrolled | 7697ns | 7697ns | -0.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 5625ns | base | --- | [5481, 5638] | --- | --- | --- | --- |
| memcpy_naive | 5635ns | no significant difference | [-2, +8]ns | [5499, 5643] | no | 0.4087 | 0.2043 | 1 |
| memcpy_unrolled | 5498ns | no significant difference | [-75, +12]ns | [5445, 5621] | no | 0.9370 | 0.9370 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 5636ns | -6.4% | +0.3% |
| 1 | 2 | 5450ns | +3.5% | +0.5% |
| 1 | 3 | 5651ns | +0.1% | -6.5% |
| 1 | 4 | 5413ns | +4.5% | +4.0% |
| 1 | 5 | 5639ns | -4.5% | -3.3% |
| 1 | 6 | 5646ns | -0.0% | -6.3% |
| 1 | 7 | 5643ns | -3.6% | +0.2% |
| 1 | 8 | 5282ns | +6.9% | +6.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.358 | moderate- |
| memcpy_naive | -0.352 | moderate- |
| memcpy_unrolled | -0.179 | ok |

**Consistency summary:**

- **memcpy_naive**: won 3/8, lost 3/8
- **memcpy_unrolled**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 3.9ns | 5527.9ns | 0.1% |  |
| memcpy_naive | 3.8ns | 5522.2ns | 0.1% |  |
| memcpy_unrolled | 3.8ns | 5505.5ns | 0.1% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 5272.5-5735.7 ns)
   5272.5 |########
   5295.7 |#######
   5318.9 |####
   5342.0 |##
   5365.2 |
   5388.3 |####
   5411.5 |##
   5434.7 |##############
   5457.8 |####
   5481.0 |####
   5504.1 |
   5527.3 |##
   5550.5 |##
   5573.6 |#
   5596.8 |####
   5619.9 |##########################
   5643.1 |########################################
   5666.3 |##
   5689.4 |###
   5712.6 |##
  (22 below, 7 above range)

memcpy_naive (n=160, range 5271.3-5701.8 ns)
   5271.3 |##########
   5292.8 |####
   5314.3 |##
   5335.9 |
   5357.4 |#
   5378.9 |###
   5400.4 |##
   5422.0 |####
   5443.5 |#####
   5465.0 |###
   5486.6 |#
   5508.1 |#
   5529.6 |
   5551.1 |##
   5572.7 |
   5594.2 |#
   5615.7 |####
   5637.2 |########################################
   5658.8 |##########
   5680.3 |##
  (17 below, 8 above range)

memcpy_unrolled (n=160, range 5270.8-5767.5 ns)
   5270.8 |################################
   5295.7 |############
   5320.5 |###
   5345.3 |#######
   5370.2 |##
   5395.0 |#
   5419.8 |######
   5444.7 |############
   5469.5 |###
   5494.3 |##
   5519.2 |###
   5544.0 |#
   5568.9 |###
   5593.7 |###
   5618.5 |##########################
   5643.4 |########################################
   5668.2 |#
   5693.0 |##
   5717.9 |#####
   5742.7 |
  (14 below, 8 above range)

```
