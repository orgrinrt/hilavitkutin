# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### inline_always is fastest but the noisiest (CV 48.6%)

inline_always wins on median (1.15 us) yet has the highest variance (CV 48.6%), while inline_default is the steadiest (CV 44.0%, 1.15 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (558 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (1.15 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### inline_default's comparison is tie-heavy (11% tied pairs)

11% of paired samples for inline_default are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader inline_always vs stability leader inline_default (+0% speed for 1.1x steadier)

inline_always is fastest (1.15 us, CV 48.6%); inline_default gives up 0.0% median for 1.1x lower variance (CV 44.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### inline_default is inconsistent: worst-20% is 2.3x its best-20%

inline_default's best 20% of batches run at 939 ns but its worst 20% at 2.17 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.3% of the fastest

All 3 variants sit between 1.15 us and 1.15 us - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### inline_never's edge over baseline is significant but tiny (3 ns, 0.23%)

inline_never differs from baseline inline_always by 3 ns (0.23%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (inline_always) is the fastest** at 1147.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 1147.7 ns, slowest 1150.7 ns)

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
| Counter frequency | 2112795995 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 3456ns | 3028ns | 3021ns | 3029ns | 5175ns | base |
| inline_default | 3431ns | 3027ns | 2476ns | 3008ns | 5657ns | -0.73% |
| inline_never | 3278ns | 3033ns | 2482ns | 3016ns | 4859ns | -5.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 1309ns | 1148ns | 1952ns | base | 0.782 |
| inline_default | 1305ns | 939ns | 2166ns | -0.25% | 0.784 |
| inline_never | 1241ns | 942ns | 1831ns | -5.15% | 0.825 |

## Performance model

- Peak throughput: **1.091 Gops/s** (inline_default; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.892 | 81.8% |
| inline_default | 0.892 | 81.8% |
| inline_never | 0.890 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 3456ns | 3456ns | base |
| inline_default | 3431ns | 3431ns | -0.73% |
| inline_never | 3278ns | 3278ns | -5.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 1148ns | base | --- | [1148, 1148] | --- | --- | --- | --- |
| inline_default | 1148ns | no significant difference | [-0, +0]ns | [1148, 1148] | no | 0.1807 | 0.1807 | **17** (11%, HIGH) |
| inline_never | 1151ns | +2.7ns (+0.2%) | [+2, +3]ns | [1150, 1151] | YES | 0.0066 | 0.0033 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 1148ns | +0.0% | -17.9% |
| 1 | 2 | 1148ns | +0.0% | +0.3% |
| 1 | 3 | 1148ns | +0.0% | -17.9% |
| 1 | 4 | 1148ns | +64.1% | +0.3% |
| 1 | 5 | 1148ns | -18.2% | +0.3% |
| 1 | 6 | 1148ns | +0.0% | +0.3% |
| 1 | 7 | 1148ns | -18.2% | +0.3% |
| 1 | 8 | 1148ns | +0.0% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | 0.342 | moderate+ |
| inline_default | -0.273 | moderate- |
| inline_never | -0.209 | moderate- |

**Consistency summary:**

- **inline_default**: won 2/8, lost 1/8
- **inline_never**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 27.1ns | 1308.7ns | 2.1% |  |
| inline_default | 26.8ns | 1305.5ns | 2.1% |  |
| inline_never | 25.9ns | 1241.4ns | 2.1% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 1147.5-1951.7 ns)
   1147.5 |########################################
   1187.7 |
   1227.9 |
   1268.1 |
   1308.3 |
   1348.6 |
   1388.8 |
   1429.0 |
   1469.2 |
   1509.4 |
   1549.6 |
   1589.8 |
   1630.0 |
   1670.2 |
   1710.4 |
   1750.6 |
   1790.9 |
   1831.1 |
   1871.3 |
   1911.5 |
  (24 below, 12 above range)

inline_default (n=160, range 939.0-2166.3 ns)
    939.0 |###########
   1000.3 |
   1061.7 |
   1123.1 |########################################
   1184.4 |
   1245.8 |
   1307.2 |
   1368.6 |
   1429.9 |
   1491.3 |
   1552.7 |
   1614.0 |
   1675.4 |
   1736.8 |
   1798.1 |
   1859.5 |########
   1920.9 |
   1982.2 |
   2043.6 |
   2105.0 |
  (10 below, 9 above range)

inline_never (n=160, range 942.2-1831.0 ns)
    942.2 |##
    986.7 |
   1031.1 |
   1075.5 |
   1120.0 |########################################
   1164.4 |
   1208.9 |
   1253.3 |
   1297.7 |
   1342.2 |
   1386.6 |
   1431.1 |
   1475.5 |
   1519.9 |
   1564.4 |
   1608.8 |
   1653.3 |
   1697.7 |
   1742.1 |
   1786.6 |
  (27 below, 11 above range)

```

## Diagnostics

- **inline_always**: CV=42.6% (high variance, measurements may be unstable)
- **inline_default**: CV=38.7% (high variance, measurements may be unstable)
- **inline_never**: CV=41.9% (high variance, measurements may be unstable)
