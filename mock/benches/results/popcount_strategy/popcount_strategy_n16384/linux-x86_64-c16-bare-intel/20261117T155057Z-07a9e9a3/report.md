# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (37.20 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 1.28 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 112% faster than the next best (popcount_swar)

popcount_std (1.28 us) leads popcount_swar (2.72 us) by 112%, a clear separation rather than a photo finish. CV 44.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 97% (significant)

popcount_std is -35.95 us (97%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 29.0x slower than the field

popcount_handrolled (37.20 us) is 29.0x the fastest (1.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 44.4%)

popcount_std wins on median (1.28 us) yet has the highest variance (CV 44.4%), while popcount_handrolled is the steadiest (CV 7.7%, 37.20 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 29.0x the fastest

Fastest popcount_std (1.28 us) to slowest popcount_handrolled (37.20 us): 29.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 2.0x its best-20%

popcount_std's best 20% of batches run at 1.05 us but its worst 20% at 2.09 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 1283.9 ns median (-96.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 28.97x (fastest 1283.9 ns, slowest 37197.5 ns)

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
| popcount_handrolled | 37495ns | 39100ns | 32244ns | 38555ns | 39562ns | base |
| popcount_std | 3450ns | 3168ns | 2595ns | 3154ns | 5196ns | -90.80% |
| popcount_swar | 4336ns | 4610ns | 3770ns | 4382ns | 4766ns | -88.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 35675ns | 30688ns | 37655ns | base | 0.459 |
| popcount_std | 1395ns | 1052ns | 2091ns | -96.09% | 11.743 |
| popcount_swar | 2561ns | 2227ns | 2811ns | -92.82% | 6.397 |

## Performance model

- Peak throughput: **15.577 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.440 | 2.8% |
| popcount_std | 12.761 | 81.9% |
| popcount_swar | 6.017 | 38.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 37495ns | 37495ns | base |
| popcount_std | 3450ns | 3450ns | -90.80% |
| popcount_swar | 4336ns | 4336ns | -88.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 37198ns | base | --- | [37164, 37225] | --- | --- | --- | --- |
| popcount_std | 1284ns | -35951.4ns (-96.7%) | [-36018, -35902]ns | [1284, 1284] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 2723ns | -34531.0ns (-92.8%) | [-34650, -34458]ns | [2722, 2724] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 37189ns | -96.5% | -92.7% |
| 1 | 2 | 37220ns | -96.6% | -94.0% |
| 1 | 3 | 37256ns | -97.2% | -94.0% |
| 1 | 4 | 30641ns | -95.8% | -91.1% |
| 1 | 5 | 37320ns | -96.6% | -92.7% |
| 1 | 6 | 37134ns | -97.2% | -92.6% |
| 1 | 7 | 37556ns | -96.6% | -92.7% |
| 1 | 8 | 30742ns | -95.8% | -92.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.237 | moderate- |
| popcount_std | -0.376 | moderate- |
| popcount_swar | 0.068 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 25.9ns | 35675.1ns | 0.1% |  |
| popcount_std | 26.1ns | 1395.3ns | 1.9% |  |
| popcount_swar | 22.9ns | 2561.3ns | 0.9% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 30688.4-37655.2 ns)
  30688.4 |#############
  31036.8 |
  31385.1 |
  31733.4 |
  32081.8 |
  32430.1 |
  32778.5 |
  33126.8 |
  33475.1 |
  33823.5 |
  34171.8 |
  34520.1 |
  34868.5 |
  35216.8 |
  35565.1 |
  35913.5 |
  36261.8 |
  36610.2 |
  36958.5 |########################################
  37306.8 |########################
  (16 below, 7 above range)

popcount_std (n=160, range 1051.8-2090.8 ns)
   1051.8 |##
   1103.8 |
   1155.7 |
   1207.7 |
   1259.6 |########################################
   1311.6 |
   1363.5 |
   1415.5 |
   1467.4 |
   1519.4 |
   1571.3 |
   1623.3 |
   1675.2 |
   1727.2 |
   1779.1 |
   1831.1 |
   1883.0 |
   1935.0 |
   1986.9 |
   2038.9 |
  (28 below, 12 above range)

popcount_swar (n=160, range 2227.4-2811.4 ns)
   2227.4 |##################
   2256.6 |
   2285.8 |
   2315.0 |
   2344.2 |
   2373.4 |
   2402.6 |
   2431.8 |
   2461.0 |
   2490.2 |
   2519.4 |
   2548.6 |
   2577.8 |
   2607.0 |
   2636.2 |
   2665.4 |
   2694.6 |#############
   2723.8 |########################################
   2753.0 |
   2782.2 |
  (24 below, 1 above range)

```

## Diagnostics

- **popcount_std**: CV=40.9% (high variance, measurements may be unstable)
