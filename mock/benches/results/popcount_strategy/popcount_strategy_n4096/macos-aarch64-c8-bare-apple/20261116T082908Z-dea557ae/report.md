# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (522 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 88 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 487% faster than the next best (popcount_swar)

popcount_std (88 ns) leads popcount_swar (519 ns) by 487%, a clear separation rather than a photo finish. CV 8.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 83% (significant)

popcount_std is -435 ns (83%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 5.9x slower than the field

popcount_handrolled (522 ns) is 5.9x the fastest (88 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 8.1%)

popcount_std wins on median (88 ns) yet has the highest variance (CV 8.1%), while popcount_handrolled is the steadiest (CV 1.0%, 522 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.9x the fastest

Fastest popcount_std (88 ns) to slowest popcount_handrolled (522 ns): 5.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_swar's edge over baseline is significant but tiny (-4 ns, 0.78%)

popcount_swar differs from baseline popcount_handrolled by -4 ns (0.78%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: popcount_std** at 88.3 ns median (-83.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.92x (fastest 88.3 ns, slowest 522.5 ns)

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
| popcount_handrolled | 2627ns | 2620ns | 2614ns | 2620ns | 2662ns | base |
| popcount_std | 2194ns | 2189ns | 2182ns | 2189ns | 2221ns | -16.49% |
| popcount_swar | 2626ns | 2618ns | 2611ns | 2619ns | 2661ns | -0.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 524ns | 518ns | 532ns | base | 7.823 |
| popcount_std | 89ns | 83ns | 98ns | -82.94% | 45.866 |
| popcount_swar | 521ns | 512ns | 535ns | -0.56% | 7.867 |

## Performance model

- Peak throughput: **49.327 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 7.839 | 15.9% |
| popcount_std | 46.387 | 94.0% |
| popcount_swar | 7.897 | 16.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2627ns | 2627ns | base |
| popcount_std | 2194ns | 2194ns | -16.49% |
| popcount_swar | 2626ns | 2626ns | -0.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 522ns | base | --- | [522, 523] | --- | --- | --- | --- |
| popcount_std | 88ns | -434.6ns (-83.2%) | [-435, -434]ns | [87, 89] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 519ns | -4.1ns (-0.8%) | [-5, -4]ns | [518, 520] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 521ns | -83.3% | -0.6% |
| 1 | 2 | 523ns | -83.1% | -0.8% |
| 1 | 3 | 522ns | -83.3% | -0.5% |
| 1 | 4 | 524ns | -83.1% | -0.7% |
| 1 | 5 | 522ns | -83.0% | -0.5% |
| 1 | 6 | 523ns | -83.3% | -1.1% |
| 1 | 7 | 523ns | -83.2% | -0.8% |
| 1 | 8 | 523ns | -82.9% | -0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.304 | moderate- |
| popcount_std | -0.190 | ok |
| popcount_swar | -0.009 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.3ns | 523.6ns | 0.6% |  |
| popcount_std | 2.4ns | 89.3ns | 2.6% |  |
| popcount_swar | 3.2ns | 520.6ns | 0.6% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 517.7-532.0 ns)
    517.7 |#####################
    518.4 |################
    519.2 |############################
    519.9 |############################
    520.6 |##############################
    521.3 |#########
    522.0 |########################################
    522.7 |##################
    523.5 |###########
    524.2 |#########################
    524.9 |################
    525.6 |#######
    526.3 |####
    527.0 |##############
    527.7 |####
    528.5 |#######
    529.2 |##
    529.9 |##############
    530.6 |###########
    531.3 |#######
  (12 below, 12 above range)

popcount_std (n=160, range 83.0-98.5 ns)
     83.0 |######################
     83.8 |#######
     84.6 |#########################
     85.4 |########################################
     86.1 |###########################
     86.9 |#################
     87.7 |##############################
     88.4 |#########################
     89.2 |####################
     90.0 |#################
     90.8 |###############
     91.5 |####################
     92.3 |#######
     93.1 |######################
     93.8 |##########
     94.6 |#######
     95.4 |##
     96.2 |##########
     96.9 |#####
     97.7 |#####
  (16 below, 9 above range)

popcount_swar (n=160, range 512.3-534.8 ns)
    512.3 |#####################
    513.5 |##################
    514.6 |#############################
    515.7 |#################################
    516.8 |#####################
    517.9 |#######################
    519.1 |########################################
    520.2 |#####################
    521.3 |########
    522.4 |##################
    523.6 |##########
    524.7 |########
    525.8 |################
    526.9 |########
    528.0 |####
    529.2 |########
    530.3 |####
    531.4 |##
    532.5 |######
    533.7 |
  (13 below, 2 above range)

```
