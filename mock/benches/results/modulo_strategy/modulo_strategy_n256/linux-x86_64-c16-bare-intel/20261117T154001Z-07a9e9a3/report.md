# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 192% faster than the next best (mod_npow2_const)

mod_pow2_const (254 ns) leads mod_npow2_const (740 ns) by 192%, a clear separation rather than a photo finish. CV 93.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 65% (significant)

mod_pow2_const is -481 ns (65%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 6.2x slower than the field

mod_var (1.58 us) is 6.2x the fastest (254 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### mod_pow2_const is fastest but the noisiest (CV 93.7%)

mod_pow2_const wins on median (254 ns) yet has the highest variance (CV 93.7%), while mod_var is the steadiest (CV 31.2%, 1.58 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### mod_pow2_const shows warm-up / thermal drift (autocorr +0.54)

mod_pow2_const's per-pass series has lag-1 autocorrelation +0.54, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.2x the fastest

Fastest mod_pow2_const (254 ns) to slowest mod_var (1.58 us): 6.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### mod_pow2_const is inconsistent: worst-20% is 3.3x its best-20%

mod_pow2_const's best 20% of batches run at 253 ns but its worst 20% at 837 ns (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: mod_pow2_const** at 253.5 ns median (-65.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.22x (fastest 253.5 ns, slowest 1576.1 ns)

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
| mod_npow2_const | 3183ns | 2289ns | 2273ns | 2500ns | 6140ns | base |
| mod_pow2_const | 2670ns | 1797ns | 1791ns | 1865ns | 5962ns | -16.11% |
| mod_var | 3470ns | 3456ns | 2830ns | 3306ns | 4601ns | +9.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 1025ns | 734ns | 1968ns | base | 0.250 |
| mod_pow2_const | 376ns | 253ns | 837ns | -63.30% | 0.681 |
| mod_var | 1578ns | 1290ns | 2085ns | +54.05% | 0.162 |

## Performance model

- Peak throughput: **1.011 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.346 | 34.2% |
| mod_pow2_const | 1.010 | 99.9% |
| mod_var | 0.162 | 16.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 3183ns | 3183ns | base |
| mod_pow2_const | 2670ns | 2670ns | -16.11% |
| mod_var | 3470ns | 3470ns | +9.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 740ns | base | --- | [734, 897] | --- | --- | --- | --- |
| mod_pow2_const | 254ns | -481.1ns (-65.0%) | [-493, -481]ns | [253, 254] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 1576ns | +566.7ns (+76.6%) | [+555, +679]ns | [1576, 1576] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 734ns | -65.5% | +75.6% |
| 1 | 2 | 898ns | -71.8% | +75.6% |
| 1 | 3 | 1474ns | -82.8% | +6.9% |
| 1 | 4 | 734ns | -65.5% | +114.6% |
| 1 | 5 | 898ns | -71.8% | +75.6% |
| 1 | 6 | 734ns | -65.5% | +75.6% |
| 1 | 7 | 734ns | -65.5% | +114.6% |
| 1 | 8 | 734ns | -65.5% | +75.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.085 | ok |
| mod_pow2_const | 0.543 | HIGH+ (drift/warm-up) |
| mod_var | -0.275 | moderate- |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 27.4ns | 1024.5ns | 2.7% |  |
| mod_pow2_const | 28.9ns | 376.0ns | 7.7% | HIGH |
| mod_var | 24.3ns | 1578.3ns | 1.5% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 734.3-1968.2 ns)
    734.3 |########################################
    796.0 |
    857.7 |#####################
    919.4 |#
    981.1 |#
   1042.8 |
   1104.5 |#
   1166.2 |#
   1227.9 |
   1289.6 |
   1351.2 |
   1412.9 |
   1474.6 |
   1536.3 |
   1598.0 |
   1659.7 |#
   1721.4 |
   1783.1 |
   1844.8 |#
   1906.5 |
  (22 below, 16 above range)

mod_pow2_const (n=160, range 253.3-837.4 ns)
    253.3 |########################################
    282.5 |
    311.7 |
    340.9 |
    370.1 |
    399.3 |
    428.5 |
    457.7 |
    486.9 |
    516.2 |
    545.4 |
    574.6 |
    603.8 |
    633.0 |
    662.2 |
    691.4 |
    720.6 |
    749.8 |
    779.0 |#
    808.2 |#
  (19 below, 17 above range)

mod_var (n=160, range 1289.5-2084.8 ns)
   1289.5 |#####################
   1329.3 |
   1369.0 |
   1408.8 |
   1448.6 |
   1488.3 |
   1528.1 |
   1567.9 |########################################
   1607.6 |
   1647.4 |
   1687.1 |
   1726.9 |
   1766.7 |
   1806.4 |
   1846.2 |
   1886.0 |
   1925.7 |
   1965.5 |
   2005.2 |
   2045.0 |
  (5 below, 9 above range)

```

## Diagnostics

- **mod_npow2_const**: CV=52.2% (high variance, measurements may be unstable)
- **mod_pow2_const**: CV=63.2% (high variance, measurements may be unstable)
- **mod_pow2_const**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **mod_pow2_const**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **mod_var**: CV=31.2% (high variance, measurements may be unstable)
