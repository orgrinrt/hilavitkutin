# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 13% faster than the next best (fpt_table_16)

fpt_match_16 (10.09 us) leads fpt_table_16 (11.38 us) by 13%, a clear separation rather than a photo finish. CV 9.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (10.09 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 10085.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.13x (fastest 10085.2 ns, slowest 11376.0 ns)

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
| fpt_match_16 | 12214ns | 12226ns | 10799ns | 12300ns | 13369ns | base |
| fpt_table_16 | 13179ns | 13502ns | 11079ns | 13495ns | 14333ns | +7.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 10077ns | 8659ns | 11235ns | base | 1.626 |
| fpt_table_16 | 11040ns | 8942ns | 12171ns | +9.56% | 1.484 |

## Performance model

- Peak throughput: **1.892 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 1.625 | 85.9% |
| fpt_table_16 | 1.440 | 76.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 12214ns | 12214ns | base |
| fpt_table_16 | 13179ns | 13179ns | +7.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 10085ns | base | --- | [9955, 10211] | --- | --- | --- | --- |
| fpt_table_16 | 11376ns | +1120.6ns (+11.1%) | [+1016, +1182]ns | [11247, 11594] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 9991ns | +8.3% |
| 1 | 2 | 10244ns | +8.9% |
| 1 | 3 | 8716ns | +29.6% |
| 1 | 4 | 10170ns | +2.8% |
| 1 | 5 | 9957ns | +16.4% |
| 1 | 6 | 9709ns | +10.6% |
| 1 | 7 | 11219ns | +6.6% |
| 1 | 8 | 10958ns | +10.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | 0.065 | ok |
| fpt_table_16 | -0.069 | ok |

**Consistency summary:**

- **fpt_table_16**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 3.0ns | 10076.5ns | 0.0% |  |
| fpt_table_16 | 3.4ns | 11039.6ns | 0.0% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 8658.7-11234.9 ns)
   8658.7 |####
   8787.5 |##
   8916.3 |
   9045.1 |####
   9173.9 |######
   9302.8 |##############
   9431.6 |########
   9560.4 |#######################
   9689.2 |#######################
   9818.0 |##################
   9946.8 |############
  10075.6 |########################################
  10204.4 |##############
  10333.2 |##########
  10462.0 |######
  10590.8 |####
  10719.6 |############
  10848.4 |############
  10977.3 |#########################
  11106.1 |#################################
  (18 below, 10 above range)

fpt_table_16 (n=160, range 8942.3-12171.4 ns)
   8942.3 |####
   9103.8 |##
   9265.2 |####
   9426.7 |##
   9588.1 |######
   9749.6 |
   9911.0 |##
  10072.5 |##
  10233.9 |##########
  10395.4 |##############
  10556.8 |############
  10718.3 |############
  10879.8 |####
  11041.2 |########################
  11202.7 |##############################
  11364.1 |########
  11525.6 |################################
  11687.0 |####################
  11848.5 |####################################
  12009.9 |########################################
  (16 below, 12 above range)

```
