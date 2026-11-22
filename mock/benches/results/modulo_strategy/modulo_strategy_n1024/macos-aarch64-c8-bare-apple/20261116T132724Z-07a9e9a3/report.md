# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 123% faster than the next best (mod_npow2_const)

mod_pow2_const (1.55 us) leads mod_npow2_const (3.46 us) by 123%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 55% (significant)

mod_pow2_const is -1.91 us (55%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.8x slower than the field

mod_var (4.39 us) is 2.8x the fastest (1.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: mod_pow2_const** at 1550.8 ns median (-55.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.83x (fastest 1550.8 ns, slowest 4390.0 ns)

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
| mod_npow2_const | 5610ns | 5570ns | 5562ns | 5573ns | 5769ns | base |
| mod_pow2_const | 3659ns | 3653ns | 3646ns | 3653ns | 3690ns | -34.77% |
| mod_var | 6563ns | 6512ns | 6504ns | 6521ns | 6747ns | +16.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 3486ns | 3460ns | 3580ns | base | 0.294 |
| mod_pow2_const | 1552ns | 1547ns | 1561ns | -55.48% | 0.660 |
| mod_var | 4426ns | 4386ns | 4553ns | +26.95% | 0.231 |

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
| mod_npow2_const | 5610ns | 5610ns | base |
| mod_pow2_const | 3659ns | 3659ns | -34.77% |
| mod_var | 6563ns | 6563ns | +16.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 3464ns | base | --- | [3463, 3464] | --- | --- | --- | --- |
| mod_pow2_const | 1551ns | -1912.9ns (-55.2%) | [-1914, -1912]ns | [1550, 1551] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 4390ns | +927.5ns (+26.8%) | [+926, +929]ns | [4389, 4391] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 3602ns | -56.9% | +27.1% |
| 1 | 2 | 3464ns | -55.2% | +26.7% |
| 1 | 3 | 3463ns | -55.2% | +27.5% |
| 1 | 4 | 3463ns | -55.2% | +26.7% |
| 1 | 5 | 3463ns | -55.2% | +26.7% |
| 1 | 6 | 3463ns | -55.2% | +26.8% |
| 1 | 7 | 3463ns | -55.2% | +26.7% |
| 1 | 8 | 3462ns | -55.2% | +26.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.014 | ok |
| mod_pow2_const | -0.391 | moderate- |
| mod_var | -0.051 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 3.1ns | 3486.3ns | 0.1% |  |
| mod_pow2_const | 2.9ns | 1552.2ns | 0.2% |  |
| mod_var | 3.7ns | 4426.1ns | 0.1% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 3460.3-3579.7 ns)
   3460.3 |########################################
   3466.3 |###
   3472.2 |
   3478.2 |
   3484.2 |
   3490.1 |
   3496.1 |#
   3502.1 |
   3508.0 |
   3514.0 |
   3520.0 |
   3525.9 |
   3531.9 |
   3537.9 |
   3543.9 |
   3549.8 |
   3555.8 |
   3561.8 |
   3567.7 |
   3573.7 |
  (11 below, 20 above range)

mod_pow2_const (n=160, range 1547.5-1561.3 ns)
   1547.5 |##################
   1548.1 |#######################
   1548.8 |################
   1549.5 |############################
   1550.2 |########################################
   1550.9 |#########################
   1551.6 |############################
   1552.3 |################
   1553.0 |###
   1553.7 |###############
   1554.4 |#
   1555.1 |###
   1555.8 |######
   1556.5 |
   1557.1 |###
   1557.8 |
   1558.5 |#
   1559.2 |
   1559.9 |#
   1560.6 |
  (12 below, 8 above range)

mod_var (n=160, range 4386.3-4553.5 ns)
   4386.3 |########################################
   4394.7 |
   4403.1 |#
   4411.4 |######
   4419.8 |##
   4428.1 |##
   4436.5 |
   4444.8 |
   4453.2 |
   4461.6 |
   4469.9 |
   4478.3 |
   4486.6 |
   4495.0 |
   4503.3 |
   4511.7 |
   4520.1 |
   4528.4 |
   4536.8 |##
   4545.1 |
  (13 below, 15 above range)

```
