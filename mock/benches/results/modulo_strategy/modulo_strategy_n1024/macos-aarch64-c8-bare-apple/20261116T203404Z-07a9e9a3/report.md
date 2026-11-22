# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 123% faster than the next best (mod_npow2_const)

mod_pow2_const (1.55 us) leads mod_npow2_const (3.46 us) by 123%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 55% (significant)

mod_pow2_const is -1.91 us (55%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.8x slower than the field

mod_var (4.39 us) is 2.8x the fastest (1.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: mod_pow2_const** at 1551.7 ns median (-55.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.83x (fastest 1551.7 ns, slowest 4392.7 ns)

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
| mod_npow2_const | 5604ns | 5572ns | 5564ns | 5575ns | 5732ns | base |
| mod_pow2_const | 3678ns | 3655ns | 3647ns | 3657ns | 3771ns | -34.37% |
| mod_var | 6563ns | 6528ns | 6503ns | 6532ns | 6717ns | +17.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 3484ns | 3461ns | 3567ns | base | 0.294 |
| mod_pow2_const | 1560ns | 1548ns | 1598ns | -55.21% | 0.656 |
| mod_var | 4427ns | 4387ns | 4536ns | +27.05% | 0.231 |

## Performance model

- Peak throughput: **0.662 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.296 | 44.7% |
| mod_pow2_const | 0.660 | 99.8% |
| mod_var | 0.233 | 35.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 5604ns | 5604ns | base |
| mod_pow2_const | 3678ns | 3678ns | -34.37% |
| mod_var | 6563ns | 6563ns | +17.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 3464ns | base | --- | [3463, 3465] | --- | --- | --- | --- |
| mod_pow2_const | 1552ns | -1912.5ns (-55.2%) | [-1913, -1912]ns | [1551, 1552] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 4393ns | +928.1ns (+26.8%) | [+927, +940]ns | [4391, 4412] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 3463ns | -55.2% | +27.6% |
| 1 | 2 | 3464ns | -55.2% | +27.5% |
| 1 | 3 | 3463ns | -55.2% | +26.7% |
| 1 | 4 | 3464ns | -53.7% | +26.7% |
| 1 | 5 | 3463ns | -55.2% | +26.8% |
| 1 | 6 | 3463ns | -55.2% | +26.7% |
| 1 | 7 | 3465ns | -55.2% | +26.7% |
| 1 | 8 | 3583ns | -56.5% | +26.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.001 | ok |
| mod_pow2_const | -0.225 | moderate- |
| mod_var | -0.022 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 3.1ns | 3484.4ns | 0.1% |  |
| mod_pow2_const | 2.7ns | 1560.5ns | 0.2% |  |
| mod_var | 3.9ns | 4426.8ns | 0.1% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 3460.8-3566.6 ns)
   3460.8 |########################################
   3466.1 |######
   3471.4 |
   3476.7 |
   3482.0 |#
   3487.3 |
   3492.5 |
   3497.8 |
   3503.1 |
   3508.4 |
   3513.7 |
   3519.0 |
   3524.3 |
   3529.5 |
   3534.8 |
   3540.1 |
   3545.4 |
   3550.7 |
   3556.0 |
   3561.3 |
  (17 below, 21 above range)

mod_pow2_const (n=160, range 1548.0-1598.1 ns)
   1548.0 |########################################
   1550.5 |################################
   1553.0 |#################
   1555.5 |#####
   1558.0 |####
   1560.5 |#
   1563.0 |#
   1565.5 |
   1568.0 |
   1570.6 |#
   1573.1 |
   1575.6 |
   1578.1 |
   1580.6 |
   1583.1 |
   1585.6 |
   1588.1 |
   1590.6 |
   1593.1 |
   1595.6 |
  (18 below, 19 above range)

mod_var (n=160, range 4387.1-4536.2 ns)
   4387.1 |########################################
   4394.6 |#
   4402.0 |
   4409.5 |###########
   4416.9 |#######
   4424.4 |
   4431.8 |#
   4439.3 |
   4446.7 |##
   4454.2 |
   4461.6 |
   4469.1 |
   4476.6 |#
   4484.0 |#
   4491.5 |
   4498.9 |
   4506.4 |
   4513.8 |
   4521.3 |
   4528.7 |
  (9 below, 25 above range)

```
