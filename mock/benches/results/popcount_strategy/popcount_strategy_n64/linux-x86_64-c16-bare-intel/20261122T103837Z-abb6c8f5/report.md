# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (92 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 15 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 42% faster than the next best (popcount_swar)

popcount_std (15 ns) leads popcount_swar (22 ns) by 42%, a clear separation rather than a photo finish. CV 83.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 85% (significant)

popcount_std is -78 ns (85%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 6.0x slower than the field

popcount_handrolled (92 ns) is 6.0x the fastest (15 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std shows warm-up / thermal drift (autocorr +0.60)

popcount_std's per-pass series has lag-1 autocorrelation +0.60, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.0x the fastest

Fastest popcount_std (15 ns) to slowest popcount_handrolled (92 ns): 6.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_swar is inconsistent: worst-20% is 3.8x its best-20%

popcount_swar's best 20% of batches run at 18 ns but its worst 20% at 67 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 15.3 ns median (-83.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.99x (fastest 15.3 ns, slowest 91.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Popcount lowering: std count_ones vs Kernighan loop vs SWAR |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x24680123456789ab |
| Counter frequency | 2112759641 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| popcount_handrolled | 2814ns | 1974ns | 1678ns | 2125ns | 6018ns | base |
| popcount_std | 2695ns | 1893ns | 1547ns | 2073ns | 5707ns | -4.25% |
| popcount_swar | 2671ns | 1900ns | 1553ns | 1964ns | 5909ns | -5.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 129ns | 75ns | 276ns | base | 0.497 |
| popcount_std | 22ns | 13ns | 45ns | -83.18% | 2.951 |
| popcount_swar | 30ns | 18ns | 67ns | -76.48% | 2.111 |

## Performance model

- Peak throughput: **5.093 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.698 | 13.7% |
| popcount_std | 4.183 | 82.1% |
| popcount_swar | 2.949 | 57.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2814ns | 2814ns | base |
| popcount_std | 2695ns | 2695ns | -4.25% |
| popcount_swar | 2671ns | 2671ns | -5.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 92ns | base | --- | [90, 93] | --- | --- | --- | --- |
| popcount_std | 15ns | -77.8ns (-84.9%) | [-78, -77]ns | [15, 15] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 22ns | -71.6ns (-78.1%) | [-73, -68]ns | [22, 22] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 84ns | -84.7% | -78.8% |
| 1 | 2 | 90ns | -85.9% | -75.5% |
| 1 | 3 | 92ns | -86.1% | -80.6% |
| 1 | 4 | 72ns | -78.9% | -75.6% |
| 1 | 5 | 74ns | -79.4% | -70.9% |
| 1 | 6 | 98ns | -84.3% | -77.6% |
| 1 | 7 | 93ns | -83.4% | -76.7% |
| 1 | 8 | 88ns | -82.1% | -75.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | 0.097 | ok |
| popcount_std | 0.600 | HIGH+ (drift/warm-up) |
| popcount_swar | 0.014 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 34.3ns | 128.9ns | 26.6% | HIGH |
| popcount_std | 33.3ns | 21.7ns | 153.4% | HIGH |
| popcount_swar | 34.1ns | 30.3ns | 112.6% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 75.3-275.9 ns)
     75.3 |#########
     85.4 |########################################
     95.4 |###########
    105.4 |
    115.4 |
    125.5 |
    135.5 |
    145.5 |
    155.6 |
    165.6 |
    175.6 |
    185.6 |#
    195.7 |##
    205.7 |#
    215.7 |
    225.8 |#
    235.8 |
    245.8 |##
    255.9 |#
    265.9 |#
  (26 below, 16 above range)

popcount_std (n=160, range 12.6-45.3 ns)
     12.6 |##########
     14.2 |########################################
     15.8 |
     17.5 |
     19.1 |
     20.8 |
     22.4 |
     24.0 |#
     25.7 |
     27.3 |
     28.9 |
     30.6 |#
     32.2 |#
     33.9 |#
     35.5 |#
     37.1 |
     38.8 |###
     40.4 |
     42.0 |#
     43.7 |#
  (20 below, 16 above range)

popcount_swar (n=160, range 17.6-66.7 ns)
     17.6 |##############
     20.1 |########################################
     22.5 |
     25.0 |
     27.4 |
     29.9 |#
     32.4 |
     34.8 |
     37.3 |
     39.7 |
     42.2 |
     44.6 |
     47.1 |
     49.6 |
     52.0 |##
     54.5 |#
     56.9 |
     59.4 |##
     61.8 |#
     64.3 |#
  (13 below, 15 above range)

```

## Diagnostics

- **popcount_handrolled**: CV=61.3% (high variance, measurements may be unstable)
- **popcount_handrolled**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **popcount_handrolled**: bridge=26.4% of algo (FFI overhead may distort results)
- **popcount_std**: CV=59.0% (high variance, measurements may be unstable)
- **popcount_std**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **popcount_std**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **popcount_std**: bridge=154.2% of algo (FFI overhead may distort results)
- **popcount_swar**: CV=63.4% (high variance, measurements may be unstable)
- **popcount_swar**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **popcount_swar**: bridge=112.0% of algo (FFI overhead may distort results)
