# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (37.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 1.28 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 113% faster than the next best (popcount_swar)

popcount_std (1.28 us) leads popcount_swar (2.73 us) by 113%, a clear separation rather than a photo finish. CV 40.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 97% (significant)

popcount_std is -36.04 us (97%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 29.0x slower than the field

popcount_handrolled (37.29 us) is 29.0x the fastest (1.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 40.6%)

popcount_std wins on median (1.28 us) yet has the highest variance (CV 40.6%), while popcount_swar is the steadiest (CV 0.3%, 2.73 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 29.0x the fastest

Fastest popcount_std (1.28 us) to slowest popcount_handrolled (37.29 us): 29.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 1.8x its best-20%

popcount_std's best 20% of batches run at 1.05 us but its worst 20% at 1.88 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 1284.1 ns median (-96.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 29.04x (fastest 1284.1 ns, slowest 37291.8 ns)

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
| popcount_handrolled | 38396ns | 39197ns | 34873ns | 39205ns | 39490ns | base |
| popcount_std | 3325ns | 3160ns | 2586ns | 3126ns | 4662ns | -91.34% |
| popcount_swar | 4620ns | 4615ns | 4602ns | 4615ns | 4654ns | -87.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 36532ns | 33185ns | 37580ns | base | 0.448 |
| popcount_std | 1349ns | 1051ns | 1885ns | -96.31% | 12.144 |
| popcount_swar | 2732ns | 2722ns | 2746ns | -92.52% | 5.998 |

## Performance model

- Peak throughput: **15.596 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.439 | 2.8% |
| popcount_std | 12.759 | 81.8% |
| popcount_swar | 6.002 | 38.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 38396ns | 38396ns | base |
| popcount_std | 3325ns | 3325ns | -91.34% |
| popcount_swar | 4620ns | 4620ns | -87.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 37292ns | base | --- | [37252, 37340] | --- | --- | --- | --- |
| popcount_std | 1284ns | -36044.2ns (-96.7%) | [-36097, -35991]ns | [1284, 1284] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 2730ns | -34555.6ns (-92.7%) | [-34599, -34524]ns | [2729, 2731] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 37216ns | -96.5% | -92.7% |
| 1 | 2 | 37310ns | -96.6% | -92.7% |
| 1 | 3 | 37233ns | -97.2% | -92.7% |
| 1 | 4 | 37269ns | -96.6% | -92.7% |
| 1 | 5 | 37330ns | -96.6% | -92.7% |
| 1 | 6 | 30820ns | -95.8% | -91.1% |
| 1 | 7 | 37506ns | -96.6% | -92.7% |
| 1 | 8 | 37463ns | -97.2% | -92.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.190 | ok |
| popcount_std | -0.209 | moderate- |
| popcount_swar | -0.460 | moderate- |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 26.6ns | 36531.8ns | 0.1% |  |
| popcount_std | 24.7ns | 1349.1ns | 1.8% |  |
| popcount_swar | 24.5ns | 2731.5ns | 0.9% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 33184.9-37580.0 ns)
  33184.9 |
  33404.6 |
  33624.4 |
  33844.1 |
  34063.9 |
  34283.6 |
  34503.4 |
  34723.1 |
  34942.9 |
  35162.7 |
  35382.4 |
  35602.2 |
  35821.9 |
  36041.7 |
  36261.4 |
  36481.2 |
  36700.9 |
  36920.7 |###
  37140.5 |########################################
  37360.2 |############################
  (20 below, 6 above range)

popcount_std (n=160, range 1050.5-1884.9 ns)
   1050.5 |########
   1092.2 |
   1134.0 |
   1175.7 |
   1217.4 |
   1259.1 |########################################
   1300.8 |#
   1342.6 |
   1384.3 |
   1426.0 |
   1467.7 |
   1509.4 |
   1551.2 |
   1592.9 |
   1634.6 |
   1676.3 |
   1718.0 |
   1759.8 |
   1801.5 |
   1843.2 |
  (16 below, 8 above range)

popcount_swar (n=160, range 2721.5-2746.3 ns)
   2721.5 |########################################
   2722.8 |###################################
   2724.0 |##
   2725.2 |
   2726.5 |
   2727.7 |#####################################
   2728.9 |###########################
   2730.2 |#####################
   2731.4 |####
   2732.7 |################
   2733.9 |################
   2735.1 |#####################
   2736.4 |############
   2737.6 |##
   2738.9 |####
   2740.1 |########
   2741.3 |##
   2742.6 |##################
   2743.8 |######
   2745.0 |##
  (16 below, 11 above range)

```

## Diagnostics

- **popcount_std**: CV=38.6% (high variance, measurements may be unstable)
