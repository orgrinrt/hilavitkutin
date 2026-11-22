# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (514 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 85 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 497% faster than the next best (popcount_swar)

popcount_std (85 ns) leads popcount_swar (510 ns) by 497%, a clear separation rather than a photo finish. CV 6.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 83% (significant)

popcount_std is -427 ns (83%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 6.0x slower than the field

popcount_handrolled (514 ns) is 6.0x the fastest (85 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 6.0%)

popcount_std wins on median (85 ns) yet has the highest variance (CV 6.0%), while popcount_handrolled is the steadiest (CV 1.6%, 514 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 6.0x the fastest

Fastest popcount_std (85 ns) to slowest popcount_handrolled (514 ns): 6.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_swar's edge over baseline is significant but tiny (-4 ns, 0.80%)

popcount_swar differs from baseline popcount_handrolled by -4 ns (0.80%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: popcount_std** at 85.4 ns median (-83.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 6.02x (fastest 85.4 ns, slowest 513.8 ns)

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
| popcount_handrolled | 2602ns | 2585ns | 2578ns | 2586ns | 2673ns | base |
| popcount_std | 2166ns | 2159ns | 2153ns | 2159ns | 2198ns | -16.75% |
| popcount_swar | 2600ns | 2583ns | 2577ns | 2584ns | 2670ns | -0.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 515ns | 506ns | 529ns | base | 7.950 |
| popcount_std | 86ns | 80ns | 94ns | -83.23% | 47.409 |
| popcount_swar | 511ns | 502ns | 525ns | -0.78% | 8.012 |

## Performance model

- Peak throughput: **51.188 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 7.972 | 15.6% |
| popcount_std | 47.963 | 93.7% |
| popcount_swar | 8.038 | 15.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2602ns | 2602ns | base |
| popcount_std | 2166ns | 2166ns | -16.75% |
| popcount_swar | 2600ns | 2600ns | -0.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 514ns | base | --- | [512, 515] | --- | --- | --- | --- |
| popcount_std | 85ns | -426.7ns (-83.0%) | [-428, -426]ns | [85, 87] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 510ns | -4.1ns (-0.8%) | [-5, -3]ns | [508, 511] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 512ns | -83.5% | -0.7% |
| 1 | 2 | 514ns | -83.4% | -0.9% |
| 1 | 3 | 515ns | -83.4% | -1.5% |
| 1 | 4 | 510ns | -83.3% | -0.5% |
| 1 | 5 | 511ns | -83.3% | +2.6% |
| 1 | 6 | 530ns | -83.8% | -3.4% |
| 1 | 7 | 513ns | -83.1% | -0.8% |
| 1 | 8 | 514ns | -82.9% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.216 | moderate- |
| popcount_std | 0.384 | moderate+ |
| popcount_swar | -0.001 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 7/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.2ns | 515.2ns | 0.6% |  |
| popcount_std | 2.1ns | 86.4ns | 2.5% |  |
| popcount_swar | 3.0ns | 511.2ns | 0.6% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 506.2-528.8 ns)
    506.2 |##
    507.4 |#########################
    508.5 |##############################
    509.6 |########################################
    510.8 |######################
    511.9 |####################
    513.0 |#################
    514.1 |################################
    515.3 |###################################
    516.4 |#########################
    517.5 |#####
    518.7 |##########
    519.8 |##########
    520.9 |#######
    522.0 |##
    523.2 |#####
    524.3 |############
    525.4 |#######
    526.6 |#####
    527.7 |
  (18 below, 16 above range)

popcount_std (n=160, range 80.0-93.9 ns)
     80.0 |###############
     80.7 |###########################
     81.4 |############
     82.1 |############
     82.8 |###############
     83.5 |#################
     84.2 |###################################
     84.9 |########################################
     85.6 |##########
     86.3 |#####
     87.0 |######################
     87.7 |######################
     88.4 |############
     89.1 |######################
     89.8 |##########
     90.4 |#################
     91.1 |####################
     91.8 |##########
     92.5 |#####
     93.2 |##########
  (14 below, 9 above range)

popcount_swar (n=160, range 501.8-525.3 ns)
    501.8 |#################################
    503.0 |#####################
    504.2 |#################################
    505.3 |########################
    506.5 |####################################
    507.7 |#################################
    508.9 |####################################
    510.0 |##############################
    511.2 |###########################
    512.4 |########################################
    513.5 |######
    514.7 |############
    515.9 |######
    517.1 |
    518.2 |######
    519.4 |############
    520.6 |##################
    521.8 |###
    522.9 |#########
    524.1 |###
  (14 below, 17 above range)

```
