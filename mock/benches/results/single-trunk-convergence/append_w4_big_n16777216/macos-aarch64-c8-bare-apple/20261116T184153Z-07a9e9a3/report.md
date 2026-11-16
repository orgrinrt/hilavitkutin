# append, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 5.1x slower than the field

sequential (11.91 ms) is 5.1x the fastest (2.34 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail_dyn, head_tail) are a dead heat (<1%)

head_tail_dyn (2.34 ms) and head_tail (2.35 ms) differ by 0.24%, inside the noise, even though the wider field spreads 408.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.65)

kernel_nway's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, head_tail, kernel_seq, kernel_nway, std_parallel, nway} vs {sequential} (265% apart)

The field splits into a fast tier {head_tail_dyn, head_tail, kernel_seq, kernel_nway, std_parallel, nway} and a slow tier {sequential} with a 265% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.1x the fastest

Fastest head_tail_dyn (2.34 ms) to slowest sequential (11.91 ms): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: head_tail_dyn** at 2343515.6 ns median (-0.2% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 5.08x (fastest 2343515.6 ns, slowest 11905843.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 4 |
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
| head_tail | 2356349ns | 2352005ns | 2322115ns | 2343378ns | 2392922ns | base |
| head_tail_dyn | 2343865ns | 2346620ns | 2287583ns | 2332549ns | 2388979ns | -0.53% |
| kernel_nway | 2388753ns | 2380161ns | 2368510ns | 2377979ns | 2415036ns | +1.38% |
| kernel_seq | 2405512ns | 2368146ns | 2305469ns | 2361476ns | 2521589ns | +2.09% |
| nway | 3339142ns | 3265177ns | 3160927ns | 3230917ns | 3590589ns | +41.71% |
| sequential | 11894387ns | 11910229ns | 11788635ns | 11895920ns | 11944964ns | +404.78% |
| std_parallel | 2415684ns | 2424526ns | 2378812ns | 2409424ns | 2443510ns | +2.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2353535ns | 2318229ns | 2390505ns | base | 7.129 |
| head_tail_dyn | 2340866ns | 2284469ns | 2386120ns | -0.54% | 7.167 |
| kernel_nway | 2384918ns | 2364594ns | 2411255ns | +1.33% | 7.035 |
| kernel_seq | 2402644ns | 2302417ns | 2519318ns | +2.09% | 6.983 |
| nway | 3334181ns | 3156271ns | 3585589ns | +41.67% | 5.032 |
| sequential | 11889750ns | 11785125ns | 11940083ns | +405.19% | 1.411 |
| std_parallel | 2413113ns | 2375708ns | 2440344ns | +2.53% | 6.953 |

## Performance model

- Peak throughput: **7.344 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.142 | 97.2% |
| head_tail_dyn | 7.159 | 97.5% |
| kernel_nway | 7.060 | 96.1% |
| kernel_seq | 7.094 | 96.6% |
| nway | 5.145 | 70.1% |
| sequential | 1.409 | 19.2% |
| std_parallel | 6.926 | 94.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2356349ns | 2356349ns | base |
| head_tail_dyn | 2343865ns | 2343865ns | -0.53% |
| kernel_nway | 2388753ns | 2388753ns | +1.38% |
| kernel_seq | 2405512ns | 2405512ns | +2.09% |
| nway | 3339142ns | 3339142ns | +41.71% |
| sequential | 11894387ns | 11894387ns | +404.78% |
| std_parallel | 2415684ns | 2415684ns | +2.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2349229ns | base | --- | [2320870, 2390505] | --- | --- | --- | --- |
| head_tail_dyn | 2343516ns | no significant difference | [-76667, +65250]ns | [2292964, 2386120] | no | 0.6875 | 0.6875 | 0 |
| kernel_nway | 2376464ns | no significant difference | [-21370, +62167]ns | [2367036, 2411255] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 2364922ns | no significant difference | [-9578, +133693]ns | [2323693, 2519318] | no | 0.6875 | 0.6875 | 0 |
| nway | 3260578ns | +901869.8ns (+38.4%) | [+807146, +1232922]ns | [3156375, 3585589] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| sequential | 11905844ns | +9538083.3ns (+406.0%) | [+9489135, +9581427]ns | [11823323, 11940083] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 2422526ns | +53203.2ns (+2.3%) | [+19375, +106156]ns | [2376469, 2440344] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2362599ns | -2.9% | +1.4% | +5.7% | +48.7% | +403.2% | +3.2% |
| 1 | 2 | 2362094ns | -0.7% | +0.2% | +0.1% | +38.2% | +402.2% | +2.7% |
| 1 | 3 | 2335911ns | +2.1% | +2.4% | +0.5% | +38.0% | +410.2% | +1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.157 | ok |
| head_tail_dyn | -0.006 | ok |
| kernel_nway | -0.652 | HIGH- (thermal bounce) |
| kernel_seq | -0.115 | ok |
| nway | -0.100 | ok |
| sequential | -0.486 | moderate- |
| std_parallel | -0.067 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 0/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 93318399.3ns | 2353534.7ns | 3965.0% | HIGH |
| head_tail_dyn | 93532041.7ns | 2340866.3ns | 3995.6% | HIGH |
| kernel_nway | 93407953.1ns | 2384918.4ns | 3916.6% | HIGH |
| kernel_seq | 93586086.8ns | 2402644.1ns | 3895.1% | HIGH |
| nway | 94754592.0ns | 3334180.6ns | 2841.9% | HIGH |
| sequential | 103084783.0ns | 11889750.0ns | 867.0% | HIGH |
| std_parallel | 93390529.5ns | 2413112.9ns | 3870.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2318229.2-2390505.2 ns)
  2318229.2 |####################
  2321843.0 |####################
  2325456.8 |
  2329070.6 |
  2332684.4 |
  2336298.2 |
  2339912.0 |
  2343525.8 |
  2347139.6 |########################################
  2350753.4 |
  2354367.2 |
  2357981.0 |
  2361594.8 |
  2365208.6 |
  2368822.4 |
  2372436.2 |####################
  2376050.0 |
  2379663.8 |
  2383277.6 |
  2386891.4 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 2284468.8-2386119.8 ns)
  2284468.8 |####################
  2289551.3 |
  2294633.9 |
  2299716.4 |####################
  2304799.0 |
  2309881.5 |
  2314964.1 |
  2320046.6 |
  2325129.2 |
  2330211.7 |
  2335294.3 |
  2340376.8 |########################################
  2345459.4 |####################
  2350541.9 |
  2355624.5 |
  2360707.0 |
  2365789.6 |
  2370872.1 |
  2375954.7 |
  2381037.2 |
  (0 below, 1 above range)

kernel_nway (n=6, range 2364593.8-2411255.2 ns)
  2364593.8 |########################################
  2366926.9 |
  2369259.9 |########################################
  2371593.0 |########################################
  2373926.1 |
  2376259.1 |
  2378592.2 |########################################
  2380925.3 |
  2383258.4 |
  2385591.4 |
  2387924.5 |
  2390257.6 |
  2392590.6 |
  2394923.7 |
  2397256.8 |
  2399589.9 |
  2401922.9 |########################################
  2404256.0 |
  2406589.1 |
  2408922.1 |
  (0 below, 1 above range)

kernel_seq (n=6, range 2302416.7-2519317.7 ns)
  2302416.7 |########################################
  2313261.8 |
  2324106.8 |
  2334951.9 |########################################
  2345796.9 |########################################
  2356642.0 |
  2367487.0 |
  2378332.1 |########################################
  2389177.1 |
  2400022.2 |
  2410867.2 |
  2421712.2 |########################################
  2432557.3 |
  2443402.4 |
  2454247.4 |
  2465092.5 |
  2475937.5 |
  2486782.6 |
  2497627.6 |
  2508472.7 |
  (0 below, 1 above range)

nway (n=6, range 3156270.8-3585588.5 ns)
  3156270.8 |########################################
  3177736.7 |
  3199202.6 |
  3220668.5 |####################
  3242134.3 |
  3263600.2 |
  3285066.1 |########################################
  3306532.0 |
  3327997.9 |
  3349463.8 |
  3370929.7 |
  3392395.6 |
  3413861.4 |
  3435327.3 |
  3456793.2 |
  3478259.1 |
  3499725.0 |
  3521190.9 |
  3542656.8 |
  3564122.7 |
  (0 below, 1 above range)

sequential (n=6, range 11785125.0-11940083.3 ns)
  11785125.0 |########################################
  11792872.9 |
  11800620.8 |
  11808368.8 |
  11816116.7 |
  11823864.6 |
  11831612.5 |
  11839360.4 |
  11847108.3 |
  11854856.3 |########################################
  11862604.2 |
  11870352.1 |
  11878100.0 |
  11885847.9 |
  11893595.8 |########################################
  11901343.8 |
  11909091.7 |########################################
  11916839.6 |
  11924587.5 |
  11932335.4 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 2375708.3-2440343.8 ns)
  2375708.3 |########################################
  2378940.1 |
  2382171.8 |
  2385403.6 |
  2388635.4 |
  2391867.2 |
  2395098.9 |
  2398330.7 |
  2401562.5 |
  2404794.3 |
  2408026.0 |
  2411257.8 |
  2414489.6 |####################
  2417721.3 |
  2420953.1 |
  2424184.9 |
  2427416.7 |####################
  2430648.4 |####################
  2433880.2 |
  2437112.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=3972.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3982.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3928.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3948.1% of algo (FFI overhead may distort results)
- **nway**: bridge=2903.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=865.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=3852.0% of algo (FFI overhead may distort results)
