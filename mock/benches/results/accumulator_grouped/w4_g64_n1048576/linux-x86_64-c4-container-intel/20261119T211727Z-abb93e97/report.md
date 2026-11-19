# W = 4, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core dominates: 12% faster than the next best (grouped)

per_core (2.31 ms) leads grouped (2.59 ms) by 12%, a clear separation rather than a photo finish. CV 5.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential is an outlier: 2.1x slower than the field

sequential (4.87 ms) is 2.1x the fastest (2.31 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.54)

sequential's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: per_core** at 2313251.6 ns median (-10.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.10x (fastest 2313251.6 ns, slowest 4868112.4 ns)

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
| Counter frequency | 2100372148 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 2622136ns | 2594870ns | 2351691ns | 2570928ns | 2834171ns | base |
| per_core | 2335102ns | 2315869ns | 2143425ns | 2305307ns | 2475633ns | -10.95% |
| sequential | 4891310ns | 4870359ns | 4632167ns | 4829327ns | 5113854ns | +86.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 2619942ns | 2349851ns | 2831623ns | base | 0.400 |
| per_core | 2332729ns | 2141567ns | 2473355ns | -10.96% | 0.450 |
| sequential | 4888651ns | 4629923ns | 5110783ns | +86.59% | 0.214 |

## Performance model

- Peak throughput: **0.490 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.404 | 82.6% |
| per_core | 0.453 | 92.6% |
| sequential | 0.215 | 44.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 2622136ns | 2622136ns | base |
| per_core | 2335102ns | 2335102ns | -10.95% |
| sequential | 4891310ns | 4891310ns | +86.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 2593082ns | base | --- | [2516715, 2727144] | --- | --- | --- | --- |
| per_core | 2313252ns | -343477.9ns (-13.2%) | [-436302, -95360]ns | [2263918, 2394255] | YES (adj: no) | 0.0963 | 0.0963 | 0 |
| sequential | 4868112ns | +2243445.5ns (+86.5%) | [+2137704, +2352352]ns | [4781458, 4997067] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 2730461ns | -13.9% | +83.2% |
| 1 | 2 | 2479124ns | -6.4% | +91.3% |
| 1 | 3 | 2573014ns | -10.6% | +90.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.411 | moderate- |
| per_core | -0.005 | ok |
| sequential | -0.544 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 2694530.7ns | 2619942.2ns | 102.8% | HIGH |
| per_core | 2241359.2ns | 2332728.6ns | 96.1% | HIGH |
| sequential | 4896102.0ns | 4888650.9ns | 100.2% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 2349851.3-2831622.9 ns)
  2349851.3 |
  2373939.8 |####################
  2398028.4 |####################
  2422117.0 |
  2446205.6 |
  2470294.2 |
  2494382.8 |####################
  2518471.3 |####################
  2542559.9 |########################################
  2566648.5 |
  2590737.1 |########################################
  2614825.7 |
  2638914.3 |
  2663002.8 |####################
  2687091.4 |####################
  2711180.0 |########################################
  2735268.6 |########################################
  2759357.2 |
  2783445.8 |
  2807534.3 |
  (2 below, 2 above range)

per_core (n=18, range 2141566.6-2473354.9 ns)
  2141566.6 |
  2158156.0 |
  2174745.4 |
  2191334.8 |#############
  2207924.3 |
  2224513.7 |
  2241103.1 |##########################
  2257692.5 |#############
  2274281.9 |#############
  2290871.3 |##########################
  2307460.8 |#############
  2324050.2 |
  2340639.6 |
  2357229.0 |
  2373818.4 |########################################
  2390407.8 |#############
  2406997.2 |#############
  2423586.7 |
  2440176.1 |
  2456765.5 |
  (2 below, 3 above range)

sequential (n=18, range 4629923.0-5110782.5 ns)
  4629923.0 |
  4653966.0 |####################
  4678009.0 |####################
  4702051.9 |
  4726094.9 |
  4750137.9 |
  4774180.9 |########################################
  4798223.8 |####################
  4822266.8 |####################
  4846309.8 |####################
  4870352.8 |####################
  4894395.7 |########################################
  4918438.7 |####################
  4942481.7 |
  4966524.7 |
  4990567.7 |
  5014610.6 |
  5038653.6 |
  5062696.6 |####################
  5086739.6 |####################
  (2 below, 3 above range)

```

## Diagnostics

- **grouped**: bridge=104.3% of algo (FFI overhead may distort results)
- **per_core**: bridge=98.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.5% of algo (FFI overhead may distort results)
