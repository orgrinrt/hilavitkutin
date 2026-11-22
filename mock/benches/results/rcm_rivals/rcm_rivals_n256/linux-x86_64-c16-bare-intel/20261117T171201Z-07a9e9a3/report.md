# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (2.58 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_rowm at 2.35 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Speed leader riv_rowm vs stability leader riv_scr (+4% speed for 1.1x steadier)

riv_rowm is fastest (2.35 ms, CV 8.1%); riv_scr gives up 3.9% median for 1.1x lower variance (CV 7.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: riv_rowm** at 2354010.1 ns median (-8.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.10x (fastest 2354010.1 ns, slowest 2578796.8 ns)

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
| Counter frequency | 2112768073 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 2540643ns | 2582852ns | 2265419ns | 2475269ns | 2776317ns | base |
| riv_rowm | 2387662ns | 2357794ns | 2129489ns | 2328166ns | 2605992ns | -6.02% |
| riv_scr | 2530879ns | 2448729ns | 2340312ns | 2441130ns | 2760785ns | -0.38% |
| riv_snake | 2469317ns | 2402711ns | 2222452ns | 2376376ns | 2732162ns | -2.81% |
| riv_spec | 2474388ns | 2534315ns | 2240503ns | 2401347ns | 2700890ns | -2.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 2536763ns | 2261711ns | 2772387ns | base | 0.000 |
| riv_rowm | 2383851ns | 2126066ns | 2601978ns | -6.03% | 0.000 |
| riv_scr | 2527040ns | 2336554ns | 2756972ns | -0.38% | 0.000 |
| riv_snake | 2465467ns | 2218626ns | 2728276ns | -2.81% | 0.000 |
| riv_spec | 2470599ns | 2236760ns | 2697006ns | -2.61% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rowm; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 82.4% |
| riv_rowm | 0.000 | 90.3% |
| riv_scr | 0.000 | 87.0% |
| riv_snake | 0.000 | 88.6% |
| riv_spec | 0.000 | 84.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 2540643ns | 2540643ns | base |
| riv_rowm | 2387662ns | 2387662ns | -6.02% |
| riv_scr | 2530879ns | 2530879ns | -0.38% |
| riv_snake | 2469317ns | 2469317ns | -2.81% |
| riv_spec | 2474388ns | 2474388ns | -2.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 2578797ns | base | --- | [2350414, 2684151] | --- | --- | --- | --- |
| riv_rowm | 2354010ns | -99486.2ns (-3.9%) | [-368269, -40245]ns | [2285841, 2440193] | YES | 0.0151 | 0.0075 | 0 |
| riv_scr | 2444712ns | no significant difference | [-235289, +134431]ns | [2386123, 2666333] | no | 1.0000 | 1.0000 | 0 |
| riv_snake | 2398836ns | no significant difference | [-132804, +24694]ns | [2282814, 2640149] | no | 0.1283 | 0.0963 | 0 |
| riv_spec | 2530633ns | -87034.8ns (-3.4%) | [-190241, -31110]ns | [2275386, 2617041] | YES | 0.0151 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 2264449ns | +16.3% | +3.0% | +2.6% | +11.8% |
| 1 | 2 | 2632540ns | -13.5% | -8.9% | -1.8% | -2.8% |
| 1 | 3 | 2417354ns | -2.1% | -0.8% | +12.0% | -5.4% |
| 2 | 1 | 2686766ns | -12.6% | -11.3% | -2.7% | -0.6% |
| 2 | 2 | 2694768ns | -22.5% | +1.3% | -14.7% | -16.8% |
| 2 | 3 | 2339546ns | -2.3% | +15.8% | -5.1% | -3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.309 | moderate- |
| riv_rowm | -0.063 | ok |
| riv_scr | 0.377 | moderate+ |
| riv_snake | 0.328 | moderate+ |
| riv_spec | -0.328 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 5/6, lost 1/6
- **riv_scr**: won 3/6, lost 3/6
- **riv_snake**: won 4/6, lost 2/6
- **riv_spec**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 11632651.1ns | 2536762.9ns | 458.6% | HIGH |
| riv_rowm | 11640625.5ns | 2383851.5ns | 488.3% | HIGH |
| riv_scr | 11735413.0ns | 2527040.1ns | 464.4% | HIGH |
| riv_snake | 11616166.1ns | 2465467.3ns | 471.2% | HIGH |
| riv_spec | 11623002.3ns | 2470599.4ns | 470.5% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 2261710.6-2772386.8 ns)
  2261710.6 |#############
  2287244.4 |##########################
  2312778.2 |
  2338312.0 |##########################
  2363845.8 |
  2389379.7 |#############
  2414913.5 |#############
  2440447.3 |
  2465981.1 |
  2491514.9 |
  2517048.7 |#############
  2542582.5 |
  2568116.3 |
  2593650.1 |
  2619184.0 |########################################
  2644717.8 |
  2670251.6 |########################################
  2695785.4 |
  2721319.2 |
  2746853.0 |
  (1 below, 3 above range)

riv_rowm (n=18, range 2126065.7-2601978.1 ns)
  2126065.7 |
  2149861.4 |
  2173657.0 |
  2197452.6 |
  2221248.2 |
  2245043.8 |##########################
  2268839.4 |##########################
  2292635.1 |
  2316430.7 |#############
  2340226.3 |########################################
  2364021.9 |##########################
  2387817.5 |
  2411613.2 |#############
  2435408.8 |#############
  2459204.4 |
  2483000.0 |
  2506795.6 |
  2530591.3 |
  2554386.9 |
  2578182.5 |#############
  (2 below, 3 above range)

riv_scr (n=18, range 2336554.4-2756971.6 ns)
  2336554.4 |
  2357575.3 |########
  2378596.1 |########################################
  2399617.0 |
  2420637.9 |########
  2441658.7 |########
  2462679.6 |
  2483700.4 |
  2504721.3 |
  2525742.2 |########
  2546763.0 |########
  2567783.9 |
  2588804.8 |
  2609825.6 |
  2630846.5 |
  2651867.3 |########
  2672888.2 |########
  2693909.1 |########
  2714929.9 |########
  2735950.8 |
  (2 below, 2 above range)

riv_snake (n=18, range 2218625.9-2728275.8 ns)
  2218625.9 |########################################
  2244108.4 |#############
  2269590.9 |
  2295073.4 |##########################
  2320555.9 |#############
  2346038.4 |
  2371520.9 |#############
  2397003.4 |#############
  2422485.9 |
  2447968.4 |
  2473450.8 |
  2498933.3 |
  2524415.8 |#############
  2549898.3 |
  2575380.8 |#############
  2600863.3 |#############
  2626345.8 |
  2651828.3 |#############
  2677310.8 |#############
  2702793.3 |#############
  (1 below, 2 above range)

riv_spec (n=18, range 2236760.4-2697006.0 ns)
  2236760.4 |#############
  2259772.6 |########################################
  2282784.9 |#############
  2305797.2 |#############
  2328809.5 |
  2351821.8 |
  2374834.1 |
  2397846.4 |
  2420858.6 |
  2443870.9 |
  2466883.2 |
  2489895.5 |
  2512907.8 |##########################
  2535920.1 |#############
  2558932.3 |##########################
  2581944.6 |
  2604956.9 |
  2627969.2 |
  2650981.5 |#############
  2673993.8 |#############
  (2 below, 3 above range)

```

## Diagnostics

- **riv_rcm**: bridge=453.2% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=489.7% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=473.0% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=485.2% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=460.7% of algo (FFI overhead may distort results)
