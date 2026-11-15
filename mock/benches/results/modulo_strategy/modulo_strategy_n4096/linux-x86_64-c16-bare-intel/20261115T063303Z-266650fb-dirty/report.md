# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 194% faster than the next best (mod_npow2_const)

mod_pow2_const (4.85 us) leads mod_npow2_const (14.27 us) by 194%, a clear separation rather than a photo finish. CV 7.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 66% (significant)

mod_pow2_const is -9.41 us (66%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 5.2x slower than the field

mod_var (25.11 us) is 5.2x the fastest (4.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### mod_pow2_const is fastest but the noisiest (CV 7.9%)

mod_pow2_const wins on median (4.85 us) yet has the highest variance (CV 7.9%), while mod_npow2_const is the steadiest (CV 7.8%, 14.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.2x the fastest

Fastest mod_pow2_const (4.85 us) to slowest mod_var (25.11 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: mod_pow2_const** at 4851.6 ns median (-66.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.18x (fastest 4851.6 ns, slowest 25109.9 ns)

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
| Counter frequency | 2112848770 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mod_npow2_const | 15434ns | 16150ns | 13213ns | 15924ns | 16184ns | base |
| mod_pow2_const | 6439ns | 6734ns | 5509ns | 6635ns | 6780ns | -58.28% |
| mod_var | 25781ns | 26994ns | 22083ns | 26589ns | 27055ns | +67.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 13633ns | 11673ns | 14284ns | base | 0.300 |
| mod_pow2_const | 4639ns | 3972ns | 4879ns | -65.97% | 0.883 |
| mod_var | 23981ns | 20544ns | 25155ns | +75.91% | 0.171 |

## Performance model

- Peak throughput: **1.031 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.287 | 27.8% |
| mod_pow2_const | 0.844 | 81.9% |
| mod_var | 0.163 | 15.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 15434ns | 15434ns | base |
| mod_pow2_const | 6439ns | 6439ns | -58.28% |
| mod_var | 25781ns | 25781ns | +67.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 14267ns | base | --- | [14267, 14268] | --- | --- | --- | --- |
| mod_pow2_const | 4852ns | -9414.8ns (-66.0%) | [-9416, -9414]ns | [4852, 4852] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 25110ns | +10841.0ns (+76.0%) | [+10840, +10842]ns | [25108, 25118] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 14267ns | -72.2% | +76.1% |
| 1 | 2 | 14273ns | -72.2% | +76.0% |
| 1 | 3 | 11673ns | -58.4% | +115.2% |
| 1 | 4 | 11675ns | -58.4% | +76.0% |
| 1 | 5 | 14268ns | -65.9% | +76.1% |
| 1 | 6 | 14271ns | -66.0% | +76.0% |
| 1 | 7 | 14267ns | -65.9% | +44.0% |
| 1 | 8 | 14277ns | -66.0% | +75.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | 0.291 | moderate+ |
| mod_pow2_const | 0.460 | moderate+ |
| mod_var | -0.375 | moderate- |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 22.7ns | 13632.6ns | 0.2% |  |
| mod_pow2_const | 22.5ns | 4639.5ns | 0.5% |  |
| mod_var | 22.8ns | 23981.1ns | 0.1% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 11673.0-14284.2 ns)
  11673.0 |#######
  11803.6 |
  11934.1 |
  12064.7 |
  12195.2 |
  12325.8 |
  12456.4 |
  12586.9 |
  12717.5 |
  12848.0 |
  12978.6 |
  13109.2 |
  13239.7 |
  13370.3 |
  13500.8 |
  13631.4 |
  13762.0 |
  13892.5 |
  14023.1 |
  14153.6 |########################################
  (15 below, 2 above range)

mod_pow2_const (n=160, range 3971.7-4879.0 ns)
   3971.7 |#####
   4017.0 |
   4062.4 |
   4107.8 |
   4153.1 |
   4198.5 |
   4243.9 |
   4289.2 |
   4334.6 |
   4380.0 |
   4425.3 |
   4470.7 |
   4516.1 |
   4561.4 |
   4606.8 |
   4652.2 |
   4697.6 |
   4742.9 |
   4788.3 |
   4833.7 |########################################
  (24 below, 6 above range)

mod_var (n=160, range 20544.2-25154.5 ns)
  20544.2 |######
  20774.7 |
  21005.2 |
  21235.8 |
  21466.3 |
  21696.8 |
  21927.3 |
  22157.8 |
  22388.3 |
  22618.8 |
  22849.4 |
  23079.9 |
  23310.4 |
  23540.9 |
  23771.4 |
  24001.9 |
  24232.5 |
  24463.0 |
  24693.5 |
  24924.0 |########################################
  (21 below, 3 above range)

```
