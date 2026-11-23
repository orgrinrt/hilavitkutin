# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (layout_aos) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline layout_aos has the worst median (89 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest layout_soa at 89 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### layout_soa is fastest but the noisiest (CV 88.8%)

layout_soa wins on median (89 ns) yet has the highest variance (CV 88.8%), while layout_aos is the steadiest (CV 84.0%, 89 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (79 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader layout_soa vs stability leader layout_aos (+0% speed for 1.1x steadier)

layout_soa is fastest (89 ns, CV 88.8%); layout_aos gives up 0.5% median for 1.1x lower variance (CV 84.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### layout_soa is inconsistent: worst-20% is 3.7x its best-20%

layout_soa's best 20% of batches run at 75 ns but its worst 20% at 276 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### layout_soa's edge over baseline is significant but tiny (-0 ns, 0.34%)

layout_soa differs from baseline layout_aos by -0 ns (0.34%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: layout_soa** at 88.8 ns median (-0.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 88.8 ns, slowest 89.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | AoS vs SoA (column-store) layout under partial-field iteration |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eadbeef01234567 |
| Counter frequency | 2112756649 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2684ns | 1967ns | 1608ns | 1963ns | 5923ns | base |
| layout_soa | 2852ns | 1968ns | 1655ns | 2138ns | 6191ns | +6.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 121ns | 73ns | 265ns | base | 33.840 |
| layout_soa | 128ns | 75ns | 276ns | +5.71% | 32.011 |

## Performance model

- Peak throughput: **56.184 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 45.919 | 81.7% |
| layout_soa | 46.126 | 82.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2684ns | 2684ns | base |
| layout_soa | 2852ns | 2852ns | +6.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 89ns | base | --- | [89, 89] | --- | --- | --- | --- |
| layout_soa | 89ns | -0.3ns (-0.3%) | [-0, -0]ns | [89, 89] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 89ns | -0.5% |
| 1 | 2 | 73ns | +21.6% |
| 1 | 3 | 89ns | -0.4% |
| 1 | 4 | 89ns | -0.4% |
| 1 | 5 | 89ns | -0.4% |
| 1 | 6 | 73ns | -0.5% |
| 1 | 7 | 89ns | -0.6% |
| 1 | 8 | 73ns | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.479 | moderate- |
| layout_soa | -0.207 | moderate- |

**Consistency summary:**

- **layout_soa**: won 7/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 32.4ns | 121.0ns | 26.8% | HIGH |
| layout_soa | 34.4ns | 128.0ns | 26.9% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 72.9-265.4 ns)
     72.9 |#########
     82.5 |########################################
     92.1 |
    101.8 |
    111.4 |
    121.0 |
    130.6 |
    140.3 |
    149.9 |
    159.5 |
    169.1 |
    178.7 |
    188.4 |
    198.0 |
    207.6 |
    217.2 |
    226.9 |#
    236.5 |
    246.1 |###
    255.7 |
  (23 below, 16 above range)

layout_soa (n=160, range 74.7-276.0 ns)
     74.7 |
     84.7 |########################################
     94.8 |
    104.9 |
    114.9 |
    125.0 |
    135.1 |
    145.1 |
    155.2 |
    165.3 |
    175.3 |
    185.4 |
    195.5 |
    205.5 |#
    215.6 |
    225.7 |#
    235.8 |
    245.8 |#
    255.9 |
    266.0 |#
  (28 below, 17 above range)

```

## Diagnostics

- **layout_aos**: CV=61.9% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **layout_aos**: bridge=26.7% of algo (FFI overhead may distort results)
- **layout_soa**: CV=61.6% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **layout_soa**: bridge=26.8% of algo (FFI overhead may distort results)
