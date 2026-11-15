# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 150 ns to slowest 150 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### load_aligned is fastest but the noisiest (CV 87.6%)

load_aligned wins on median (150 ns) yet has the highest variance (CV 87.6%), while load_byte_pack is the steadiest (CV 84.8%, 150 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (131 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (150 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### load_byte_pack's comparison is tie-heavy (12% tied pairs)

12% of paired samples for load_byte_pack are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader load_aligned vs stability leader load_byte_pack (+0% speed for 1.0x steadier)

load_aligned is fastest (150 ns, CV 87.6%); load_byte_pack gives up 0.0% median for 1.0x lower variance (CV 84.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### load_aligned is inconsistent: worst-20% is 3.7x its best-20%

load_aligned's best 20% of batches run at 122 ns but its worst 20% at 451 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (load_aligned) is the fastest** at 149.5 ns median
- Spread: 1.00x (fastest 149.5 ns, slowest 149.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x2c8af49106b7ed53 |
| Counter frequency | 2112776509 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 2718ns | 2022ns | 1656ns | 1922ns | 6171ns | base |
| load_byte_pack | 2790ns | 2025ns | 1656ns | 2050ns | 6143ns | +2.63% |
| load_unaligned | 2831ns | 2027ns | 1705ns | 2051ns | 6300ns | +4.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 200ns | 122ns | 451ns | base | 5.125 |
| load_byte_pack | 205ns | 122ns | 450ns | +2.77% | 4.987 |
| load_unaligned | 206ns | 126ns | 448ns | +2.98% | 4.976 |

## Performance model

- Peak throughput: **8.375 Gops/s** (load_aligned; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 6.849 | 81.8% |
| load_byte_pack | 6.849 | 81.8% |
| load_unaligned | 6.849 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2718ns | 2718ns | base |
| load_byte_pack | 2790ns | 2790ns | +2.63% |
| load_unaligned | 2831ns | 2831ns | +4.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 150ns | base | --- | [149, 150] | --- | --- | --- | --- |
| load_byte_pack | 150ns | no significant difference | [+0, +0]ns | [150, 150] | no | 0.4470 | 0.4470 | **20** (12%, HIGH) |
| load_unaligned | 150ns | no significant difference | [+0, +0]ns | [150, 150] | no | 0.0502 | 0.0251 | 14 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 150ns | +0.0% | +0.0% |
| 1 | 2 | 122ns | +22.3% | -0.1% |
| 1 | 3 | 122ns | +0.2% | +22.3% |
| 1 | 4 | 150ns | +0.1% | +0.0% |
| 1 | 5 | 122ns | +22.1% | +22.2% |
| 1 | 6 | 150ns | +0.0% | -18.1% |
| 1 | 7 | 150ns | -18.1% | +0.0% |
| 1 | 8 | 122ns | -0.0% | +22.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | -0.375 | moderate- |
| load_byte_pack | 0.059 | ok |
| load_unaligned | -0.375 | moderate- |

**Consistency summary:**

- **load_byte_pack**: won 1/8, lost 4/8
- **load_unaligned**: won 1/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 31.7ns | 199.8ns | 15.9% | HIGH |
| load_byte_pack | 32.6ns | 205.3ns | 15.9% | HIGH |
| load_unaligned | 32.5ns | 205.8ns | 15.8% | HIGH |

## Distribution (algo ns)

```
load_aligned (n=160, range 122.3-450.7 ns)
    122.3 |###################################
    138.7 |########################################
    155.1 |
    171.5 |
    188.0 |
    204.4 |
    220.8 |
    237.2 |
    253.7 |
    270.1 |
    286.5 |
    302.9 |
    319.4 |
    335.8 |
    352.2 |#
    368.6 |#
    385.1 |
    401.5 |#
    417.9 |#
    434.3 |##
  (8 below, 16 above range)

load_byte_pack (n=160, range 122.3-449.7 ns)
    122.3 |###########
    138.7 |########################################
    155.0 |
    171.4 |#
    187.8 |
    204.2 |
    220.5 |
    236.9 |
    253.3 |
    269.6 |
    286.0 |
    302.4 |#
    318.7 |
    335.1 |
    351.5 |
    367.8 |#
    384.2 |
    400.6 |###
    417.0 |
    433.3 |###
  (22 below, 14 above range)

load_unaligned (n=160, range 126.2-448.5 ns)
    126.2 |
    142.3 |########################################
    158.4 |#
    174.5 |
    190.7 |
    206.8 |
    222.9 |
    239.0 |
    255.1 |
    271.2 |
    287.3 |
    303.4 |
    319.6 |
    335.7 |
    351.8 |
    367.9 |
    384.0 |#
    400.1 |
    416.2 |
    432.3 |
  (27 below, 17 above range)

```

## Diagnostics

- **load_aligned**: CV=65.6% (high variance, measurements may be unstable)
- **load_aligned**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **load_aligned**: bridge=15.9% of algo (FFI overhead may distort results)
- **load_byte_pack**: CV=61.8% (high variance, measurements may be unstable)
- **load_byte_pack**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **load_byte_pack**: bridge=15.9% of algo (FFI overhead may distort results)
- **load_unaligned**: CV=62.4% (high variance, measurements may be unstable)
- **load_unaligned**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **load_unaligned**: bridge=15.9% of algo (FFI overhead may distort results)
