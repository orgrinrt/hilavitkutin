# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (4.67 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 276 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 828% faster than the next best (iter_seq)

iter_strided (276 ns) leads iter_seq (2.56 us) by 828%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 94% (significant)

iter_strided is -4.40 us (94%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 16.9x slower than the field

iter_gather (4.67 us) is 16.9x the fastest (276 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 16.9x the fastest

Fastest iter_strided (276 ns) to slowest iter_gather (4.67 us): 16.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: iter_strided** at 275.8 ns median (-94.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 16.95x (fastest 275.8 ns, slowest 4673.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Column iteration patterns: sequential vs strided vs data-dependent gather |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x3a5c8e91b2d4f607 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 6830ns | 6805ns | 6777ns | 6810ns | 6942ns | base |
| iter_seq | 4723ns | 4699ns | 4679ns | 4707ns | 4814ns | -30.85% |
| iter_strided | 2419ns | 2404ns | 2394ns | 2406ns | 2483ns | -64.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 4694ns | 4655ns | 4776ns | base | 3.490 |
| iter_seq | 2577ns | 2554ns | 2631ns | -45.09% | 6.357 |
| iter_strided | 276ns | 271ns | 283ns | -94.12% | 59.313 |

## Performance model

- Peak throughput: **60.472 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.506 | 5.8% |
| iter_seq | 6.399 | 10.6% |
| iter_strided | 59.405 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 6830ns | 6830ns | base |
| iter_seq | 4723ns | 4723ns | -30.85% |
| iter_strided | 2419ns | 2419ns | -64.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 4674ns | base | --- | [4671, 4681] | --- | --- | --- | --- |
| iter_seq | 2560ns | -2108.0ns (-45.1%) | [-2113, -2105]ns | [2559, 2567] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 276ns | -4397.8ns (-94.1%) | [-4406, -4395]ns | [275, 276] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 4671ns | -45.2% | -94.1% |
| 1 | 2 | 4671ns | -45.0% | -94.1% |
| 1 | 3 | 4670ns | -44.8% | -94.1% |
| 1 | 4 | 4693ns | -45.3% | -94.1% |
| 1 | 5 | 4692ns | -45.4% | -94.1% |
| 1 | 6 | 4670ns | -45.2% | -94.1% |
| 1 | 7 | 4667ns | -45.2% | -94.1% |
| 1 | 8 | 4670ns | -45.3% | -94.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | 0.316 | moderate+ |
| iter_seq | 0.459 | moderate+ |
| iter_strided | -0.041 | ok |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 3.1ns | 4693.9ns | 0.1% |  |
| iter_seq | 2.4ns | 2577.4ns | 0.1% |  |
| iter_strided | 2.6ns | 276.2ns | 1.0% |  |

## Distribution (algo ns)

```
iter_gather (n=160, range 4655.4-4776.3 ns)
   4655.4 |#############################
   4661.4 |############################
   4667.5 |########################################
   4673.5 |#################
   4679.6 |##############
   4685.6 |###########
   4691.6 |#######
   4697.7 |#####
   4703.7 |##
   4709.8 |#####
   4715.8 |
   4721.9 |#######
   4727.9 |#
   4734.0 |########
   4740.0 |####
   4746.1 |####
   4752.1 |####
   4758.2 |#
   4764.2 |
   4770.2 |#
  (14 below, 12 above range)

iter_seq (n=160, range 2553.8-2631.5 ns)
   2553.8 |########################################
   2557.7 |####################
   2561.5 |#####
   2565.4 |######
   2569.3 |######
   2573.2 |####
   2577.1 |###
   2581.0 |##
   2584.8 |
   2588.7 |
   2592.6 |##
   2596.5 |#######
   2600.4 |#
   2604.3 |###
   2608.2 |
   2612.0 |###
   2615.9 |
   2619.8 |#
   2623.7 |#
   2627.6 |
  (13 below, 14 above range)

iter_strided (n=160, range 270.9-282.8 ns)
    270.9 |###########
    271.5 |##################
    272.1 |#######
    272.7 |############################
    273.3 |#####################
    273.9 |#########
    274.5 |##############################
    275.1 |##################
    275.7 |########################################
    276.3 |#########
    276.9 |##############
    277.5 |################################
    278.1 |#####################
    278.7 |#########################
    279.3 |###########
    279.9 |####
    280.5 |#######
    281.1 |##
    281.6 |####
    282.2 |##
  (15 below, 8 above range)

```
