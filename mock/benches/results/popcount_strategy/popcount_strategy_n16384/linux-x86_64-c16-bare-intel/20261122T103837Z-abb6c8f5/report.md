# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (37.28 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 1.28 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 112% faster than the next best (popcount_swar)

popcount_std (1.28 us) leads popcount_swar (2.72 us) by 112%, a clear separation rather than a photo finish. CV 39.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 97% (significant)

popcount_std is -35.99 us (97%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 29.0x slower than the field

popcount_handrolled (37.28 us) is 29.0x the fastest (1.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 39.6%)

popcount_std wins on median (1.28 us) yet has the highest variance (CV 39.6%), while popcount_handrolled is the steadiest (CV 8.2%, 37.28 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 29.0x the fastest

Fastest popcount_std (1.28 us) to slowest popcount_handrolled (37.28 us): 29.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 1.8x its best-20%

popcount_std's best 20% of batches run at 1.05 us but its worst 20% at 1.90 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 1283.9 ns median (-96.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 29.04x (fastest 1283.9 ns, slowest 37278.7 ns)

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
| popcount_handrolled | 37552ns | 39184ns | 32071ns | 38623ns | 39823ns | base |
| popcount_std | 3285ns | 3161ns | 2584ns | 3037ns | 4728ns | -91.25% |
| popcount_swar | 4430ns | 4610ns | 3772ns | 4552ns | 4720ns | -88.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 35729ns | 30509ns | 37904ns | base | 0.459 |
| popcount_std | 1330ns | 1050ns | 1903ns | -96.28% | 12.318 |
| popcount_swar | 2618ns | 2232ns | 2783ns | -92.67% | 6.258 |

## Performance model

- Peak throughput: **15.598 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.440 | 2.8% |
| popcount_std | 12.761 | 81.8% |
| popcount_swar | 6.016 | 38.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 37552ns | 37552ns | base |
| popcount_std | 3285ns | 3285ns | -91.25% |
| popcount_swar | 4430ns | 4430ns | -88.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 37279ns | base | --- | [37223, 37319] | --- | --- | --- | --- |
| popcount_std | 1284ns | -35989.6ns (-96.5%) | [-36052, -35933]ns | [1284, 1284] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 2723ns | -34545.3ns (-92.7%) | [-34580, -34494]ns | [2722, 2728] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 30504ns | -96.6% | -91.1% |
| 1 | 2 | 37223ns | -96.6% | -92.7% |
| 1 | 3 | 37280ns | -96.6% | -92.7% |
| 1 | 4 | 30544ns | -96.6% | -92.7% |
| 1 | 5 | 37266ns | -96.6% | -92.7% |
| 1 | 6 | 37305ns | -97.2% | -92.7% |
| 1 | 7 | 37490ns | -96.6% | -94.0% |
| 1 | 8 | 37635ns | -96.6% | -92.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.181 | ok |
| popcount_std | -0.475 | moderate- |
| popcount_swar | -0.382 | moderate- |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 26.2ns | 35729.1ns | 0.1% |  |
| popcount_std | 24.4ns | 1330.1ns | 1.8% |  |
| popcount_swar | 23.4ns | 2617.9ns | 0.9% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 30509.0-37903.5 ns)
  30509.0 |#############
  30878.7 |
  31248.5 |
  31618.2 |
  31987.9 |
  32357.6 |
  32727.4 |
  33097.1 |
  33466.8 |
  33836.6 |
  34206.3 |
  34576.0 |
  34945.7 |
  35315.5 |
  35685.2 |
  36054.9 |
  36424.6 |
  36794.4 |######
  37164.1 |########################################
  37533.8 |############
  (14 below, 5 above range)

popcount_std (n=160, range 1050.4-1903.4 ns)
   1050.4 |################
   1093.0 |
   1135.7 |
   1178.3 |
   1221.0 |
   1263.6 |########################################
   1306.3 |
   1348.9 |
   1391.6 |
   1434.2 |
   1476.9 |
   1519.5 |
   1562.2 |
   1604.8 |
   1647.5 |
   1690.1 |
   1732.8 |
   1775.4 |
   1818.1 |
   1860.7 |
  (14 below, 12 above range)

popcount_swar (n=160, range 2231.7-2782.8 ns)
   2231.7 |##############
   2259.3 |
   2286.8 |
   2314.4 |
   2341.9 |
   2369.5 |
   2397.0 |
   2424.6 |
   2452.2 |
   2479.7 |
   2507.3 |
   2534.8 |
   2562.4 |
   2589.9 |
   2617.5 |
   2645.0 |
   2672.6 |
   2700.1 |#################################
   2727.7 |########################################
   2755.3 |##
  (15 below, 2 above range)

```

## Diagnostics

- **popcount_std**: CV=38.2% (high variance, measurements may be unstable)
