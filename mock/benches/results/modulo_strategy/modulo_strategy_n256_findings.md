# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 137% faster than the next best (mod_npow2_const)

mod_pow2_const (370 ns) leads mod_npow2_const (877 ns) by 137%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 57% (significant)

mod_pow2_const is -503 ns (57%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.9x slower than the field

mod_var (1.08 us) is 2.9x the fastest (370 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: mod_pow2_const** at 370.4 ns median (-57.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.92x (fastest 370.4 ns, slowest 1080.7 ns)

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
| mod_npow2_const | 3077ns | 3113ns | 2947ns | 3087ns | 3175ns | base |
| mod_pow2_const | 2561ns | 2593ns | 2440ns | 2563ns | 2676ns | -16.76% |
| mod_var | 3292ns | 3280ns | 3153ns | 3288ns | 3441ns | +6.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 867ns | 830ns | 900ns | base | 0.295 |
| mod_pow2_const | 369ns | 352ns | 386ns | -57.44% | 0.694 |
| mod_var | 1085ns | 1035ns | 1141ns | +25.09% | 0.236 |

## Performance model

- Peak throughput: **0.727 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.292 | 40.2% |
| mod_pow2_const | 0.691 | 95.1% |
| mod_var | 0.237 | 32.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 3077ns | 3077ns | base |
| mod_pow2_const | 2561ns | 2561ns | -16.76% |
| mod_var | 3292ns | 3292ns | +6.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 877ns | base | --- | [868, 879] | --- | --- | --- | --- |
| mod_pow2_const | 370ns | -503.4ns (-57.4%) | [-506, -492]ns | [365, 375] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 1081ns | +212.9ns (+24.3%) | [+193, +225]ns | [1072, 1097] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 878ns | -58.0% | +23.4% |
| 1 | 2 | 881ns | -59.6% | +25.9% |
| 1 | 3 | 880ns | -58.9% | +22.1% |
| 1 | 4 | 851ns | -55.3% | +30.2% |
| 1 | 5 | 881ns | -57.2% | +22.2% |
| 1 | 6 | 821ns | -56.4% | +34.8% |
| 1 | 7 | 878ns | -59.0% | +21.7% |
| 1 | 8 | 850ns | -55.6% | +21.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.429 | moderate- |
| mod_pow2_const | 0.010 | ok |
| mod_var | -0.070 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 2.6ns | 867.3ns | 0.3% |  |
| mod_pow2_const | 2.5ns | 369.1ns | 0.7% |  |
| mod_var | 2.9ns | 1084.8ns | 0.3% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 829.7-900.2 ns)
    829.7 |
    833.2 |##
    836.7 |
    840.2 |#
    843.8 |########
    847.3 |####################
    850.8 |######
    854.3 |######
    857.9 |##
    861.4 |
    864.9 |####
    868.4 |##
    872.0 |#####
    875.5 |#######################
    879.0 |########################################
    882.5 |###########
    886.1 |
    889.6 |
    893.1 |
    896.7 |
  (20 below, 10 above range)

mod_pow2_const (n=160, range 352.2-386.1 ns)
    352.2 |###################################
    353.9 |############
    355.6 |####################
    357.3 |#################
    359.0 |#######
    360.7 |############
    362.4 |###############
    364.1 |#################
    365.8 |#####
    367.5 |############
    369.2 |##########
    370.8 |#######
    372.5 |#######
    374.2 |###################################
    375.9 |########################################
    377.6 |###########################
    379.3 |################################
    381.0 |####################
    382.7 |#####
    384.4 |
  (15 below, 9 above range)

mod_var (n=160, range 1035.0-1141.1 ns)
   1035.0 |####################
   1040.3 |#########
   1045.6 |#####
   1050.9 |#######
   1056.2 |#####
   1061.5 |######
   1066.8 |################
   1072.1 |##########
   1077.4 |#####
   1082.7 |###
   1088.0 |##
   1093.3 |#######
   1098.6 |###
   1103.9 |########################
   1109.2 |########################################
   1114.6 |#####
   1119.9 |#
   1125.2 |
   1130.5 |
   1135.8 |
  (15 below, 9 above range)

```
