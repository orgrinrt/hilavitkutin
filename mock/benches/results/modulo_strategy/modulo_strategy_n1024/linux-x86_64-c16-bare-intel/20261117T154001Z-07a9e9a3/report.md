# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 141% faster than the next best (mod_npow2_const)

mod_pow2_const (1.22 us) leads mod_npow2_const (2.93 us) by 141%, a clear separation rather than a photo finish. CV 49.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 65% (significant)

mod_pow2_const is -1.91 us (65%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 5.2x slower than the field

mod_var (6.28 us) is 5.2x the fastest (1.22 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### mod_pow2_const is fastest but the noisiest (CV 49.0%)

mod_pow2_const wins on median (1.22 us) yet has the highest variance (CV 49.0%), while mod_var is the steadiest (CV 8.8%, 6.28 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.2x the fastest

Fastest mod_pow2_const (1.22 us) to slowest mod_var (6.28 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### mod_pow2_const is inconsistent: worst-20% is 2.2x its best-20%

mod_pow2_const's best 20% of batches run at 997 ns but its worst 20% at 2.22 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: mod_pow2_const** at 1218.2 ns median (-58.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.15x (fastest 1218.2 ns, slowest 6278.2 ns)

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
| Counter frequency | 2112814420 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mod_npow2_const | 5014ns | 4475ns | 4461ns | 4763ns | 6319ns | base |
| mod_pow2_const | 3483ns | 3102ns | 2539ns | 3061ns | 5694ns | -30.53% |
| mod_var | 7626ns | 8168ns | 6683ns | 7750ns | 8198ns | +52.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 3277ns | 2920ns | 4127ns | base | 0.312 |
| mod_pow2_const | 1365ns | 997ns | 2221ns | -58.35% | 0.750 |
| mod_var | 5861ns | 5138ns | 6295ns | +78.85% | 0.175 |

## Performance model

- Peak throughput: **1.027 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.349 | 34.0% |
| mod_pow2_const | 0.841 | 81.8% |
| mod_var | 0.163 | 15.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 5014ns | 5014ns | base |
| mod_pow2_const | 3483ns | 3483ns | -30.53% |
| mod_var | 7626ns | 7626ns | +52.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 2930ns | base | --- | [2921, 2932] | --- | --- | --- | --- |
| mod_pow2_const | 1218ns | -1913.7ns (-65.3%) | [-1924, -1712]ns | [1218, 1218] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 6278ns | +2708.3ns (+92.4%) | [+2590, +2715]ns | [6278, 6279] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 2921ns | -65.9% | +76.2% |
| 1 | 2 | 2921ns | -58.3% | +115.3% |
| 1 | 3 | 3575ns | -51.5% | +75.9% |
| 1 | 4 | 2920ns | -58.3% | +115.4% |
| 1 | 5 | 2921ns | -58.3% | +76.1% |
| 1 | 6 | 3570ns | -65.9% | +43.9% |
| 1 | 7 | 3575ns | -65.9% | +75.6% |
| 1 | 8 | 2921ns | -65.9% | +115.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.143 | ok |
| mod_pow2_const | -0.013 | ok |
| mod_var | 0.061 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 22.0ns | 3277.2ns | 0.7% |  |
| mod_pow2_const | 26.8ns | 1365.0ns | 2.0% |  |
| mod_var | 22.8ns | 5861.1ns | 0.4% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 2920.4-4126.6 ns)
   2920.4 |########################################
   2980.7 |
   3041.0 |
   3101.3 |
   3161.7 |
   3222.0 |
   3282.3 |
   3342.6 |
   3402.9 |
   3463.2 |
   3523.5 |######################
   3583.8 |#
   3644.2 |
   3704.5 |
   3764.8 |
   3825.1 |
   3885.4 |
   3945.7 |
   4006.0 |
   4066.3 |
  (19 below, 6 above range)

mod_pow2_const (n=160, range 996.7-2221.1 ns)
    996.7 |######
   1057.9 |
   1119.2 |
   1180.4 |########################################
   1241.6 |
   1302.8 |
   1364.0 |
   1425.3 |
   1486.5 |
   1547.7 |
   1608.9 |
   1670.2 |
   1731.4 |
   1792.6 |
   1853.8 |
   1915.0 |
   1976.3 |
   2037.5 |
   2098.7 |
   2159.9 |
  (22 below, 15 above range)

mod_var (n=160, range 5138.2-6295.1 ns)
   5138.2 |############
   5196.0 |
   5253.9 |
   5311.7 |
   5369.6 |
   5427.4 |
   5485.3 |
   5543.1 |
   5600.9 |
   5658.8 |
   5716.6 |
   5774.5 |
   5832.3 |
   5890.2 |
   5948.0 |
   6005.9 |
   6063.7 |
   6121.6 |
   6179.4 |
   6237.3 |########################################
  (27 below, 3 above range)

```

## Diagnostics

- **mod_npow2_const**: CV=23.1% (high variance, measurements may be unstable)
- **mod_pow2_const**: CV=43.7% (high variance, measurements may be unstable)
