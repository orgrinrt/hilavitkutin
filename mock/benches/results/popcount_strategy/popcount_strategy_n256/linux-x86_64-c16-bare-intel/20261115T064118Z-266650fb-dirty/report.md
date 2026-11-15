# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (354 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 33 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 105% faster than the next best (popcount_swar)

popcount_std (33 ns) leads popcount_swar (67 ns) by 105%, a clear separation rather than a photo finish. CV 92.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 91% (significant)

popcount_std is -321 ns (91%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 10.8x slower than the field

popcount_handrolled (354 ns) is 10.8x the fastest (33 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 92.4%)

popcount_std wins on median (33 ns) yet has the highest variance (CV 92.4%), while popcount_handrolled is the steadiest (CV 79.7%, 354 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 10.8x the fastest

Fastest popcount_std (33 ns) to slowest popcount_handrolled (354 ns): 10.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_swar is inconsistent: worst-20% is 3.6x its best-20%

popcount_swar's best 20% of batches run at 55 ns but its worst 20% at 199 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 32.7 ns median (-90.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 10.83x (fastest 32.7 ns, slowest 354.1 ns)

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
| Counter frequency | 2112588197 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| popcount_handrolled | 2911ns | 2244ns | 2031ns | 2241ns | 5799ns | base |
| popcount_std | 2834ns | 1911ns | 1743ns | 2054ns | 6264ns | -2.64% |
| popcount_swar | 2809ns | 1944ns | 1587ns | 2205ns | 5845ns | -3.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 466ns | 322ns | 932ns | base | 0.550 |
| popcount_std | 48ns | 30ns | 106ns | -89.61% | 5.293 |
| popcount_swar | 96ns | 55ns | 199ns | -79.34% | 2.661 |

## Performance model

- Peak throughput: **8.575 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.723 | 8.4% |
| popcount_std | 7.829 | 91.3% |
| popcount_swar | 3.821 | 44.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2911ns | 2911ns | base |
| popcount_std | 2834ns | 2834ns | -2.64% |
| popcount_swar | 2809ns | 2809ns | -3.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 354ns | base | --- | [353, 370] | --- | --- | --- | --- |
| popcount_std | 33ns | -320.9ns (-90.6%) | [-335, -309]ns | [33, 33] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 67ns | -286.1ns (-80.8%) | [-287, -281]ns | [67, 67] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 333ns | -90.1% | -83.5% |
| 1 | 2 | 311ns | -91.4% | -82.4% |
| 1 | 3 | 342ns | -90.4% | -80.4% |
| 1 | 4 | 354ns | -90.8% | -81.0% |
| 1 | 5 | 370ns | -91.2% | -85.2% |
| 1 | 6 | 371ns | -91.2% | -81.9% |
| 1 | 7 | 386ns | -91.5% | -82.6% |
| 1 | 8 | 336ns | -90.2% | -83.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | 0.410 | moderate+ |
| popcount_std | -0.166 | ok |
| popcount_swar | -0.127 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 32.4ns | 465.6ns | 7.0% | HIGH |
| popcount_std | 35.5ns | 48.4ns | 73.3% | HIGH |
| popcount_swar | 35.7ns | 96.2ns | 37.1% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 321.7-932.0 ns)
    321.7 |#######################################
    352.2 |########################################
    382.7 |#############
    413.2 |
    443.7 |
    474.3 |#
    504.8 |
    535.3 |
    565.8 |
    596.3 |
    626.8 |
    657.3 |
    687.9 |
    718.4 |
    748.9 |
    779.4 |
    809.9 |
    840.4 |
    870.9 |
    901.5 |
  (16 below, 20 above range)

popcount_std (n=160, range 29.9-106.1 ns)
     29.9 |########################################
     33.7 |
     37.5 |
     41.3 |
     45.1 |
     48.9 |
     52.7 |
     56.5 |
     60.4 |
     64.2 |
     68.0 |
     71.8 |
     75.6 |
     79.4 |
     83.2 |
     87.0 |
     90.9 |#
     94.7 |#
     98.5 |
    102.3 |#
  (15 below, 18 above range)

popcount_swar (n=160, range 54.6-198.7 ns)
     54.6 |########################
     61.8 |########################################
     69.0 |
     76.2 |
     83.4 |
     90.6 |
     97.8 |
    105.1 |
    112.3 |#
    119.5 |
    126.7 |
    133.9 |
    141.1 |
    148.3 |#
    155.5 |####
    162.7 |#
    169.9 |####
    177.1 |##
    184.3 |###
    191.5 |##
  (17 below, 14 above range)

```

## Diagnostics

- **popcount_handrolled**: CV=60.6% (high variance, measurements may be unstable)
- **popcount_std**: CV=62.5% (high variance, measurements may be unstable)
- **popcount_std**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **popcount_std**: bridge=72.8% of algo (FFI overhead may distort results)
- **popcount_swar**: CV=60.2% (high variance, measurements may be unstable)
- **popcount_swar**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **popcount_swar**: bridge=36.7% of algo (FFI overhead may distort results)
