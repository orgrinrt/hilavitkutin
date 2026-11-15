# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 6.3x slower than the field

std_parallel (11.84 us) is 6.3x the fastest (1.89 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail_dyn, kernel_seq) are a dead heat (<1%)

head_tail_dyn (1.89 us) and kernel_seq (1.89 us) differ by 0.11%, inside the noise, even though the wider field spreads 526.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, kernel_seq, kernel_nway, head_tail} vs {sequential, nway, std_parallel} (240% apart)

The field splits into a fast tier {head_tail_dyn, kernel_seq, kernel_nway, head_tail} and a slow tier {sequential, nway, std_parallel} with a 240% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.3x the fastest

Fastest head_tail_dyn (1.89 us) to slowest std_parallel (11.84 us): 6.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail is inconsistent: worst-20% is 1.6x its best-20%

head_tail's best 20% of batches run at 1.90 us but its worst 20% at 3.05 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 1888.8 ns median (-20.5% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 6.27x (fastest 1888.8 ns, slowest 11836.1 ns)

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
| head_tail | 2762ns | 2670ns | 2168ns | 2549ns | 3378ns | base |
| head_tail_dyn | 2182ns | 2131ns | 1926ns | 2107ns | 2422ns | -20.99% |
| kernel_nway | 2683ns | 2378ns | 2172ns | 2366ns | 3415ns | -2.83% |
| kernel_seq | 2362ns | 2189ns | 1992ns | 2157ns | 2855ns | -14.47% |
| nway | 11783ns | 11528ns | 11398ns | 11549ns | 12327ns | +326.69% |
| sequential | 9303ns | 8337ns | 7284ns | 8321ns | 11786ns | +236.88% |
| std_parallel | 12702ns | 12141ns | 9933ns | 11727ns | 15549ns | +359.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2452ns | 1902ns | 3047ns | base | 0.104 |
| head_tail_dyn | 1908ns | 1714ns | 2068ns | -22.21% | 0.134 |
| kernel_nway | 2296ns | 1935ns | 2922ns | -6.38% | 0.112 |
| kernel_seq | 2062ns | 1725ns | 2515ns | -15.90% | 0.124 |
| nway | 11496ns | 11098ns | 12038ns | +368.78% | 0.022 |
| sequential | 8901ns | 7064ns | 11072ns | +262.94% | 0.029 |
| std_parallel | 12345ns | 9653ns | 15141ns | +403.41% | 0.021 |

## Performance model

- Peak throughput: **0.149 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.108 | 72.1% |
| head_tail_dyn | 0.136 | 90.7% |
| kernel_nway | 0.127 | 85.1% |
| kernel_seq | 0.135 | 90.6% |
| nway | 0.023 | 15.2% |
| sequential | 0.032 | 21.2% |
| std_parallel | 0.022 | 14.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2762ns | 2762ns | base |
| head_tail_dyn | 2182ns | 2182ns | -20.99% |
| kernel_nway | 2683ns | 2683ns | -2.83% |
| kernel_seq | 2362ns | 2362ns | -14.47% |
| nway | 11783ns | 11783ns | +326.69% |
| sequential | 9303ns | 9303ns | +236.88% |
| std_parallel | 12702ns | 12702ns | +359.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2377ns | base | --- | [2017, 2645] | --- | --- | --- | --- |
| head_tail_dyn | 1889ns | -272.8ns (-11.5%) | [-831, -138]ns | [1869, 1909] | YES | 0.0002 | 0.0001 | 0 |
| kernel_nway | 2014ns | no significant difference | [-650, +293]ns | [1978, 2542] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 1891ns | -293.2ns (-12.3%) | [-694, -46]ns | [1873, 1959] | YES | 0.0371 | 0.0309 | 0 |
| nway | 11265ns | +9013.6ns (+379.2%) | [+8574, +9310]ns | [11176, 11649] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 8092ns | +5431.5ns (+228.5%) | [+5024, +8057]ns | [7475, 10293] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 11836ns | +9435.5ns (+396.9%) | [+7946, +11952]ns | [10111, 13970] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2029ns | -7.4% | +31.5% | -14.3% | +469.1% | +253.9% | +476.0% |
| 1 | 2 | 2684ns | -35.8% | -24.9% | -29.0% | +316.4% | +199.3% | +269.8% |
| 1 | 3 | 2068ns | -8.1% | -5.9% | -7.2% | +443.5% | +432.0% | +583.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.665 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.657 | HIGH- (thermal bounce) |
| kernel_nway | -0.121 | ok |
| kernel_seq | -0.127 | ok |
| nway | -0.271 | moderate- |
| sequential | -0.062 | ok |
| std_parallel | -0.443 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 2406.6ns | 2452.3ns | 98.1% | HIGH |
| head_tail_dyn | 2152.2ns | 1907.7ns | 112.8% | HIGH |
| kernel_nway | 2388.1ns | 2295.8ns | 104.0% | HIGH |
| kernel_seq | 2247.0ns | 2062.3ns | 109.0% | HIGH |
| nway | 11822.1ns | 11496.1ns | 102.8% | HIGH |
| sequential | 9017.9ns | 8900.6ns | 101.3% | HIGH |
| std_parallel | 12317.3ns | 12345.2ns | 99.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 1901.7-3047.0 ns)
   1901.7 |####################
   1959.0 |########################################
   2016.3 |########################################
   2073.5 |
   2130.8 |
   2188.1 |########################################
   2245.3 |
   2302.6 |
   2359.8 |
   2417.1 |
   2474.4 |########################################
   2531.6 |####################
   2588.9 |####################
   2646.2 |########################################
   2703.4 |
   2760.7 |
   2818.0 |
   2875.2 |
   2932.5 |####################
   2989.8 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 1713.6-2067.5 ns)
   1713.6 |####################
   1731.3 |
   1749.0 |
   1766.7 |
   1784.4 |
   1802.1 |
   1819.8 |
   1837.4 |
   1855.1 |##########
   1872.8 |########################################
   1890.5 |########################################
   1908.2 |##########
   1925.9 |
   1943.6 |##########
   1961.3 |
   1979.0 |
   1996.7 |##########
   2014.4 |
   2032.1 |
   2049.8 |
  (2 below, 2 above range)

kernel_nway (n=18, range 1934.7-2922.3 ns)
   1934.7 |######################
   1984.1 |########################################
   2033.5 |
   2082.8 |
   2132.2 |
   2181.6 |
   2231.0 |
   2280.4 |
   2329.7 |
   2379.1 |
   2428.5 |
   2477.9 |
   2527.3 |#####
   2576.6 |#####
   2626.0 |#####
   2675.4 |#####
   2724.8 |
   2774.2 |
   2823.5 |#####
   2872.9 |
  (1 below, 1 above range)

kernel_seq (n=18, range 1724.7-2514.9 ns)
   1724.7 |##############################
   1764.2 |
   1803.7 |
   1843.2 |########################################
   1882.7 |##############################
   1922.2 |####################
   1961.7 |##########
   2001.2 |
   2040.7 |
   2080.3 |##########
   2119.8 |
   2159.3 |
   2198.8 |
   2238.3 |
   2277.8 |
   2317.3 |
   2356.8 |
   2396.3 |
   2435.8 |
   2475.4 |##########
  (1 below, 2 above range)

nway (n=18, range 11098.1-12038.1 ns)
  11098.1 |##########
  11145.1 |########################################
  11192.1 |####################
  11239.1 |
  11286.1 |##########
  11333.1 |
  11380.1 |
  11427.1 |##########
  11474.1 |##########
  11521.1 |
  11568.1 |##########
  11615.1 |
  11662.1 |
  11709.1 |##########
  11756.1 |##########
  11803.1 |
  11850.1 |##########
  11897.1 |
  11944.1 |
  11991.1 |
  (2 below, 2 above range)

sequential (n=18, range 7064.3-11071.8 ns)
   7064.3 |##############################
   7264.7 |
   7465.1 |
   7665.5 |########################################
   7865.8 |
   8066.2 |
   8266.6 |##########
   8467.0 |
   8667.3 |##########
   8867.7 |
   9068.1 |
   9268.5 |
   9468.8 |
   9669.2 |
   9869.6 |
  10070.0 |####################
  10270.3 |
  10470.7 |##########
  10671.1 |##########
  10871.5 |
  (2 below, 3 above range)

std_parallel (n=18, range 9652.9-15140.7 ns)
   9652.9 |##########################
   9927.3 |##########################
  10201.7 |
  10476.1 |##########################
  10750.5 |#############
  11024.9 |
  11299.2 |
  11573.6 |
  11848.0 |
  12122.4 |
  12396.8 |
  12671.2 |#############
  12945.6 |
  13220.0 |
  13494.4 |#############
  13768.8 |########################################
  14043.2 |#############
  14317.6 |
  14591.9 |
  14866.3 |#############
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=22.4% (high variance, measurements may be unstable)
- **head_tail**: bridge=95.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=106.0% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=24.4% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=107.4% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=25.0% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=106.5% of algo (FFI overhead may distort results)
- **nway**: bridge=105.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=108.4% of algo (FFI overhead may distort results)
