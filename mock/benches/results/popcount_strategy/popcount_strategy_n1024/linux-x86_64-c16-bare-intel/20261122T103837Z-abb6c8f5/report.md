# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (1.77 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 93 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 104% faster than the next best (popcount_swar)

popcount_std (93 ns) leads popcount_swar (189 ns) by 104%, a clear separation rather than a photo finish. CV 87.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 89% (significant)

popcount_std is -1.57 us (89%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 19.1x slower than the field

popcount_handrolled (1.77 us) is 19.1x the fastest (93 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 87.3%)

popcount_std wins on median (93 ns) yet has the highest variance (CV 87.3%), while popcount_handrolled is the steadiest (CV 27.6%, 1.77 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### popcount_handrolled shows alternating (throttle bounce) (autocorr -0.56)

popcount_handrolled's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 19.1x the fastest

Fastest popcount_std (93 ns) to slowest popcount_handrolled (1.77 us): 19.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 3.7x its best-20%

popcount_std's best 20% of batches run at 77 ns but its worst 20% at 287 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 92.9 ns median (-94.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 19.07x (fastest 92.9 ns, slowest 1771.8 ns)

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
| popcount_handrolled | 3569ns | 3667ns | 2911ns | 3414ns | 4695ns | base |
| popcount_std | 2788ns | 1971ns | 1643ns | 2040ns | 6180ns | -21.88% |
| popcount_swar | 2782ns | 2069ns | 1696ns | 2071ns | 6004ns | -22.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 1740ns | 1360ns | 2327ns | base | 0.589 |
| popcount_std | 131ns | 77ns | 287ns | -92.49% | 7.839 |
| popcount_swar | 253ns | 155ns | 542ns | -85.45% | 4.045 |

## Performance model

- Peak throughput: **13.215 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.578 | 4.4% |
| popcount_std | 11.023 | 83.4% |
| popcount_swar | 5.407 | 40.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 3569ns | 3569ns | base |
| popcount_std | 2788ns | 2788ns | -21.88% |
| popcount_swar | 2782ns | 2782ns | -22.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 1772ns | base | --- | [1496, 1814] | --- | --- | --- | --- |
| popcount_std | 93ns | -1568.8ns (-88.5%) | [-1699, -1392]ns | [93, 93] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 189ns | -1348.4ns (-76.1%) | [-1588, -1313]ns | [189, 190] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 1893ns | -95.1% | -90.0% |
| 1 | 2 | 1485ns | -93.7% | -89.6% |
| 1 | 3 | 1331ns | -94.3% | -85.8% |
| 1 | 4 | 1782ns | -95.7% | -89.4% |
| 1 | 5 | 1446ns | -93.6% | -86.8% |
| 1 | 6 | 1947ns | -95.2% | -90.3% |
| 1 | 7 | 1409ns | -93.4% | -89.0% |
| 1 | 8 | 1842ns | -95.0% | -89.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.556 | HIGH- (thermal bounce) |
| popcount_std | 0.292 | moderate+ |
| popcount_swar | -0.369 | moderate- |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 25.3ns | 1739.5ns | 1.5% |  |
| popcount_std | 33.6ns | 130.6ns | 25.7% | HIGH |
| popcount_swar | 32.3ns | 253.1ns | 12.8% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 1359.8-2326.6 ns)
   1359.8 |################
   1408.1 |########################################
   1456.4 |#################################
   1504.8 |#####
   1553.1 |#
   1601.5 |
   1649.8 |
   1698.2 |
   1746.5 |#########################
   1794.8 |#########################
   1843.2 |##########################
   1891.5 |##############################
   1939.9 |################
   1988.2 |###
   2036.6 |#
   2084.9 |
   2133.3 |
   2181.6 |#
   2229.9 |
   2278.3 |
  (17 below, 7 above range)

popcount_std (n=160, range 77.5-287.5 ns)
     77.5 |
     88.0 |########################################
     98.5 |
    109.0 |
    119.5 |
    130.0 |
    140.5 |
    151.0 |
    161.5 |
    172.0 |
    182.5 |
    193.0 |
    203.5 |
    214.0 |
    224.5 |
    235.0 |
    245.5 |
    256.0 |#
    266.5 |#
    277.0 |##
  (29 below, 15 above range)

popcount_swar (n=160, range 155.1-542.2 ns)
    155.1 |#####
    174.5 |########################################
    193.8 |
    213.2 |
    232.5 |
    251.9 |
    271.2 |
    290.6 |
    309.9 |
    329.3 |
    348.7 |
    368.0 |
    387.4 |
    406.7 |
    426.1 |
    445.4 |
    464.8 |
    484.1 |
    503.5 |#
    522.9 |
  (18 below, 21 above range)

```

## Diagnostics

- **popcount_handrolled**: CV=28.1% (high variance, measurements may be unstable)
- **popcount_std**: CV=62.1% (high variance, measurements may be unstable)
- **popcount_std**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **popcount_std**: bridge=25.6% of algo (FFI overhead may distort results)
- **popcount_swar**: CV=62.3% (high variance, measurements may be unstable)
- **popcount_swar**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **popcount_swar**: bridge=12.7% of algo (FFI overhead may distort results)
