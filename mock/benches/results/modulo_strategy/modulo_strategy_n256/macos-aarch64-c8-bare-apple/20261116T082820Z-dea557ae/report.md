# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 133% faster than the next best (mod_npow2_const)

mod_pow2_const (359 ns) leads mod_npow2_const (834 ns) by 133%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 57% (significant)

mod_pow2_const is -475 ns (57%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.9x slower than the field

mod_var (1.06 us) is 2.9x the fastest (359 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### mod_pow2_const shows alternating (throttle bounce) (autocorr -0.62)

mod_pow2_const's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: mod_pow2_const** at 358.7 ns median (-57.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.94x (fastest 358.7 ns, slowest 1056.2 ns)

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
| mod_npow2_const | 2966ns | 2962ns | 2954ns | 2962ns | 2991ns | base |
| mod_pow2_const | 2486ns | 2481ns | 2475ns | 2481ns | 2509ns | -16.20% |
| mod_var | 3209ns | 3201ns | 3191ns | 3204ns | 3242ns | +8.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 836ns | 832ns | 843ns | base | 0.306 |
| mod_pow2_const | 360ns | 355ns | 367ns | -56.96% | 0.712 |
| mod_var | 1064ns | 1053ns | 1090ns | +27.29% | 0.241 |

## Performance model

- Peak throughput: **0.721 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.307 | 42.6% |
| mod_pow2_const | 0.714 | 99.0% |
| mod_var | 0.242 | 33.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 2966ns | 2966ns | base |
| mod_pow2_const | 2486ns | 2486ns | -16.20% |
| mod_var | 3209ns | 3209ns | +8.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 834ns | base | --- | [834, 835] | --- | --- | --- | --- |
| mod_pow2_const | 359ns | -475.4ns (-57.0%) | [-476, -475]ns | [358, 360] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 1056ns | +221.7ns (+26.6%) | [+221, +223]ns | [1056, 1057] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 836ns | -57.2% | +26.3% |
| 1 | 2 | 834ns | -56.7% | +26.6% |
| 1 | 3 | 833ns | -57.0% | +26.6% |
| 1 | 4 | 834ns | -56.8% | +26.6% |
| 1 | 5 | 835ns | -57.0% | +29.9% |
| 1 | 6 | 834ns | -57.1% | +26.5% |
| 1 | 7 | 834ns | -57.0% | +29.6% |
| 1 | 8 | 834ns | -57.0% | +26.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | 0.039 | ok |
| mod_pow2_const | -0.619 | HIGH- (thermal bounce) |
| mod_var | -0.366 | moderate- |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 2.9ns | 835.6ns | 0.3% |  |
| mod_pow2_const | 2.6ns | 359.6ns | 0.7% |  |
| mod_var | 2.9ns | 1063.6ns | 0.3% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 831.6-843.4 ns)
    831.6 |#############################
    832.2 |##################
    832.8 |########################################
    833.4 |###########################
    834.0 |##################
    834.5 |##############################
    835.1 |#######################
    835.7 |#########################
    836.3 |#######
    836.9 |#########
    837.5 |############
    838.1 |###
    838.7 |###
    839.2 |###
    839.8 |
    840.4 |
    841.0 |
    841.6 |#
    842.2 |
    842.8 |
  (12 below, 8 above range)

mod_pow2_const (n=160, range 355.0-366.6 ns)
    355.0 |#################
    355.6 |###
    356.1 |########################################
    356.7 |###########
    357.3 |#############
    357.9 |####################################
    358.5 |###########
    359.0 |############################
    359.6 |#############
    360.2 |###########
    360.8 |######################
    361.4 |###########
    362.0 |#################
    362.5 |###
    363.1 |#########
    363.7 |#####
    364.3 |
    364.9 |#######
    365.5 |###
    366.0 |#
  (13 below, 5 above range)

mod_var (n=160, range 1052.6-1089.9 ns)
   1052.6 |####################
   1054.4 |########################################
   1056.3 |##############
   1058.2 |##########
   1060.0 |
   1061.9 |
   1063.8 |
   1065.6 |
   1067.5 |
   1069.4 |
   1071.2 |
   1073.1 |###
   1075.0 |###
   1076.9 |##
   1078.7 |###
   1080.6 |#####
   1082.5 |####
   1084.3 |#####
   1086.2 |#
   1088.1 |###
  (13 below, 4 above range)

```
