# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (91 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 15 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 41% faster than the next best (popcount_swar)

popcount_std (15 ns) leads popcount_swar (22 ns) by 41%, a clear separation rather than a photo finish. CV 90.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 86% (significant)

popcount_std is -78 ns (86%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 5.9x slower than the field

popcount_handrolled (91 ns) is 5.9x the fastest (15 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_handrolled shows warm-up / thermal drift (autocorr +0.61)

popcount_handrolled's per-pass series has lag-1 autocorrelation +0.61, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.9x the fastest

Fastest popcount_std (15 ns) to slowest popcount_handrolled (91 ns): 5.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_swar is inconsistent: worst-20% is 4.3x its best-20%

popcount_swar's best 20% of batches run at 18 ns but its worst 20% at 75 ns (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 15.3 ns median (-83.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.94x (fastest 15.3 ns, slowest 91.0 ns)

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
| Counter frequency | 2112756608 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| popcount_handrolled | 2715ns | 1975ns | 1611ns | 2016ns | 5915ns | base |
| popcount_std | 2629ns | 1898ns | 1553ns | 1856ns | 6023ns | -3.16% |
| popcount_swar | 2636ns | 1906ns | 1559ns | 1909ns | 5892ns | -2.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 124ns | 71ns | 271ns | base | 0.516 |
| popcount_std | 21ns | 13ns | 48ns | -82.97% | 3.029 |
| popcount_swar | 34ns | 18ns | 75ns | -72.91% | 1.904 |

## Performance model

- Peak throughput: **5.117 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.704 | 13.8% |
| popcount_std | 4.183 | 81.7% |
| popcount_swar | 2.963 | 57.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2715ns | 2715ns | base |
| popcount_std | 2629ns | 2629ns | -3.16% |
| popcount_swar | 2636ns | 2636ns | -2.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 91ns | base | --- | [88, 92] | --- | --- | --- | --- |
| popcount_std | 15ns | -78.2ns (-86.0%) | [-79, -72]ns | [15, 15] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 22ns | -69.8ns (-76.7%) | [-70, -69]ns | [22, 22] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 68ns | -77.6% | -68.1% |
| 1 | 2 | 72ns | -82.6% | -38.0% |
| 1 | 3 | 75ns | -83.4% | -76.5% |
| 1 | 4 | 72ns | -78.3% | -70.1% |
| 1 | 5 | 91ns | -86.0% | -76.3% |
| 1 | 6 | 98ns | -84.3% | -77.7% |
| 1 | 7 | 93ns | -86.3% | -76.8% |
| 1 | 8 | 88ns | -82.1% | -79.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | 0.611 | HIGH+ (drift/warm-up) |
| popcount_std | -0.575 | HIGH- (thermal bounce) |
| popcount_swar | -0.238 | moderate- |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 33.1ns | 124.1ns | 26.7% | HIGH |
| popcount_std | 32.6ns | 21.1ns | 154.4% | HIGH |
| popcount_swar | 34.6ns | 33.6ns | 102.9% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 70.5-271.1 ns)
     70.5 |########################################
     80.6 |#############
     90.6 |#####################################
    100.6 |
    110.6 |
    120.7 |
    130.7 |
    140.7 |
    150.8 |
    160.8 |
    170.8 |
    180.9 |
    190.9 |
    200.9 |
    210.9 |###
    221.0 |
    231.0 |##
    241.0 |###
    251.1 |###
    261.1 |######
  (14 below, 11 above range)

popcount_std (n=160, range 12.5-47.8 ns)
     12.5 |#####################
     14.3 |########################################
     16.0 |
     17.8 |
     19.6 |
     21.3 |#
     23.1 |
     24.9 |
     26.6 |
     28.4 |
     30.1 |
     31.9 |
     33.7 |
     35.4 |#
     37.2 |
     39.0 |
     40.7 |#
     42.5 |#
     44.3 |###
     46.0 |
  (27 below, 16 above range)

popcount_swar (n=160, range 17.7-75.2 ns)
     17.7 |############
     20.6 |########################################
     23.4 |
     26.3 |
     29.2 |
     32.1 |
     35.0 |
     37.8 |
     40.7 |
     43.6 |#########
     46.5 |
     49.3 |
     52.2 |
     55.1 |
     58.0 |#
     60.8 |
     63.7 |#
     66.6 |
     69.5 |#
     72.3 |
  (9 below, 15 above range)

```

## Diagnostics

- **popcount_handrolled**: CV=63.4% (high variance, measurements may be unstable)
- **popcount_handrolled**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **popcount_handrolled**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **popcount_handrolled**: bridge=26.6% of algo (FFI overhead may distort results)
- **popcount_std**: CV=65.3% (high variance, measurements may be unstable)
- **popcount_std**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **popcount_std**: bridge=153.6% of algo (FFI overhead may distort results)
- **popcount_swar**: CV=75.2% (high variance, measurements may be unstable)
- **popcount_swar**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **popcount_swar**: bridge=113.0% of algo (FFI overhead may distort results)
