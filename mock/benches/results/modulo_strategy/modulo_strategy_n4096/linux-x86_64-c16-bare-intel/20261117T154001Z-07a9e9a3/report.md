# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 175% faster than the next best (mod_npow2_const)

mod_pow2_const (4.85 us) leads mod_npow2_const (13.33 us) by 175%, a clear separation rather than a photo finish. CV 93.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 66% (significant)

mod_pow2_const is -8.81 us (66%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 5.2x slower than the field

mod_var (25.12 us) is 5.2x the fastest (4.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### mod_pow2_const is fastest but the noisiest (CV 93.4%)

mod_pow2_const wins on median (4.85 us) yet has the highest variance (CV 93.4%), while mod_var is the steadiest (CV 6.1%, 25.12 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.2x the fastest

Fastest mod_pow2_const (4.85 us) to slowest mod_var (25.12 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### mod_pow2_const is inconsistent: worst-20% is 1.9x its best-20%

mod_pow2_const's best 20% of batches run at 3.97 us but its worst 20% at 7.73 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: mod_pow2_const** at 4851.4 ns median (-63.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.18x (fastest 4851.4 ns, slowest 25119.2 ns)

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
| mod_npow2_const | 14707ns | 15042ns | 13213ns | 14705ns | 16209ns | base |
| mod_pow2_const | 7010ns | 6733ns | 5511ns | 6216ns | 10893ns | -52.34% |
| mod_var | 26412ns | 27002ns | 23936ns | 27002ns | 27117ns | +79.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 12987ns | 11671ns | 14308ns | base | 0.315 |
| mod_pow2_const | 5026ns | 3969ns | 7734ns | -61.30% | 0.815 |
| mod_var | 24562ns | 22257ns | 25207ns | +89.13% | 0.167 |

## Performance model

- Peak throughput: **1.032 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.307 | 29.8% |
| mod_pow2_const | 0.844 | 81.8% |
| mod_var | 0.163 | 15.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 14707ns | 14707ns | base |
| mod_pow2_const | 7010ns | 7010ns | -52.34% |
| mod_var | 26412ns | 26412ns | +79.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 13330ns | base | --- | [11683, 14267] | --- | --- | --- | --- |
| mod_pow2_const | 4851ns | -8814.8ns (-66.1%) | [-9404, -7712]ns | [3982, 4852] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 25119ns | +12334.4ns (+92.5%) | [+10853, +13432]ns | [25113, 25120] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 14268ns | -65.9% | +76.1% |
| 1 | 2 | 11673ns | -58.4% | +115.2% |
| 1 | 3 | 14280ns | -65.9% | +75.9% |
| 1 | 4 | 14270ns | -72.2% | +76.0% |
| 1 | 5 | 11675ns | -65.9% | +115.1% |
| 1 | 6 | 11674ns | -65.9% | +115.2% |
| 1 | 7 | 11674ns | -66.0% | +115.2% |
| 1 | 8 | 14278ns | -66.0% | +43.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.125 | ok |
| mod_pow2_const | 0.373 | moderate+ |
| mod_var | -0.018 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 21.7ns | 12987.2ns | 0.2% |  |
| mod_pow2_const | 24.6ns | 5025.9ns | 0.5% |  |
| mod_var | 23.6ns | 24562.3ns | 0.1% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 11671.1-14307.5 ns)
  11671.1 |####################################
  11802.9 |
  11934.7 |
  12066.6 |
  12198.4 |
  12330.2 |
  12462.0 |
  12593.8 |
  12725.7 |
  12857.5 |
  12989.3 |
  13121.1 |
  13252.9 |
  13384.8 |
  13516.6 |
  13648.4 |
  13780.2 |
  13912.1 |
  14043.9 |
  14175.7 |########################################
  (7 below, 4 above range)

mod_pow2_const (n=160, range 3969.5-7734.4 ns)
   3969.5 |###############################
   4157.7 |
   4346.0 |
   4534.2 |
   4722.5 |########################################
   4910.7 |#
   5099.0 |
   5287.2 |
   5475.5 |
   5663.7 |
   5852.0 |
   6040.2 |
   6228.5 |
   6416.7 |
   6604.9 |
   6793.2 |
   6981.4 |
   7169.7 |
   7357.9 |
   7546.2 |
  (10 below, 3 above range)

mod_var (n=160, range 22257.2-25206.7 ns)
  22257.2 |
  22404.7 |
  22552.2 |
  22699.6 |
  22847.1 |
  22994.6 |
  23142.1 |
  23289.5 |
  23437.0 |
  23584.5 |
  23732.0 |
  23879.4 |
  24026.9 |
  24174.4 |
  24321.9 |
  24469.3 |
  24616.8 |
  24764.3 |
  24911.7 |
  25059.2 |########################################
  (20 below, 4 above range)

```

## Diagnostics

- **mod_pow2_const**: CV=90.1% (high variance, measurements may be unstable)
