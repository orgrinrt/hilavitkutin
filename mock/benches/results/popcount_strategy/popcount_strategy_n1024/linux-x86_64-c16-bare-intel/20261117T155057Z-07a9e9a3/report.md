# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (1.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 93 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 102% faster than the next best (popcount_swar)

popcount_std (93 ns) leads popcount_swar (188 ns) by 102%, a clear separation rather than a photo finish. CV 84.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 93% (significant)

popcount_std is -1.40 us (93%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 16.3x slower than the field

popcount_handrolled (1.51 us) is 16.3x the fastest (93 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 84.3%)

popcount_std wins on median (93 ns) yet has the highest variance (CV 84.3%), while popcount_handrolled is the steadiest (CV 27.9%, 1.51 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 16.3x the fastest

Fastest popcount_std (93 ns) to slowest popcount_handrolled (1.51 us): 16.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 3.6x its best-20%

popcount_std's best 20% of batches run at 76 ns but its worst 20% at 275 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 93.0 ns median (-93.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 16.27x (fastest 93.0 ns, slowest 1513.0 ns)

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
| popcount_handrolled | 3408ns | 3068ns | 2906ns | 3277ns | 4305ns | base |
| popcount_std | 2693ns | 1979ns | 1616ns | 1978ns | 5919ns | -20.98% |
| popcount_swar | 2800ns | 2072ns | 1693ns | 2045ns | 6172ns | -17.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 1662ns | 1355ns | 2156ns | base | 0.616 |
| popcount_std | 126ns | 76ns | 275ns | -92.42% | 8.124 |
| popcount_swar | 253ns | 154ns | 554ns | -84.78% | 4.048 |

## Performance model

- Peak throughput: **13.472 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.677 | 5.0% |
| popcount_std | 11.011 | 81.7% |
| popcount_swar | 5.451 | 40.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 3408ns | 3408ns | base |
| popcount_std | 2693ns | 2693ns | -20.98% |
| popcount_swar | 2800ns | 2800ns | -17.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 1513ns | base | --- | [1480, 1544] | --- | --- | --- | --- |
| popcount_std | 93ns | -1400.8ns (-92.6%) | [-1447, -1370]ns | [93, 93] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 188ns | -1294.2ns (-85.5%) | [-1327, -1273]ns | [187, 188] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 1906ns | -95.1% | -90.1% |
| 1 | 2 | 1819ns | -95.8% | -89.7% |
| 1 | 3 | 1321ns | -94.2% | -88.3% |
| 1 | 4 | 1469ns | -93.7% | -87.2% |
| 1 | 5 | 1444ns | -93.6% | -86.9% |
| 1 | 6 | 1953ns | -94.8% | -92.1% |
| 1 | 7 | 1402ns | -93.4% | -89.0% |
| 1 | 8 | 1520ns | -95.0% | -87.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.105 | ok |
| popcount_std | 0.176 | ok |
| popcount_swar | -0.138 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 23.4ns | 1662.4ns | 1.4% |  |
| popcount_std | 32.5ns | 126.1ns | 25.8% | HIGH |
| popcount_swar | 32.7ns | 252.9ns | 12.9% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 1355.2-2155.7 ns)
   1355.2 |##
   1395.2 |########################
   1435.2 |########################################
   1475.2 |#################
   1515.3 |#################
   1555.3 |##
   1595.3 |
   1635.3 |#
   1675.4 |
   1715.4 |
   1755.4 |##
   1795.4 |#######################
   1835.5 |########
   1875.5 |#############
   1915.5 |####################
   1955.5 |#########
   1995.6 |#
   2035.6 |##
   2075.6 |
   2115.6 |
  (18 below, 4 above range)

popcount_std (n=160, range 76.0-274.8 ns)
     76.0 |###############
     85.9 |########################################
     95.9 |####
    105.8 |####
    115.8 |
    125.7 |
    135.6 |#
    145.6 |
    155.5 |
    165.5 |
    175.4 |
    185.3 |
    195.3 |#
    205.2 |
    215.2 |
    225.1 |
    235.0 |#
    245.0 |
    254.9 |##
    264.9 |#
  (18 below, 18 above range)

popcount_swar (n=160, range 153.6-554.1 ns)
    153.6 |################
    173.6 |########################################
    193.6 |#
    213.6 |
    233.7 |
    253.7 |#
    273.7 |#
    293.8 |
    313.8 |
    333.8 |
    353.8 |
    373.9 |
    393.9 |
    413.9 |
    434.0 |
    454.0 |
    474.0 |##
    494.0 |
    514.1 |
    534.1 |#
  (14 below, 21 above range)

```

## Diagnostics

- **popcount_handrolled**: CV=25.4% (high variance, measurements may be unstable)
- **popcount_std**: CV=62.2% (high variance, measurements may be unstable)
- **popcount_std**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **popcount_std**: bridge=25.7% of algo (FFI overhead may distort results)
- **popcount_swar**: CV=62.3% (high variance, measurements may be unstable)
- **popcount_swar**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **popcount_swar**: bridge=12.8% of algo (FFI overhead may distort results)
