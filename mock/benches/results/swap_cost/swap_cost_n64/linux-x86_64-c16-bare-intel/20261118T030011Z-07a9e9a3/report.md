# replace_value blob install cost across value sizes

3 variants, 160 samples per variant.
Baseline: **sc_v1k**

## Highlights

Baseline for all deltas below: **sc_v1k**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sc_v64b dominates: 61% faster than the next best (sc_v1k)

sc_v64b (10 ns) leads sc_v1k (16 ns) by 61%, a clear separation rather than a photo finish. CV 97.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sc_v64b beats baseline by 39% (significant)

sc_v64b is -6 ns (39%) faster than baseline sc_v1k, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sc_v64k is an outlier: 212.3x slower than the field

sc_v64k (2.14 us) is 212.3x the fastest (10 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sc_v64b is fastest but the noisiest (CV 97.1%)

sc_v64b wins on median (10 ns) yet has the highest variance (CV 97.1%), while sc_v64k is the steadiest (CV 10.7%, 2.14 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 212.3x the fastest

Fastest sc_v64b (10 ns) to slowest sc_v64k (2.14 us): 212.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sc_v64b is inconsistent: worst-20% is 3.4x its best-20%

sc_v64b's best 20% of batches run at 10 ns but its worst 20% at 34 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sc_v64b** at 10.1 ns median (-38.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 212.29x (fastest 10.1 ns, slowest 2144.2 ns)

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
| Counter frequency | 2112834712 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sc_v1k | 2316ns | 1560ns | 1555ns | 1619ns | 5167ns | base |
| sc_v64b | 2450ns | 1554ns | 1548ns | 1812ns | 5268ns | +5.81% |
| sc_v64k | 3775ns | 3706ns | 3645ns | 3705ns | 4117ns | +63.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sc_v1k | 24ns | 16ns | 54ns | base | 2.640 |
| sc_v64b | 16ns | 10ns | 34ns | -34.35% | 4.022 |
| sc_v64k | 2186ns | 2082ns | 2429ns | +8919.24% | 0.029 |

## Performance model

- Peak throughput: **6.404 Gops/s** (sc_v64b; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sc_v1k | 3.926 | 61.3% |
| sc_v64b | 6.337 | 98.9% |
| sc_v64k | 0.030 | 0.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sc_v1k | 2316ns | 2316ns | base |
| sc_v64b | 2450ns | 2450ns | +5.81% |
| sc_v64k | 3775ns | 3775ns | +63.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sc_v1k | 16ns | base | --- | [16, 16] | --- | --- | --- | --- |
| sc_v64b | 10ns | -6.3ns (-38.7%) | [-6, -6]ns | [10, 10] | YES | 0.0000 | 0.0000 | 0 |
| sc_v64k | 2144ns | +2125.0ns (+13036.8%) | [+2110, +2128]ns | [2142, 2146] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sc_v1k | sc_v64b | sc_v64k |
|---|---|---|---|---|
| 1 | 1 | 16ns | -38.4% | +12972.0% |
| 1 | 2 | 16ns | -38.0% | +14208.6% |
| 1 | 3 | 16ns | -35.6% | +12945.1% |
| 1 | 4 | 16ns | -38.4% | +12636.9% |
| 1 | 5 | 16ns | -38.7% | +13079.4% |
| 1 | 6 | 16ns | -35.1% | +12975.9% |
| 1 | 7 | 16ns | -38.0% | +12646.0% |
| 1 | 8 | 16ns | -38.0% | +13140.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sc_v1k | -0.475 | moderate- |
| sc_v64b | -0.410 | moderate- |
| sc_v64k | -0.096 | ok |

**Consistency summary:**

- **sc_v64b**: won 8/8, lost 0/8
- **sc_v64k**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sc_v1k | 120.0ns | 24.2ns | 495.0% | HIGH |
| sc_v64b | 127.7ns | 15.9ns | 802.5% | HIGH |
| sc_v64k | 114.4ns | 2186.3ns | 5.2% | HIGH |

## Distribution (algo ns)

```
sc_v1k (n=160, range 16.2-53.7 ns)
     16.2 |########################################
     18.1 |
     20.0 |
     21.9 |
     23.7 |
     25.6 |
     27.5 |
     29.4 |
     31.2 |
     33.1 |
     35.0 |
     36.9 |
     38.7 |
     40.6 |
     42.5 |
     44.4 |
     46.2 |#
     48.1 |
     50.0 |###
     51.9 |
  (19 below, 15 above range)

sc_v64b (n=160, range 10.0-33.9 ns)
     10.0 |########################################
     11.2 |#
     12.4 |
     13.6 |
     14.8 |
     16.0 |
     17.2 |
     18.3 |
     19.5 |
     20.7 |#
     21.9 |
     23.1 |
     24.3 |
     25.5 |
     26.7 |
     27.9 |
     29.1 |##
     30.3 |
     31.5 |#
     32.7 |#
  (2 below, 17 above range)

sc_v64k (n=160, range 2081.6-2429.2 ns)
   2081.6 |##################
   2099.0 |###
   2116.4 |#################
   2133.8 |########################################
   2151.1 |###########################
   2168.5 |##
   2185.9 |
   2203.3 |
   2220.7 |
   2238.0 |
   2255.4 |
   2272.8 |
   2290.2 |
   2307.5 |#
   2324.9 |##############
   2342.3 |
   2359.7 |
   2377.1 |
   2394.4 |
   2411.8 |
  (17 below, 5 above range)

```

## Diagnostics

- **sc_v1k**: CV=63.3% (high variance, measurements may be unstable)
- **sc_v1k**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **sc_v1k**: bridge=495.7% of algo (FFI overhead may distort results)
- **sc_v64b**: CV=61.6% (high variance, measurements may be unstable)
- **sc_v64b**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **sc_v64b**: bridge=804.0% of algo (FFI overhead may distort results)
