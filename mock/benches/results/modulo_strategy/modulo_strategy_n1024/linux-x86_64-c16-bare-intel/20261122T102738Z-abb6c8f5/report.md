# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 193% faster than the next best (mod_npow2_const)

mod_pow2_const (1.22 us) leads mod_npow2_const (3.57 us) by 193%, a clear separation rather than a photo finish. CV 41.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 66% (significant)

mod_pow2_const is -2.35 us (66%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 5.2x slower than the field

mod_var (6.28 us) is 5.2x the fastest (1.22 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### mod_pow2_const is fastest but the noisiest (CV 41.4%)

mod_pow2_const wins on median (1.22 us) yet has the highest variance (CV 41.4%), while mod_npow2_const is the steadiest (CV 8.0%, 3.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.2x the fastest

Fastest mod_pow2_const (1.22 us) to slowest mod_var (6.28 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### mod_pow2_const is inconsistent: worst-20% is 1.7x its best-20%

mod_pow2_const's best 20% of batches run at 1.10 us but its worst 20% at 1.87 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: mod_pow2_const** at 1218.2 ns median (-65.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.15x (fastest 1218.2 ns, slowest 6278.6 ns)

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
| Counter frequency | 2112750330 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mod_npow2_const | 5221ns | 5452ns | 4461ns | 5373ns | 5525ns | base |
| mod_pow2_const | 3376ns | 3098ns | 2796ns | 3099ns | 4790ns | -35.33% |
| mod_var | 7618ns | 8165ns | 6678ns | 7738ns | 8199ns | +45.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 3417ns | 2922ns | 3610ns | base | 0.300 |
| mod_pow2_const | 1325ns | 1101ns | 1868ns | -61.23% | 0.773 |
| mod_var | 5859ns | 5138ns | 6298ns | +71.46% | 0.175 |

## Performance model

- Peak throughput: **0.930 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.287 | 30.8% |
| mod_pow2_const | 0.841 | 90.4% |
| mod_var | 0.163 | 17.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 5221ns | 5221ns | base |
| mod_pow2_const | 3376ns | 3376ns | -35.33% |
| mod_var | 7618ns | 7618ns | +45.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 3570ns | base | --- | [3570, 3570] | --- | --- | --- | --- |
| mod_pow2_const | 1218ns | -2351.4ns (-65.9%) | [-2352, -2351]ns | [1218, 1218] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 6279ns | +2708.4ns (+75.9%) | [+2697, +2709]ns | [6278, 6289] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 3570ns | -65.9% | +76.2% |
| 1 | 2 | 3570ns | -65.9% | +76.2% |
| 1 | 3 | 3570ns | -65.9% | +76.2% |
| 1 | 4 | 2921ns | -65.9% | +76.2% |
| 1 | 5 | 3570ns | -65.9% | +44.0% |
| 1 | 6 | 3575ns | -65.9% | +75.9% |
| 1 | 7 | 2921ns | -58.3% | +115.3% |
| 1 | 8 | 3570ns | -65.9% | +44.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.376 | moderate- |
| mod_pow2_const | -0.161 | ok |
| mod_var | 0.059 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 23.0ns | 3417.1ns | 0.7% |  |
| mod_pow2_const | 25.9ns | 1324.7ns | 2.0% |  |
| mod_var | 22.6ns | 5859.0ns | 0.4% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 2921.5-3610.5 ns)
   2921.5 |#####
   2956.0 |
   2990.4 |
   3024.9 |
   3059.3 |
   3093.8 |
   3128.2 |
   3162.7 |
   3197.1 |
   3231.6 |
   3266.0 |
   3300.5 |
   3334.9 |
   3369.3 |
   3403.8 |
   3438.2 |
   3472.7 |
   3507.1 |
   3541.6 |########################################
   3576.0 |####################
  (29 below, 6 above range)

mod_pow2_const (n=160, range 1101.1-1867.7 ns)
   1101.1 |
   1139.5 |
   1177.8 |
   1216.1 |########################################
   1254.5 |
   1292.8 |
   1331.1 |
   1369.4 |
   1407.8 |
   1446.1 |
   1484.4 |
   1522.7 |
   1561.1 |
   1599.4 |
   1637.7 |
   1676.0 |
   1714.4 |
   1752.7 |
   1791.0 |
   1829.4 |
  (17 below, 11 above range)

mod_var (n=160, range 5138.1-6297.7 ns)
   5138.1 |#############
   5196.1 |
   5254.1 |
   5312.0 |
   5370.0 |
   5428.0 |
   5486.0 |
   5544.0 |
   5601.9 |
   5659.9 |
   5717.9 |
   5775.9 |
   5833.9 |
   5891.9 |
   5949.8 |
   6007.8 |
   6065.8 |
   6123.8 |
   6181.8 |
   6239.7 |########################################
  (28 below, 4 above range)

```

## Diagnostics

- **mod_pow2_const**: CV=38.1% (high variance, measurements may be unstable)
