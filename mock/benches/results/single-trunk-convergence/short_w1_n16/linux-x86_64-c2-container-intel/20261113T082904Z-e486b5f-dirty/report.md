# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq beats baseline by 27% (significant)

kernel_seq is -692 ns (27%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 8.0x slower than the field

std_parallel (13.50 us) is 8.0x the fastest (1.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_seq, kernel_nway) are a dead heat (<1%)

kernel_seq (1.69 us) and kernel_nway (1.69 us) differ by 0.35%, inside the noise, even though the wider field spreads 699.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {kernel_seq, kernel_nway, head_tail_dyn, head_tail} vs {sequential, nway, std_parallel} (209% apart)

The field splits into a fast tier {kernel_seq, kernel_nway, head_tail_dyn, head_tail} and a slow tier {sequential, nway, std_parallel} with a 209% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.0x the fastest

Fastest kernel_seq (1.69 us) to slowest std_parallel (13.50 us): 8.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail_dyn is inconsistent: worst-20% is 1.8x its best-20%

head_tail_dyn's best 20% of batches run at 1.68 us but its worst 20% at 3.01 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 1688.8 ns median (-33.1% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 7.99x (fastest 1688.8 ns, slowest 13500.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 1 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100306664 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 2705ns | 2817ns | 1884ns | 2597ns | 3277ns | base |
| head_tail_dyn | 2674ns | 2636ns | 1907ns | 2461ns | 3376ns | -1.13% |
| kernel_nway | 2317ns | 2024ns | 1875ns | 2051ns | 2937ns | -14.33% |
| kernel_seq | 2049ns | 1952ns | 1878ns | 1944ns | 2291ns | -24.26% |
| nway | 13707ns | 13224ns | 12433ns | 13032ns | 15358ns | +406.82% |
| sequential | 8386ns | 8039ns | 7321ns | 7972ns | 9539ns | +210.07% |
| std_parallel | 12898ns | 13834ns | 9483ns | 12377ns | 15388ns | +376.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2386ns | 1661ns | 2876ns | base | 0.007 |
| head_tail_dyn | 2367ns | 1678ns | 3007ns | -0.80% | 0.007 |
| kernel_nway | 2032ns | 1663ns | 2597ns | -14.85% | 0.008 |
| kernel_seq | 1781ns | 1662ns | 1985ns | -25.35% | 0.009 |
| nway | 13355ns | 12134ns | 14916ns | +459.77% | 0.001 |
| sequential | 8109ns | 7087ns | 9245ns | +239.89% | 0.002 |
| std_parallel | 12548ns | 9221ns | 14950ns | +425.91% | 0.001 |

## Performance model

- Peak throughput: **0.010 Gops/s** (head_tail; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.006 | 65.8% |
| head_tail_dyn | 0.007 | 73.1% |
| kernel_nway | 0.009 | 98.0% |
| kernel_seq | 0.009 | 98.3% |
| nway | 0.001 | 12.9% |
| sequential | 0.002 | 21.3% |
| std_parallel | 0.001 | 12.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2705ns | 2705ns | base |
| head_tail_dyn | 2674ns | 2674ns | -1.13% |
| kernel_nway | 2317ns | 2317ns | -14.33% |
| kernel_seq | 2049ns | 2049ns | -24.26% |
| nway | 13707ns | 13707ns | +406.82% |
| sequential | 8386ns | 8386ns | +210.07% |
| std_parallel | 12898ns | 12898ns | +376.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2524ns | base | --- | [1949, 2584] | --- | --- | --- | --- |
| head_tail_dyn | 2272ns | no significant difference | [-568, +781]ns | [1952, 2714] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 1695ns | no significant difference | [-908, +725]ns | [1675, 2526] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 1689ns | -692.4ns (-27.4%) | [-904, -62]ns | [1666, 1811] | YES | 0.0463 | 0.0309 | 0 |
| nway | 12910ns | +10393.2ns (+411.7%) | [+9941, +10858]ns | [12489, 13029] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7789ns | +5437.8ns (+215.4%) | [+5292, +6029]ns | [7526, 8222] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 13500ns | +11361.6ns (+450.1%) | [+7297, +12126]ns | [9607, 14591] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1672ns | +64.1% | +53.0% | -0.4% | +751.5% | +343.5% | +728.0% |
| 1 | 2 | 2784ns | -38.9% | -39.8% | -39.7% | +363.8% | +193.7% | +424.7% |
| 1 | 3 | 2524ns | -20.9% | -33.6% | -27.8% | +404.0% | +209.1% | +270.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.309 | moderate- |
| head_tail_dyn | -0.344 | moderate- |
| kernel_nway | -0.166 | ok |
| kernel_seq | -0.129 | ok |
| nway | -0.106 | ok |
| sequential | -0.492 | moderate- |
| std_parallel | -0.250 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 2644.4ns | 2385.9ns | 110.8% | HIGH |
| head_tail_dyn | 2346.0ns | 2366.7ns | 99.1% | HIGH |
| kernel_nway | 2273.4ns | 2031.6ns | 111.9% | HIGH |
| kernel_seq | 2001.0ns | 1781.1ns | 112.3% | HIGH |
| nway | 14322.9ns | 13355.5ns | 107.2% | HIGH |
| sequential | 8455.4ns | 8109.3ns | 104.3% | HIGH |
| std_parallel | 12743.6ns | 12547.5ns | 101.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 1660.9-2876.5 ns)
   1660.9 |####################
   1721.6 |######
   1782.4 |
   1843.2 |
   1904.0 |
   1964.8 |
   2025.5 |
   2086.3 |
   2147.1 |######
   2207.9 |
   2268.7 |
   2329.4 |
   2390.2 |
   2451.0 |######
   2511.8 |########################################
   2572.6 |######
   2633.3 |
   2694.1 |
   2754.9 |####################
   2815.7 |
  (1 below, 1 above range)

head_tail_dyn (n=18, range 1677.7-3007.0 ns)
   1677.7 |##########################
   1744.1 |
   1810.6 |
   1877.1 |#############
   1943.5 |##########################
   2010.0 |#############
   2076.5 |
   2142.9 |#############
   2209.4 |
   2275.9 |#############
   2342.3 |
   2408.8 |
   2475.2 |
   2541.7 |
   2608.2 |#############
   2674.6 |########################################
   2741.1 |##########################
   2807.6 |
   2874.0 |
   2940.5 |
  (2 below, 2 above range)

kernel_nway (n=18, range 1663.4-2596.7 ns)
   1663.4 |########################################
   1710.1 |####
   1756.8 |
   1803.4 |
   1850.1 |
   1896.8 |
   1943.4 |
   1990.1 |
   2036.7 |
   2083.4 |
   2130.1 |
   2176.7 |
   2223.4 |
   2270.1 |
   2316.7 |
   2363.4 |
   2410.0 |
   2456.7 |
   2503.4 |#############
   2550.0 |#############
  (1 below, 1 above range)

kernel_seq (n=18, range 1661.7-1984.8 ns)
   1661.7 |########################################
   1677.9 |#####
   1694.0 |#####
   1710.2 |#####
   1726.3 |#####
   1742.5 |
   1758.6 |
   1774.8 |
   1790.9 |
   1807.1 |#################
   1823.2 |#####
   1839.4 |#####
   1855.6 |
   1871.7 |
   1887.9 |
   1904.0 |
   1920.2 |
   1936.3 |
   1952.5 |
   1968.6 |
  (1 below, 1 above range)

nway (n=18, range 12134.3-14915.5 ns)
  12134.3 |
  12273.4 |####################
  12412.5 |####################
  12551.5 |##########
  12690.6 |##########
  12829.6 |####################
  12968.7 |########################################
  13107.8 |
  13246.8 |
  13385.9 |
  13524.9 |
  13664.0 |
  13803.1 |##########
  13942.1 |
  14081.2 |
  14220.2 |
  14359.3 |
  14498.4 |
  14637.4 |
  14776.5 |
  (2 below, 3 above range)

sequential (n=18, range 7087.3-9244.7 ns)
   7087.3 |##########################
   7195.1 |
   7303.0 |
   7410.9 |#############
   7518.8 |##########################
   7626.6 |########################################
   7734.5 |
   7842.4 |##########################
   7950.3 |##########################
   8058.1 |
   8166.0 |
   8273.9 |
   8381.7 |#############
   8489.6 |
   8597.5 |
   8705.4 |
   8813.2 |#############
   8921.1 |#############
   9029.0 |
   9136.9 |
  (1 below, 2 above range)

std_parallel (n=18, range 9220.7-14950.0 ns)
   9220.7 |########################################
   9507.1 |
   9793.6 |#############
  10080.1 |
  10366.5 |
  10653.0 |
  10939.5 |
  11225.9 |
  11512.4 |
  11798.9 |
  12085.3 |
  12371.8 |#############
  12658.2 |#############
  12944.7 |
  13231.2 |#############
  13517.6 |#############
  13804.1 |##########################
  14090.6 |
  14377.0 |#############
  14663.5 |##########################
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=22.2% (high variance, measurements may be unstable)
- **head_tail**: bridge=106.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=25.6% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=92.9% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=21.9% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=108.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=112.0% of algo (FFI overhead may distort results)
- **nway**: bridge=101.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=106.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=98.6% of algo (FFI overhead may distort results)
