# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 121% faster than the next best (mod_npow2_const)

mod_pow2_const (25.58 us) leads mod_npow2_const (56.51 us) by 121%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 55% (significant)

mod_pow2_const is -30.90 us (55%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.8x slower than the field

mod_var (71.72 us) is 2.8x the fastest (25.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: mod_pow2_const** at 25583.9 ns median (-54.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.80x (fastest 25583.9 ns, slowest 71724.8 ns)

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
| mod_npow2_const | 58747ns | 58682ns | 58515ns | 58703ns | 59113ns | base |
| mod_pow2_const | 27757ns | 27699ns | 27625ns | 27714ns | 28018ns | -52.75% |
| mod_var | 74113ns | 73966ns | 73747ns | 73970ns | 74907ns | +26.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 56567ns | 56361ns | 56896ns | base | 0.290 |
| mod_pow2_const | 25642ns | 25527ns | 25887ns | -54.67% | 0.639 |
| mod_var | 71890ns | 71569ns | 72658ns | +27.09% | 0.228 |

## Performance model

- Peak throughput: **0.642 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.290 | 45.2% |
| mod_pow2_const | 0.640 | 99.8% |
| mod_var | 0.228 | 35.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 58747ns | 58747ns | base |
| mod_pow2_const | 27757ns | 27757ns | -52.75% |
| mod_var | 74113ns | 74113ns | +26.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 56505ns | base | --- | [56442, 56587] | --- | --- | --- | --- |
| mod_pow2_const | 25584ns | -30895.4ns (-54.7%) | [-30939, -30860]ns | [25577, 25604] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 71725ns | +15220.8ns (+26.9%) | [+15174, +15251]ns | [71687, 71785] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 56452ns | -54.7% | +27.0% |
| 1 | 2 | 56539ns | -54.7% | +26.9% |
| 1 | 3 | 56461ns | -54.7% | +27.0% |
| 1 | 4 | 56572ns | -54.8% | +27.0% |
| 1 | 5 | 56421ns | -54.6% | +27.2% |
| 1 | 6 | 56429ns | -54.7% | +27.1% |
| 1 | 7 | 56710ns | -54.9% | +26.5% |
| 1 | 8 | 56458ns | -54.7% | +27.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.455 | moderate- |
| mod_pow2_const | -0.256 | moderate- |
| mod_var | -0.279 | moderate- |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 2.8ns | 56566.9ns | 0.0% |  |
| mod_pow2_const | 2.8ns | 25641.7ns | 0.0% |  |
| mod_var | 3.2ns | 71890.4ns | 0.0% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 56361.0-56895.8 ns)
  56361.0 |##################################
  56387.7 |########################################
  56414.4 |##################################
  56441.2 |##########
  56467.9 |######
  56494.7 |############
  56521.4 |########
  56548.2 |############
  56574.9 |##########
  56601.6 |################
  56628.4 |####################
  56655.1 |##########
  56681.9 |##########
  56708.6 |##########
  56735.4 |############
  56762.1 |############
  56788.9 |####
  56815.6 |####
  56842.3 |####
  56869.1 |########
  (14 below, 8 above range)

mod_pow2_const (n=160, range 25527.0-25886.6 ns)
  25527.0 |########################################
  25544.9 |##################
  25562.9 |################
  25580.9 |##############
  25598.9 |##################
  25616.9 |#######
  25634.8 |#######
  25652.8 |##
  25670.8 |###
  25688.8 |###
  25706.8 |###
  25724.7 |###
  25742.7 |#
  25760.7 |##
  25778.7 |#######
  25796.7 |####
  25814.6 |##
  25832.6 |#
  25850.6 |##
  25868.6 |###
  (13 below, 11 above range)

mod_var (n=160, range 71568.9-72658.0 ns)
  71568.9 |##################################
  71623.4 |########################################
  71677.8 |#########################
  71732.3 |################
  71786.7 |#################
  71841.2 |###############################
  71895.6 |########
  71950.1 |##########
  72004.5 |##
  72059.0 |#######
  72113.4 |##
  72167.9 |#
  72222.4 |#
  72276.8 |
  72331.3 |
  72385.7 |
  72440.2 |#
  72494.6 |
  72549.1 |
  72603.5 |
  (15 below, 9 above range)

```
