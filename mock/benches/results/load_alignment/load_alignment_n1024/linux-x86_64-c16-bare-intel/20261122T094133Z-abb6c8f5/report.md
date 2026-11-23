# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### load_byte_pack dominates: 22% faster than the next best (load_aligned)

load_byte_pack (123 ns) leads load_aligned (149 ns) by 22%, a clear separation rather than a photo finish. CV 102.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### load_byte_pack is fastest but the noisiest (CV 102.8%)

load_byte_pack wins on median (123 ns) yet has the highest variance (CV 102.8%), while load_unaligned is the steadiest (CV 81.3%, 150 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (27 ns) is smaller than the fastest variant's own run-to-run std-dev (126 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### load_byte_pack's comparison is tie-heavy (14% tied pairs)

14% of paired samples for load_byte_pack are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### load_aligned is inconsistent: worst-20% is 3.7x its best-20%

load_aligned's best 20% of batches run at 122 ns but its worst 20% at 448 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### load_unaligned's edge over baseline is significant but tiny (0 ns, 0.20%)

load_unaligned differs from baseline load_aligned by 0 ns (0.20%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: load_byte_pack** at 122.6 ns median (-17.9% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.22x (fastest 122.6 ns, slowest 149.5 ns)

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
| Counter frequency | 2112580123 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 2724ns | 2022ns | 1655ns | 1937ns | 6153ns | base |
| load_byte_pack | 2718ns | 1713ns | 1655ns | 2005ns | 5918ns | -0.23% |
| load_unaligned | 2732ns | 2029ns | 1709ns | 2030ns | 5862ns | +0.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 200ns | 122ns | 448ns | base | 5.130 |
| load_byte_pack | 199ns | 122ns | 433ns | -0.08% | 5.134 |
| load_unaligned | 200ns | 126ns | 426ns | +0.28% | 5.116 |

## Performance model

- Peak throughput: **8.376 Gops/s** (load_aligned; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 6.854 | 81.8% |
| load_byte_pack | 8.352 | 99.7% |
| load_unaligned | 6.849 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2724ns | 2724ns | base |
| load_byte_pack | 2718ns | 2718ns | -0.23% |
| load_unaligned | 2732ns | 2732ns | +0.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 149ns | base | --- | [122, 150] | --- | --- | --- | --- |
| load_byte_pack | 123ns | no significant difference | [-0, +0]ns | [122, 149] | no | 0.4944 | 0.4944 | **23** (14%, HIGH) |
| load_unaligned | 150ns | +0.3ns (+0.2%) | [+0, +27]ns | [150, 150] | YES | 0.0010 | 0.0005 | 13 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 122ns | +0.1% | +0.1% |
| 1 | 2 | 150ns | -0.1% | -0.1% |
| 1 | 3 | 150ns | +0.0% | -18.1% |
| 1 | 4 | 122ns | +0.0% | +22.2% |
| 1 | 5 | 122ns | +0.0% | +22.2% |
| 1 | 6 | 122ns | +0.0% | +22.2% |
| 1 | 7 | 122ns | +0.1% | +22.1% |
| 1 | 8 | 150ns | -18.2% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.059 | ok |
| load_byte_pack | 0.292 | moderate+ |
| load_unaligned | -0.209 | moderate- |

**Consistency summary:**

- **load_byte_pack**: won 1/8, lost 0/8
- **load_unaligned**: won 1/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 31.8ns | 199.6ns | 15.9% | HIGH |
| load_byte_pack | 32.0ns | 199.4ns | 16.0% | HIGH |
| load_unaligned | 32.1ns | 200.2ns | 16.0% | HIGH |

## Distribution (algo ns)

```
load_aligned (n=160, range 122.3-447.9 ns)
    122.3 |########################################
    138.5 |##############################
    154.8 |
    171.1 |
    187.4 |
    203.7 |
    219.9 |
    236.2 |
    252.5 |#
    268.8 |#
    285.1 |
    301.3 |
    317.6 |
    333.9 |
    350.2 |
    366.5 |
    382.7 |
    399.0 |###
    415.3 |##
    431.6 |
  (15 below, 18 above range)

load_byte_pack (n=160, range 122.3-432.7 ns)
    122.3 |########################################
    137.8 |##################
    153.3 |
    168.8 |
    184.4 |
    199.9 |
    215.4 |
    230.9 |
    246.4 |
    262.0 |
    277.5 |
    293.0 |
    308.5 |
    324.1 |
    339.6 |#
    355.1 |##
    370.6 |#
    386.1 |##
    401.7 |###
    417.2 |
  (10 below, 13 above range)

load_unaligned (n=160, range 126.0-426.3 ns)
    126.0 |
    141.0 |########################################
    156.0 |
    171.0 |
    186.1 |
    201.1 |
    216.1 |
    231.1 |
    246.1 |
    261.1 |
    276.2 |
    291.2 |
    306.2 |
    321.2 |
    336.2 |
    351.2 |#
    366.3 |
    381.3 |
    396.3 |
    411.3 |#
  (27 below, 17 above range)

```

## Diagnostics

- **load_aligned**: CV=64.2% (high variance, measurements may be unstable)
- **load_aligned**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **load_aligned**: bridge=15.9% of algo (FFI overhead may distort results)
- **load_byte_pack**: CV=63.2% (high variance, measurements may be unstable)
- **load_byte_pack**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **load_byte_pack**: bridge=16.1% of algo (FFI overhead may distort results)
- **load_unaligned**: CV=60.7% (high variance, measurements may be unstable)
- **load_unaligned**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **load_unaligned**: bridge=16.0% of algo (FFI overhead may distort results)
