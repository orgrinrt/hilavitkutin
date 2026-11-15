# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 141% faster than the next best (mod_npow2_const)

mod_pow2_const (1.22 us) leads mod_npow2_const (2.93 us) by 141%, a clear separation rather than a photo finish. CV 44.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 58% (significant)

mod_pow2_const is -1.71 us (58%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 5.2x slower than the field

mod_var (6.29 us) is 5.2x the fastest (1.22 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### mod_pow2_const is fastest but the noisiest (CV 44.6%)

mod_pow2_const wins on median (1.22 us) yet has the highest variance (CV 44.6%), while mod_var is the steadiest (CV 0.3%, 6.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### mod_var shows alternating (throttle bounce) (autocorr -0.50)

mod_var's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.2x the fastest

Fastest mod_pow2_const (1.22 us) to slowest mod_var (6.29 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### mod_pow2_const is inconsistent: worst-20% is 2.0x its best-20%

mod_pow2_const's best 20% of batches run at 997 ns but its worst 20% at 1.96 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: mod_pow2_const** at 1218.1 ns median (-58.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.16x (fastest 1218.1 ns, slowest 6288.8 ns)

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
| mod_npow2_const | 4837ns | 4468ns | 4453ns | 4755ns | 5467ns | base |
| mod_pow2_const | 3302ns | 3093ns | 2530ns | 2985ns | 5026ns | -31.72% |
| mod_var | 8173ns | 8166ns | 8156ns | 8166ns | 8211ns | +68.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 3168ns | 2921ns | 3578ns | base | 0.323 |
| mod_pow2_const | 1297ns | 997ns | 1964ns | -59.06% | 0.790 |
| mod_var | 6287ns | 6278ns | 6300ns | +98.48% | 0.163 |

## Performance model

- Peak throughput: **1.027 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.350 | 34.0% |
| mod_pow2_const | 0.841 | 81.8% |
| mod_var | 0.163 | 15.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 4837ns | 4837ns | base |
| mod_pow2_const | 3302ns | 3302ns | -31.72% |
| mod_var | 8173ns | 8173ns | +68.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 2930ns | base | --- | [2929, 2931] | --- | --- | --- | --- |
| mod_pow2_const | 1218ns | -1711.7ns (-58.4%) | [-1924, -1703]ns | [1218, 1218] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 6289ns | +3357.1ns (+114.6%) | [+3348, +3358]ns | [6288, 6289] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 2921ns | -65.9% | +115.3% |
| 1 | 2 | 3569ns | -72.1% | +76.2% |
| 1 | 3 | 2921ns | -58.3% | +115.3% |
| 1 | 4 | 3570ns | -65.9% | +76.2% |
| 1 | 5 | 3570ns | -72.1% | +76.2% |
| 1 | 6 | 2921ns | -58.3% | +115.3% |
| 1 | 7 | 2921ns | -58.3% | +115.3% |
| 1 | 8 | 2921ns | -58.3% | +115.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.141 | ok |
| mod_pow2_const | 0.058 | ok |
| mod_var | -0.502 | HIGH- (thermal bounce) |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 21.2ns | 3167.7ns | 0.7% |  |
| mod_pow2_const | 25.4ns | 1296.9ns | 2.0% |  |
| mod_var | 24.2ns | 6287.1ns | 0.4% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 2920.5-3577.5 ns)
   2920.5 |########################################
   2953.4 |
   2986.2 |
   3019.1 |
   3051.9 |
   3084.8 |
   3117.6 |
   3150.5 |
   3183.3 |
   3216.2 |
   3249.0 |
   3281.9 |
   3314.7 |
   3347.6 |
   3380.4 |
   3413.3 |
   3446.1 |
   3479.0 |
   3511.8 |
   3544.7 |##################
  (16 below, 21 above range)

mod_pow2_const (n=160, range 996.7-1963.6 ns)
    996.7 |###########
   1045.1 |
   1093.4 |
   1141.7 |
   1190.1 |########################################
   1238.4 |
   1286.8 |
   1335.1 |
   1383.5 |
   1431.8 |
   1480.2 |
   1528.5 |
   1576.9 |
   1625.2 |
   1673.6 |
   1721.9 |
   1770.3 |
   1818.6 |
   1867.0 |
   1915.3 |
  (23 below, 13 above range)

mod_var (n=160, range 6277.8-6299.7 ns)
   6277.8 |#################################
   6278.9 |
   6280.0 |
   6281.1 |
   6282.2 |
   6283.3 |
   6284.4 |
   6285.5 |
   6286.6 |
   6287.7 |##############
   6288.8 |########################################
   6289.9 |##########
   6291.0 |####
   6292.0 |
   6293.1 |#
   6294.2 |
   6295.3 |
   6296.4 |
   6297.5 |#
   6298.6 |
  (15 below, 3 above range)

```

## Diagnostics

- **mod_pow2_const**: CV=41.9% (high variance, measurements may be unstable)
