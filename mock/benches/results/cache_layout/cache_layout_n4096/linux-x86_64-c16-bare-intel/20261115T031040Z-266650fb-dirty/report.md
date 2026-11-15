# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (layout_aos) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline layout_aos has the worst median (90 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest layout_soa at 89 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (77 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### layout_soa is inconsistent: worst-20% is 3.7x its best-20%

layout_soa's best 20% of batches run at 74 ns but its worst 20% at 273 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### layout_soa's edge over baseline is significant but tiny (-1 ns, 0.89%)

layout_soa differs from baseline layout_aos by -1 ns (0.89%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: layout_soa** at 88.8 ns median (-0.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 88.8 ns, slowest 89.6 ns)

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
| Counter frequency | 2112817009 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2848ns | 1968ns | 1796ns | 2068ns | 6241ns | base |
| layout_soa | 2789ns | 1966ns | 1642ns | 2065ns | 6108ns | -2.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 129ns | 82ns | 280ns | base | 31.771 |
| layout_soa | 125ns | 74ns | 273ns | -3.29% | 32.853 |

## Performance model

- Peak throughput: **55.012 Gops/s** (layout_soa; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 45.714 | 83.1% |
| layout_soa | 46.126 | 83.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2848ns | 2848ns | base |
| layout_soa | 2789ns | 2789ns | -2.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 90ns | base | --- | [90, 90] | --- | --- | --- | --- |
| layout_soa | 89ns | -0.8ns (-0.9%) | [-1, -1]ns | [89, 89] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 90ns | -0.9% |
| 1 | 2 | 90ns | -18.8% |
| 1 | 3 | 90ns | -0.7% |
| 1 | 4 | 73ns | -0.8% |
| 1 | 5 | 90ns | -0.9% |
| 1 | 6 | 90ns | -0.9% |
| 1 | 7 | 90ns | -0.6% |
| 1 | 8 | 89ns | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.161 | ok |
| layout_soa | -0.372 | moderate- |

**Consistency summary:**

- **layout_soa**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 34.6ns | 128.9ns | 26.9% | HIGH |
| layout_soa | 33.3ns | 124.7ns | 26.7% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 81.7-280.4 ns)
     81.7 |########################################
     91.6 |
    101.6 |
    111.5 |
    121.4 |
    131.4 |
    141.3 |
    151.2 |
    161.2 |
    171.1 |
    181.0 |
    191.0 |
    200.9 |
    210.8 |
    220.8 |
    230.7 |
    240.6 |#
    250.6 |#
    260.5 |##
    270.4 |
  (15 below, 15 above range)

layout_soa (n=160, range 74.5-272.6 ns)
     74.5 |
     84.4 |########################################
     94.3 |#
    104.2 |
    114.1 |
    124.0 |
    133.9 |
    143.8 |
    153.7 |
    163.6 |
    173.5 |
    183.4 |
    193.4 |
    203.3 |
    213.2 |
    223.1 |
    233.0 |
    242.9 |#
    252.8 |
    262.7 |##
  (28 below, 16 above range)

```

## Diagnostics

- **layout_aos**: CV=61.1% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **layout_aos**: bridge=26.6% of algo (FFI overhead may distort results)
- **layout_soa**: CV=61.4% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **layout_soa**: bridge=26.8% of algo (FFI overhead may distort results)
