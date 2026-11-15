# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (1.80 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 93 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 103% faster than the next best (popcount_swar)

popcount_std (93 ns) leads popcount_swar (189 ns) by 103%, a clear separation rather than a photo finish. CV 95.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 92% (significant)

popcount_std is -1.66 us (92%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 19.4x slower than the field

popcount_handrolled (1.80 us) is 19.4x the fastest (93 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 95.1%)

popcount_std wins on median (93 ns) yet has the highest variance (CV 95.1%), while popcount_handrolled is the steadiest (CV 38.1%, 1.80 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 19.4x the fastest

Fastest popcount_std (93 ns) to slowest popcount_handrolled (1.80 us): 19.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 3.6x its best-20%

popcount_std's best 20% of batches run at 85 ns but its worst 20% at 303 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 93.0 ns median (-94.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 19.36x (fastest 93.0 ns, slowest 1800.8 ns)

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
| popcount_handrolled | 4012ns | 3692ns | 3281ns | 3681ns | 5735ns | base |
| popcount_std | 2943ns | 1974ns | 1812ns | 2182ns | 6356ns | -26.64% |
| popcount_swar | 2781ns | 2067ns | 1690ns | 2050ns | 6066ns | -30.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 1950ns | 1584ns | 2811ns | base | 0.525 |
| popcount_std | 139ns | 85ns | 303ns | -92.85% | 7.344 |
| popcount_swar | 253ns | 154ns | 546ns | -87.04% | 4.053 |

## Performance model

- Peak throughput: **11.992 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.569 | 4.7% |
| popcount_std | 11.011 | 91.8% |
| popcount_swar | 5.427 | 45.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 4012ns | 4012ns | base |
| popcount_std | 2943ns | 2943ns | -26.64% |
| popcount_swar | 2781ns | 2781ns | -30.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 1801ns | base | --- | [1765, 1816] | --- | --- | --- | --- |
| popcount_std | 93ns | -1664.2ns (-92.4%) | [-1705, -1632]ns | [93, 93] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 189ns | -1566.7ns (-87.0%) | [-1612, -1543]ns | [188, 189] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 1566ns | -94.1% | -87.9% |
| 1 | 2 | 1811ns | -94.9% | -89.6% |
| 1 | 3 | 1623ns | -94.3% | -90.4% |
| 1 | 4 | 2356ns | -96.1% | -92.0% |
| 1 | 5 | 1758ns | -94.7% | -89.3% |
| 1 | 6 | 1906ns | -96.0% | -90.1% |
| 1 | 7 | 1715ns | -94.6% | -91.0% |
| 1 | 8 | 1837ns | -94.9% | -91.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.365 | moderate- |
| popcount_std | -0.159 | ok |
| popcount_swar | 0.067 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 27.7ns | 1949.9ns | 1.4% |  |
| popcount_std | 35.6ns | 139.4ns | 25.5% | HIGH |
| popcount_swar | 32.2ns | 252.7ns | 12.7% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 1584.3-2811.3 ns)
   1584.3 |#####################
   1645.7 |########
   1707.0 |##############################
   1768.4 |########################################
   1829.7 |########################
   1891.1 |######################
   1952.4 |
   2013.8 |
   2075.1 |#
   2136.5 |
   2197.8 |#
   2259.2 |
   2320.5 |
   2381.8 |
   2443.2 |##
   2504.5 |
   2565.9 |#
   2627.2 |
   2688.6 |##
   2749.9 |
  (18 below, 10 above range)

popcount_std (n=160, range 85.4-303.4 ns)
     85.4 |########################################
     96.3 |
    107.2 |
    118.1 |
    129.0 |
    139.9 |
    150.8 |
    161.7 |
    172.6 |
    183.5 |
    194.4 |
    205.3 |
    216.2 |
    227.1 |
    238.0 |
    248.9 |#
    259.8 |#
    270.7 |#
    281.6 |#
    292.5 |
  (14 below, 15 above range)

popcount_swar (n=160, range 154.2-545.7 ns)
    154.2 |##############
    173.8 |########################################
    193.4 |
    212.9 |
    232.5 |
    252.1 |
    271.6 |
    291.2 |
    310.8 |
    330.4 |
    349.9 |
    369.5 |
    389.1 |
    408.7 |
    428.2 |
    447.8 |#
    467.4 |##
    486.9 |
    506.5 |##
    526.1 |
  (13 below, 15 above range)

```

## Diagnostics

- **popcount_handrolled**: CV=35.2% (high variance, measurements may be unstable)
- **popcount_std**: CV=63.5% (high variance, measurements may be unstable)
- **popcount_std**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **popcount_std**: bridge=25.6% of algo (FFI overhead may distort results)
- **popcount_swar**: CV=60.3% (high variance, measurements may be unstable)
- **popcount_swar**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **popcount_swar**: bridge=12.7% of algo (FFI overhead may distort results)
