# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 194% faster than the next best (mod_npow2_const)

mod_pow2_const (19.41 us) leads mod_npow2_const (57.07 us) by 194%, a clear separation rather than a photo finish. CV 9.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 66% (significant)

mod_pow2_const is -37.65 us (66%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 5.2x slower than the field

mod_var (100.43 us) is 5.2x the fastest (19.41 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.2x the fastest

Fastest mod_pow2_const (19.41 us) to slowest mod_var (100.43 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: mod_pow2_const** at 19407.7 ns median (-66.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.17x (fastest 19407.7 ns, slowest 100433.3 ns)

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
| mod_npow2_const | 56317ns | 58954ns | 48228ns | 58102ns | 59052ns | base |
| mod_pow2_const | 20698ns | 21293ns | 17417ns | 20974ns | 23151ns | -63.25% |
| mod_var | 98620ns | 102330ns | 83710ns | 100788ns | 107026ns | +75.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 54509ns | 46683ns | 57142ns | base | 0.301 |
| mod_pow2_const | 18756ns | 15874ns | 20566ns | -65.59% | 0.874 |
| mod_var | 96787ns | 82159ns | 105024ns | +77.56% | 0.169 |

## Performance model

- Peak throughput: **1.032 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.287 | 27.8% |
| mod_pow2_const | 0.844 | 81.8% |
| mod_var | 0.163 | 15.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 56317ns | 56317ns | base |
| mod_pow2_const | 20698ns | 20698ns | -63.25% |
| mod_var | 98620ns | 98620ns | +75.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 57067ns | base | --- | [57066, 57068] | --- | --- | --- | --- |
| mod_pow2_const | 19408ns | -37652.6ns (-66.0%) | [-37658, -37647]ns | [19407, 19408] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 100433ns | +43365.3ns (+76.0%) | [+43358, +43370]ns | [100431, 100435] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 57068ns | -62.9% | +76.0% |
| 1 | 2 | 46691ns | -66.0% | +76.0% |
| 1 | 3 | 57071ns | -66.0% | +76.0% |
| 1 | 4 | 57067ns | -66.0% | +76.0% |
| 1 | 5 | 57072ns | -72.2% | +76.0% |
| 1 | 6 | 46683ns | -58.4% | +115.2% |
| 1 | 7 | 57069ns | -66.0% | +76.0% |
| 1 | 8 | 57069ns | -66.0% | +44.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.375 | moderate- |
| mod_pow2_const | -0.463 | moderate- |
| mod_var | -0.208 | moderate- |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 23.0ns | 54508.7ns | 0.0% |  |
| mod_pow2_const | 24.7ns | 18755.8ns | 0.1% |  |
| mod_var | 23.7ns | 96787.1ns | 0.0% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 46683.4-57142.3 ns)
  46683.4 |########
  47206.3 |
  47729.3 |
  48252.2 |
  48775.2 |
  49298.1 |
  49821.1 |
  50344.0 |
  50867.0 |
  51389.9 |
  51912.8 |
  52435.8 |
  52958.7 |
  53481.7 |
  54004.6 |
  54527.6 |
  55050.5 |
  55573.5 |
  56096.4 |
  56619.4 |########################################
  (14 below, 5 above range)

mod_pow2_const (n=160, range 15874.1-20566.3 ns)
  15874.1 |########
  16108.7 |
  16343.3 |
  16577.9 |
  16812.5 |
  17047.1 |
  17281.7 |
  17516.4 |
  17751.0 |
  17985.6 |
  18220.2 |
  18454.8 |
  18689.4 |
  18924.0 |
  19158.6 |
  19393.3 |########################################
  19627.9 |
  19862.5 |
  20097.1 |
  20331.7 |
  (18 below, 20 above range)

mod_var (n=160, range 82158.8-105024.1 ns)
  82158.8 |##########
  83302.0 |
  84445.3 |
  85588.6 |
  86731.8 |
  87875.1 |
  89018.4 |
  90161.6 |
  91304.9 |
  92448.2 |
  93591.4 |
  94734.7 |
  95878.0 |
  97021.2 |
  98164.5 |
  99307.8 |########################################
  100451.0 |########
  101594.3 |
  102737.6 |
  103880.8 |
  (16 below, 2 above range)

```
