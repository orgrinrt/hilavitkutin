# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 124% faster than the next best (mod_npow2_const)

mod_pow2_const (1.66 us) leads mod_npow2_const (3.71 us) by 124%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 55% (significant)

mod_pow2_const is -2.05 us (55%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.7x slower than the field

mod_var (4.50 us) is 2.7x the fastest (1.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: mod_pow2_const** at 1660.4 ns median (-55.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.71x (fastest 1660.4 ns, slowest 4504.8 ns)

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
| mod_npow2_const | 5869ns | 5957ns | 5569ns | 5885ns | 6123ns | base |
| mod_pow2_const | 3813ns | 3893ns | 3638ns | 3816ns | 3977ns | -35.04% |
| mod_var | 6741ns | 6673ns | 6519ns | 6699ns | 7090ns | +14.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 3654ns | 3467ns | 3813ns | base | 0.280 |
| mod_pow2_const | 1625ns | 1551ns | 1699ns | -55.52% | 0.630 |
| mod_var | 4554ns | 4401ns | 4799ns | +24.63% | 0.225 |

## Performance model

- Peak throughput: **0.660 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.276 | 41.8% |
| mod_pow2_const | 0.617 | 93.4% |
| mod_var | 0.227 | 34.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 5869ns | 5869ns | base |
| mod_pow2_const | 3813ns | 3813ns | -35.04% |
| mod_var | 6741ns | 6741ns | +14.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 3712ns | base | --- | [3661, 3715] | --- | --- | --- | --- |
| mod_pow2_const | 1660ns | -2051.0ns (-55.3%) | [-2058, -2009]ns | [1614, 1662] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 4505ns | +839.2ns (+22.6%) | [+826, +913]ns | [4466, 4543] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 3711ns | -58.2% | +19.0% |
| 1 | 2 | 3589ns | -53.7% | +23.7% |
| 1 | 3 | 3738ns | -55.4% | +20.0% |
| 1 | 4 | 3725ns | -55.4% | +26.4% |
| 1 | 5 | 3608ns | -53.9% | +22.2% |
| 1 | 6 | 3718ns | -58.2% | +19.5% |
| 1 | 7 | 3656ns | -54.5% | +24.4% |
| 1 | 8 | 3467ns | -54.7% | +35.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.155 | ok |
| mod_pow2_const | -0.270 | moderate- |
| mod_var | -0.044 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 3.3ns | 3654.1ns | 0.1% |  |
| mod_pow2_const | 2.9ns | 1625.4ns | 0.2% |  |
| mod_var | 3.6ns | 4554.3ns | 0.1% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 3467.5-3812.8 ns)
   3467.5 |#######
   3484.7 |###
   3502.0 |
   3519.3 |
   3536.5 |
   3553.8 |
   3571.1 |#####
   3588.3 |#####
   3605.6 |#
   3622.8 |#
   3640.1 |#
   3657.4 |##
   3674.6 |
   3691.9 |
   3709.2 |########################################
   3726.4 |######
   3743.7 |#
   3761.0 |
   3778.2 |
   3795.5 |
  (21 below, 9 above range)

mod_pow2_const (n=160, range 1550.7-1698.8 ns)
   1550.7 |##########################
   1558.1 |######
   1565.5 |#####
   1572.9 |###
   1580.3 |
   1587.7 |#
   1595.1 |
   1602.5 |
   1609.9 |#
   1617.3 |
   1624.7 |
   1632.1 |
   1639.5 |
   1646.9 |
   1654.3 |####################
   1661.7 |########################################
   1669.1 |#########
   1676.5 |
   1684.0 |
   1691.4 |
  (16 below, 8 above range)

mod_var (n=160, range 4401.3-4799.1 ns)
   4401.3 |######################################
   4421.2 |########################
   4441.1 |########
   4461.0 |########
   4480.9 |###########
   4500.8 |####
   4520.6 |######
   4540.5 |####################
   4560.4 |######
   4580.3 |#
   4600.2 |#
   4620.1 |#
   4640.0 |#
   4659.9 |#
   4679.8 |####
   4699.7 |########################################
   4719.6 |###########
   4739.5 |#
   4759.4 |
   4779.2 |
  (14 below, 6 above range)

```
