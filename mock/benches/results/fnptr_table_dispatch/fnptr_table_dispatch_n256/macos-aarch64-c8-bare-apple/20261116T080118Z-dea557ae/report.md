# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fpt_match_16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fpt_match_16 has the worst median (65 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest fpt_table_16 at 44 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### fpt_table_16 dominates: 47% faster than the next best (fpt_match_16)

fpt_table_16 (44 ns) leads fpt_match_16 (65 ns) by 47%, a clear separation rather than a photo finish. CV 10.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fpt_table_16 beats baseline by 33% (significant)

fpt_table_16 is -21 ns (33%) faster than baseline fpt_match_16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: fpt_table_16** at 44.2 ns median (-32.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.47x (fastest 44.2 ns, slowest 65.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fn-pointer table vs match dispatch for 16-variant hot loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x478ab261e0d9f352 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 2174ns | 2163ns | 2156ns | 2164ns | 2223ns | base |
| fpt_table_16 | 2164ns | 2142ns | 2135ns | 2144ns | 2253ns | -0.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 67ns | 62ns | 77ns | base | 3.831 |
| fpt_table_16 | 45ns | 39ns | 51ns | -33.39% | 5.752 |

## Performance model

- Peak throughput: **6.546 Gops/s** (fpt_table_16; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 3.938 | 60.2% |
| fpt_table_16 | 5.792 | 88.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2174ns | 2174ns | base |
| fpt_table_16 | 2164ns | 2164ns | -0.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 65ns | base | --- | [65, 65] | --- | --- | --- | --- |
| fpt_table_16 | 44ns | -21.3ns (-32.8%) | [-22, -21]ns | [43, 45] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 65ns | -32.5% |
| 1 | 2 | 65ns | -32.8% |
| 1 | 3 | 64ns | -31.7% |
| 1 | 4 | 65ns | -30.2% |
| 1 | 5 | 66ns | -31.5% |
| 1 | 6 | 65ns | -31.7% |
| 1 | 7 | 66ns | -31.2% |
| 1 | 8 | 65ns | -34.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.002 | ok |
| fpt_table_16 | -0.111 | ok |

**Consistency summary:**

- **fpt_table_16**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 3.0ns | 66.8ns | 4.4% |  |
| fpt_table_16 | 2.4ns | 44.5ns | 5.4% | HIGH |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 61.9-76.6 ns)
     61.9 |##############
     62.6 |###################################
     63.4 |###########
     64.1 |##############################
     64.8 |########################################
     65.6 |#########################
     66.3 |#########
     67.0 |################
     67.8 |###################
     68.5 |###
     69.2 |###
     70.0 |######
     70.7 |####
     71.4 |######
     72.2 |####
     72.9 |
     73.6 |#
     74.4 |
     75.1 |#
     75.8 |
  (12 below, 2 above range)

fpt_table_16 (n=160, range 39.1-50.9 ns)
     39.1 |############
     39.7 |##########
     40.3 |########
     40.9 |############
     41.5 |################
     42.0 |#####################
     42.6 |##########
     43.2 |########################################
     43.8 |########
     44.4 |##################
     45.0 |###############################
     45.6 |################
     46.2 |#######################
     46.7 |######
     47.3 |#####################
     47.9 |##########
     48.5 |######
     49.1 |########
     49.7 |
     50.3 |####
  (14 below, 9 above range)

```
