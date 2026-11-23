# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (8.04 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 331 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 109% faster than the next best (popcount_swar)

popcount_std (331 ns) leads popcount_swar (693 ns) by 109%, a clear separation rather than a photo finish. CV 76.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 95% (significant)

popcount_std is -7.65 us (95%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 24.3x slower than the field

popcount_handrolled (8.04 us) is 24.3x the fastest (331 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 76.5%)

popcount_std wins on median (331 ns) yet has the highest variance (CV 76.5%), while popcount_handrolled is the steadiest (CV 9.4%, 8.04 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 24.3x the fastest

Fastest popcount_std (331 ns) to slowest popcount_handrolled (8.04 us): 24.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 3.3x its best-20%

popcount_std's best 20% of batches run at 277 ns but its worst 20% at 918 ns (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 331.1 ns median (-95.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 24.29x (fastest 331.1 ns, slowest 8043.9 ns)

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
| popcount_handrolled | 9358ns | 9942ns | 8104ns | 9484ns | 10233ns | base |
| popcount_std | 2951ns | 2210ns | 1844ns | 2246ns | 6175ns | -68.46% |
| popcount_swar | 3051ns | 2571ns | 2104ns | 2568ns | 5444ns | -67.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 7582ns | 6546ns | 8319ns | base | 0.540 |
| popcount_std | 441ns | 277ns | 918ns | -94.19% | 9.292 |
| popcount_swar | 820ns | 567ns | 1458ns | -89.18% | 4.994 |

## Performance model

- Peak throughput: **14.795 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.509 | 3.4% |
| popcount_std | 12.371 | 83.6% |
| popcount_swar | 5.912 | 40.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 9358ns | 9358ns | base |
| popcount_std | 2951ns | 2951ns | -68.46% |
| popcount_swar | 3051ns | 3051ns | -67.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 8044ns | base | --- | [8018, 8073] | --- | --- | --- | --- |
| popcount_std | 331ns | -7654.6ns (-95.2%) | [-7728, -7204]ns | [331, 331] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 693ns | -7394.6ns (-91.9%) | [-7430, -6369]ns | [692, 694] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 6691ns | -95.1% | -89.6% |
| 1 | 2 | 8094ns | -95.9% | -91.4% |
| 1 | 3 | 8073ns | -95.9% | -93.0% |
| 1 | 4 | 8292ns | -96.0% | -91.6% |
| 1 | 5 | 8048ns | -96.6% | -93.0% |
| 1 | 6 | 8229ns | -96.0% | -91.6% |
| 1 | 7 | 6683ns | -95.0% | -89.6% |
| 1 | 8 | 6483ns | -95.8% | -89.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | 0.269 | moderate+ |
| popcount_std | -0.210 | moderate- |
| popcount_swar | -0.354 | moderate- |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 25.2ns | 7582.2ns | 0.3% |  |
| popcount_std | 31.7ns | 440.8ns | 7.2% | HIGH |
| popcount_swar | 28.7ns | 820.3ns | 3.5% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 6546.0-8318.5 ns)
   6546.0 |#####
   6634.6 |########################################
   6723.2 |########
   6811.9 |
   6900.5 |
   6989.1 |
   7077.8 |
   7166.4 |
   7255.0 |
   7343.6 |
   7432.3 |
   7520.9 |
   7609.5 |
   7698.1 |
   7786.8 |
   7875.4 |
   7964.0 |#################################
   8052.6 |######################################
   8141.3 |####################
   8229.9 |######################
  (20 below, 14 above range)

popcount_std (n=160, range 276.9-917.9 ns)
    276.9 |
    308.9 |########################################
    341.0 |##
    373.0 |
    405.1 |
    437.1 |
    469.2 |
    501.2 |
    533.3 |
    565.3 |
    597.4 |
    629.4 |
    661.5 |
    693.6 |
    725.6 |
    757.7 |
    789.7 |
    821.8 |#
    853.8 |#
    885.9 |#
  (28 below, 13 above range)

popcount_swar (n=160, range 567.1-1457.7 ns)
    567.1 |###
    611.6 |
    656.1 |########################################
    700.7 |##
    745.2 |
    789.7 |
    834.3 |
    878.8 |
    923.3 |
    967.8 |
   1012.4 |
   1056.9 |
   1101.4 |
   1146.0 |
   1190.5 |
   1235.0 |
   1279.5 |
   1324.1 |
   1368.6 |
   1413.1 |
  (25 below, 18 above range)

```

## Diagnostics

- **popcount_std**: CV=57.4% (high variance, measurements may be unstable)
- **popcount_std**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **popcount_swar**: CV=52.3% (high variance, measurements may be unstable)
