# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (530 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 92 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 470% faster than the next best (popcount_swar)

popcount_std (92 ns) leads popcount_swar (526 ns) by 470%, a clear separation rather than a photo finish. CV 7.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 82% (significant)

popcount_std is -436 ns (82%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 5.7x slower than the field

popcount_handrolled (530 ns) is 5.7x the fastest (92 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 7.3%)

popcount_std wins on median (92 ns) yet has the highest variance (CV 7.3%), while popcount_handrolled is the steadiest (CV 4.7%, 530 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.7x the fastest

Fastest popcount_std (92 ns) to slowest popcount_handrolled (530 ns): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: popcount_std** at 92.3 ns median (-82.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 5.74x (fastest 92.3 ns, slowest 529.6 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| popcount_handrolled | 2698ns | 2684ns | 2572ns | 2680ns | 2881ns | base |
| popcount_std | 2288ns | 2299ns | 2144ns | 2273ns | 2473ns | -15.23% |
| popcount_swar | 2708ns | 2659ns | 2569ns | 2672ns | 2956ns | +0.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 532ns | 504ns | 570ns | base | 7.705 |
| popcount_std | 93ns | 85ns | 103ns | -82.47% | 43.945 |
| popcount_swar | 538ns | 506ns | 595ns | +1.14% | 7.618 |

## Performance model

- Peak throughput: **48.321 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 7.734 | 16.0% |
| popcount_std | 44.377 | 91.8% |
| popcount_swar | 7.787 | 16.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2698ns | 2698ns | base |
| popcount_std | 2288ns | 2288ns | -15.23% |
| popcount_swar | 2708ns | 2708ns | +0.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 530ns | base | --- | [521, 536] | --- | --- | --- | --- |
| popcount_std | 92ns | -435.9ns (-82.3%) | [-441, -428]ns | [91, 93] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 526ns | no significant difference | [-14, +7]ns | [522, 529] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 510ns | -82.6% | +0.5% |
| 1 | 2 | 544ns | -81.9% | -3.1% |
| 1 | 3 | 506ns | -81.8% | +4.0% |
| 1 | 4 | 536ns | -83.8% | +9.7% |
| 1 | 5 | 507ns | -81.6% | +14.9% |
| 1 | 6 | 536ns | -83.1% | -5.4% |
| 1 | 7 | 525ns | -81.4% | -2.0% |
| 1 | 8 | 584ns | -84.5% | -9.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.281 | moderate- |
| popcount_std | -0.424 | moderate- |
| popcount_swar | 0.246 | moderate+ |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.1ns | 531.6ns | 0.6% |  |
| popcount_std | 2.3ns | 93.2ns | 2.5% |  |
| popcount_swar | 3.4ns | 537.6ns | 0.6% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 504.1-570.1 ns)
    504.1 |########################################
    507.4 |#######################
    510.7 |##################
    514.0 |#######################
    517.3 |##########
    520.6 |########
    523.9 |######
    527.2 |##################
    530.5 |############
    533.8 |##########
    537.1 |#####################
    540.4 |#################################
    543.7 |#########################
    547.0 |##
    550.3 |##########
    553.6 |####
    556.9 |####
    560.2 |
    563.5 |
    566.8 |##
  (12 below, 17 above range)

popcount_std (n=160, range 84.8-103.3 ns)
     84.8 |##################
     85.7 |#########
     86.6 |##################
     87.5 |#######################
     88.5 |###########
     89.4 |#####################
     90.3 |########################################
     91.2 |#########
     92.2 |################
     93.1 |##############
     94.0 |################
     94.9 |##############
     95.9 |################
     96.8 |##############
     97.7 |#######
     98.6 |##################
     99.6 |#####################
    100.5 |##
    101.4 |#########
    102.3 |#######
  (15 below, 13 above range)

popcount_swar (n=160, range 506.3-595.2 ns)
    506.3 |################################
    510.7 |######################
    515.1 |########################
    519.6 |########################################
    524.0 |######################################
    528.5 |####################
    532.9 |######
    537.4 |########
    541.8 |################
    546.3 |####
    550.7 |########
    555.2 |######
    559.6 |##
    564.0 |
    568.5 |##
    572.9 |
    577.4 |####
    581.8 |############
    586.3 |################
    590.7 |############
  (14 below, 10 above range)

```
