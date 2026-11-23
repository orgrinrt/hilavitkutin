# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 190% faster than the next best (mod_npow2_const)

mod_pow2_const (310 ns) leads mod_npow2_const (898 ns) by 190%, a clear separation rather than a photo finish. CV 75.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 65% (significant)

mod_pow2_const is -588 ns (65%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 5.1x slower than the field

mod_var (1.57 us) is 5.1x the fastest (310 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### mod_pow2_const is fastest but the noisiest (CV 75.9%)

mod_pow2_const wins on median (310 ns) yet has the highest variance (CV 75.9%), while mod_var is the steadiest (CV 36.3%, 1.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### mod_var shows alternating (throttle bounce) (autocorr -0.88)

mod_var's per-pass series has lag-1 autocorrelation -0.88, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.1x the fastest

Fastest mod_pow2_const (310 ns) to slowest mod_var (1.57 us): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### mod_pow2_const is inconsistent: worst-20% is 3.3x its best-20%

mod_pow2_const's best 20% of batches run at 253 ns but its worst 20% at 827 ns (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: mod_pow2_const** at 309.7 ns median (-65.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.08x (fastest 309.7 ns, slowest 1574.7 ns)

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
| mod_npow2_const | 3144ns | 2772ns | 2269ns | 2605ns | 5637ns | base |
| mod_pow2_const | 2781ns | 2184ns | 1788ns | 2067ns | 5916ns | -11.56% |
| mod_var | 3453ns | 3452ns | 2825ns | 3191ns | 4867ns | +9.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 1015ns | 734ns | 1813ns | base | 0.252 |
| mod_pow2_const | 392ns | 253ns | 827ns | -61.41% | 0.654 |
| mod_var | 1572ns | 1289ns | 2205ns | +54.88% | 0.163 |

## Performance model

- Peak throughput: **1.010 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.285 | 28.2% |
| mod_pow2_const | 0.827 | 81.8% |
| mod_var | 0.163 | 16.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 3144ns | 3144ns | base |
| mod_pow2_const | 2781ns | 2781ns | -11.56% |
| mod_var | 3453ns | 3453ns | +9.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 898ns | base | --- | [898, 898] | --- | --- | --- | --- |
| mod_pow2_const | 310ns | -587.9ns (-65.5%) | [-588, -481]ns | [310, 310] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 1575ns | +555.5ns (+61.9%) | [+555, +678]ns | [1301, 1576] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 898ns | -71.8% | +75.6% |
| 1 | 2 | 735ns | -65.5% | +75.6% |
| 1 | 3 | 898ns | -71.8% | +75.6% |
| 1 | 4 | 735ns | -65.5% | +75.6% |
| 1 | 5 | 735ns | -57.8% | +114.6% |
| 1 | 6 | 898ns | -65.5% | +43.6% |
| 1 | 7 | 734ns | -57.8% | +114.6% |
| 1 | 8 | 898ns | -65.5% | +43.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.625 | HIGH- (thermal bounce) |
| mod_pow2_const | 0.625 | HIGH+ (drift/warm-up) |
| mod_var | -0.875 | HIGH- (thermal bounce) |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 27.1ns | 1014.9ns | 2.7% |  |
| mod_pow2_const | 30.2ns | 391.7ns | 7.7% | HIGH |
| mod_var | 23.9ns | 1571.9ns | 1.5% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 734.4-1813.3 ns)
    734.4 |#########################
    788.4 |
    842.3 |
    896.3 |########################################
    950.2 |
   1004.1 |
   1058.1 |
   1112.0 |
   1166.0 |
   1219.9 |#
   1273.9 |
   1327.8 |
   1381.7 |#
   1435.7 |
   1489.6 |
   1543.6 |
   1597.5 |
   1651.5 |
   1705.4 |
   1759.4 |
  (19 below, 18 above range)

mod_pow2_const (n=160, range 253.4-827.2 ns)
    253.4 |##############################
    282.1 |########################################
    310.7 |#
    339.4 |
    368.1 |
    396.8 |
    425.5 |
    454.2 |
    482.9 |
    511.6 |
    540.3 |
    569.0 |
    597.7 |
    626.4 |#
    655.1 |
    683.8 |#
    712.5 |
    741.2 |
    769.8 |#
    798.5 |
  (13 below, 18 above range)

mod_var (n=160, range 1289.4-2204.6 ns)
   1289.4 |###################################
   1335.2 |
   1380.9 |
   1426.7 |
   1472.4 |
   1518.2 |
   1564.0 |########################################
   1609.7 |
   1655.5 |
   1701.2 |
   1747.0 |
   1792.8 |
   1838.5 |
   1884.3 |
   1930.0 |
   1975.8 |
   2021.6 |
   2067.3 |
   2113.1 |
   2158.8 |
  (4 below, 9 above range)

```

## Diagnostics

- **mod_npow2_const**: CV=50.1% (high variance, measurements may be unstable)
- **mod_pow2_const**: CV=60.0% (high variance, measurements may be unstable)
- **mod_pow2_const**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **mod_pow2_const**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **mod_var**: CV=36.3% (high variance, measurements may be unstable)
