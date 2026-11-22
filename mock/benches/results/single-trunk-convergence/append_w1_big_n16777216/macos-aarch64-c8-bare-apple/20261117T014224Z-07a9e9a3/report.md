# append, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 5.1x slower than the field

sequential (11.71 ms) is 5.1x the fastest (2.29 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.64)

kernel_nway's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, kernel_seq, head_tail_dyn, std_parallel, head_tail} vs {nway, sequential} (357% apart)

The field splits into a fast tier {kernel_nway, kernel_seq, head_tail_dyn, std_parallel, head_tail} and a slow tier {nway, sequential} with a 357% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.1x the fastest

Fastest kernel_nway (2.29 ms) to slowest sequential (11.71 ms): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 2289312.5 ns median (-6.2% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 5.11x (fastest 2289312.5 ns, slowest 11705833.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 1 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 2452875ns | 2444234ns | 2252073ns | 2386198ns | 2653292ns | base |
| head_tail_dyn | 2392057ns | 2367094ns | 2255292ns | 2342785ns | 2534349ns | -2.48% |
| kernel_nway | 2384241ns | 2291740ns | 2248302ns | 2288444ns | 2595906ns | -2.80% |
| kernel_seq | 2380733ns | 2360984ns | 2257979ns | 2331271ns | 2516302ns | -2.94% |
| nway | 11218033ns | 11150109ns | 11088042ns | 11139066ns | 11401479ns | +357.34% |
| sequential | 11762986ns | 11710906ns | 11472125ns | 11679243ns | 12034031ns | +379.56% |
| std_parallel | 2410738ns | 2376823ns | 2279646ns | 2345639ns | 2573932ns | -1.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2449788ns | 2249052ns | 2650146ns | base | 6.848 |
| head_tail_dyn | 2389281ns | 2253302ns | 2530786ns | -2.47% | 7.022 |
| kernel_nway | 2381533ns | 2245469ns | 2593224ns | -2.79% | 7.045 |
| kernel_seq | 2377158ns | 2253375ns | 2512750ns | -2.96% | 7.058 |
| nway | 11212825ns | 11079354ns | 11396682ns | +357.71% | 1.496 |
| sequential | 11756359ns | 11463271ns | 12027479ns | +379.89% | 1.427 |
| std_parallel | 2407578ns | 2277115ns | 2570125ns | -1.72% | 6.969 |

## Performance model

- Peak throughput: **7.472 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.872 | 92.0% |
| head_tail_dyn | 7.095 | 95.0% |
| kernel_nway | 7.328 | 98.1% |
| kernel_seq | 7.116 | 95.2% |
| nway | 1.505 | 20.1% |
| sequential | 1.433 | 19.2% |
| std_parallel | 7.068 | 94.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2452875ns | 2452875ns | base |
| head_tail_dyn | 2392057ns | 2392057ns | -2.48% |
| kernel_nway | 2384241ns | 2384241ns | -2.80% |
| kernel_seq | 2380733ns | 2380733ns | -2.94% |
| nway | 11218033ns | 11218033ns | +357.34% |
| sequential | 11762986ns | 11762986ns | +379.56% |
| std_parallel | 2410738ns | 2410738ns | -1.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2441318ns | base | --- | [2257901, 2650146] | --- | --- | --- | --- |
| head_tail_dyn | 2364667ns | no significant difference | [-280120, +79120]ns | [2272391, 2530786] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 2289312ns | no significant difference | [-239813, +75531]ns | [2262062, 2593224] | no | 1.0000 | 0.6875 | 0 |
| kernel_seq | 2357662ns | no significant difference | [-200865, +23359]ns | [2261062, 2512750] | no | 1.0000 | 0.6875 | 0 |
| nway | 11145870ns | +8764661.4ns (+359.0%) | [+8562500, +8961948]ns | [11095922, 11396682] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| sequential | 11705833ns | +9257390.6ns (+379.2%) | [+9198760, +9463562]ns | [11535766, 12027479] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| std_parallel | 2373615ns | no significant difference | [-343984, +259380]ns | [2278995, 2570125] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2397323ns | +1.5% | +2.0% | +0.3% | +363.9% | +385.0% | +5.2% |
| 1 | 2 | 2301896ns | +2.7% | -0.5% | -1.0% | +389.3% | +404.6% | +4.0% |
| 1 | 3 | 2650146ns | -10.6% | -9.0% | -7.6% | +324.7% | +353.8% | -13.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.338 | moderate- |
| head_tail_dyn | -0.209 | moderate- |
| kernel_nway | -0.637 | HIGH- (thermal bounce) |
| kernel_seq | -0.624 | HIGH- (thermal bounce) |
| nway | -0.204 | moderate- |
| sequential | -0.180 | ok |
| std_parallel | -0.008 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 92455909.7ns | 2449788.2ns | 3774.0% | HIGH |
| head_tail_dyn | 92232835.1ns | 2389281.3ns | 3860.3% | HIGH |
| kernel_nway | 92136975.7ns | 2381533.0ns | 3868.8% | HIGH |
| kernel_seq | 92806611.1ns | 2377158.0ns | 3904.1% | HIGH |
| nway | 101380166.7ns | 11212824.7ns | 904.1% | HIGH |
| sequential | 101985769.1ns | 11756359.3ns | 867.5% | HIGH |
| std_parallel | 92349588.6ns | 2407578.1ns | 3835.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2249052.1-2650145.9 ns)
  2249052.1 |########################################
  2269106.8 |
  2289161.5 |
  2309216.2 |
  2329270.9 |
  2349325.5 |####################
  2369380.2 |
  2389434.9 |
  2409489.6 |
  2429544.3 |
  2449599.0 |
  2469653.7 |
  2489708.4 |
  2509763.0 |####################
  2529817.7 |
  2549872.4 |
  2569927.1 |
  2589981.8 |
  2610036.5 |####################
  2630091.2 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 2253302.1-2530786.5 ns)
  2253302.1 |########################################
  2267176.3 |
  2281050.5 |########################################
  2294924.8 |########################################
  2308799.0 |
  2322673.2 |
  2336547.4 |
  2350421.6 |
  2364295.8 |
  2378170.1 |
  2392044.3 |
  2405918.5 |
  2419792.7 |########################################
  2433666.9 |
  2447541.1 |########################################
  2461415.4 |
  2475289.6 |
  2489163.8 |
  2503038.0 |
  2516912.2 |
  (0 below, 1 above range)

kernel_nway (n=6, range 2245468.8-2593224.0 ns)
  2245468.8 |####################
  2262856.6 |####################
  2280244.3 |########################################
  2297632.1 |
  2315019.8 |
  2332407.6 |
  2349795.3 |
  2367183.1 |
  2384570.9 |
  2401958.6 |
  2419346.4 |
  2436734.1 |
  2454121.9 |
  2471509.6 |
  2488897.4 |
  2506285.2 |
  2523672.9 |
  2541060.7 |####################
  2558448.4 |
  2575836.2 |
  (0 below, 1 above range)

kernel_seq (n=6, range 2253375.0-2512750.0 ns)
  2253375.0 |########################################
  2266343.8 |########################################
  2279312.5 |########################################
  2292281.2 |
  2305250.0 |
  2318218.8 |
  2331187.5 |
  2344156.2 |
  2357125.0 |
  2370093.8 |
  2383062.5 |
  2396031.2 |
  2409000.0 |
  2421968.8 |########################################
  2434937.5 |
  2447906.2 |
  2460875.0 |########################################
  2473843.8 |
  2486812.5 |
  2499781.2 |
  (0 below, 1 above range)

nway (n=6, range 11079354.2-11396682.3 ns)
  11079354.2 |########################################
  11095220.6 |
  11111087.0 |########################################
  11126953.4 |########################################
  11142819.8 |
  11158686.2 |########################################
  11174552.6 |
  11190419.0 |
  11206285.4 |
  11222151.8 |
  11238018.2 |
  11253884.7 |
  11269751.1 |
  11285617.5 |
  11301483.9 |
  11317350.3 |
  11333216.7 |
  11349083.1 |
  11364949.5 |
  11380815.9 |########################################
  (0 below, 1 above range)

sequential (n=6, range 11463270.8-12027479.2 ns)
  11463270.8 |####################
  11491481.2 |
  11519691.6 |
  11547902.1 |
  11576112.5 |
  11604322.9 |########################################
  11632533.3 |
  11660743.7 |
  11688954.1 |
  11717164.6 |
  11745375.0 |
  11773585.4 |####################
  11801795.8 |####################
  11830006.2 |
  11858216.6 |
  11886427.1 |
  11914637.5 |
  11942847.9 |
  11971058.3 |
  11999268.7 |
  (0 below, 1 above range)

std_parallel (n=6, range 2277114.6-2570125.0 ns)
  2277114.6 |########################################
  2291765.1 |
  2306415.6 |
  2321066.2 |####################
  2335716.7 |
  2350367.2 |
  2365017.7 |
  2379668.2 |
  2394318.8 |
  2408969.3 |####################
  2423619.8 |
  2438270.3 |
  2452920.8 |
  2467571.4 |
  2482221.9 |
  2496872.4 |####################
  2511522.9 |
  2526173.4 |
  2540824.0 |
  2555474.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=3776.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3908.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=4031.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3913.8% of algo (FFI overhead may distort results)
- **nway**: bridge=908.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=870.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=3885.5% of algo (FFI overhead may distort results)
