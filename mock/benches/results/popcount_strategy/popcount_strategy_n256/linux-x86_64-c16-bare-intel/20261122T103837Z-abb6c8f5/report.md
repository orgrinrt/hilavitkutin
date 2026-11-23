# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (371 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 33 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 106% faster than the next best (popcount_swar)

popcount_std (33 ns) leads popcount_swar (67 ns) by 106%, a clear separation rather than a photo finish. CV 95.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 91% (significant)

popcount_std is -338 ns (91%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 11.4x slower than the field

popcount_handrolled (371 ns) is 11.4x the fastest (33 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 95.1%)

popcount_std wins on median (33 ns) yet has the highest variance (CV 95.1%), while popcount_handrolled is the steadiest (CV 73.6%, 371 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 11.4x the fastest

Fastest popcount_std (33 ns) to slowest popcount_handrolled (371 ns): 11.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 4.0x its best-20%

popcount_std's best 20% of batches run at 27 ns but its worst 20% at 106 ns (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 32.6 ns median (-91.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 11.38x (fastest 32.6 ns, slowest 371.0 ns)

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
| popcount_handrolled | 2958ns | 2252ns | 2218ns | 2252ns | 5817ns | base |
| popcount_std | 2736ns | 1909ns | 1573ns | 1980ns | 6168ns | -7.51% |
| popcount_swar | 3041ns | 1953ns | 1646ns | 2562ns | 5874ns | +2.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 473ns | 336ns | 927ns | base | 0.541 |
| popcount_std | 47ns | 27ns | 106ns | -90.10% | 5.460 |
| popcount_swar | 105ns | 57ns | 201ns | -77.92% | 2.449 |

## Performance model

- Peak throughput: **9.543 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.690 | 7.2% |
| popcount_std | 7.853 | 82.3% |
| popcount_swar | 3.810 | 39.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2958ns | 2958ns | base |
| popcount_std | 2736ns | 2736ns | -7.51% |
| popcount_swar | 3041ns | 3041ns | +2.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 371ns | base | --- | [370, 372] | --- | --- | --- | --- |
| popcount_std | 33ns | -338.0ns (-91.1%) | [-344, -322]ns | [33, 33] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 67ns | -303.5ns (-81.8%) | [-304, -291]ns | [67, 67] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 334ns | -90.2% | -59.6% |
| 1 | 2 | 385ns | -91.5% | -82.5% |
| 1 | 3 | 346ns | -90.6% | -84.1% |
| 1 | 4 | 354ns | -90.8% | -56.1% |
| 1 | 5 | 371ns | -91.2% | -81.9% |
| 1 | 6 | 371ns | -92.8% | -81.9% |
| 1 | 7 | 387ns | -91.5% | -85.8% |
| 1 | 8 | 337ns | -92.1% | -80.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.394 | moderate- |
| popcount_std | -0.227 | moderate- |
| popcount_swar | -0.228 | moderate- |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 33.0ns | 473.4ns | 7.0% | HIGH |
| popcount_std | 34.1ns | 46.9ns | 72.8% | HIGH |
| popcount_swar | 38.3ns | 104.5ns | 36.7% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 335.7-926.7 ns)
    335.7 |#############################
    365.3 |########################################
    394.8 |#
    424.4 |
    453.9 |
    483.5 |
    513.0 |
    542.6 |
    572.1 |
    601.7 |
    631.2 |
    660.8 |
    690.3 |
    719.9 |
    749.4 |
    778.9 |
    808.5 |
    838.0 |
    867.6 |
    897.1 |
  (15 below, 20 above range)

popcount_std (n=160, range 26.8-106.1 ns)
     26.8 |
     30.8 |########################################
     34.8 |#
     38.7 |
     42.7 |
     46.7 |
     50.6 |
     54.6 |
     58.6 |
     62.5 |
     66.5 |
     70.5 |
     74.4 |
     78.4 |
     82.4 |
     86.3 |#
     90.3 |
     94.2 |##
     98.2 |
    102.2 |
  (31 below, 17 above range)

popcount_swar (n=160, range 56.6-201.4 ns)
     56.6 |
     63.9 |########################################
     71.1 |
     78.4 |
     85.6 |#
     92.8 |#
    100.1 |
    107.3 |#
    114.6 |##
    121.8 |
    129.0 |####
    136.3 |##
    143.5 |
    150.8 |#
    158.0 |###
    165.2 |#
    172.5 |##
    179.7 |##
    187.0 |
    194.2 |#
  (27 below, 14 above range)

```

## Diagnostics

- **popcount_handrolled**: CV=57.7% (high variance, measurements may be unstable)
- **popcount_std**: CV=66.2% (high variance, measurements may be unstable)
- **popcount_std**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **popcount_std**: bridge=73.0% of algo (FFI overhead may distort results)
- **popcount_swar**: CV=54.9% (high variance, measurements may be unstable)
- **popcount_swar**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **popcount_swar**: bridge=36.8% of algo (FFI overhead may distort results)
