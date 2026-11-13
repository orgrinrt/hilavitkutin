# fold_heavy, frames of tens of milliseconds, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel dominates: 29% faster than the next best (kernel_nway)

std_parallel (1.70 ms) leads kernel_nway (2.19 ms) by 29%, a clear separation rather than a photo finish. CV 12.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel beats baseline by 27% (significant)

std_parallel is -667.97 us (27%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 2.7x slower than the field

sequential (4.53 ms) is 2.7x the fastest (1.70 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {std_parallel, kernel_nway, nway, head_tail, head_tail_dyn} vs {kernel_seq, sequential} (61% apart)

The field splits into a fast tier {std_parallel, kernel_nway, nway, head_tail, head_tail_dyn} and a slow tier {kernel_seq, sequential} with a 61% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### head_tail is inconsistent: worst-20% is 1.7x its best-20%

head_tail's best 20% of batches run at 2.07 ms but its worst 20% at 3.61 ms (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 1697267.4 ns median (-32.4% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.67x (fastest 1697267.4 ns, slowest 4533192.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, frames of tens of milliseconds, W = 4 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 2776177ns | 2513410ns | 2068374ns | 2451165ns | 3617598ns | base |
| head_tail_dyn | 2521609ns | 2545983ns | 2225772ns | 2514597ns | 2680046ns | -9.17% |
| kernel_nway | 2255755ns | 2191426ns | 1784662ns | 2120635ns | 2693981ns | -18.75% |
| kernel_seq | 4173089ns | 4087378ns | 3985233ns | 4059310ns | 4437683ns | +50.32% |
| nway | 2568875ns | 2448118ns | 2149926ns | 2354228ns | 3100321ns | -7.47% |
| sequential | 4516041ns | 4538355ns | 4270332ns | 4483247ns | 4688085ns | +62.67% |
| std_parallel | 1782596ns | 1700598ns | 1626246ns | 1694305ns | 1993208ns | -35.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2773272ns | 2066169ns | 3613463ns | base | 0.378 |
| head_tail_dyn | 2518252ns | 2224836ns | 2676305ns | -9.20% | 0.416 |
| kernel_nway | 2252696ns | 1780408ns | 2690765ns | -18.77% | 0.465 |
| kernel_seq | 4167575ns | 3982757ns | 4428295ns | +50.28% | 0.252 |
| nway | 2567046ns | 2147896ns | 3098557ns | -7.44% | 0.408 |
| sequential | 4513093ns | 4268230ns | 4686151ns | +62.74% | 0.232 |
| std_parallel | 1780325ns | 1624984ns | 1990945ns | -35.80% | 0.589 |

## Performance model

- Peak throughput: **0.645 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.418 | 64.7% |
| head_tail_dyn | 0.412 | 63.9% |
| kernel_nway | 0.479 | 74.3% |
| kernel_seq | 0.257 | 39.8% |
| nway | 0.429 | 66.4% |
| sequential | 0.231 | 35.8% |
| std_parallel | 0.618 | 95.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2776177ns | 2776177ns | base |
| head_tail_dyn | 2521609ns | 2521609ns | -9.17% |
| kernel_nway | 2255755ns | 2255755ns | -18.75% |
| kernel_seq | 4173089ns | 4173089ns | +50.32% |
| nway | 2568875ns | 2568875ns | -7.47% |
| sequential | 4516041ns | 4516041ns | +62.67% |
| std_parallel | 1782596ns | 1782596ns | -35.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2511370ns | base | --- | [2194984, 3613463] | --- | --- | --- | --- |
| head_tail_dyn | 2542128ns | no significant difference | [-1132710, +288663]ns | [2336323, 2676305] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 2188493ns | no significant difference | [-1051551, +107313]ns | [1878832, 2690765] | no | 0.3281 | 0.2188 | 0 |
| kernel_seq | 4082535ns | +1624390.8ns (+64.7%) | [+469072, +2089446]ns | [3991896, 4428295] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| nway | 2446373ns | no significant difference | [-926663, +576028]ns | [2156209, 3098557] | no | 0.8250 | 0.6875 | 0 |
| sequential | 4533193ns | +2116331.4ns (+84.3%) | [+747575, +2355556]ns | [4319936, 4686151] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 1697267ns | -667965.7ns (-26.6%) | [-1916196, -394681]ns | [1652763, 1990945] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3230619ns | -19.8% | -25.1% | +25.4% | -24.3% | +40.8% | -38.6% |
| 1 | 2 | 2750349ns | -15.1% | -13.2% | +46.3% | -9.9% | +57.1% | -38.7% |
| 1 | 3 | 2338849ns | +12.3% | -16.7% | +89.3% | +18.8% | +99.7% | -28.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.001 | ok |
| head_tail_dyn | -0.658 | HIGH- (thermal bounce) |
| kernel_nway | -0.132 | ok |
| kernel_seq | -0.205 | moderate- |
| nway | -0.121 | ok |
| sequential | -0.588 | HIGH- (thermal bounce) |
| std_parallel | -0.142 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 2773954.6ns | 2773272.5ns | 100.0% | HIGH |
| head_tail_dyn | 2631578.8ns | 2518252.1ns | 104.5% | HIGH |
| kernel_nway | 2365500.9ns | 2252696.5ns | 105.0% | HIGH |
| kernel_seq | 4187730.5ns | 4167575.4ns | 100.5% | HIGH |
| nway | 2535674.4ns | 2567046.0ns | 98.8% | HIGH |
| sequential | 4604756.1ns | 4513093.3ns | 102.0% | HIGH |
| std_parallel | 1759132.4ns | 1780325.1ns | 98.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2066168.6-3613463.2 ns)
  2066168.6 |####################
  2143533.3 |
  2220898.1 |
  2298262.8 |########################################
  2375627.5 |
  2452992.2 |
  2530357.0 |
  2607721.7 |####################
  2685086.4 |
  2762451.1 |
  2839815.9 |
  2917180.6 |
  2994545.3 |
  3071910.1 |
  3149274.8 |
  3226639.5 |
  3304004.2 |
  3381369.0 |####################
  3458733.7 |
  3536098.4 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 2224836.4-2676305.2 ns)
  2224836.4 |########################################
  2247409.8 |
  2269983.3 |
  2292556.7 |
  2315130.2 |
  2337703.6 |
  2360277.0 |
  2382850.5 |
  2405423.9 |
  2427997.4 |########################################
  2450570.8 |
  2473144.2 |
  2495717.7 |########################################
  2518291.1 |
  2540864.6 |
  2563438.0 |########################################
  2586011.4 |
  2608584.9 |
  2631158.3 |
  2653731.8 |########################################
  (0 below, 1 above range)

kernel_nway (n=6, range 1780408.3-2690764.8 ns)
  1780408.3 |########################################
  1825926.1 |
  1871443.9 |
  1916961.8 |
  1962479.6 |########################################
  2007997.4 |
  2053515.2 |
  2099033.1 |########################################
  2144550.9 |
  2190068.7 |
  2235586.5 |########################################
  2281104.4 |
  2326622.2 |
  2372140.0 |
  2417657.8 |
  2463175.7 |
  2508693.5 |########################################
  2554211.3 |
  2599729.1 |
  2645247.0 |
  (0 below, 1 above range)

kernel_seq (n=6, range 3982757.2-4428295.0 ns)
  3982757.2 |########################################
  4005034.1 |
  4027311.0 |
  4049587.9 |####################
  4071864.8 |
  4094141.7 |####################
  4116418.5 |
  4138695.4 |
  4160972.3 |
  4183249.2 |
  4205526.1 |
  4227803.0 |
  4250079.9 |
  4272356.8 |
  4294633.7 |
  4316910.5 |
  4339187.4 |####################
  4361464.3 |
  4383741.2 |
  4406018.1 |
  (0 below, 1 above range)

nway (n=6, range 2147895.9-3098556.5 ns)
  2147895.9 |########################################
  2195428.9 |
  2242962.0 |
  2290495.0 |####################
  2338028.0 |
  2385561.1 |
  2433094.1 |
  2480627.1 |
  2528160.2 |
  2575693.2 |####################
  2623226.2 |
  2670759.3 |
  2718292.3 |
  2765825.3 |####################
  2813358.4 |
  2860891.4 |
  2908424.4 |
  2955957.5 |
  3003490.5 |
  3051023.5 |
  (0 below, 1 above range)

sequential (n=6, range 4268229.8-4686150.8 ns)
  4268229.8 |########################################
  4289125.8 |
  4310021.9 |
  4330918.0 |
  4351814.0 |########################################
  4372710.0 |
  4393606.1 |
  4414502.1 |
  4435398.2 |########################################
  4456294.2 |
  4477190.3 |
  4498086.3 |
  4518982.4 |
  4539878.5 |
  4560774.5 |
  4581670.5 |
  4602566.6 |########################################
  4623462.6 |########################################
  4644358.7 |
  4665254.8 |
  (0 below, 1 above range)

std_parallel (n=6, range 1624983.5-1990944.6 ns)
  1624983.5 |####################
  1643281.6 |
  1661579.6 |
  1679877.7 |########################################
  1698175.7 |####################
  1716473.8 |####################
  1734771.8 |
  1753069.9 |
  1771367.9 |
  1789666.0 |
  1807964.1 |
  1826262.1 |
  1844560.2 |
  1862858.2 |
  1881156.3 |
  1899454.3 |
  1917752.4 |
  1936050.4 |
  1954348.5 |
  1972646.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=22.6% (high variance, measurements may be unstable)
- **head_tail**: bridge=106.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=106.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.4% of algo (FFI overhead may distort results)
- **nway**: bridge=104.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=97.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=102.9% of algo (FFI overhead may distort results)
