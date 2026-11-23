# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 191% faster than the next best (mod_npow2_const)

mod_pow2_const (4.50 us) leads mod_npow2_const (13.10 us) by 191%, a clear separation rather than a photo finish. CV 9.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 65% (significant)

mod_pow2_const is -8.53 us (65%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 5.1x slower than the field

mod_var (22.95 us) is 5.1x the fastest (4.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.1x the fastest

Fastest mod_pow2_const (4.50 us) to slowest mod_var (22.95 us): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: mod_pow2_const** at 4501.6 ns median (-65.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.10x (fastest 4501.6 ns, slowest 22954.4 ns)

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
| mod_npow2_const | 14699ns | 14841ns | 13212ns | 14698ns | 16188ns | base |
| mod_pow2_const | 6134ns | 6271ns | 5505ns | 6134ns | 6762ns | -58.27% |
| mod_var | 24557ns | 24686ns | 22082ns | 24551ns | 27049ns | +67.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 12980ns | 11671ns | 14291ns | base | 0.316 |
| mod_pow2_const | 4418ns | 3969ns | 4868ns | -65.96% | 0.927 |
| mod_var | 22838ns | 20541ns | 25148ns | +75.95% | 0.179 |

## Performance model

- Peak throughput: **1.032 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.313 | 30.3% |
| mod_pow2_const | 0.910 | 88.2% |
| mod_var | 0.178 | 17.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 14699ns | 14699ns | base |
| mod_pow2_const | 6134ns | 6134ns | -58.27% |
| mod_var | 24557ns | 24557ns | +67.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 13096ns | base | --- | [11684, 14267] | --- | --- | --- | --- |
| mod_pow2_const | 4502ns | -8534.1ns (-65.2%) | [-9410, -7705]ns | [3980, 4852] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 22954ns | +9982.5ns (+76.2%) | [+8873, +10840]ns | [20553, 25108] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 11675ns | -58.4% | +76.0% |
| 1 | 2 | 11674ns | -66.0% | +76.0% |
| 1 | 3 | 14270ns | -72.2% | +76.0% |
| 1 | 4 | 11674ns | -66.0% | +115.2% |
| 1 | 5 | 14273ns | -72.2% | +43.9% |
| 1 | 6 | 14278ns | -65.9% | +43.9% |
| 1 | 7 | 14269ns | -66.0% | +76.0% |
| 1 | 8 | 11674ns | -58.4% | +115.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.124 | ok |
| mod_pow2_const | 0.370 | moderate+ |
| mod_var | 0.125 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 21.8ns | 12979.8ns | 0.2% |  |
| mod_pow2_const | 21.6ns | 4418.1ns | 0.5% |  |
| mod_var | 21.7ns | 22838.3ns | 0.1% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 11671.4-14290.6 ns)
  11671.4 |#####################################
  11802.3 |
  11933.3 |
  12064.2 |
  12195.2 |
  12326.2 |
  12457.1 |
  12588.1 |
  12719.1 |
  12850.0 |
  12981.0 |
  13112.0 |
  13242.9 |
  13373.9 |
  13504.8 |
  13635.8 |
  13766.8 |
  13897.7 |
  14028.7 |
  14159.7 |########################################
  (6 below, 4 above range)

mod_pow2_const (n=160, range 3969.4-4868.1 ns)
   3969.4 |##################################
   4014.3 |
   4059.3 |
   4104.2 |
   4149.1 |
   4194.1 |
   4239.0 |
   4283.9 |
   4328.9 |
   4373.8 |
   4418.8 |
   4463.7 |
   4508.6 |
   4553.6 |
   4598.5 |
   4643.4 |
   4688.4 |
   4733.3 |
   4778.2 |
   4823.2 |########################################
  (10 below, 2 above range)

mod_var (n=160, range 20541.0-25147.5 ns)
  20541.0 |####################################
  20771.3 |
  21001.6 |
  21232.0 |
  21462.3 |
  21692.6 |
  21922.9 |
  22153.3 |
  22383.6 |
  22613.9 |
  22844.2 |
  23074.6 |
  23304.9 |
  23535.2 |
  23765.6 |
  23995.9 |
  24226.2 |
  24456.5 |
  24686.9 |
  24917.2 |########################################
  (10 below, 4 above range)

```
