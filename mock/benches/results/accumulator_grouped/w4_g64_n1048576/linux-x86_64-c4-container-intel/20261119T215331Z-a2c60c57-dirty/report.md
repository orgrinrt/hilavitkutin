# W = 4, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 2.1x slower than the field

sequential (4.97 ms) is 2.1x the fastest (2.34 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### per_core is fastest but the noisiest (CV 8.3%)

per_core wins on median (2.34 ms) yet has the highest variance (CV 8.3%), while grouped is the steadiest (CV 5.0%, 2.56 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Speed leader per_core vs stability leader grouped (+9% speed for 1.6x steadier)

per_core is fastest (2.34 ms, CV 8.3%); grouped gives up 9.4% median for 1.6x lower variance (CV 5.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: per_core** at 2337265.6 ns median (-8.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.13x (fastest 2337265.6 ns, slowest 4970180.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 2549837ns | 2559610ns | 2342302ns | 2530080ns | 2683240ns | base |
| per_core | 2323648ns | 2339337ns | 2018009ns | 2286683ns | 2531914ns | -8.87% |
| sequential | 5046833ns | 4972943ns | 4547966ns | 4924618ns | 5479588ns | +97.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 2547212ns | 2339882ns | 2680097ns | base | 0.412 |
| per_core | 2321491ns | 2016097ns | 2529706ns | -8.86% | 0.452 |
| sequential | 5044350ns | 4545717ns | 5476784ns | +98.03% | 0.208 |

## Performance model

- Peak throughput: **0.520 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.410 | 78.9% |
| per_core | 0.449 | 86.3% |
| sequential | 0.211 | 40.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 2549837ns | 2549837ns | base |
| per_core | 2323648ns | 2323648ns | -8.87% |
| sequential | 5046833ns | 5046833ns | +97.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 2556697ns | base | --- | [2469229, 2614470] | --- | --- | --- | --- |
| per_core | 2337266ns | -235803.4ns (-9.2%) | [-337533, -65706]ns | [2215056, 2445030] | YES | 0.0013 | 0.0013 | 0 |
| sequential | 4970180ns | +2561281.9ns (+100.2%) | [+2145396, +2750308]ns | [4769596, 5278341] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 2587021ns | -3.8% | +87.4% |
| 1 | 2 | 2564934ns | -7.6% | +92.0% |
| 1 | 3 | 2548136ns | -12.0% | +104.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.004 | ok |
| per_core | -0.000 | ok |
| sequential | -0.069 | ok |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 2671035.6ns | 2547212.1ns | 104.9% | HIGH |
| per_core | 2285518.4ns | 2321491.1ns | 98.5% | HIGH |
| sequential | 4960062.5ns | 5044350.3ns | 98.3% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 2339882.3-2680097.4 ns)
  2339882.3 |
  2356893.0 |
  2373903.8 |########################################
  2390914.5 |
  2407925.3 |
  2424936.1 |####################
  2441946.8 |
  2458957.6 |########################################
  2475968.3 |
  2492979.1 |
  2509989.9 |####################
  2527000.6 |
  2544011.4 |########################################
  2561022.1 |####################
  2578032.9 |####################
  2595043.7 |########################################
  2612054.4 |####################
  2629065.2 |
  2646075.9 |
  2663086.7 |####################
  (1 below, 3 above range)

per_core (n=18, range 2016097.4-2529706.5 ns)
  2016097.4 |
  2041777.9 |####################
  2067458.3 |
  2093138.8 |
  2118819.2 |
  2144499.7 |
  2170180.1 |####################
  2195860.6 |####################
  2221541.0 |########################################
  2247221.5 |####################
  2272901.9 |
  2298582.4 |####################
  2324262.8 |
  2349943.3 |########################################
  2375623.7 |
  2401304.2 |########################################
  2426984.6 |
  2452665.1 |########################################
  2478345.5 |
  2504026.0 |####################
  (2 below, 2 above range)

sequential (n=18, range 4545716.6-5476784.3 ns)
  4545716.6 |
  4592270.0 |
  4638823.4 |####################
  4685376.8 |####################
  4731930.1 |####################
  4778483.5 |########################################
  4825036.9 |
  4871590.3 |########################################
  4918143.7 |
  4964697.1 |
  5011250.5 |####################
  5057803.9 |
  5104357.2 |
  5150910.6 |####################
  5197464.0 |########################################
  5244017.4 |
  5290570.8 |####################
  5337124.2 |########################################
  5383677.6 |####################
  5430230.9 |
  (2 below, 1 above range)

```

## Diagnostics

- **grouped**: bridge=104.6% of algo (FFI overhead may distort results)
- **per_core**: bridge=98.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.9% of algo (FFI overhead may distort results)
