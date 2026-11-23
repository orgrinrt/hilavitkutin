# replace_value blob install cost across value sizes

3 variants, 160 samples per variant.
Baseline: **sc_v1k**

## Highlights

Baseline for all deltas below: **sc_v1k**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sc_v64b dominates: 62% faster than the next best (sc_v1k)

sc_v64b (10 ns) leads sc_v1k (16 ns) by 62%, a clear separation rather than a photo finish. CV 96.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sc_v64b beats baseline by 38% (significant)

sc_v64b is -6 ns (38%) faster than baseline sc_v1k, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sc_v64k is an outlier: 212.7x slower than the field

sc_v64k (2.15 us) is 212.7x the fastest (10 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sc_v64b is fastest but the noisiest (CV 96.7%)

sc_v64b wins on median (10 ns) yet has the highest variance (CV 96.7%), while sc_v64k is the steadiest (CV 6.2%, 2.15 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 212.7x the fastest

Fastest sc_v64b (10 ns) to slowest sc_v64k (2.15 us): 212.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sc_v64b is inconsistent: worst-20% is 3.4x its best-20%

sc_v64b's best 20% of batches run at 10 ns but its worst 20% at 34 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sc_v64b** at 10.1 ns median (-38.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 212.70x (fastest 10.1 ns, slowest 2148.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | replace_value blob install cost across value sizes |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000001 |
| Counter frequency | 2112846322 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sc_v1k | 2381ns | 1553ns | 1548ns | 1738ns | 5143ns | base |
| sc_v64b | 2387ns | 1546ns | 1541ns | 1704ns | 5283ns | +0.28% |
| sc_v64k | 3728ns | 3706ns | 3662ns | 3706ns | 3861ns | +56.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sc_v1k | 25ns | 16ns | 54ns | base | 2.550 |
| sc_v64b | 15ns | 10ns | 34ns | -38.47% | 4.145 |
| sc_v64k | 2155ns | 2104ns | 2238ns | +8489.13% | 0.030 |

## Performance model

- Peak throughput: **6.412 Gops/s** (sc_v64b; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sc_v1k | 3.902 | 60.9% |
| sc_v64b | 6.337 | 98.8% |
| sc_v64k | 0.030 | 0.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sc_v1k | 2381ns | 2381ns | base |
| sc_v64b | 2387ns | 2387ns | +0.28% |
| sc_v64k | 3728ns | 3728ns | +56.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sc_v1k | 16ns | base | --- | [16, 16] | --- | --- | --- | --- |
| sc_v64b | 10ns | -6.3ns (-38.4%) | [-6, -6]ns | [10, 10] | YES | 0.0000 | 0.0000 | 0 |
| sc_v64k | 2148ns | +2120.2ns (+12928.4%) | [+2113, +2128]ns | [2142, 2151] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sc_v1k | sc_v64b | sc_v64k |
|---|---|---|---|---|
| 1 | 1 | 16ns | -38.4% | +13050.3% |
| 1 | 2 | 16ns | -38.0% | +12935.3% |
| 1 | 3 | 17ns | -40.2% | +12572.8% |
| 1 | 4 | 16ns | -38.4% | +12639.9% |
| 1 | 5 | 16ns | -38.4% | +13024.4% |
| 1 | 6 | 16ns | -38.4% | +13129.0% |
| 1 | 7 | 16ns | -38.0% | +13090.8% |
| 1 | 8 | 16ns | -38.0% | +12930.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sc_v1k | -0.175 | ok |
| sc_v64b | 0.000 | ok |
| sc_v64k | -0.115 | ok |

**Consistency summary:**

- **sc_v64b**: won 8/8, lost 0/8
- **sc_v64k**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sc_v1k | 123.5ns | 25.1ns | 492.3% | HIGH |
| sc_v64b | 124.9ns | 15.4ns | 808.8% | HIGH |
| sc_v64k | 110.2ns | 2155.3ns | 5.1% | HIGH |

## Distribution (algo ns)

```
sc_v1k (n=160, range 16.3-54.2 ns)
     16.3 |########################################
     18.1 |
     20.0 |
     21.9 |
     23.8 |
     25.7 |
     27.6 |
     29.5 |
     31.4 |
     33.3 |
     35.2 |
     37.1 |
     39.0 |
     40.9 |
     42.8 |
     44.7 |
     46.6 |##
     48.5 |#
     50.4 |#
     52.3 |#
  (16 below, 15 above range)

sc_v64b (n=160, range 10.0-34.0 ns)
     10.0 |########################################
     11.2 |
     12.4 |
     13.6 |
     14.8 |
     16.0 |
     17.2 |
     18.4 |
     19.6 |
     20.8 |
     22.0 |
     23.2 |
     24.4 |
     25.6 |
     26.8 |
     28.0 |
     29.2 |
     30.4 |#
     31.6 |##
     32.8 |#
  (6 below, 16 above range)

sc_v64k (n=160, range 2103.7-2237.8 ns)
   2103.7 |#
   2110.4 |
   2117.1 |########################
   2123.8 |########################
   2130.5 |####################
   2137.2 |##########################
   2143.9 |#############################
   2150.6 |########################################
   2157.3 |###################
   2164.0 |#############################
   2170.8 |#############
   2177.5 |#####
   2184.2 |#
   2190.9 |
   2197.6 |
   2204.3 |
   2211.0 |#
   2217.7 |#
   2224.4 |#
   2231.1 |
  (18 below, 3 above range)

```

## Diagnostics

- **sc_v1k**: CV=62.3% (high variance, measurements may be unstable)
- **sc_v1k**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **sc_v1k**: bridge=492.1% of algo (FFI overhead may distort results)
- **sc_v64b**: CV=63.2% (high variance, measurements may be unstable)
- **sc_v64b**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **sc_v64b**: bridge=802.0% of algo (FFI overhead may distort results)
