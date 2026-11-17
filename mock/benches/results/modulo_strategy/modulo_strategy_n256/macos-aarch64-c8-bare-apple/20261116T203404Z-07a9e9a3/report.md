# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 132% faster than the next best (mod_npow2_const)

mod_pow2_const (353 ns) leads mod_npow2_const (820 ns) by 132%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 57% (significant)

mod_pow2_const is -468 ns (57%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.9x slower than the field

mod_var (1.04 us) is 2.9x the fastest (353 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: mod_pow2_const** at 352.9 ns median (-57.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.94x (fastest 352.9 ns, slowest 1035.8 ns)

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
| mod_npow2_const | 2935ns | 2923ns | 2916ns | 2923ns | 2992ns | base |
| mod_pow2_const | 2463ns | 2450ns | 2442ns | 2451ns | 2521ns | -16.10% |
| mod_var | 3170ns | 3156ns | 3148ns | 3157ns | 3230ns | +7.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 824ns | 817ns | 839ns | base | 0.311 |
| mod_pow2_const | 354ns | 349ns | 362ns | -57.01% | 0.723 |
| mod_var | 1044ns | 1033ns | 1081ns | +26.81% | 0.245 |

## Performance model

- Peak throughput: **0.733 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.312 | 42.6% |
| mod_pow2_const | 0.725 | 98.9% |
| mod_var | 0.247 | 33.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 2935ns | 2935ns | base |
| mod_pow2_const | 2463ns | 2463ns | -16.10% |
| mod_var | 3170ns | 3170ns | +7.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 820ns | base | --- | [820, 821] | --- | --- | --- | --- |
| mod_pow2_const | 353ns | -467.5ns (-57.0%) | [-468, -467]ns | [352, 354] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 1036ns | +215.4ns (+26.3%) | [+215, +216]ns | [1035, 1036] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 819ns | -56.9% | +26.3% |
| 1 | 2 | 821ns | -56.9% | +26.3% |
| 1 | 3 | 820ns | -57.0% | +30.8% |
| 1 | 4 | 822ns | -57.1% | +26.0% |
| 1 | 5 | 821ns | -57.0% | +26.1% |
| 1 | 6 | 820ns | -57.1% | +26.2% |
| 1 | 7 | 821ns | -57.1% | +26.1% |
| 1 | 8 | 821ns | -55.6% | +26.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.102 | ok |
| mod_pow2_const | -0.085 | ok |
| mod_var | -0.130 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 2.6ns | 823.5ns | 0.3% |  |
| mod_pow2_const | 2.3ns | 354.0ns | 0.7% |  |
| mod_var | 2.6ns | 1044.3ns | 0.2% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 816.7-839.1 ns)
    816.7 |#############################
    817.8 |#################################
    818.9 |########################################
    820.0 |########################################
    821.2 |####################################
    822.3 |######################
    823.4 |########
    824.5 |###############
    825.7 |#####
    826.8 |
    827.9 |###
    829.0 |
    830.1 |###
    831.3 |
    832.4 |
    833.5 |#
    834.6 |
    835.8 |
    836.9 |#
    838.0 |
  (10 below, 11 above range)

mod_pow2_const (n=160, range 349.2-361.7 ns)
    349.2 |######################
    349.8 |######################
    350.4 |###
    351.0 |#############
    351.7 |###########################
    352.3 |########################################
    352.9 |###############
    353.5 |########
    354.2 |#################################
    354.8 |#################
    355.4 |#
    356.0 |######
    356.7 |#####
    357.3 |#####
    357.9 |#
    358.5 |######
    359.2 |#
    359.8 |###
    360.4 |###
    361.0 |
  (8 below, 13 above range)

mod_var (n=160, range 1032.7-1080.6 ns)
   1032.7 |#####################################
   1035.1 |########################################
   1037.5 |##########
   1039.9 |#
   1042.3 |
   1044.7 |
   1047.1 |
   1049.5 |
   1051.9 |#
   1054.3 |
   1056.7 |
   1059.1 |
   1061.5 |#
   1063.9 |##
   1066.3 |
   1068.7 |
   1071.0 |#
   1073.4 |#
   1075.8 |
   1078.2 |
  (14 below, 11 above range)

```
