# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (layout_aos) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline layout_aos has the worst median (90 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest layout_soa at 89 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (78 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### layout_aos is inconsistent: worst-20% is 3.7x its best-20%

layout_aos's best 20% of batches run at 73 ns but its worst 20% at 271 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### layout_soa's edge over baseline is significant but tiny (-1 ns, 0.73%)

layout_soa differs from baseline layout_aos by -1 ns (0.73%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: layout_soa** at 88.8 ns median (-0.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 88.8 ns, slowest 89.5 ns)

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
| Counter frequency | 2112773604 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2715ns | 1975ns | 1618ns | 1973ns | 6036ns | base |
| layout_soa | 2769ns | 1976ns | 1804ns | 2071ns | 5827ns | +2.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 122ns | 73ns | 271ns | base | 33.440 |
| layout_soa | 125ns | 81ns | 262ns | +1.81% | 32.844 |

## Performance model

- Peak throughput: **55.952 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 45.765 | 81.8% |
| layout_soa | 46.126 | 82.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2715ns | 2715ns | base |
| layout_soa | 2769ns | 2769ns | +2.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 90ns | base | --- | [89, 90] | --- | --- | --- | --- |
| layout_soa | 89ns | -0.6ns (-0.7%) | [-1, -1]ns | [89, 89] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 90ns | -0.9% |
| 1 | 2 | 90ns | -0.8% |
| 1 | 3 | 73ns | +21.2% |
| 1 | 4 | 73ns | +102.0% |
| 1 | 5 | 90ns | -0.8% |
| 1 | 6 | 89ns | -0.7% |
| 1 | 7 | 90ns | -0.8% |
| 1 | 8 | 90ns | -18.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.291 | moderate+ |
| layout_soa | -0.098 | ok |

**Consistency summary:**

- **layout_soa**: won 6/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 32.6ns | 122.5ns | 26.6% | HIGH |
| layout_soa | 33.3ns | 124.7ns | 26.7% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 73.2-271.4 ns)
     73.2 |#####
     83.1 |########################################
     93.0 |
    102.9 |
    112.8 |
    122.8 |
    132.7 |
    142.6 |
    152.5 |
    162.4 |
    172.3 |
    182.2 |
    192.1 |
    202.0 |
    211.9 |
    221.8 |
    231.8 |
    241.7 |
    251.6 |
    261.5 |#
  (20 below, 18 above range)

layout_soa (n=160, range 81.2-262.2 ns)
     81.2 |########################################
     90.3 |
     99.3 |
    108.4 |
    117.4 |
    126.4 |
    135.5 |
    144.5 |####
    153.6 |
    162.6 |
    171.7 |
    180.7 |
    189.8 |
    198.8 |
    207.9 |
    216.9 |
    226.0 |
    235.0 |
    244.1 |
    253.1 |
  (15 below, 19 above range)

```

## Diagnostics

- **layout_aos**: CV=64.5% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **layout_aos**: bridge=26.7% of algo (FFI overhead may distort results)
- **layout_soa**: CV=62.5% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **layout_soa**: bridge=26.9% of algo (FFI overhead may distort results)
