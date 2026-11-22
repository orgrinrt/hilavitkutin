# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fpt_match_16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fpt_match_16 has the worst median (63 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest fpt_table_16 at 42 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### fpt_table_16 dominates: 52% faster than the next best (fpt_match_16)

fpt_table_16 (42 ns) leads fpt_match_16 (63 ns) by 52%, a clear separation rather than a photo finish. CV 9.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fpt_table_16 beats baseline by 34% (significant)

fpt_table_16 is -22 ns (34%) faster than baseline fpt_match_16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fpt_table_16 is fastest but the noisiest (CV 9.0%)

fpt_table_16 wins on median (42 ns) yet has the highest variance (CV 9.0%), while fpt_match_16 is the steadiest (CV 4.5%, 63 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: fpt_table_16** at 41.7 ns median (-34.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.52x (fastest 41.7 ns, slowest 63.3 ns)

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
| fpt_match_16 | 2138ns | 2134ns | 2129ns | 2134ns | 2158ns | base |
| fpt_table_16 | 2118ns | 2113ns | 2107ns | 2114ns | 2141ns | -0.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 63ns | 59ns | 67ns | base | 4.040 |
| fpt_table_16 | 42ns | 36ns | 47ns | -34.39% | 6.156 |

## Performance model

- Peak throughput: **7.070 Gops/s** (fpt_table_16; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 4.044 | 57.2% |
| fpt_table_16 | 6.139 | 86.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2138ns | 2138ns | base |
| fpt_table_16 | 2118ns | 2118ns | -0.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 63ns | base | --- | [63, 64] | --- | --- | --- | --- |
| fpt_table_16 | 42ns | -21.7ns (-34.3%) | [-22, -21]ns | [41, 42] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 64ns | -34.0% |
| 1 | 2 | 63ns | -32.1% |
| 1 | 3 | 62ns | -33.8% |
| 1 | 4 | 62ns | -35.0% |
| 1 | 5 | 65ns | -35.3% |
| 1 | 6 | 64ns | -34.0% |
| 1 | 7 | 63ns | -34.1% |
| 1 | 8 | 65ns | -35.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.025 | ok |
| fpt_table_16 | 0.057 | ok |

**Consistency summary:**

- **fpt_table_16**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 2.9ns | 63.4ns | 4.6% |  |
| fpt_table_16 | 2.0ns | 41.6ns | 4.9% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 59.3-67.3 ns)
     59.3 |####################
     59.7 |###########
     60.1 |########
     60.5 |#################
     60.9 |########
     61.3 |#################
     61.7 |##################################
     62.1 |########################################
     62.5 |############################
     62.9 |######################
     63.3 |
     63.7 |#########################
     64.1 |#########################
     64.5 |###############################
     64.9 |###############################
     65.3 |####################
     65.7 |####################
     66.1 |#####
     66.5 |##############
     66.9 |#####
  (11 below, 13 above range)

fpt_table_16 (n=160, range 36.2-46.7 ns)
     36.2 |
     36.7 |#########
     37.3 |#######
     37.8 |#######
     38.3 |#######################
     38.8 |################
     39.3 |##############
     39.9 |###########
     40.4 |##############################
     40.9 |#########
     41.4 |#####################
     42.0 |#######################
     42.5 |########################################
     43.0 |##############
     43.5 |###########
     44.0 |###########
     44.6 |#####################################
     45.1 |#########
     45.6 |
     46.1 |
  (17 below, 16 above range)

```
