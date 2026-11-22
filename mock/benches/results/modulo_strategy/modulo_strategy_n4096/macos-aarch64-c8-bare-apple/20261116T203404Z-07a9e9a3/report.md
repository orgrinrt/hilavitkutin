# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 121% faster than the next best (mod_npow2_const)

mod_pow2_const (6.35 us) leads mod_npow2_const (14.04 us) by 121%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 55% (significant)

mod_pow2_const is -7.69 us (55%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.8x slower than the field

mod_var (17.83 us) is 2.8x the fastest (6.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: mod_pow2_const** at 6347.7 ns median (-54.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.81x (fastest 6347.7 ns, slowest 17833.9 ns)

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
| mod_npow2_const | 16204ns | 16154ns | 16139ns | 16168ns | 16375ns | base |
| mod_pow2_const | 8475ns | 8450ns | 8443ns | 8458ns | 8559ns | -47.70% |
| mod_var | 19988ns | 19957ns | 19929ns | 19964ns | 20120ns | +23.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 14087ns | 14035ns | 14233ns | base | 0.291 |
| mod_pow2_const | 6366ns | 6343ns | 6432ns | -54.81% | 0.643 |
| mod_var | 17863ns | 17813ns | 17985ns | +26.81% | 0.229 |

## Performance model

- Peak throughput: **0.646 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.292 | 45.2% |
| mod_pow2_const | 0.645 | 99.9% |
| mod_var | 0.230 | 35.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 16204ns | 16204ns | base |
| mod_pow2_const | 8475ns | 8475ns | -47.70% |
| mod_var | 19988ns | 19988ns | +23.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 14041ns | base | --- | [14039, 14050] | --- | --- | --- | --- |
| mod_pow2_const | 6348ns | -7692.9ns (-54.8%) | [-7695, -7692]ns | [6347, 6349] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 17834ns | +3780.6ns (+26.9%) | [+3778, +3786]ns | [17819, 17844] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 14038ns | -54.8% | +27.1% |
| 1 | 2 | 14039ns | -54.8% | +27.0% |
| 1 | 3 | 14041ns | -54.8% | +27.1% |
| 1 | 4 | 14048ns | -54.8% | +26.8% |
| 1 | 5 | 14071ns | -54.9% | +26.7% |
| 1 | 6 | 14084ns | -54.9% | +26.5% |
| 1 | 7 | 14041ns | -54.7% | +26.9% |
| 1 | 8 | 14046ns | -54.8% | +26.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | 0.313 | moderate+ |
| mod_pow2_const | -0.300 | moderate- |
| mod_var | 0.022 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 2.9ns | 14086.5ns | 0.0% |  |
| mod_pow2_const | 2.6ns | 6365.6ns | 0.0% |  |
| mod_var | 3.3ns | 17863.2ns | 0.0% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 14035.3-14233.2 ns)
  14035.3 |########################################
  14045.2 |##
  14055.1 |##
  14065.0 |#
  14074.9 |####
  14084.8 |##
  14094.6 |#####
  14104.5 |#
  14114.4 |#
  14124.3 |##
  14134.2 |#
  14144.1 |
  14154.0 |
  14163.9 |##
  14173.8 |
  14183.7 |
  14193.6 |
  14203.5 |#
  14213.4 |##
  14223.3 |
  (14 below, 7 above range)

mod_pow2_const (n=160, range 6343.5-6431.8 ns)
   6343.5 |########################################
   6347.9 |###################
   6352.3 |###
   6356.7 |
   6361.1 |
   6365.5 |
   6370.0 |
   6374.4 |#
   6378.8 |##
   6383.2 |
   6387.6 |#
   6392.1 |###
   6396.5 |
   6400.9 |#
   6405.3 |
   6409.7 |#
   6414.1 |
   6418.6 |
   6423.0 |
   6427.4 |
  (14 below, 16 above range)

mod_var (n=160, range 17812.9-17985.3 ns)
  17812.9 |########################################
  17821.5 |##
  17830.2 |####
  17838.8 |########
  17847.4 |####
  17856.0 |####
  17864.6 |###
  17873.2 |####
  17881.9 |###
  17890.5 |####
  17899.1 |####
  17907.7 |##
  17916.3 |
  17924.9 |#
  17933.6 |
  17942.2 |
  17950.8 |##
  17959.4 |
  17968.0 |
  17976.6 |
  (16 below, 9 above range)

```
