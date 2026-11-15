# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (158 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 35 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 338% faster than the next best (popcount_swar)

popcount_std (35 ns) leads popcount_swar (155 ns) by 338%, a clear separation rather than a photo finish. CV 16.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 76% (significant)

popcount_std is -121 ns (76%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 4.5x slower than the field

popcount_handrolled (158 ns) is 4.5x the fastest (35 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 16.1%)

popcount_std wins on median (35 ns) yet has the highest variance (CV 16.1%), while popcount_swar is the steadiest (CV 5.5%, 155 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.5x the fastest

Fastest popcount_std (35 ns) to slowest popcount_handrolled (158 ns): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 1.5x its best-20%

popcount_std's best 20% of batches run at 30 ns but its worst 20% at 45 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 35.4 ns median (-77.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.47x (fastest 35.4 ns, slowest 158.3 ns)

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
| popcount_handrolled | 2378ns | 2372ns | 2216ns | 2368ns | 2571ns | base |
| popcount_std | 2268ns | 2250ns | 2099ns | 2268ns | 2437ns | -4.62% |
| popcount_swar | 2297ns | 2290ns | 2211ns | 2286ns | 2417ns | -3.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 160ns | 147ns | 176ns | base | 6.414 |
| popcount_std | 37ns | 30ns | 45ns | -77.02% | 27.908 |
| popcount_swar | 156ns | 145ns | 169ns | -2.55% | 6.582 |

## Performance model

- Peak throughput: **34.551 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 6.469 | 18.7% |
| popcount_std | 28.927 | 83.7% |
| popcount_swar | 6.606 | 19.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2378ns | 2378ns | base |
| popcount_std | 2268ns | 2268ns | -4.62% |
| popcount_swar | 2297ns | 2297ns | -3.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 158ns | base | --- | [156, 161] | --- | --- | --- | --- |
| popcount_std | 35ns | -120.7ns (-76.2%) | [-123, -119]ns | [35, 37] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 155ns | no significant difference | [-6, +1]ns | [153, 157] | no | 0.3382 | 0.3382 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 165ns | -76.4% | -10.1% |
| 1 | 2 | 159ns | -77.9% | +0.6% |
| 1 | 3 | 149ns | -76.5% | +5.6% |
| 1 | 4 | 155ns | -78.8% | -5.1% |
| 1 | 5 | 149ns | -76.9% | +6.2% |
| 1 | 6 | 167ns | -78.0% | -11.4% |
| 1 | 7 | 154ns | -76.9% | +3.6% |
| 1 | 8 | 169ns | -77.8% | -7.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.229 | moderate- |
| popcount_std | 0.136 | ok |
| popcount_swar | -0.491 | moderate- |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.0ns | 159.7ns | 1.9% |  |
| popcount_std | 2.6ns | 36.7ns | 7.0% | HIGH |
| popcount_swar | 3.2ns | 155.6ns | 2.0% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 146.5-176.0 ns)
    146.5 |###############
    148.0 |########################
    149.5 |###########################
    150.9 |########################################
    152.4 |####################################
    153.9 |#########
    155.4 |#################################
    156.8 |##################
    158.3 |##################
    159.8 |###########################
    161.3 |########################
    162.7 |##################
    164.2 |###############
    165.7 |###############
    167.2 |###############
    168.6 |#####################
    170.1 |###############
    171.6 |#####################
    173.1 |######
    174.5 |############
  (14 below, 10 above range)

popcount_std (n=160, range 29.6-45.4 ns)
     29.6 |#############
     30.4 |#################################
     31.2 |####################
     32.0 |####################################
     32.8 |#################################
     33.6 |##############################
     34.4 |########################################
     35.2 |#################################
     35.9 |################
     36.7 |##########################
     37.5 |##########################
     38.3 |##########
     39.1 |##########
     39.9 |##############################
     40.7 |####################
     41.5 |#############
     42.2 |##########
     43.0 |#######################
     43.8 |################
     44.6 |
  (13 below, 14 above range)

popcount_swar (n=160, range 145.0-168.8 ns)
    145.0 |######################
    146.1 |############################
    147.3 |#################
    148.5 |######################
    149.7 |###############################
    150.9 |#################
    152.1 |#########################
    153.3 |############################
    154.5 |###########
    155.7 |#########################
    156.9 |########################################
    158.0 |#########################
    159.2 |############################
    160.4 |########
    161.6 |#####
    162.8 |##
    164.0 |####################
    165.2 |###########
    166.4 |##
    167.6 |##############
  (11 below, 12 above range)

```
