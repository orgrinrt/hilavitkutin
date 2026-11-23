# fold_heavy, frames of tens of milliseconds, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn dominates: 13% faster than the next best (kernel_nway)

head_tail_dyn (1.81 ms) leads kernel_nway (2.04 ms) by 13%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 2.1x slower than the field

std_parallel (3.83 ms) is 2.1x the fastest (1.81 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {head_tail_dyn, kernel_nway, head_tail} vs {sequential, kernel_seq, nway, std_parallel} (43% apart)

The field splits into a fast tier {head_tail_dyn, kernel_nway, head_tail} and a slow tier {sequential, kernel_seq, nway, std_parallel} with a 43% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### std_parallel is inconsistent: worst-20% is 3.6x its best-20%

std_parallel's best 20% of batches run at 1.61 ms but its worst 20% at 5.75 ms (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 1809886.6 ns median (-15.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.12x (fastest 1809886.6 ns, slowest 3829232.5 ns)

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
| Counter frequency | 2112817522 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 2594973ns | 2140178ns | 1722054ns | 2064645ns | 3826925ns | base |
| head_tail_dyn | 1837019ns | 1815695ns | 1766822ns | 1801519ns | 1925369ns | -29.21% |
| kernel_nway | 2100218ns | 2048211ns | 1470768ns | 1879673ns | 2745761ns | -19.07% |
| kernel_seq | 3176638ns | 3178167ns | 3133631ns | 3163722ns | 3217514ns | +22.42% |
| nway | 3453817ns | 3429834ns | 1699589ns | 2987733ns | 5030057ns | +33.10% |
| sequential | 3040539ns | 3049204ns | 3000786ns | 3042088ns | 3058091ns | +17.17% |
| std_parallel | 3765488ns | 3832263ns | 1614997ns | 3157558ns | 5752628ns | +45.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2589195ns | 1718561ns | 3822880ns | base | 0.405 |
| head_tail_dyn | 1830721ns | 1764163ns | 1915447ns | -29.29% | 0.573 |
| kernel_nway | 2094401ns | 1463977ns | 2738406ns | -19.11% | 0.501 |
| kernel_seq | 3171290ns | 3132717ns | 3208600ns | +22.48% | 0.331 |
| nway | 3452524ns | 1698817ns | 5028440ns | +33.34% | 0.304 |
| sequential | 3040120ns | 3000328ns | 3057558ns | +17.42% | 0.345 |
| std_parallel | 3762302ns | 1613914ns | 5748770ns | +45.31% | 0.279 |

## Performance model

- Peak throughput: **0.716 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.492 | 68.6% |
| head_tail_dyn | 0.579 | 80.9% |
| kernel_nway | 0.514 | 71.7% |
| kernel_seq | 0.331 | 46.1% |
| nway | 0.306 | 42.7% |
| sequential | 0.344 | 48.0% |
| std_parallel | 0.274 | 38.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2594973ns | 2594973ns | base |
| head_tail_dyn | 1837019ns | 1837019ns | -29.21% |
| kernel_nway | 2100218ns | 2100218ns | -19.07% |
| kernel_seq | 3176638ns | 3176638ns | +22.42% |
| nway | 3453817ns | 3453817ns | +33.10% |
| sequential | 3040539ns | 3040539ns | +17.17% |
| std_parallel | 3765488ns | 3765488ns | +45.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2132818ns | base | --- | [1811885, 3822880] | --- | --- | --- | --- |
| head_tail_dyn | 1809887ns | -217371.1ns (-10.2%) | [-2056051, -1998]ns | [1766830, 1915447] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| kernel_nway | 2041871ns | no significant difference | [-2319954, +869250]ns | [1502927, 2738406] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 3172514ns | no significant difference | [-690123, +1384462]ns | [3132757, 3208600] | no | 0.4375 | 0.2188 | 0 |
| nway | 3428565ns | no significant difference | [-1885436, +3159284]ns | [1900568, 5028440] | no | 0.8250 | 0.6875 | 0 |
| sequential | 3048878ns | no significant difference | [-774002, +1202040]ns | [3013924, 3057558] | no | 0.4375 | 0.2188 | 0 |
| std_parallel | 3829232ns | no significant difference | [-403520, +2592161]ns | [1708903, 5748770] | no | 0.8250 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1811885ns | -0.1% | +41.4% | +76.4% | +94.4% | +66.3% | +67.7% |
| 1 | 2 | 2169695ns | -14.9% | -6.5% | +45.7% | +127.4% | +40.6% | +15.2% |
| 1 | 3 | 3786004ns | -51.5% | -55.3% | -16.6% | -49.8% | -19.3% | +51.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.080 | ok |
| head_tail_dyn | -0.325 | moderate- |
| kernel_nway | -0.011 | ok |
| kernel_seq | -0.121 | ok |
| nway | -0.476 | moderate- |
| sequential | -0.117 | ok |
| std_parallel | -0.263 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 2033688.6ns | 2589194.5ns | 78.5% | HIGH |
| head_tail_dyn | 1824649.6ns | 1830721.2ns | 99.7% | HIGH |
| kernel_nway | 2292323.4ns | 2094401.5ns | 109.5% | HIGH |
| kernel_seq | 3207556.2ns | 3171290.4ns | 101.1% | HIGH |
| nway | 3284706.6ns | 3452524.4ns | 95.1% | HIGH |
| sequential | 3043030.9ns | 3040120.4ns | 100.1% | HIGH |
| std_parallel | 3585212.1ns | 3762301.9ns | 95.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1718561.2-3822880.4 ns)
  1718561.2 |####################
  1823777.2 |####################
  1928993.1 |####################
  2034209.1 |
  2139425.0 |
  2244641.0 |########################################
  2349856.9 |
  2455072.9 |
  2560288.9 |
  2665504.8 |
  2770720.8 |
  2875936.7 |
  2981152.7 |
  3086368.6 |
  3191584.6 |
  3296800.6 |
  3402016.5 |
  3507232.5 |
  3612448.4 |
  3717664.4 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 1764162.7-1915447.4 ns)
  1764162.7 |########################################
  1771726.9 |
  1779291.2 |
  1786855.4 |
  1794419.6 |####################
  1801983.9 |
  1809548.1 |
  1817112.3 |
  1824676.6 |####################
  1832240.8 |
  1839805.0 |
  1847369.3 |
  1854933.5 |
  1862497.7 |
  1870062.0 |
  1877626.2 |
  1885190.4 |
  1892754.7 |
  1900318.9 |####################
  1907883.1 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1463976.7-2738406.4 ns)
  1463976.7 |########################################
  1527698.2 |########################################
  1591419.7 |
  1655141.2 |
  1718862.6 |
  1782584.1 |
  1846305.6 |
  1910027.1 |########################################
  1973748.6 |
  2037470.1 |
  2101191.6 |
  2164913.0 |########################################
  2228634.5 |
  2292356.0 |
  2356077.5 |
  2419799.0 |
  2483520.5 |########################################
  2547241.9 |
  2610963.4 |
  2674684.9 |
  (0 below, 1 above range)

kernel_seq (n=6, range 3132716.9-3208599.8 ns)
  3132716.9 |########################################
  3136511.0 |
  3140305.2 |
  3144099.3 |
  3147893.5 |
  3151687.6 |
  3155481.8 |
  3159275.9 |
  3163070.0 |####################
  3166864.2 |
  3170658.3 |
  3174452.5 |
  3178246.6 |####################
  3182040.8 |
  3185834.9 |####################
  3189629.0 |
  3193423.2 |
  3197217.3 |
  3201011.5 |
  3204805.6 |
  (0 below, 1 above range)

nway (n=6, range 1698816.6-5028440.4 ns)
  1698816.6 |####################
  1865297.8 |
  2031779.0 |########################################
  2198260.2 |
  2364741.4 |
  2531222.5 |
  2697703.7 |
  2864184.9 |
  3030666.1 |
  3197147.3 |
  3363628.5 |
  3530109.7 |
  3696590.9 |
  3863072.1 |
  4029553.3 |
  4196034.5 |
  4362515.6 |
  4528996.8 |####################
  4695478.0 |
  4861959.2 |####################
  (0 below, 1 above range)

sequential (n=6, range 3000328.3-3057558.5 ns)
  3000328.3 |########################################
  3003189.8 |
  3006051.3 |
  3008912.8 |
  3011774.3 |
  3014635.8 |
  3017497.3 |
  3020358.9 |
  3023220.4 |
  3026081.9 |########################################
  3028943.4 |
  3031804.9 |
  3034666.4 |
  3037527.9 |
  3040389.4 |
  3043250.9 |########################################
  3046112.4 |
  3048973.9 |
  3051835.4 |########################################
  3054696.9 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 1613914.4-5748770.4 ns)
  1613914.4 |########################################
  1820657.2 |
  2027400.0 |
  2234142.8 |
  2440885.6 |
  2647628.4 |
  2854371.2 |
  3061114.0 |
  3267856.8 |####################
  3474599.6 |
  3681342.4 |
  3888085.2 |
  4094828.0 |####################
  4301570.8 |
  4508313.6 |
  4715056.4 |####################
  4921799.2 |
  5128542.0 |
  5335284.8 |
  5542027.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=47.9% (high variance, measurements may be unstable)
- **head_tail**: bridge=94.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=25.2% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=116.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.2% of algo (FFI overhead may distort results)
- **nway**: CV=42.8% (high variance, measurements may be unstable)
- **nway**: bridge=80.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.9% of algo (FFI overhead may distort results)
- **std_parallel**: CV=46.4% (high variance, measurements may be unstable)
- **std_parallel**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **std_parallel**: bridge=97.4% of algo (FFI overhead may distort results)
