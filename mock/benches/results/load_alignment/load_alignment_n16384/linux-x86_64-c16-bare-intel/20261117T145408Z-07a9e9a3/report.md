# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5 ns) is smaller than the fastest variant's own run-to-run std-dev (454 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### load_aligned is inconsistent: worst-20% is 2.2x its best-20%

load_aligned's best 20% of batches run at 1.87 us but its worst 20% at 4.13 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.3% of the fastest

All 3 variants sit between 1.87 us and 1.88 us - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### load_byte_pack's edge over baseline is significant but tiny (-0 ns, 0.02%)

load_byte_pack differs from baseline load_aligned by -0 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: load_byte_pack** at 1871.7 ns median (-0.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 1871.7 ns, slowest 1876.4 ns)

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
| Counter frequency | 2112743394 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 4315ns | 3422ns | 3412ns | 3540ns | 7543ns | base |
| load_byte_pack | 3575ns | 3416ns | 3411ns | 3418ns | 4210ns | -17.15% |
| load_unaligned | 3977ns | 3425ns | 3412ns | 3625ns | 5596ns | -7.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 2364ns | 1871ns | 4130ns | base | 6.931 |
| load_byte_pack | 1957ns | 1871ns | 2297ns | -17.23% | 8.374 |
| load_unaligned | 2177ns | 1871ns | 3056ns | -7.92% | 7.527 |

## Performance model

- Peak throughput: **8.755 Gops/s** (load_byte_pack; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 8.753 | 100.0% |
| load_byte_pack | 8.754 | 100.0% |
| load_unaligned | 8.732 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 4315ns | 4315ns | base |
| load_byte_pack | 3575ns | 3575ns | -17.15% |
| load_unaligned | 3977ns | 3977ns | -7.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 1872ns | base | --- | [1872, 1872] | --- | --- | --- | --- |
| load_byte_pack | 1872ns | -0.3ns (-0.0%) | [-0, -0]ns | [1872, 1872] | YES | 0.0000 | 0.0000 | 15 |
| load_unaligned | 1876ns | no significant difference | [-0, +0]ns | [1872, 1884] | no | 0.4684 | 0.4684 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 1872ns | -0.0% | -0.0% |
| 1 | 2 | 1872ns | +0.0% | +28.5% |
| 1 | 3 | 1872ns | +0.0% | +22.2% |
| 1 | 4 | 1872ns | -0.0% | -0.0% |
| 1 | 5 | 1872ns | -0.0% | -0.0% |
| 1 | 6 | 1872ns | +0.0% | +0.0% |
| 1 | 7 | 2288ns | -18.2% | -18.2% |
| 1 | 8 | 2288ns | -18.2% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.458 | moderate+ |
| load_byte_pack | -0.286 | moderate- |
| load_unaligned | 0.077 | ok |

**Consistency summary:**

- **load_byte_pack**: won 2/8, lost 0/8
- **load_unaligned**: won 1/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 25.3ns | 2363.8ns | 1.1% |  |
| load_byte_pack | 20.4ns | 1956.6ns | 1.0% |  |
| load_unaligned | 22.7ns | 2176.7ns | 1.0% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 1871.5-4130.1 ns)
   1871.5 |########################################
   1984.4 |
   2097.4 |
   2210.3 |###############
   2323.2 |
   2436.2 |
   2549.1 |
   2662.0 |
   2775.0 |
   2887.9 |
   3000.8 |
   3113.7 |
   3226.7 |
   3339.6 |
   3452.5 |
   3565.5 |
   3678.4 |
   3791.3 |
   3904.3 |
   4017.2 |
  (9 below, 4 above range)

load_byte_pack (n=160, range 1871.4-2296.7 ns)
   1871.4 |########################################
   1892.6 |
   1913.9 |
   1935.2 |
   1956.4 |
   1977.7 |
   1999.0 |
   2020.2 |
   2041.5 |
   2062.8 |
   2084.0 |
   2105.3 |
   2126.6 |
   2147.8 |
   2169.1 |
   2190.4 |
   2211.6 |
   2232.9 |
   2254.2 |
   2275.4 |
  (12 below, 5 above range)

load_unaligned (n=160, range 1871.5-3055.6 ns)
   1871.5 |########################################
   1930.7 |
   1989.9 |
   2049.1 |
   2108.3 |
   2167.5 |
   2226.7 |
   2285.9 |#################
   2345.1 |
   2404.3 |
   2463.5 |
   2522.7 |
   2581.9 |
   2641.1 |
   2700.4 |
   2759.6 |
   2818.8 |
   2878.0 |
   2937.2 |
   2996.4 |
  (15 below, 11 above range)

```

## Diagnostics

- **load_aligned**: CV=104.2% (high variance, measurements may be unstable)
- **load_byte_pack**: CV=23.2% (high variance, measurements may be unstable)
- **load_unaligned**: CV=29.8% (high variance, measurements may be unstable)
