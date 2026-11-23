# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bc_index) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bc_index has the worst median (152 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bc_iter at 152 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (119 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bc_unchecked is inconsistent: worst-20% is 3.6x its best-20%

bc_unchecked's best 20% of batches run at 126 ns but its worst 20% at 452 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.2% of the fastest

All 3 variants sit between 152 ns and 152 ns - a 0.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bc_iter's edge over baseline is significant but tiny (-0 ns, 0.13%)

bc_iter differs from baseline bc_index by -0 ns (0.13%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bc_iter** at 152.2 ns median (-0.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 152.2 ns, slowest 152.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Bounds-check elision: iter() vs explicit index vs get_unchecked |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5a3ec07491bf2d68 |
| Counter frequency | 2112744983 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 2817ns | 2030ns | 1660ns | 2156ns | 5957ns | base |
| bc_iter | 2658ns | 2028ns | 1659ns | 1988ns | 5666ns | -5.64% |
| bc_unchecked | 2792ns | 2032ns | 1686ns | 2062ns | 6087ns | -0.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 211ns | 125ns | 443ns | base | 4.861 |
| bc_iter | 199ns | 125ns | 422ns | -5.67% | 5.153 |
| bc_unchecked | 209ns | 126ns | 452ns | -0.94% | 4.907 |

## Performance model

- Peak throughput: **8.216 Gops/s** (bc_iter; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 6.715 | 81.7% |
| bc_iter | 6.728 | 81.9% |
| bc_unchecked | 6.724 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2817ns | 2817ns | base |
| bc_iter | 2658ns | 2658ns | -5.64% |
| bc_unchecked | 2792ns | 2792ns | -0.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 152ns | base | --- | [152, 152] | --- | --- | --- | --- |
| bc_iter | 152ns | -0.2ns (-0.1%) | [-0, -0]ns | [136, 152] | YES | 0.0045 | 0.0022 | 12 |
| bc_unchecked | 152ns | no significant difference | [-0, +0]ns | [152, 152] | no | 0.5728 | 0.5728 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 152ns | -18.2% | -18.2% |
| 1 | 2 | 152ns | -18.2% | +0.0% |
| 1 | 3 | 152ns | +0.0% | -18.2% |
| 1 | 4 | 125ns | +0.0% | +22.2% |
| 1 | 5 | 125ns | +0.4% | +22.1% |
| 1 | 6 | 125ns | +0.0% | +22.2% |
| 1 | 7 | 152ns | -0.1% | -0.1% |
| 1 | 8 | 153ns | -0.1% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | 0.393 | moderate+ |
| bc_iter | 0.050 | ok |
| bc_unchecked | -0.214 | moderate- |

**Consistency summary:**

- **bc_iter**: won 3/8, lost 1/8
- **bc_unchecked**: won 3/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 33.1ns | 210.6ns | 15.7% | HIGH |
| bc_iter | 31.2ns | 198.7ns | 15.7% | HIGH |
| bc_unchecked | 33.2ns | 208.7ns | 15.9% | HIGH |

## Distribution (algo ns)

```
bc_index (n=160, range 124.6-443.3 ns)
    124.6 |############
    140.6 |########################################
    156.5 |#
    172.4 |
    188.4 |
    204.3 |
    220.2 |
    236.2 |
    252.1 |
    268.0 |
    284.0 |
    299.9 |
    315.8 |
    331.8 |#
    347.7 |
    363.6 |##
    379.6 |##
    395.5 |#
    411.4 |##
    427.4 |
  (17 below, 15 above range)

bc_iter (n=160, range 124.6-421.5 ns)
    124.6 |########################################
    139.5 |######################################
    154.3 |##
    169.2 |
    184.0 |
    198.9 |
    213.7 |
    228.5 |
    243.4 |
    258.2 |
    273.1 |
    287.9 |
    302.8 |
    317.6 |##
    332.4 |##
    347.3 |
    362.1 |###
    377.0 |#
    391.8 |##
    406.7 |#
  (15 below, 14 above range)

bc_unchecked (n=160, range 126.4-452.1 ns)
    126.4 |
    142.7 |########################################
    158.9 |
    175.2 |
    191.5 |
    207.8 |
    224.1 |#
    240.4 |
    256.7 |
    272.9 |
    289.2 |
    305.5 |
    321.8 |
    338.1 |
    354.4 |
    370.6 |
    386.9 |
    403.2 |
    419.5 |#
    435.8 |#
  (30 below, 19 above range)

```

## Diagnostics

- **bc_index**: CV=59.4% (high variance, measurements may be unstable)
- **bc_index**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **bc_index**: bridge=15.7% of algo (FFI overhead may distort results)
- **bc_iter**: CV=59.9% (high variance, measurements may be unstable)
- **bc_iter**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **bc_iter**: bridge=15.6% of algo (FFI overhead may distort results)
- **bc_unchecked**: CV=61.4% (high variance, measurements may be unstable)
- **bc_unchecked**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **bc_unchecked**: bridge=15.9% of algo (FFI overhead may distort results)
