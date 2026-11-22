# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (342 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 33 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 106% faster than the next best (popcount_swar)

popcount_std (33 ns) leads popcount_swar (67 ns) by 106%, a clear separation rather than a photo finish. CV 96.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 90% (significant)

popcount_std is -310 ns (90%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 10.5x slower than the field

popcount_handrolled (342 ns) is 10.5x the fastest (33 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 96.2%)

popcount_std wins on median (33 ns) yet has the highest variance (CV 96.2%), while popcount_handrolled is the steadiest (CV 79.2%, 342 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 10.5x the fastest

Fastest popcount_std (33 ns) to slowest popcount_handrolled (342 ns): 10.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 3.9x its best-20%

popcount_std's best 20% of batches run at 27 ns but its worst 20% at 105 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 32.6 ns median (-90.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 10.50x (fastest 32.6 ns, slowest 342.2 ns)

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
| popcount_handrolled | 2783ns | 2227ns | 1847ns | 2176ns | 5542ns | base |
| popcount_std | 2679ns | 1918ns | 1567ns | 1928ns | 6046ns | -3.74% |
| popcount_swar | 2689ns | 1950ns | 1595ns | 1970ns | 5936ns | -3.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 445ns | 304ns | 892ns | base | 0.575 |
| popcount_std | 46ns | 27ns | 105ns | -89.69% | 5.578 |
| popcount_swar | 92ns | 55ns | 202ns | -79.27% | 2.774 |

## Performance model

- Peak throughput: **9.618 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.748 | 7.8% |
| popcount_std | 7.853 | 81.6% |
| popcount_swar | 3.810 | 39.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2783ns | 2783ns | base |
| popcount_std | 2679ns | 2679ns | -3.74% |
| popcount_swar | 2689ns | 2689ns | -3.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 342ns | base | --- | [336, 343] | --- | --- | --- | --- |
| popcount_std | 33ns | -309.5ns (-90.4%) | [-310, -303]ns | [33, 33] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 67ns | -275.3ns (-80.5%) | [-281, -268]ns | [67, 67] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 335ns | -90.3% | -79.9% |
| 1 | 2 | 314ns | -91.5% | -78.5% |
| 1 | 3 | 343ns | -90.4% | -80.3% |
| 1 | 4 | 354ns | -90.8% | -84.3% |
| 1 | 5 | 304ns | -89.2% | -81.8% |
| 1 | 6 | 305ns | -91.2% | -81.9% |
| 1 | 7 | 386ns | -91.5% | -82.5% |
| 1 | 8 | 336ns | -92.1% | -83.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.211 | moderate- |
| popcount_std | -0.467 | moderate- |
| popcount_swar | 0.121 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 31.1ns | 445.0ns | 7.0% | HIGH |
| popcount_std | 33.3ns | 45.9ns | 72.5% | HIGH |
| popcount_swar | 33.8ns | 92.3ns | 36.6% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 304.3-891.5 ns)
    304.3 |#######################
    333.7 |########################################
    363.0 |#########
    392.4 |
    421.8 |
    451.1 |
    480.5 |#
    509.8 |
    539.2 |
    568.6 |
    597.9 |
    627.3 |
    656.6 |
    686.0 |
    715.4 |
    744.7 |
    774.1 |
    803.5 |
    832.8 |
    862.2 |
  (14 below, 21 above range)

popcount_std (n=160, range 26.6-104.6 ns)
     26.6 |############
     30.5 |########################################
     34.4 |
     38.3 |
     42.2 |
     46.1 |
     50.0 |
     53.9 |
     57.8 |
     61.7 |#
     65.6 |
     69.5 |
     73.4 |
     77.3 |
     81.2 |#
     85.1 |
     89.0 |##
     92.9 |
     96.8 |#
    100.7 |#
  (22 below, 14 above range)

popcount_swar (n=160, range 55.1-202.4 ns)
     55.1 |#######################
     62.5 |########################################
     69.9 |
     77.2 |
     84.6 |
     92.0 |
     99.3 |
    106.7 |
    114.0 |
    121.4 |#
    128.8 |
    136.1 |#
    143.5 |
    150.9 |###
    158.2 |#
    165.6 |
    173.0 |
    180.3 |###
    187.7 |
    195.1 |#
  (18 below, 17 above range)

```

## Diagnostics

- **popcount_handrolled**: CV=60.9% (high variance, measurements may be unstable)
- **popcount_std**: CV=68.3% (high variance, measurements may be unstable)
- **popcount_std**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **popcount_std**: bridge=73.3% of algo (FFI overhead may distort results)
- **popcount_swar**: CV=63.5% (high variance, measurements may be unstable)
- **popcount_swar**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **popcount_swar**: bridge=36.8% of algo (FFI overhead may distort results)
