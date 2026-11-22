# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (4.59 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 270 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 829% faster than the next best (iter_seq)

iter_strided (270 ns) leads iter_seq (2.51 us) by 829%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 94% (significant)

iter_strided is -4.32 us (94%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 17.0x slower than the field

iter_gather (4.59 us) is 17.0x the fastest (270 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 17.0x the fastest

Fastest iter_strided (270 ns) to slowest iter_gather (4.59 us): 17.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: iter_strided** at 270.2 ns median (-94.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 17.00x (fastest 270.2 ns, slowest 4594.2 ns)

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
| iter_gather | 6700ns | 6689ns | 6676ns | 6690ns | 6752ns | base |
| iter_seq | 4643ns | 4609ns | 4602ns | 4612ns | 4776ns | -30.70% |
| iter_strided | 2387ns | 2369ns | 2362ns | 2370ns | 2466ns | -64.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 4601ns | 4582ns | 4638ns | base | 3.561 |
| iter_seq | 2527ns | 2504ns | 2599ns | -45.09% | 6.485 |
| iter_strided | 272ns | 266ns | 282ns | -94.09% | 60.270 |

## Performance model

- Peak throughput: **61.637 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.566 | 5.8% |
| iter_seq | 6.530 | 10.6% |
| iter_strided | 60.637 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 6700ns | 6700ns | base |
| iter_seq | 4643ns | 4643ns | -30.70% |
| iter_strided | 2387ns | 2387ns | -64.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 4594ns | base | --- | [4592, 4597] | --- | --- | --- | --- |
| iter_seq | 2509ns | -2083.0ns (-45.3%) | [-2086, -2081]ns | [2509, 2510] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 270ns | -4324.4ns (-94.1%) | [-4327, -4322]ns | [270, 271] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 4595ns | -45.4% | -94.1% |
| 1 | 2 | 4599ns | -45.4% | -94.1% |
| 1 | 3 | 4591ns | -45.4% | -94.1% |
| 1 | 4 | 4604ns | -42.8% | -93.9% |
| 1 | 5 | 4598ns | -45.4% | -94.1% |
| 1 | 6 | 4591ns | -45.4% | -94.1% |
| 1 | 7 | 4594ns | -45.4% | -94.1% |
| 1 | 8 | 4592ns | -45.4% | -94.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.272 | moderate- |
| iter_seq | -0.171 | ok |
| iter_strided | -0.089 | ok |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 3.2ns | 4601.1ns | 0.1% |  |
| iter_seq | 3.1ns | 2526.6ns | 0.1% |  |
| iter_strided | 2.9ns | 271.8ns | 1.1% |  |

## Distribution (algo ns)

```
iter_gather (n=160, range 4581.8-4638.0 ns)
   4581.8 |##########
   4584.6 |############################
   4587.4 |####################################
   4590.2 |########################################
   4593.0 |##############################
   4595.8 |################
   4598.6 |##################################
   4601.5 |##########################
   4604.3 |################
   4607.1 |######
   4609.9 |######
   4612.7 |##
   4615.5 |##
   4618.3 |
   4621.2 |##
   4624.0 |##
   4626.8 |####
   4629.6 |
   4632.4 |##
   4635.2 |####
  (15 below, 12 above range)

iter_seq (n=160, range 2504.1-2599.5 ns)
   2504.1 |########################################
   2508.9 |################################
   2513.6 |###
   2518.4 |#
   2523.2 |
   2527.9 |#
   2532.7 |##
   2537.5 |
   2542.2 |
   2547.0 |
   2551.8 |##
   2556.6 |
   2561.3 |
   2566.1 |
   2570.9 |
   2575.6 |
   2580.4 |
   2585.2 |
   2589.9 |#
   2594.7 |#
  (13 below, 16 above range)

iter_strided (n=160, range 265.8-282.5 ns)
    265.8 |###
    266.6 |#####################
    267.5 |####################################
    268.3 |######
    269.1 |#######################
    270.0 |########################################
    270.8 |###############
    271.6 |#####################
    272.5 |##################
    273.3 |#############
    274.1 |###
    275.0 |#####
    275.8 |###
    276.6 |#
    277.5 |###
    278.3 |#
    279.1 |#
    280.0 |#
    280.8 |###
    281.6 |
  (14 below, 11 above range)

```
