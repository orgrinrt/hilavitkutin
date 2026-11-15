# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 194% faster than the next best (mod_npow2_const)

mod_pow2_const (19.41 us) leads mod_npow2_const (57.06 us) by 194%, a clear separation rather than a photo finish. CV 8.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 66% (significant)

mod_pow2_const is -37.66 us (66%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 5.2x slower than the field

mod_var (100.43 us) is 5.2x the fastest (19.41 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### mod_pow2_const is fastest but the noisiest (CV 8.8%)

mod_pow2_const wins on median (19.41 us) yet has the highest variance (CV 8.8%), while mod_var is the steadiest (CV 0.1%, 100.43 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.2x the fastest

Fastest mod_pow2_const (19.41 us) to slowest mod_var (100.43 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: mod_pow2_const** at 19405.5 ns median (-66.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.18x (fastest 19405.5 ns, slowest 100428.5 ns)

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
| mod_npow2_const | 56278ns | 58943ns | 48219ns | 58058ns | 58998ns | base |
| mod_pow2_const | 19842ns | 21285ns | 17413ns | 20162ns | 21309ns | -64.74% |
| mod_var | 102346ns | 102323ns | 102311ns | 102323ns | 102452ns | +81.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 54476ns | 46679ns | 57096ns | base | 0.301 |
| mod_pow2_const | 18083ns | 15872ns | 19412ns | -66.81% | 0.906 |
| mod_var | 100448ns | 100417ns | 100534ns | +84.39% | 0.163 |

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
| mod_npow2_const | 56278ns | 56278ns | base |
| mod_pow2_const | 19842ns | 19842ns | -64.74% |
| mod_var | 102346ns | 102346ns | +81.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 57061ns | base | --- | [57056, 57063] | --- | --- | --- | --- |
| mod_pow2_const | 19406ns | -37661.0ns (-66.0%) | [-37669, -37659]ns | [19396, 19407] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 100428ns | +43373.1ns (+76.0%) | [+43369, +43376]ns | [100428, 100430] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 57060ns | -72.2% | +76.0% |
| 1 | 2 | 46679ns | -58.4% | +115.1% |
| 1 | 3 | 57064ns | -72.2% | +76.0% |
| 1 | 4 | 46682ns | -58.4% | +115.1% |
| 1 | 5 | 57064ns | -66.0% | +76.0% |
| 1 | 6 | 57068ns | -66.0% | +76.0% |
| 1 | 7 | 57065ns | -72.2% | +76.0% |
| 1 | 8 | 57064ns | -66.0% | +76.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.375 | moderate- |
| mod_pow2_const | -0.475 | moderate- |
| mod_var | 0.104 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 22.9ns | 54475.7ns | 0.0% |  |
| mod_pow2_const | 22.4ns | 18083.1ns | 0.1% |  |
| mod_var | 24.4ns | 100447.6ns | 0.0% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 46678.8-57096.1 ns)
  46678.8 |########
  47199.7 |
  47720.6 |
  48241.4 |
  48762.3 |
  49283.2 |
  49804.0 |
  50324.9 |
  50845.8 |
  51366.6 |
  51887.5 |
  52408.4 |
  52929.2 |
  53450.1 |
  53970.9 |
  54491.8 |
  55012.7 |
  55533.5 |
  56054.4 |
  56575.3 |########################################
  (15 below, 3 above range)

mod_pow2_const (n=160, range 15872.0-19412.4 ns)
  15872.0 |################
  16049.0 |
  16226.0 |
  16403.0 |
  16580.1 |
  16757.1 |
  16934.1 |
  17111.1 |
  17288.1 |
  17465.2 |
  17642.2 |
  17819.2 |
  17996.2 |
  18173.3 |
  18350.3 |
  18527.3 |
  18704.3 |
  18881.3 |
  19058.4 |
  19235.4 |########################################
  (23 below, 12 above range)

mod_var (n=160, range 100417.5-100533.9 ns)
  100417.5 |##################
  100423.3 |########################################
  100429.1 |###########################
  100434.9 |##################
  100440.7 |######
  100446.6 |###
  100452.4 |
  100458.2 |
  100464.0 |
  100469.8 |
  100475.7 |
  100481.5 |
  100487.3 |
  100493.1 |
  100498.9 |
  100504.8 |
  100510.6 |
  100516.4 |
  100522.2 |
  100528.0 |
  (15 below, 6 above range)

```
