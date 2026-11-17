# append, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 5.1x slower than the field

sequential (11.57 ms) is 5.1x the fastest (2.25 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (2.25 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.1x the fastest

Fastest head_tail (2.25 ms) to slowest sequential (11.57 ms): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (head_tail) is the fastest** at 2254328.2 ns median
- 4 variants significantly slower than baseline
- Spread: 5.13x (fastest 2254328.2 ns, slowest 11568843.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 2 |
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
| head_tail | 2304892ns | 2258000ns | 2252948ns | 2256413ns | 2403583ns | base |
| head_tail_dyn | 2287226ns | 2281276ns | 2250510ns | 2271403ns | 2329318ns | -0.77% |
| kernel_nway | 2341201ns | 2359927ns | 2243594ns | 2327403ns | 2410703ns | +1.58% |
| kernel_seq | 2374535ns | 2303818ns | 2256323ns | 2292469ns | 2556740ns | +3.02% |
| nway | 5823674ns | 5801026ns | 5760802ns | 5787910ns | 5908755ns | +152.67% |
| sequential | 11692917ns | 11574766ns | 11451240ns | 11542674ns | 12039120ns | +407.31% |
| std_parallel | 2400771ns | 2319323ns | 2283000ns | 2318688ns | 2582781ns | +4.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2301280ns | 2248708ns | 2400438ns | base | 7.290 |
| head_tail_dyn | 2284118ns | 2246688ns | 2326479ns | -0.75% | 7.345 |
| kernel_nway | 2336885ns | 2238583ns | 2407021ns | +1.55% | 7.179 |
| kernel_seq | 2371472ns | 2253865ns | 2553203ns | +3.05% | 7.075 |
| nway | 5816905ns | 5753000ns | 5901823ns | +152.77% | 2.884 |
| sequential | 11687290ns | 11445604ns | 12034745ns | +407.86% | 1.436 |
| std_parallel | 2397477ns | 2279698ns | 2579057ns | +4.18% | 6.998 |

## Performance model

- Peak throughput: **7.495 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.442 | 99.3% |
| head_tail_dyn | 7.365 | 98.3% |
| kernel_nway | 7.124 | 95.1% |
| kernel_seq | 7.291 | 97.3% |
| nway | 2.895 | 38.6% |
| sequential | 1.450 | 19.4% |
| std_parallel | 7.242 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2304892ns | 2304892ns | base |
| head_tail_dyn | 2287226ns | 2287226ns | -0.77% |
| kernel_nway | 2341201ns | 2341201ns | +1.58% |
| kernel_seq | 2374535ns | 2374535ns | +3.02% |
| nway | 5823674ns | 5823674ns | +152.67% |
| sequential | 11692917ns | 11692917ns | +407.31% |
| std_parallel | 2400771ns | 2400771ns | +4.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2254328ns | base | --- | [2249073, 2400438] | --- | --- | --- | --- |
| head_tail_dyn | 2278088ns | no significant difference | [-121974, +71776]ns | [2247786, 2326479] | no | 0.6875 | 0.6875 | 0 |
| kernel_nway | 2355010ns | no significant difference | [-76688, +137297]ns | [2248625, 2407021] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 2301062ns | +50505.2ns (+2.2%) | [+7307, +152766]ns | [2260151, 2553203] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| nway | 5794974ns | +3501072.9ns (+155.3%) | [+3446401, +3599401]ns | [5753917, 5901823] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| sequential | 11568844ns | +9243630.2ns (+410.0%) | [+9205073, +9709328]ns | [11458281, 12034745] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| std_parallel | 2316635ns | +63427.0ns (+2.8%) | [+6286, +218880]ns | [2296740, 2579057] | YES (adj: no) | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2250193ns | +1.1% | -0.1% | +2.1% | +155.9% | +414.1% | +6.6% |
| 1 | 2 | 2400438ns | -5.1% | -1.2% | +6.4% | +143.6% | +401.4% | +3.2% |
| 1 | 3 | 2253208ns | +2.0% | +6.1% | +0.5% | +159.5% | +408.5% | +2.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.666 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.109 | ok |
| kernel_nway | -0.101 | ok |
| kernel_seq | -0.659 | HIGH- (thermal bounce) |
| nway | -0.167 | ok |
| sequential | -0.645 | HIGH- (thermal bounce) |
| std_parallel | -0.492 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 1/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 92190951.4ns | 2301279.5ns | 4006.1% | HIGH |
| head_tail_dyn | 92013038.2ns | 2284118.0ns | 4028.4% | HIGH |
| kernel_nway | 92343993.1ns | 2336885.4ns | 3951.6% | HIGH |
| kernel_seq | 91966225.7ns | 2371472.2ns | 3878.0% | HIGH |
| nway | 95763533.0ns | 5816904.5ns | 1646.3% | HIGH |
| sequential | 101915866.3ns | 11687289.9ns | 872.0% | HIGH |
| std_parallel | 92195206.6ns | 2397477.4ns | 3845.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2248708.3-2400437.5 ns)
  2248708.3 |########################################
  2256294.8 |#############
  2263881.2 |
  2271467.7 |
  2279054.1 |
  2286640.6 |
  2294227.1 |
  2301813.5 |
  2309400.0 |
  2316986.4 |
  2324572.9 |
  2332159.4 |#############
  2339745.8 |
  2347332.3 |
  2354918.7 |
  2362505.2 |
  2370091.7 |
  2377678.1 |
  2385264.6 |
  2392851.0 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 2246687.5-2326479.1 ns)
  2246687.5 |########################################
  2250677.1 |####################
  2254666.7 |
  2258656.2 |
  2262645.8 |
  2266635.4 |
  2270625.0 |
  2274614.6 |
  2278604.2 |
  2282593.7 |
  2286583.3 |
  2290572.9 |
  2294562.5 |
  2298552.1 |
  2302541.7 |########################################
  2306531.2 |
  2310520.8 |
  2314510.4 |
  2318500.0 |
  2322489.6 |
  (0 below, 1 above range)

kernel_nway (n=6, range 2238583.3-2407020.8 ns)
  2238583.3 |########################################
  2247005.2 |
  2255427.0 |########################################
  2263848.9 |
  2272270.8 |
  2280692.7 |
  2289114.5 |
  2297536.4 |
  2305958.3 |
  2314380.2 |
  2322802.0 |########################################
  2331223.9 |
  2339645.8 |
  2348067.7 |
  2356489.5 |
  2364911.4 |
  2373333.3 |########################################
  2381755.2 |
  2390177.0 |
  2398598.9 |########################################
  (0 below, 1 above range)

kernel_seq (n=6, range 2253864.6-2553203.1 ns)
  2253864.6 |########################################
  2268831.5 |####################
  2283798.5 |
  2298765.4 |
  2313732.3 |
  2328699.2 |####################
  2343666.1 |
  2358633.1 |
  2373600.0 |
  2388566.9 |
  2403533.9 |
  2418500.8 |
  2433467.7 |
  2448434.6 |
  2463401.6 |####################
  2478368.5 |
  2493335.4 |
  2508302.3 |
  2523269.2 |
  2538236.2 |
  (0 below, 1 above range)

nway (n=6, range 5753000.0-5901822.9 ns)
  5753000.0 |########################################
  5760441.1 |####################
  5767882.3 |
  5775323.4 |
  5782764.6 |
  5790205.7 |
  5797646.9 |
  5805088.0 |
  5812529.2 |
  5819970.3 |
  5827411.5 |####################
  5834852.6 |
  5842293.8 |
  5849734.9 |
  5857176.1 |
  5864617.2 |####################
  5872058.4 |
  5879499.5 |
  5886940.7 |
  5894381.8 |
  (0 below, 1 above range)

sequential (n=6, range 11445604.2-12034744.8 ns)
  11445604.2 |########################################
  11475061.2 |####################
  11504518.3 |
  11533975.3 |
  11563432.3 |
  11592889.3 |
  11622346.4 |
  11651803.4 |####################
  11681260.4 |
  11710717.5 |####################
  11740174.5 |
  11769631.5 |
  11799088.6 |
  11828545.6 |
  11858002.6 |
  11887459.7 |
  11916916.7 |
  11946373.7 |
  11975830.7 |
  12005287.8 |
  (0 below, 1 above range)

std_parallel (n=6, range 2279697.9-2579057.2 ns)
  2279697.9 |#############
  2294665.9 |
  2309633.8 |########################################
  2324601.8 |
  2339569.8 |
  2354537.7 |
  2369505.7 |
  2384473.7 |
  2399441.6 |
  2414409.6 |
  2429377.6 |
  2444345.5 |
  2459313.5 |
  2474281.5 |
  2489249.4 |
  2504217.4 |#############
  2519185.4 |
  2534153.3 |
  2549121.3 |
  2564089.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=4079.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=4041.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3922.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3989.2% of algo (FFI overhead may distort results)
- **nway**: bridge=1651.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=877.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=3965.6% of algo (FFI overhead may distort results)
