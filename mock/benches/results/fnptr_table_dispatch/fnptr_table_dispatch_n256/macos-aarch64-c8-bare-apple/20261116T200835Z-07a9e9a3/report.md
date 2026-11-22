# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fpt_match_16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fpt_match_16 has the worst median (64 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest fpt_table_16 at 42 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### fpt_table_16 dominates: 53% faster than the next best (fpt_match_16)

fpt_table_16 (42 ns) leads fpt_match_16 (64 ns) by 53%, a clear separation rather than a photo finish. CV 8.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fpt_table_16 beats baseline by 35% (significant)

fpt_table_16 is -22 ns (35%) faster than baseline fpt_match_16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fpt_table_16 is fastest but the noisiest (CV 8.4%)

fpt_table_16 wins on median (42 ns) yet has the highest variance (CV 8.4%), while fpt_match_16 is the steadiest (CV 4.6%, 64 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: fpt_table_16** at 41.7 ns median (-34.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.53x (fastest 41.7 ns, slowest 63.8 ns)

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
| fpt_match_16 | 2146ns | 2135ns | 2127ns | 2137ns | 2189ns | base |
| fpt_table_16 | 2123ns | 2115ns | 2109ns | 2115ns | 2159ns | -1.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 64ns | 60ns | 68ns | base | 4.026 |
| fpt_table_16 | 42ns | 37ns | 47ns | -34.45% | 6.141 |

## Performance model

- Peak throughput: **6.917 Gops/s** (fpt_table_16; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 4.013 | 58.0% |
| fpt_table_16 | 6.139 | 88.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2146ns | 2146ns | base |
| fpt_table_16 | 2123ns | 2123ns | -1.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 64ns | base | --- | [63, 64] | --- | --- | --- | --- |
| fpt_table_16 | 42ns | -22.1ns (-34.6%) | [-22, -22]ns | [41, 42] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 65ns | -37.5% |
| 1 | 2 | 64ns | -33.8% |
| 1 | 3 | 62ns | -34.7% |
| 1 | 4 | 64ns | -34.3% |
| 1 | 5 | 64ns | -33.6% |
| 1 | 6 | 63ns | -34.8% |
| 1 | 7 | 64ns | -36.4% |
| 1 | 8 | 64ns | -33.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.063 | ok |
| fpt_table_16 | -0.366 | moderate- |

**Consistency summary:**

- **fpt_table_16**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 3.1ns | 63.6ns | 4.9% |  |
| fpt_table_16 | 2.0ns | 41.7ns | 4.8% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 59.5-67.7 ns)
     59.5 |####
     59.9 |################
     60.3 |#########
     60.7 |#########
     61.2 |##############
     61.6 |####
     62.0 |#####################
     62.4 |#####################
     62.8 |################
     63.2 |############################
     63.6 |########################################
     64.0 |############################
     64.4 |############################
     64.8 |#########################
     65.2 |##############
     65.6 |###########
     66.0 |####
     66.5 |#########
     66.9 |#########
     67.3 |
  (12 below, 13 above range)

fpt_table_16 (n=160, range 37.0-46.9 ns)
     37.0 |#####################
     37.5 |########
     38.0 |####
     38.5 |############
     39.0 |##########
     39.5 |##################
     40.0 |########################################
     40.5 |##############
     41.0 |##############
     41.5 |#########################
     42.0 |############
     42.5 |###########################
     43.0 |#######################
     43.5 |########
     43.9 |##########
     44.4 |########
     44.9 |##########
     45.4 |##
     45.9 |####
     46.4 |####
  (10 below, 16 above range)

```
