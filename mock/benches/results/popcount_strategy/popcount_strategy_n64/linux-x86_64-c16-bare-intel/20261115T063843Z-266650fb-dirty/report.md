# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (91 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 15 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 42% faster than the next best (popcount_swar)

popcount_std (15 ns) leads popcount_swar (22 ns) by 42%, a clear separation rather than a photo finish. CV 93.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 85% (significant)

popcount_std is -77 ns (85%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 6.0x slower than the field

popcount_handrolled (91 ns) is 6.0x the fastest (15 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_swar shows warm-up / thermal drift (autocorr +0.56)

popcount_swar's per-pass series has lag-1 autocorrelation +0.56, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.0x the fastest

Fastest popcount_std (15 ns) to slowest popcount_handrolled (91 ns): 6.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_handrolled is inconsistent: worst-20% is 4.0x its best-20%

popcount_handrolled's best 20% of batches run at 70 ns but its worst 20% at 278 ns (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 15.3 ns median (-83.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.96x (fastest 15.3 ns, slowest 91.2 ns)

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
| popcount_handrolled | 2764ns | 1969ns | 1606ns | 2055ns | 6049ns | base |
| popcount_std | 2796ns | 1893ns | 1707ns | 1993ns | 6295ns | +1.17% |
| popcount_swar | 2679ns | 1897ns | 1552ns | 1935ns | 6035ns | -3.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 126ns | 70ns | 278ns | base | 0.506 |
| popcount_std | 23ns | 14ns | 50ns | -82.03% | 2.816 |
| popcount_swar | 31ns | 18ns | 69ns | -75.77% | 2.088 |

## Performance model

- Peak throughput: **4.640 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.702 | 15.1% |
| popcount_std | 4.183 | 90.2% |
| popcount_swar | 2.949 | 63.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2764ns | 2764ns | base |
| popcount_std | 2796ns | 2796ns | +1.17% |
| popcount_swar | 2679ns | 2679ns | -3.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 91ns | base | --- | [88, 93] | --- | --- | --- | --- |
| popcount_std | 15ns | -77.5ns (-84.9%) | [-78, -72]ns | [15, 16] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 22ns | -70.7ns (-77.5%) | [-72, -66]ns | [22, 22] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 68ns | -77.0% | -74.1% |
| 1 | 2 | 72ns | -78.5% | -75.6% |
| 1 | 3 | 91ns | -86.1% | -80.6% |
| 1 | 4 | 88ns | -82.6% | -75.0% |
| 1 | 5 | 74ns | -79.5% | -70.6% |
| 1 | 6 | 97ns | -81.6% | -77.8% |
| 1 | 7 | 93ns | -83.6% | -76.5% |
| 1 | 8 | 87ns | -82.4% | -75.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | 0.139 | ok |
| popcount_std | -0.061 | ok |
| popcount_swar | 0.564 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 33.8ns | 126.5ns | 26.7% | HIGH |
| popcount_std | 34.6ns | 22.7ns | 152.1% | HIGH |
| popcount_swar | 34.3ns | 30.6ns | 111.9% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 70.1-278.1 ns)
     70.1 |########################
     80.5 |##########################
     90.9 |########################################
    101.3 |#
    111.7 |
    122.1 |
    132.5 |
    142.9 |##
    153.3 |
    163.7 |
    174.1 |
    184.5 |
    194.9 |
    205.3 |
    215.7 |##
    226.1 |##
    236.5 |####
    246.9 |##
    257.3 |
    267.7 |##
  (16 below, 16 above range)

popcount_std (n=160, range 13.8-50.3 ns)
     13.8 |########################################
     15.6 |######
     17.4 |###
     19.3 |
     21.1 |
     22.9 |
     24.7 |
     26.6 |
     28.4 |
     30.2 |
     32.0 |
     33.8 |
     35.7 |
     37.5 |
     39.3 |
     41.1 |#
     43.0 |#
     44.8 |#
     46.6 |#
     48.4 |##
  (17 below, 14 above range)

popcount_swar (n=160, range 17.6-69.4 ns)
     17.6 |###############
     20.2 |########################################
     22.8 |
     25.4 |
     27.9 |
     30.5 |
     33.1 |#
     35.7 |
     38.3 |
     40.9 |
     43.5 |
     46.1 |
     48.7 |
     51.3 |#
     53.9 |#
     56.5 |#
     59.1 |#
     61.7 |##
     64.3 |#
     66.8 |
  (13 below, 16 above range)

```

## Diagnostics

- **popcount_handrolled**: CV=63.5% (high variance, measurements may be unstable)
- **popcount_handrolled**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **popcount_handrolled**: bridge=26.4% of algo (FFI overhead may distort results)
- **popcount_std**: CV=63.1% (high variance, measurements may be unstable)
- **popcount_std**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **popcount_std**: bridge=153.6% of algo (FFI overhead may distort results)
- **popcount_swar**: CV=66.3% (high variance, measurements may be unstable)
- **popcount_swar**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **popcount_swar**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **popcount_swar**: bridge=111.5% of algo (FFI overhead may distort results)
