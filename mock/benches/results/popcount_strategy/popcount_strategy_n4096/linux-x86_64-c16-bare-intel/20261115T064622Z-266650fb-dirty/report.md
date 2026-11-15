# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (8.04 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 331 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 109% faster than the next best (popcount_swar)

popcount_std (331 ns) leads popcount_swar (693 ns) by 109%, a clear separation rather than a photo finish. CV 77.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 96% (significant)

popcount_std is -7.69 us (96%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 24.3x slower than the field

popcount_handrolled (8.04 us) is 24.3x the fastest (331 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 77.1%)

popcount_std wins on median (331 ns) yet has the highest variance (CV 77.1%), while popcount_handrolled is the steadiest (CV 14.8%, 8.04 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 24.3x the fastest

Fastest popcount_std (331 ns) to slowest popcount_handrolled (8.04 us): 24.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 3.1x its best-20%

popcount_std's best 20% of batches run at 271 ns but its worst 20% at 830 ns (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 331.0 ns median (-95.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 24.30x (fastest 331.0 ns, slowest 8043.9 ns)

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
| popcount_handrolled | 9779ns | 9948ns | 8195ns | 9836ns | 11194ns | base |
| popcount_std | 2766ns | 2207ns | 1804ns | 2139ns | 5608ns | -71.72% |
| popcount_swar | 2979ns | 2571ns | 2105ns | 2567ns | 5087ns | -69.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 7932ns | 6643ns | 9129ns | base | 0.516 |
| popcount_std | 412ns | 271ns | 830ns | -94.80% | 9.931 |
| popcount_swar | 802ns | 569ns | 1363ns | -89.89% | 5.107 |

## Performance model

- Peak throughput: **15.126 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.509 | 3.4% |
| popcount_std | 12.375 | 81.8% |
| popcount_swar | 5.907 | 39.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 9779ns | 9779ns | base |
| popcount_std | 2766ns | 2766ns | -71.72% |
| popcount_swar | 2979ns | 2979ns | -69.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 8044ns | base | --- | [8015, 8076] | --- | --- | --- | --- |
| popcount_std | 331ns | -7685.8ns (-95.5%) | [-7729, -7667]ns | [331, 331] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 693ns | -7380.8ns (-91.8%) | [-7406, -7341]ns | [693, 694] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 6699ns | -96.0% | -89.6% |
| 1 | 2 | 6614ns | -95.0% | -89.5% |
| 1 | 3 | 8126ns | -95.9% | -91.5% |
| 1 | 4 | 8272ns | -96.7% | -91.6% |
| 1 | 5 | 8007ns | -95.9% | -91.3% |
| 1 | 6 | 8387ns | -96.8% | -93.2% |
| 1 | 7 | 8003ns | -95.9% | -92.9% |
| 1 | 8 | 8066ns | -95.9% | -91.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | 0.431 | moderate+ |
| popcount_std | -0.475 | moderate- |
| popcount_swar | 0.280 | moderate+ |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 26.1ns | 7932.3ns | 0.3% |  |
| popcount_std | 30.1ns | 412.5ns | 7.3% | HIGH |
| popcount_swar | 28.0ns | 802.1ns | 3.5% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 6642.8-9128.7 ns)
   6642.8 |###################
   6767.1 |#
   6891.4 |
   7015.6 |
   7139.9 |
   7264.2 |
   7388.5 |
   7512.8 |
   7637.1 |
   7761.4 |
   7885.7 |###################
   8010.0 |########################################
   8134.3 |###############
   8258.6 |##################
   8382.9 |####
   8507.2 |
   8631.5 |
   8755.8 |
   8880.1 |
   9004.4 |
  (14 below, 5 above range)

popcount_std (n=160, range 270.8-829.5 ns)
    270.8 |#################
    298.7 |
    326.7 |########################################
    354.6 |
    382.5 |
    410.5 |
    438.4 |
    466.3 |
    494.3 |
    522.2 |
    550.1 |#
    578.1 |
    606.0 |
    634.0 |
    661.9 |
    689.8 |
    717.8 |
    745.7 |
    773.6 |
    801.6 |
  (11 below, 18 above range)

popcount_swar (n=160, range 569.0-1363.0 ns)
    569.0 |#####
    608.7 |
    648.4 |
    688.1 |########################################
    727.8 |
    767.5 |
    807.2 |
    846.9 |
    886.6 |
    926.3 |
    966.0 |
   1005.7 |
   1045.4 |
   1085.1 |
   1124.8 |
   1164.5 |
   1204.2 |
   1243.9 |
   1283.6 |
   1323.3 |
  (17 below, 16 above range)

```

## Diagnostics

- **popcount_std**: CV=61.9% (high variance, measurements may be unstable)
- **popcount_std**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **popcount_swar**: CV=50.9% (high variance, measurements may be unstable)
