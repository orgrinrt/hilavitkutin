# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.52 us) is smaller than the fastest variant's own run-to-run std-dev (1.67 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader inline_default vs stability leader inline_never (+9% speed for 1.7x steadier)

inline_default is fastest (16.75 us, CV 9.9%); inline_never gives up 9.1% median for 1.7x lower variance (CV 5.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### inline_default's edge over baseline is significant but tiny (-4 ns, 0.02%)

inline_default differs from baseline inline_always by -4 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: inline_default** at 16748.8 ns median (-8.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.09x (fastest 16748.8 ns, slowest 18271.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x39f847b2c651de80 |
| Counter frequency | 2112764099 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 20072ns | 20150ns | 16482ns | 19847ns | 24334ns | base |
| inline_default | 18330ns | 18476ns | 16478ns | 18326ns | 20192ns | -8.68% |
| inline_never | 19724ns | 20160ns | 17929ns | 20160ns | 20210ns | -1.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 18188ns | 14942ns | 22030ns | base | 0.901 |
| inline_default | 16612ns | 14937ns | 18292ns | -8.67% | 0.986 |
| inline_never | 17871ns | 16247ns | 18298ns | -1.74% | 0.917 |

## Performance model

- Peak throughput: **1.097 Gops/s** (inline_default; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.897 | 81.8% |
| inline_default | 0.978 | 89.2% |
| inline_never | 0.897 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 20072ns | 20072ns | base |
| inline_default | 18330ns | 18330ns | -8.68% |
| inline_never | 19724ns | 19724ns | -1.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 18268ns | base | --- | [18267, 18268] | --- | --- | --- | --- |
| inline_default | 16749ns | -4.5ns (-0.0%) | [-22, -1]ns | [14949, 18267] | YES | 0.0003 | 0.0003 | 4 |
| inline_never | 18272ns | +3.6ns (+0.0%) | [+3, +5]ns | [18271, 18272] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 18268ns | -18.2% | +0.0% |
| 1 | 2 | 18268ns | -18.2% | +0.0% |
| 1 | 3 | 14947ns | -0.0% | +22.2% |
| 1 | 4 | 18268ns | +0.0% | +0.0% |
| 1 | 5 | 18272ns | -0.0% | -0.0% |
| 1 | 6 | 14946ns | +22.2% | +22.3% |
| 1 | 7 | 18268ns | +0.0% | +0.0% |
| 1 | 8 | 18268ns | -18.2% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.375 | moderate- |
| inline_default | 0.374 | moderate+ |
| inline_never | -0.018 | ok |

**Consistency summary:**

- **inline_default**: won 3/8, lost 1/8
- **inline_never**: won 1/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 23.7ns | 18188.0ns | 0.1% |  |
| inline_default | 21.7ns | 16611.7ns | 0.1% |  |
| inline_never | 23.6ns | 17871.4ns | 0.1% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 14941.7-22030.1 ns)
  14941.7 |########
  15296.1 |
  15650.5 |
  16004.9 |
  16359.3 |
  16713.8 |
  17068.2 |
  17422.6 |
  17777.0 |
  18131.4 |########################################
  18485.9 |
  18840.3 |
  19194.7 |
  19549.1 |
  19903.6 |
  20258.0 |
  20612.4 |
  20966.8 |
  21321.2 |
  21675.7 |
  (17 below, 1 above range)

inline_default (n=160, range 14936.7-18292.4 ns)
  14936.7 |######################################
  15104.5 |
  15272.3 |
  15440.0 |
  15607.8 |
  15775.6 |
  15943.4 |
  16111.2 |
  16279.0 |
  16446.8 |
  16614.5 |
  16782.3 |
  16950.1 |
  17117.9 |
  17285.7 |
  17453.5 |
  17621.3 |
  17789.0 |
  17956.8 |
  18124.6 |########################################
  (4 below, 3 above range)

inline_never (n=160, range 16247.0-18298.0 ns)
  16247.0 |
  16349.6 |
  16452.1 |
  16554.6 |
  16657.2 |
  16759.7 |
  16862.3 |
  16964.8 |
  17067.4 |
  17169.9 |
  17272.5 |
  17375.0 |
  17477.6 |
  17580.1 |
  17682.7 |
  17785.2 |
  17887.8 |
  17990.3 |
  18092.9 |
  18195.4 |########################################
  (19 below, 7 above range)

```

## Diagnostics

- **inline_always**: CV=51.8% (high variance, measurements may be unstable)
