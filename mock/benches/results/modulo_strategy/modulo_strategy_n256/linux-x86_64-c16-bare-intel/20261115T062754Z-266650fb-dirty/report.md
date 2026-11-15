# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 190% faster than the next best (mod_npow2_const)

mod_pow2_const (310 ns) leads mod_npow2_const (898 ns) by 190%, a clear separation rather than a photo finish. CV 81.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 65% (significant)

mod_pow2_const is -588 ns (65%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 5.1x slower than the field

mod_var (1.58 us) is 5.1x the fastest (310 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### mod_pow2_const is fastest but the noisiest (CV 81.0%)

mod_pow2_const wins on median (310 ns) yet has the highest variance (CV 81.0%), while mod_var is the steadiest (CV 43.5%, 1.58 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.1x the fastest

Fastest mod_pow2_const (310 ns) to slowest mod_var (1.58 us): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### mod_pow2_const is inconsistent: worst-20% is 3.5x its best-20%

mod_pow2_const's best 20% of batches run at 253 ns but its worst 20% at 890 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: mod_pow2_const** at 309.7 ns median (-65.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.09x (fastest 309.7 ns, slowest 1576.0 ns)

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
| mod_npow2_const | 3114ns | 2772ns | 2268ns | 2671ns | 5289ns | base |
| mod_pow2_const | 2930ns | 2187ns | 1786ns | 2176ns | 6333ns | -5.92% |
| mod_var | 3727ns | 3449ns | 2822ns | 3201ns | 6210ns | +19.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 1004ns | 734ns | 1695ns | base | 0.255 |
| mod_pow2_const | 414ns | 253ns | 890ns | -58.79% | 0.619 |
| mod_var | 1723ns | 1289ns | 2940ns | +71.52% | 0.149 |

## Performance model

- Peak throughput: **1.010 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.285 | 28.2% |
| mod_pow2_const | 0.827 | 81.8% |
| mod_var | 0.162 | 16.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 3114ns | 3114ns | base |
| mod_pow2_const | 2930ns | 2930ns | -5.92% |
| mod_var | 3727ns | 3727ns | +19.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 898ns | base | --- | [898, 898] | --- | --- | --- | --- |
| mod_pow2_const | 310ns | -587.7ns (-65.5%) | [-588, -588]ns | [310, 310] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 1576ns | +666.2ns (+74.2%) | [+555, +679]ns | [1301, 1576] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 734ns | -65.5% | +114.6% |
| 1 | 2 | 898ns | -71.8% | +43.7% |
| 1 | 3 | 898ns | -65.5% | +214.6% |
| 1 | 4 | 734ns | -65.5% | +114.6% |
| 1 | 5 | 898ns | -65.5% | +43.7% |
| 1 | 6 | 898ns | -65.5% | +43.6% |
| 1 | 7 | 735ns | -57.8% | +75.6% |
| 1 | 8 | 898ns | -65.5% | +75.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.475 | moderate- |
| mod_pow2_const | 0.058 | ok |
| mod_var | -0.103 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 26.7ns | 1004.3ns | 2.7% |  |
| mod_pow2_const | 31.7ns | 413.9ns | 7.7% | HIGH |
| mod_var | 25.7ns | 1722.6ns | 1.5% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 734.4-1695.4 ns)
    734.4 |###################
    782.4 |
    830.5 |
    878.5 |########################################
    926.6 |
    974.6 |
   1022.7 |
   1070.7 |
   1118.8 |
   1166.8 |
   1214.9 |
   1262.9 |
   1311.0 |
   1359.1 |
   1407.1 |
   1455.2 |
   1503.2 |
   1551.3 |
   1599.3 |
   1647.4 |
  (9 below, 16 above range)

mod_pow2_const (n=160, range 253.4-889.9 ns)
    253.4 |##################
    285.2 |########################################
    317.0 |#
    348.9 |
    380.7 |
    412.5 |
    444.3 |
    476.2 |
    508.0 |
    539.8 |
    571.6 |
    603.5 |
    635.3 |
    667.1 |##
    698.9 |
    730.8 |##
    762.6 |#
    794.4 |#
    826.2 |#
    858.1 |#
  (8 below, 15 above range)

mod_var (n=160, range 1289.4-2940.0 ns)
   1289.4 |#######################################
   1372.0 |
   1454.5 |
   1537.0 |########################################
   1619.6 |
   1702.1 |
   1784.6 |
   1867.1 |
   1949.7 |
   2032.2 |
   2114.7 |
   2197.3 |
   2279.8 |
   2362.3 |
   2444.8 |
   2527.4 |
   2609.9 |
   2692.4 |
   2775.0 |#############
   2857.5 |
  (14 below, 7 above range)

```

## Diagnostics

- **mod_npow2_const**: CV=49.7% (high variance, measurements may be unstable)
- **mod_pow2_const**: CV=60.6% (high variance, measurements may be unstable)
- **mod_pow2_const**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **mod_var**: CV=39.8% (high variance, measurements may be unstable)
