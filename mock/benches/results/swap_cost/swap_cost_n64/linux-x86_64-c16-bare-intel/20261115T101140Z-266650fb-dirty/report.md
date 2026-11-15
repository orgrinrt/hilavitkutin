# replace_value blob install cost across value sizes

3 variants, 160 samples per variant.
Baseline: **sc_v1k**

## Highlights

Baseline for all deltas below: **sc_v1k**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sc_v64b dominates: 62% faster than the next best (sc_v1k)

sc_v64b (10 ns) leads sc_v1k (16 ns) by 62%, a clear separation rather than a photo finish. CV 99.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sc_v64b beats baseline by 38% (significant)

sc_v64b is -6 ns (38%) faster than baseline sc_v1k, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sc_v64k is an outlier: 213.5x slower than the field

sc_v64k (2.16 us) is 213.5x the fastest (10 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sc_v64b is fastest but the noisiest (CV 99.0%)

sc_v64b wins on median (10 ns) yet has the highest variance (CV 99.0%), while sc_v64k is the steadiest (CV 10.0%, 2.16 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 213.5x the fastest

Fastest sc_v64b (10 ns) to slowest sc_v64k (2.16 us): 213.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sc_v64b is inconsistent: worst-20% is 3.4x its best-20%

sc_v64b's best 20% of batches run at 10 ns but its worst 20% at 34 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sc_v64b** at 10.1 ns median (-38.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 213.46x (fastest 10.1 ns, slowest 2155.9 ns)

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
| Counter frequency | 2112779100 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sc_v1k | 2307ns | 1556ns | 1552ns | 1609ns | 5153ns | base |
| sc_v64b | 2428ns | 1548ns | 1543ns | 1811ns | 5162ns | +5.24% |
| sc_v64k | 3787ns | 3714ns | 3649ns | 3754ns | 4027ns | +64.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sc_v1k | 24ns | 16ns | 54ns | base | 2.621 |
| sc_v64b | 16ns | 10ns | 34ns | -34.09% | 3.977 |
| sc_v64k | 2215ns | 2091ns | 2400ns | +8970.86% | 0.029 |

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
| sc_v1k | 2307ns | 2307ns | base |
| sc_v64b | 2428ns | 2428ns | +5.24% |
| sc_v64k | 3787ns | 3787ns | +64.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sc_v1k | 16ns | base | --- | [16, 16] | --- | --- | --- | --- |
| sc_v64b | 10ns | -6.3ns (-38.4%) | [-6, -6]ns | [10, 10] | YES | 0.0000 | 0.0000 | 0 |
| sc_v64k | 2156ns | +2138.9ns (+13042.4%) | [+2132, +2143]ns | [2151, 2161] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sc_v1k | sc_v64b | sc_v64k |
|---|---|---|---|---|
| 1 | 1 | 16ns | -38.2% | +13095.1% |
| 1 | 2 | 16ns | -29.0% | +13009.8% |
| 1 | 3 | 18ns | -42.3% | +13083.4% |
| 1 | 4 | 16ns | -38.0% | +13061.3% |
| 1 | 5 | 16ns | -38.4% | +13972.3% |
| 1 | 6 | 16ns | -38.4% | +14107.0% |
| 1 | 7 | 16ns | -39.0% | +12655.5% |
| 1 | 8 | 16ns | -37.7% | +12724.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sc_v1k | -0.199 | ok |
| sc_v64b | -0.146 | ok |
| sc_v64k | -0.040 | ok |

**Consistency summary:**

- **sc_v64b**: won 8/8, lost 0/8
- **sc_v64k**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sc_v1k | 119.5ns | 24.4ns | 489.2% | HIGH |
| sc_v64b | 126.6ns | 16.1ns | 786.3% | HIGH |
| sc_v64k | 110.7ns | 2215.1ns | 5.0% |  |

## Distribution (algo ns)

```
sc_v1k (n=160, range 16.3-54.5 ns)
     16.3 |########################################
     18.2 |
     20.1 |
     22.0 |
     23.9 |
     25.8 |
     27.7 |
     29.7 |
     31.6 |
     33.5 |
     35.4 |
     37.3 |
     39.2 |
     41.1 |
     43.0 |
     44.9 |#
     46.9 |#
     48.8 |
     50.7 |##
     52.6 |
  (7 below, 16 above range)

sc_v64b (n=160, range 10.0-34.3 ns)
     10.0 |########################################
     11.2 |#####
     12.4 |
     13.6 |
     14.8 |
     16.1 |
     17.3 |
     18.5 |
     19.7 |
     20.9 |
     22.1 |
     23.4 |
     24.6 |
     25.8 |
     27.0 |#
     28.2 |#
     29.4 |#
     30.6 |#
     31.9 |#
     33.1 |###
  (6 below, 13 above range)

sc_v64k (n=160, range 2090.7-2400.5 ns)
   2090.7 |############################
   2106.2 |
   2121.7 |
   2137.2 |########################################
   2152.7 |####################################
   2168.2 |#
   2183.7 |
   2199.1 |
   2214.6 |
   2230.1 |
   2245.6 |
   2261.1 |
   2276.6 |
   2292.1 |###########################
   2307.6 |############################
   2323.0 |###################
   2338.5 |
   2354.0 |
   2369.5 |#
   2385.0 |#
  (17 below, 1 above range)

```

## Diagnostics

- **sc_v1k**: CV=63.5% (high variance, measurements may be unstable)
- **sc_v1k**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **sc_v1k**: bridge=491.5% of algo (FFI overhead may distort results)
- **sc_v64b**: CV=62.1% (high variance, measurements may be unstable)
- **sc_v64b**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **sc_v64b**: bridge=800.0% of algo (FFI overhead may distort results)
