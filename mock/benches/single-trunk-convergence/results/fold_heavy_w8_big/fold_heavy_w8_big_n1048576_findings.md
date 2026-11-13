# fold_heavy, frames of tens of milliseconds, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway beats baseline by 25% (significant)

nway is -723.09 us (25%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 2.2x slower than the field

sequential (4.17 ms) is 2.2x the fastest (1.88 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.65)

nway's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, nway, std_parallel, head_tail_dyn, head_tail} vs {kernel_seq, sequential} (43% apart)

The field splits into a fast tier {kernel_nway, nway, std_parallel, head_tail_dyn, head_tail} and a slow tier {kernel_seq, sequential} with a 43% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### std_parallel is inconsistent: worst-20% is 1.9x its best-20%

std_parallel's best 20% of batches run at 1.68 ms but its worst 20% at 3.20 ms (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 1882269.5 ns median (-35.1% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.22x (fastest 1882269.5 ns, slowest 4169708.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, frames of tens of milliseconds, W = 8 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100242360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 2860946ns | 2904122ns | 2098772ns | 2670122ns | 3528269ns | base |
| head_tail_dyn | 2492708ns | 2531238ns | 2196643ns | 2421626ns | 2747364ns | -12.87% |
| kernel_nway | 1902504ns | 1885408ns | 1249416ns | 1878359ns | 2265266ns | -33.50% |
| kernel_seq | 4213179ns | 4152366ns | 3995956ns | 4106971ns | 4481102ns | +47.27% |
| nway | 2082771ns | 1975315ns | 1861928ns | 1956540ns | 2382539ns | -27.20% |
| sequential | 4210336ns | 4171550ns | 3964312ns | 4130308ns | 4453390ns | +47.17% |
| std_parallel | 2516905ns | 2375999ns | 1681662ns | 2334353ns | 3208354ns | -12.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2857010ns | 2096290ns | 3523106ns | base | 0.367 |
| head_tail_dyn | 2490238ns | 2195310ns | 2744605ns | -12.84% | 0.421 |
| kernel_nway | 1899478ns | 1246112ns | 2262129ns | -33.52% | 0.552 |
| kernel_seq | 4209577ns | 3993860ns | 4477326ns | +47.34% | 0.249 |
| nway | 2080888ns | 1860689ns | 2380502ns | -27.17% | 0.504 |
| sequential | 4208571ns | 3962341ns | 4451671ns | +47.31% | 0.249 |
| std_parallel | 2511054ns | 1680359ns | 3196523ns | -12.11% | 0.418 |

## Performance model

- Peak throughput: **0.841 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.361 | 43.0% |
| head_tail_dyn | 0.415 | 49.3% |
| kernel_nway | 0.557 | 66.2% |
| kernel_seq | 0.253 | 30.0% |
| nway | 0.531 | 63.2% |
| sequential | 0.251 | 29.9% |
| std_parallel | 0.442 | 52.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2860946ns | 2860946ns | base |
| head_tail_dyn | 2492708ns | 2492708ns | -12.87% |
| kernel_nway | 1902504ns | 1902504ns | -33.50% |
| kernel_seq | 4213179ns | 4213179ns | +47.27% |
| nway | 2082771ns | 2082771ns | -27.20% |
| sequential | 4210336ns | 4210336ns | +47.17% |
| std_parallel | 2516905ns | 2516905ns | -12.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2901307ns | base | --- | [2146618, 3523106] | --- | --- | --- | --- |
| head_tail_dyn | 2528532ns | no significant difference | [-1242874, +515148]ns | [2197579, 2744605] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 1882270ns | no significant difference | [-1961146, +32673]ns | [1554034, 2262129] | no | 0.3281 | 0.2188 | 0 |
| kernel_seq | 4148601ns | +1370196.1ns (+47.2%) | [+625957, +2061547]ns | [4002804, 4477326] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| nway | 1973087ns | -723090.8ns (-24.9%) | [-1347732, -257544]ns | [1889074, 2380502] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| sequential | 4169709ns | +1238940.1ns (+42.7%) | [+729256, +2086487]ns | [4004334, 4451671] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 2372957ns | no significant difference | [-1022224, +290693]ns | [1963682, 3196523] | no | 0.8250 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2818930ns | -19.0% | -44.4% | +58.8% | -30.2% | +52.4% | -30.3% |
| 1 | 2 | 3522644ns | -30.6% | -46.9% | +17.8% | -32.4% | +22.8% | -9.3% |
| 1 | 3 | 2229456ns | +23.1% | +1.5% | +79.5% | -15.0% | +79.6% | +6.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.529 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.020 | ok |
| kernel_nway | -0.004 | ok |
| kernel_seq | -0.031 | ok |
| nway | -0.655 | HIGH- (thermal bounce) |
| sequential | -0.217 | moderate- |
| std_parallel | -0.596 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 2907576.0ns | 2857010.2ns | 101.8% | HIGH |
| head_tail_dyn | 2394224.0ns | 2490238.5ns | 96.1% | HIGH |
| kernel_nway | 1866905.0ns | 1899477.7ns | 98.3% | HIGH |
| kernel_seq | 4213362.5ns | 4209576.9ns | 100.1% | HIGH |
| nway | 2154643.4ns | 2080888.0ns | 103.5% | HIGH |
| sequential | 4291910.2ns | 4208571.2ns | 102.0% | HIGH |
| std_parallel | 2317348.5ns | 2511054.1ns | 92.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2096290.2-3523105.8 ns)
  2096290.2 |####################
  2167631.0 |####################
  2238971.8 |
  2310312.5 |####################
  2381653.3 |
  2452994.1 |
  2524334.9 |
  2595675.7 |
  2667016.5 |
  2738357.2 |
  2809698.0 |
  2881038.8 |
  2952379.6 |
  3023720.4 |
  3095061.2 |
  3166401.9 |
  3237742.7 |
  3309083.5 |
  3380424.3 |########################################
  3451765.1 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 2195309.7-2744604.8 ns)
  2195309.7 |########################################
  2222774.5 |
  2250239.2 |
  2277704.0 |
  2305168.7 |
  2332633.5 |
  2360098.2 |####################
  2387563.0 |
  2415027.7 |
  2442492.5 |
  2469957.2 |
  2497422.0 |
  2524886.8 |
  2552351.5 |
  2579816.3 |
  2607281.0 |
  2634745.8 |
  2662210.5 |
  2689675.3 |####################
  2717140.0 |####################
  (0 below, 1 above range)

kernel_nway (n=6, range 1246111.6-2262129.4 ns)
  1246111.6 |#############
  1296912.5 |
  1347713.4 |
  1398514.3 |
  1449315.2 |
  1500116.0 |
  1550916.9 |
  1601717.8 |
  1652518.7 |
  1703319.6 |
  1754120.5 |
  1804921.4 |
  1855722.2 |########################################
  1906523.1 |
  1957324.0 |
  2008124.9 |#############
  2058925.8 |
  2109726.7 |
  2160527.6 |
  2211328.5 |
  (0 below, 1 above range)

kernel_seq (n=6, range 3993859.9-4477326.1 ns)
  3993859.9 |########################################
  4018033.2 |
  4042206.5 |
  4066379.8 |
  4090553.1 |
  4114726.4 |
  4138899.7 |########################################
  4163073.1 |
  4187246.4 |
  4211419.7 |
  4235593.0 |
  4259766.3 |
  4283939.6 |
  4308112.9 |
  4332286.2 |
  4356459.5 |
  4380632.8 |
  4404806.1 |####################
  4428979.4 |
  4453152.7 |
  (0 below, 1 above range)

nway (n=6, range 1860689.1-2380502.2 ns)
  1860689.1 |####################
  1886679.8 |
  1912670.4 |########################################
  1938661.1 |
  1964651.7 |
  1990642.4 |####################
  2016633.0 |
  2042623.7 |
  2068614.3 |
  2094605.0 |
  2120595.7 |
  2146586.3 |
  2172577.0 |
  2198567.6 |
  2224558.3 |
  2250548.9 |
  2276539.6 |
  2302530.2 |
  2328520.9 |####################
  2354511.5 |
  (0 below, 1 above range)

sequential (n=6, range 3962341.0-4451670.6 ns)
  3962341.0 |####################
  3986807.5 |
  4011274.0 |
  4035740.4 |####################
  4060206.9 |
  4084673.4 |
  4109139.9 |
  4133606.3 |
  4158072.8 |########################################
  4182539.3 |
  4207005.8 |
  4231472.3 |
  4255938.7 |
  4280405.2 |
  4304871.7 |
  4329338.2 |
  4353804.6 |
  4378271.1 |
  4402737.6 |####################
  4427204.1 |
  (0 below, 1 above range)

std_parallel (n=6, range 1680359.1-3196523.4 ns)
  1680359.1 |####################
  1756167.3 |
  1831975.5 |
  1907783.7 |
  1983592.0 |
  2059400.2 |
  2135208.4 |
  2211016.6 |########################################
  2286824.8 |
  2362633.0 |
  2438441.2 |####################
  2514249.5 |
  2590057.7 |
  2665865.9 |
  2741674.1 |####################
  2817482.3 |
  2893290.5 |
  2969098.8 |
  3044907.0 |
  3120715.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=22.6% (high variance, measurements may be unstable)
- **head_tail**: bridge=99.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=89.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.2% of algo (FFI overhead may distort results)
- **nway**: bridge=103.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=103.8% of algo (FFI overhead may distort results)
- **std_parallel**: CV=23.8% (high variance, measurements may be unstable)
- **std_parallel**: bridge=99.2% of algo (FFI overhead may distort results)
