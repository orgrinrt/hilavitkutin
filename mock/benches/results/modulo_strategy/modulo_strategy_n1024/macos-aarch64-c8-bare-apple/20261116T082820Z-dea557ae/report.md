# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 123% faster than the next best (mod_npow2_const)

mod_pow2_const (1.58 us) leads mod_npow2_const (3.53 us) by 123%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 55% (significant)

mod_pow2_const is -1.95 us (55%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.8x slower than the field

mod_var (4.47 us) is 2.8x the fastest (1.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: mod_pow2_const** at 1580.4 ns median (-55.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.83x (fastest 1580.4 ns, slowest 4473.8 ns)

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
| mod_npow2_const | 5677ns | 5664ns | 5647ns | 5665ns | 5745ns | base |
| mod_pow2_const | 3719ns | 3711ns | 3700ns | 3711ns | 3762ns | -34.49% |
| mod_var | 6617ns | 6620ns | 6498ns | 6620ns | 6725ns | +16.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 3536ns | 3521ns | 3575ns | base | 0.290 |
| mod_pow2_const | 1585ns | 1576ns | 1607ns | -55.17% | 0.646 |
| mod_var | 4474ns | 4393ns | 4555ns | +26.51% | 0.229 |

## Performance model

- Peak throughput: **0.650 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.290 | 44.7% |
| mod_pow2_const | 0.648 | 99.7% |
| mod_var | 0.229 | 35.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 5677ns | 5677ns | base |
| mod_pow2_const | 3719ns | 3719ns | -34.49% |
| mod_var | 6617ns | 6617ns | +16.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 3529ns | base | --- | [3528, 3529] | --- | --- | --- | --- |
| mod_pow2_const | 1580ns | -1947.7ns (-55.2%) | [-1949, -1947]ns | [1580, 1581] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 4474ns | +945.0ns (+26.8%) | [+943, +946]ns | [4472, 4475] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 3526ns | -55.2% | +26.9% |
| 1 | 2 | 3529ns | -55.2% | +26.7% |
| 1 | 3 | 3529ns | -55.2% | +26.8% |
| 1 | 4 | 3530ns | -55.2% | +26.7% |
| 1 | 5 | 3528ns | -55.2% | +26.7% |
| 1 | 6 | 3529ns | -55.3% | +26.7% |
| 1 | 7 | 3528ns | -55.2% | +26.9% |
| 1 | 8 | 3528ns | -55.2% | +26.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.130 | ok |
| mod_pow2_const | -0.172 | ok |
| mod_var | -0.289 | moderate- |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 3.2ns | 3536.3ns | 0.1% |  |
| mod_pow2_const | 2.9ns | 1585.2ns | 0.2% |  |
| mod_var | 3.3ns | 4473.6ns | 0.1% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 3521.0-3574.5 ns)
   3521.0 |####
   3523.7 |########################################
   3526.4 |##################################
   3529.1 |#################################
   3531.7 |#######
   3534.4 |####
   3537.1 |
   3539.8 |
   3542.4 |
   3545.1 |
   3547.8 |
   3550.5 |
   3553.1 |##
   3555.8 |
   3558.5 |
   3561.2 |#
   3563.8 |#
   3566.5 |
   3569.2 |
   3571.9 |
  (3 below, 14 above range)

mod_pow2_const (n=160, range 1576.3-1607.0 ns)
   1576.3 |#######################
   1577.8 |####################################
   1579.4 |########################################
   1580.9 |################
   1582.4 |##################
   1584.0 |###############
   1585.5 |#########
   1587.0 |#####
   1588.6 |###
   1590.1 |##
   1591.6 |
   1593.2 |
   1594.7 |
   1596.2 |
   1597.8 |##
   1599.3 |#
   1600.8 |##
   1602.4 |
   1603.9 |#
   1605.4 |#
  (13 below, 8 above range)

mod_var (n=160, range 4392.5-4555.2 ns)
   4392.5 |#####
   4400.6 |
   4408.8 |##
   4416.9 |
   4425.1 |
   4433.2 |
   4441.3 |#
   4449.5 |
   4457.6 |
   4465.7 |########################################
   4473.9 |##############################
   4482.0 |########
   4490.1 |
   4498.3 |###
   4506.4 |####
   4514.5 |#
   4522.7 |
   4530.8 |#
   4538.9 |##
   4547.1 |#
  (22 below, 7 above range)

```
