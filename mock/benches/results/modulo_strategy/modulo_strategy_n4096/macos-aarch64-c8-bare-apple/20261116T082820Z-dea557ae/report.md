# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 118% faster than the next best (mod_npow2_const)

mod_pow2_const (6.47 us) leads mod_npow2_const (14.09 us) by 118%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 55% (significant)

mod_pow2_const is -7.70 us (55%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.8x slower than the field

mod_var (17.88 us) is 2.8x the fastest (6.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: mod_pow2_const** at 6466.7 ns median (-54.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.76x (fastest 6466.7 ns, slowest 17877.2 ns)

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
| mod_npow2_const | 16315ns | 16190ns | 16125ns | 16255ns | 16688ns | base |
| mod_pow2_const | 8578ns | 8598ns | 8447ns | 8592ns | 8667ns | -47.43% |
| mod_var | 20160ns | 19998ns | 19920ns | 20085ns | 20625ns | +23.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 14193ns | 14035ns | 14504ns | base | 0.289 |
| mod_pow2_const | 6451ns | 6351ns | 6515ns | -54.55% | 0.635 |
| mod_var | 18028ns | 17816ns | 18444ns | +27.02% | 0.227 |

## Performance model

- Peak throughput: **0.645 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.291 | 45.1% |
| mod_pow2_const | 0.633 | 98.2% |
| mod_var | 0.229 | 35.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 16315ns | 16315ns | base |
| mod_pow2_const | 8578ns | 8578ns | -47.43% |
| mod_var | 20160ns | 20160ns | +23.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 14092ns | base | --- | [14082, 14146] | --- | --- | --- | --- |
| mod_pow2_const | 6467ns | -7695.6ns (-54.6%) | [-7740, -7677]ns | [6466, 6468] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 17877ns | +3810.2ns (+27.0%) | [+3794, +3827]ns | [17864, 17928] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 14175ns | -54.4% | +27.5% |
| 1 | 2 | 14063ns | -54.4% | +26.8% |
| 1 | 3 | 14071ns | -54.6% | +26.9% |
| 1 | 4 | 14060ns | -54.0% | +27.4% |
| 1 | 5 | 14069ns | -54.0% | +28.7% |
| 1 | 6 | 14551ns | -55.5% | +27.0% |
| 1 | 7 | 14143ns | -54.3% | +26.3% |
| 1 | 8 | 14085ns | -54.1% | +26.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.081 | ok |
| mod_pow2_const | 0.274 | moderate+ |
| mod_var | 0.049 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 2.8ns | 14193.3ns | 0.0% |  |
| mod_pow2_const | 2.7ns | 6450.5ns | 0.0% |  |
| mod_var | 3.0ns | 18027.8ns | 0.0% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 14035.1-14503.6 ns)
  14035.1 |########################################
  14058.5 |##########
  14082.0 |##############
  14105.4 |###
  14128.8 |#####
  14152.2 |#
  14175.7 |###
  14199.1 |
  14222.5 |#
  14245.9 |####
  14269.4 |###
  14292.8 |###############
  14316.2 |#
  14339.6 |###
  14363.1 |
  14386.5 |##
  14409.9 |
  14433.3 |
  14456.8 |
  14480.2 |
  (12 below, 21 above range)

mod_pow2_const (n=160, range 6350.7-6515.5 ns)
   6350.7 |
   6359.0 |#
   6367.2 |
   6375.5 |
   6383.7 |
   6391.9 |
   6400.2 |#
   6408.4 |###
   6416.6 |
   6424.9 |
   6433.1 |
   6441.4 |
   6449.6 |
   6457.8 |###############
   6466.1 |########################################
   6474.3 |###
   6482.5 |
   6490.8 |#
   6499.0 |
   6507.3 |#
  (27 below, 16 above range)

mod_var (n=160, range 17816.1-18444.0 ns)
  17816.1 |########################################
  17847.5 |##############################
  17878.9 |#######
  17910.3 |#####
  17941.7 |##
  17973.1 |##
  18004.5 |###
  18035.9 |#
  18067.3 |##
  18098.7 |##
  18130.1 |########
  18161.5 |#######
  18192.9 |###
  18224.2 |#######
  18255.6 |##
  18287.0 |##
  18318.4 |
  18349.8 |#
  18381.2 |#
  18412.6 |##########
  (16 below, 14 above range)

```
