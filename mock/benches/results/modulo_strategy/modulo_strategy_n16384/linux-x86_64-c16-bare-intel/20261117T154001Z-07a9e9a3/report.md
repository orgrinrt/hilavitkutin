# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 259% faster than the next best (mod_npow2_const)

mod_pow2_const (15.88 us) leads mod_npow2_const (57.06 us) by 259%, a clear separation rather than a photo finish. CV 60.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 66% (significant)

mod_pow2_const is -37.67 us (66%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 6.3x slower than the field

mod_var (100.43 us) is 6.3x the fastest (15.88 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### mod_pow2_const is fastest but the noisiest (CV 60.7%)

mod_pow2_const wins on median (15.88 us) yet has the highest variance (CV 60.7%), while mod_var is the steadiest (CV 7.9%, 100.43 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 6.3x the fastest

Fastest mod_pow2_const (15.88 us) to slowest mod_var (100.43 us): 6.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: mod_pow2_const** at 15880.7 ns median (-72.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.32x (fastest 15880.7 ns, slowest 100426.4 ns)

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
| mod_npow2_const | 54976ns | 58950ns | 48223ns | 55872ns | 59044ns | base |
| mod_pow2_const | 19827ns | 17428ns | 17414ns | 18649ns | 25773ns | -63.94% |
| mod_var | 97771ns | 102323ns | 83705ns | 100779ns | 102816ns | +77.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 53211ns | 46678ns | 57135ns | base | 0.308 |
| mod_pow2_const | 18064ns | 15869ns | 23474ns | -66.05% | 0.907 |
| mod_var | 95953ns | 82155ns | 100883ns | +80.32% | 0.171 |

## Performance model

- Peak throughput: **1.032 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.287 | 27.8% |
| mod_pow2_const | 1.032 | 99.9% |
| mod_var | 0.163 | 15.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 54976ns | 54976ns | base |
| mod_pow2_const | 19827ns | 19827ns | -63.94% |
| mod_var | 97771ns | 97771ns | +77.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 57063ns | base | --- | [57056, 57066] | --- | --- | --- | --- |
| mod_pow2_const | 15881ns | -37670.2ns (-66.0%) | [-41177, -37654]ns | [15880, 15924] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 100426ns | +43361.2ns (+76.0%) | [+43355, +43367]ns | [100424, 100428] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 46685ns | -58.4% | +115.1% |
| 1 | 2 | 57066ns | -72.2% | +76.0% |
| 1 | 3 | 57069ns | -66.0% | +76.0% |
| 1 | 4 | 46683ns | -58.4% | +115.1% |
| 1 | 5 | 57069ns | -72.2% | +44.0% |
| 1 | 6 | 57066ns | -72.2% | +76.0% |
| 1 | 7 | 46682ns | -66.0% | +76.0% |
| 1 | 8 | 57076ns | -72.2% | +76.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.475 | moderate- |
| mod_pow2_const | 0.059 | ok |
| mod_var | -0.375 | moderate- |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 22.4ns | 53211.3ns | 0.0% |  |
| mod_pow2_const | 22.5ns | 18064.1ns | 0.1% |  |
| mod_var | 23.5ns | 95952.6ns | 0.0% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 46678.0-57135.5 ns)
  46678.0 |#################
  47200.8 |
  47723.7 |
  48246.6 |
  48769.5 |
  49292.3 |
  49815.2 |
  50338.1 |
  50861.0 |
  51383.8 |
  51906.7 |
  52429.6 |
  52952.5 |
  53475.3 |
  53998.2 |
  54521.1 |
  55044.0 |
  55566.9 |
  56089.7 |
  56612.6 |########################################
  (18 below, 8 above range)

mod_pow2_const (n=160, range 15869.5-23474.1 ns)
  15869.5 |########################################
  16249.7 |
  16629.9 |
  17010.2 |
  17390.4 |
  17770.6 |
  18150.9 |
  18531.1 |
  18911.3 |
  19291.6 |##########################
  19671.8 |
  20052.0 |
  20432.3 |
  20812.5 |
  21192.7 |
  21573.0 |
  21953.2 |
  22333.4 |
  22713.7 |
  23093.9 |
  (8 below, 2 above range)

mod_var (n=160, range 82154.8-100883.0 ns)
  82154.8 |#######
  83091.2 |
  84027.6 |
  84964.0 |
  85900.4 |
  86836.8 |
  87773.2 |
  88709.7 |
  89646.1 |
  90582.5 |
  91518.9 |
  92455.3 |
  93391.7 |
  94328.1 |
  95264.5 |
  96201.0 |
  97137.4 |
  98073.8 |
  99010.2 |
  99946.6 |########################################
  (19 below, 8 above range)

```

## Diagnostics

- **mod_pow2_const**: CV=53.4% (high variance, measurements may be unstable)
