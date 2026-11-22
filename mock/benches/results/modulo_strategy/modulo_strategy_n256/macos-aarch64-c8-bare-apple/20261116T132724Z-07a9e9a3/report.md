# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 133% faster than the next best (mod_npow2_const)

mod_pow2_const (352 ns) leads mod_npow2_const (821 ns) by 133%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 57% (significant)

mod_pow2_const is -469 ns (57%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.9x slower than the field

mod_var (1.04 us) is 2.9x the fastest (352 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: mod_pow2_const** at 352.1 ns median (-57.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.94x (fastest 352.1 ns, slowest 1036.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv) |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4dd582c1e7fa3b96 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mod_npow2_const | 2936ns | 2925ns | 2918ns | 2925ns | 2987ns | base |
| mod_pow2_const | 2450ns | 2448ns | 2442ns | 2448ns | 2467ns | -16.54% |
| mod_var | 3161ns | 3154ns | 3146ns | 3157ns | 3188ns | +7.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 824ns | 817ns | 841ns | base | 0.311 |
| mod_pow2_const | 353ns | 349ns | 357ns | -57.22% | 0.726 |
| mod_var | 1042ns | 1033ns | 1064ns | +26.46% | 0.246 |

## Performance model

- Peak throughput: **0.734 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.312 | 42.5% |
| mod_pow2_const | 0.727 | 99.1% |
| mod_var | 0.247 | 33.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 2936ns | 2936ns | base |
| mod_pow2_const | 2450ns | 2450ns | -16.54% |
| mod_var | 3161ns | 3161ns | +7.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 821ns | base | --- | [820, 821] | --- | --- | --- | --- |
| mod_pow2_const | 352ns | -468.8ns (-57.1%) | [-470, -468]ns | [352, 352] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 1037ns | +216.3ns (+26.4%) | [+216, +217]ns | [1036, 1037] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 821ns | -57.1% | +26.2% |
| 1 | 2 | 821ns | -57.1% | +26.2% |
| 1 | 3 | 820ns | -57.2% | +26.3% |
| 1 | 4 | 821ns | -56.9% | +29.5% |
| 1 | 5 | 821ns | -57.1% | +26.1% |
| 1 | 6 | 821ns | -57.1% | +29.3% |
| 1 | 7 | 821ns | -57.1% | +26.2% |
| 1 | 8 | 820ns | -57.2% | +26.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.145 | ok |
| mod_pow2_const | -0.499 | moderate- |
| mod_var | -0.407 | moderate- |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 2.6ns | 824.1ns | 0.3% |  |
| mod_pow2_const | 2.4ns | 352.6ns | 0.7% |  |
| mod_var | 2.7ns | 1042.2ns | 0.3% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 817.1-841.2 ns)
    817.1 |###########################
    818.3 |################################
    819.5 |########################################
    820.7 |#####################################
    821.9 |###############################
    823.1 |###############
    824.3 |###########
    825.5 |##
    826.7 |##
    827.9 |##
    829.1 |#
    830.3 |#####
    831.5 |
    832.7 |
    833.9 |
    835.1 |
    836.3 |
    837.5 |
    838.8 |
    840.0 |
  (8 below, 4 above range)

mod_pow2_const (n=160, range 349.0-357.1 ns)
    349.0 |################
    349.4 |####################################
    349.8 |####################
    350.2 |##############################
    350.6 |########################################
    351.0 |#######################
    351.4 |########################################
    351.8 |####################################
    352.2 |##########################
    352.6 |###
    353.0 |##########################
    353.5 |####################
    353.9 |#############
    354.3 |####################
    354.7 |#######################
    355.1 |#############
    355.5 |#######################
    355.9 |###
    356.3 |##########
    356.7 |##########
  (15 below, 14 above range)

mod_var (n=160, range 1032.9-1064.1 ns)
   1032.9 |####################
   1034.4 |######################################
   1036.0 |########################################
   1037.5 |#######
   1039.1 |########
   1040.7 |#
   1042.2 |##
   1043.8 |#
   1045.4 |#
   1046.9 |##
   1048.5 |
   1050.1 |
   1051.6 |
   1053.2 |#
   1054.7 |#
   1056.3 |
   1057.9 |####
   1059.4 |####
   1061.0 |#######
   1062.6 |###
  (13 below, 16 above range)

```
