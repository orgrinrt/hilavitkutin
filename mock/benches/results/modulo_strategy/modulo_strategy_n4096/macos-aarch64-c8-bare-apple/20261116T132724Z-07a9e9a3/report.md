# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 121% faster than the next best (mod_npow2_const)

mod_pow2_const (6.35 us) leads mod_npow2_const (14.04 us) by 121%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 55% (significant)

mod_pow2_const is -7.69 us (55%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.8x slower than the field

mod_var (17.84 us) is 2.8x the fastest (6.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: mod_pow2_const** at 6347.1 ns median (-54.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.81x (fastest 6347.1 ns, slowest 17840.4 ns)

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
| mod_npow2_const | 16209ns | 16150ns | 16139ns | 16159ns | 16429ns | base |
| mod_pow2_const | 8462ns | 8447ns | 8442ns | 8450ns | 8519ns | -47.79% |
| mod_var | 19970ns | 19959ns | 19929ns | 19961ns | 20038ns | +23.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 14094ns | 14036ns | 14288ns | base | 0.291 |
| mod_pow2_const | 6358ns | 6344ns | 6405ns | -54.89% | 0.644 |
| mod_var | 17848ns | 17813ns | 17914ns | +26.64% | 0.229 |

## Performance model

- Peak throughput: **0.646 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.292 | 45.2% |
| mod_pow2_const | 0.645 | 99.9% |
| mod_var | 0.230 | 35.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 16209ns | 16209ns | base |
| mod_pow2_const | 8462ns | 8462ns | -47.79% |
| mod_var | 19970ns | 19970ns | +23.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 14039ns | base | --- | [14039, 14040] | --- | --- | --- | --- |
| mod_pow2_const | 6347ns | -7692.1ns (-54.8%) | [-7693, -7691]ns | [6347, 6348] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 17840ns | +3779.0ns (+26.9%) | [+3778, +3791]ns | [17831, 17845] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 14039ns | -54.8% | +26.9% |
| 1 | 2 | 14040ns | -54.8% | +27.1% |
| 1 | 3 | 14039ns | -54.8% | +27.1% |
| 1 | 4 | 14048ns | -54.8% | +26.9% |
| 1 | 5 | 14039ns | -54.8% | +26.9% |
| 1 | 6 | 14039ns | -54.8% | +26.9% |
| 1 | 7 | 14040ns | -54.8% | +26.9% |
| 1 | 8 | 14039ns | -54.8% | +27.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.235 | moderate- |
| mod_pow2_const | -0.069 | ok |
| mod_var | 0.063 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 2.6ns | 14094.0ns | 0.0% |  |
| mod_pow2_const | 2.6ns | 6358.3ns | 0.0% |  |
| mod_var | 3.1ns | 17848.4ns | 0.0% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 14035.6-14288.4 ns)
  14035.6 |########################################
  14048.3 |##
  14060.9 |#
  14073.5 |####
  14086.2 |#####
  14098.8 |#
  14111.5 |#
  14124.1 |#
  14136.7 |
  14149.4 |
  14162.0 |
  14174.7 |
  14187.3 |
  14199.9 |
  14212.6 |
  14225.2 |
  14237.8 |
  14250.5 |
  14263.1 |
  14275.8 |
  (14 below, 6 above range)

mod_pow2_const (n=160, range 6343.9-6404.8 ns)
   6343.9 |########################################
   6346.9 |#############################
   6350.0 |########
   6353.0 |#
   6356.1 |
   6359.1 |
   6362.1 |
   6365.2 |#
   6368.2 |
   6371.3 |
   6374.3 |
   6377.4 |
   6380.4 |
   6383.5 |##
   6386.5 |###
   6389.6 |##
   6392.6 |
   6395.6 |
   6398.7 |
   6401.7 |
  (12 below, 13 above range)

mod_var (n=160, range 17812.9-17914.2 ns)
  17812.9 |########################################
  17817.9 |##
  17823.0 |
  17828.1 |###
  17833.1 |#
  17838.2 |############
  17843.3 |##########
  17848.3 |#
  17853.4 |##
  17858.5 |#######
  17863.5 |##
  17868.6 |#
  17873.7 |###
  17878.7 |###
  17883.8 |#
  17888.8 |######
  17893.9 |###
  17899.0 |
  17904.0 |#
  17909.1 |
  (12 below, 10 above range)

```
