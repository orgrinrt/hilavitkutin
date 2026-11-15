# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (riv_rcm, riv_spec) are a dead heat (<1%)

riv_rcm (2.31 ms) and riv_spec (2.32 ms) differ by 0.74%, inside the noise, even though the wider field spreads 17.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (riv_rcm)

The baseline riv_rcm is the fastest (2.31 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (riv_rcm) is the fastest** at 2307462.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.17x (fastest 2307462.2 ns, slowest 2710462.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | ordering theories on a grid: RCM vs spectral vs greedy vs row-major |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345679 |
| Counter frequency | 2112802731 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 2415248ns | 2311390ns | 2223842ns | 2296463ns | 2689128ns | base |
| riv_rowm | 2399237ns | 2370497ns | 2101148ns | 2335987ns | 2643157ns | -0.66% |
| riv_scr | 2662873ns | 2714332ns | 2382716ns | 2621815ns | 2864538ns | +10.25% |
| riv_snake | 2456278ns | 2493789ns | 2217790ns | 2393485ns | 2669709ns | +1.70% |
| riv_spec | 2435177ns | 2328330ns | 2254543ns | 2315011ns | 2705742ns | +0.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 2411312ns | 2219969ns | 2685118ns | base | 0.000 |
| riv_rowm | 2395355ns | 2097611ns | 2639173ns | -0.66% | 0.000 |
| riv_scr | 2658962ns | 2378924ns | 2860571ns | +10.27% | 0.000 |
| riv_snake | 2452337ns | 2213938ns | 2665771ns | +1.70% | 0.000 |
| riv_spec | 2431268ns | 2250812ns | 2701765ns | +0.83% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rowm; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 90.9% |
| riv_rowm | 0.000 | 88.6% |
| riv_scr | 0.000 | 77.4% |
| riv_snake | 0.000 | 84.3% |
| riv_spec | 0.000 | 90.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 2415248ns | 2415248ns | base |
| riv_rowm | 2399237ns | 2399237ns | -0.66% |
| riv_scr | 2662873ns | 2662873ns | +10.25% |
| riv_snake | 2456278ns | 2456278ns | +1.70% |
| riv_spec | 2435177ns | 2435177ns | +0.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 2307462ns | base | --- | [2265031, 2612000] | --- | --- | --- | --- |
| riv_rowm | 2366818ns | no significant difference | [-271139, +100799]ns | [2288386, 2408764] | no | 1.0000 | 0.8145 | 0 |
| riv_scr | 2710463ns | +305202.2ns (+13.2%) | [+111836, +480505]ns | [2453283, 2785920] | YES | 0.0302 | 0.0075 | 0 |
| riv_snake | 2489723ns | no significant difference | [-392681, +355026]ns | [2258415, 2603540] | no | 1.0000 | 1.0000 | 0 |
| riv_spec | 2324547ns | no significant difference | [-318260, +388440]ns | [2282941, 2609401] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 2217529ns | +2.0% | +7.1% | +17.2% | +18.3% |
| 1 | 2 | 2307971ns | +4.1% | +21.2% | -2.1% | -2.5% |
| 1 | 3 | 2642848ns | -10.0% | -9.2% | -16.2% | -12.6% |
| 2 | 1 | 2286231ns | +18.7% | +17.4% | +14.1% | +17.7% |
| 2 | 2 | 2279218ns | -10.6% | +20.4% | +15.0% | +2.0% |
| 2 | 3 | 2640729ns | -13.3% | +3.3% | -15.8% | -14.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.263 | moderate- |
| riv_rowm | -0.355 | moderate- |
| riv_scr | -0.444 | moderate- |
| riv_snake | -0.168 | ok |
| riv_spec | -0.308 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 3/6, lost 3/6
- **riv_scr**: won 1/6, lost 5/6
- **riv_snake**: won 3/6, lost 3/6
- **riv_spec**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 11567914.6ns | 2411311.9ns | 479.7% | HIGH |
| riv_rowm | 11533152.1ns | 2395354.5ns | 481.5% | HIGH |
| riv_scr | 11624323.6ns | 2658961.8ns | 437.2% | HIGH |
| riv_snake | 11687515.2ns | 2452336.7ns | 476.6% | HIGH |
| riv_spec | 11562338.9ns | 2431267.6ns | 475.6% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 2219968.9-2685117.9 ns)
  2219968.9 |#############
  2243226.4 |##########################
  2266483.8 |########################################
  2289741.2 |##########################
  2312998.7 |#############
  2336256.1 |#############
  2359513.6 |
  2382771.0 |
  2406028.5 |
  2429285.9 |
  2452543.4 |
  2475800.9 |
  2499058.3 |
  2522315.8 |
  2545573.2 |
  2568830.6 |#############
  2592088.1 |
  2615345.5 |
  2638603.0 |########################################
  2661860.4 |
  (2 below, 2 above range)

riv_rowm (n=18, range 2097610.9-2639173.5 ns)
  2097610.9 |
  2124689.1 |
  2151767.2 |
  2178845.3 |
  2205923.4 |
  2233001.6 |####################
  2260079.7 |########################################
  2287157.8 |########################################
  2314236.0 |####################
  2341314.1 |########################################
  2368392.2 |########################################
  2395470.3 |########################################
  2422548.5 |
  2449626.6 |
  2476704.7 |
  2503782.8 |
  2530861.0 |
  2557939.1 |
  2585017.2 |
  2612095.4 |
  (2 below, 4 above range)

riv_scr (n=18, range 2378923.7-2860571.4 ns)
  2378923.7 |########################################
  2403006.1 |
  2427088.5 |####################
  2451170.9 |####################
  2475253.3 |
  2499335.7 |
  2523418.0 |
  2547500.4 |
  2571582.8 |
  2595665.2 |
  2619747.6 |
  2643830.0 |
  2667912.3 |########################################
  2691994.7 |########################################
  2716077.1 |####################
  2740159.5 |####################
  2764241.9 |####################
  2788324.3 |####################
  2812406.6 |
  2836489.0 |########################################
  (2 below, 2 above range)

riv_snake (n=18, range 2213938.1-2665770.6 ns)
  2213938.1 |########################################
  2236529.8 |##########################
  2259121.4 |
  2281713.0 |
  2304304.6 |##########################
  2326896.2 |
  2349487.9 |
  2372079.5 |
  2394671.1 |#############
  2417262.7 |
  2439854.4 |
  2462446.0 |
  2485037.6 |
  2507629.2 |
  2530220.9 |
  2552812.5 |
  2575404.1 |########################################
  2597995.7 |########################################
  2620587.4 |
  2643179.0 |
  (1 below, 3 above range)

riv_spec (n=18, range 2250811.8-2701765.4 ns)
  2250811.8 |########################################
  2273359.5 |#############
  2295907.2 |##########################
  2318454.9 |########################################
  2341002.6 |
  2363550.2 |#############
  2386097.9 |
  2408645.6 |
  2431193.3 |
  2453741.0 |
  2476288.6 |
  2498836.3 |
  2521384.0 |
  2543931.7 |
  2566479.4 |
  2589027.0 |#############
  2611574.7 |#############
  2634122.4 |
  2656670.1 |#############
  2679217.8 |#############
  (2 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=499.1% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=485.8% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=431.0% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=469.1% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=494.7% of algo (FFI overhead may distort results)
