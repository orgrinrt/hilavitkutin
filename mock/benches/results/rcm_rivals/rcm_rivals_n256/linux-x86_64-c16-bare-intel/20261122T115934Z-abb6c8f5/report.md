# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### riv_rcm shows alternating (throttle bounce) (autocorr -0.88)

riv_rcm's per-pass series has lag-1 autocorrelation -0.88, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader riv_spec vs stability leader riv_rowm (+2% speed for 1.8x steadier)

riv_spec is fastest (2.24 ms, CV 9.4%); riv_rowm gives up 2.4% median for 1.8x lower variance (CV 5.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: riv_spec** at 2242548.5 ns median (-5.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.13x (fastest 2242548.5 ns, slowest 2532352.0 ns)

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
| Counter frequency | 2112741408 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 2439470ns | 2368189ns | 2166923ns | 2355990ns | 2700962ns | base |
| riv_rowm | 2338515ns | 2300450ns | 2245982ns | 2287395ns | 2461462ns | -4.14% |
| riv_scr | 2593827ns | 2535963ns | 2382242ns | 2512608ns | 2821447ns | +6.33% |
| riv_snake | 2417203ns | 2395719ns | 2223648ns | 2341689ns | 2627251ns | -0.91% |
| riv_spec | 2389033ns | 2246453ns | 2183276ns | 2281122ns | 2653779ns | -2.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 2435538ns | 2163155ns | 2697047ns | base | 0.000 |
| riv_rowm | 2334633ns | 2242145ns | 2457495ns | -4.14% | 0.000 |
| riv_scr | 2590037ns | 2378461ns | 2817627ns | +6.34% | 0.000 |
| riv_snake | 2413316ns | 2219676ns | 2623443ns | -0.91% | 0.000 |
| riv_spec | 2385138ns | 2179521ns | 2649665ns | -2.07% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rcm; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 91.5% |
| riv_rowm | 0.000 | 94.2% |
| riv_scr | 0.000 | 85.4% |
| riv_snake | 0.000 | 90.4% |
| riv_spec | 0.000 | 96.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 2439470ns | 2439470ns | base |
| riv_rowm | 2338515ns | 2338515ns | -4.14% |
| riv_scr | 2593827ns | 2593827ns | +6.33% |
| riv_snake | 2417203ns | 2417203ns | -0.91% |
| riv_spec | 2389033ns | 2389033ns | -2.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 2364009ns | base | --- | [2273484, 2569178] | --- | --- | --- | --- |
| riv_rowm | 2296678ns | no significant difference | [-237052, +7366]ns | [2264833, 2341486] | no | 0.1925 | 0.0963 | 0 |
| riv_scr | 2532352ns | +137014.9ns (+5.8%) | [+3726, +301298]ns | [2444149, 2698539] | YES (adj: no) | 0.1925 | 0.0963 | 0 |
| riv_snake | 2391708ns | no significant difference | [-97194, +52550]ns | [2261590, 2500799] | no | 0.8145 | 0.8145 | 0 |
| riv_spec | 2242548ns | no significant difference | [-226014, +144701]ns | [2214685, 2557872] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 2275615ns | -0.4% | +5.8% | +12.0% | -1.3% |
| 1 | 2 | 2598821ns | -6.1% | -1.4% | -4.4% | -14.7% |
| 1 | 3 | 2205776ns | +1.8% | +14.2% | +2.4% | -0.7% |
| 2 | 1 | 2579600ns | -12.2% | -8.2% | -13.9% | -14.5% |
| 2 | 2 | 2201222ns | +4.2% | +26.3% | +3.0% | +15.4% |
| 2 | 3 | 2515472ns | -8.0% | +5.7% | -3.1% | +4.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.883 | HIGH- (thermal bounce) |
| riv_rowm | -0.407 | moderate- |
| riv_scr | -0.111 | ok |
| riv_snake | 0.344 | moderate+ |
| riv_spec | 0.429 | moderate+ |

**Consistency summary:**

- **riv_rowm**: won 4/6, lost 2/6
- **riv_scr**: won 2/6, lost 4/6
- **riv_snake**: won 3/6, lost 3/6
- **riv_spec**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 11603069.1ns | 2435537.7ns | 476.4% | HIGH |
| riv_rowm | 11613203.4ns | 2334632.5ns | 497.4% | HIGH |
| riv_scr | 11690305.4ns | 2590037.2ns | 451.4% | HIGH |
| riv_snake | 11600748.7ns | 2413316.3ns | 480.7% | HIGH |
| riv_spec | 11712101.3ns | 2385137.8ns | 491.0% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 2163155.1-2697047.4 ns)
  2163155.1 |####################
  2189849.7 |########################################
  2216544.4 |
  2243239.0 |
  2269933.6 |########################################
  2296628.2 |####################
  2323322.8 |########################################
  2350017.4 |
  2376712.1 |####################
  2403406.7 |
  2430101.3 |
  2456795.9 |
  2483490.5 |
  2510185.1 |########################################
  2536879.8 |####################
  2563574.4 |####################
  2590269.0 |####################
  2616963.6 |
  2643658.2 |
  2670352.8 |
  (1 below, 3 above range)

riv_rowm (n=18, range 2242144.7-2457495.0 ns)
  2242144.7 |########################################
  2252912.2 |########################################
  2263679.8 |########################################
  2274447.3 |
  2285214.8 |########################################
  2295982.3 |########################################
  2306749.8 |####################
  2317517.3 |####################
  2328284.8 |
  2339052.3 |
  2349819.8 |####################
  2360587.4 |
  2371354.9 |####################
  2382122.4 |
  2392889.9 |
  2403657.4 |
  2414424.9 |
  2425192.4 |
  2435959.9 |####################
  2446727.4 |
  (1 below, 2 above range)

riv_scr (n=18, range 2378461.0-2817626.6 ns)
  2378461.0 |
  2400419.3 |########################################
  2422377.6 |####################
  2444335.9 |####################
  2466294.1 |####################
  2488252.4 |
  2510210.7 |########################################
  2532169.0 |####################
  2554127.3 |####################
  2576085.5 |
  2598043.8 |
  2620002.1 |
  2641960.4 |########################################
  2663918.6 |
  2685876.9 |
  2707835.2 |
  2729793.5 |####################
  2751751.8 |####################
  2773710.0 |####################
  2795668.3 |
  (2 below, 2 above range)

riv_snake (n=18, range 2219676.2-2623442.9 ns)
  2219676.2 |########################################
  2239864.5 |########################################
  2260052.9 |########################################
  2280241.2 |
  2300429.5 |####################
  2320617.9 |
  2340806.2 |
  2360994.5 |
  2381182.9 |########################################
  2401371.2 |
  2421559.5 |####################
  2441747.9 |
  2461936.2 |
  2482124.5 |########################################
  2502312.9 |####################
  2522501.2 |
  2542689.5 |####################
  2562877.9 |
  2583066.2 |
  2603254.5 |
  (1 below, 3 above range)

riv_spec (n=18, range 2179520.9-2649664.5 ns)
  2179520.9 |####################
  2203028.1 |##############################
  2226535.3 |########################################
  2250042.4 |
  2273549.6 |
  2297056.8 |
  2320564.0 |
  2344071.2 |##########
  2367578.3 |
  2391085.5 |
  2414592.7 |
  2438099.9 |
  2461607.1 |
  2485114.3 |
  2508621.4 |
  2532128.6 |##########
  2555635.8 |####################
  2579143.0 |
  2602650.2 |##########
  2626157.3 |##########
  (1 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=489.4% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=501.8% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=461.8% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=482.6% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=515.8% of algo (FFI overhead may distort results)
