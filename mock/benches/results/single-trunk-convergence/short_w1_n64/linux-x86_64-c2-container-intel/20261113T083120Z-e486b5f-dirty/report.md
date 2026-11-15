# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 8.2x slower than the field

std_parallel (13.72 us) is 8.2x the fastest (1.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail, kernel_nway) are a dead heat (<1%)

head_tail (1.68 us) and kernel_nway (1.69 us) differ by 0.79%, inside the noise, even though the wider field spreads 717.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.65)

kernel_nway's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (1.68 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, kernel_nway, head_tail_dyn, kernel_seq} vs {sequential, nway, std_parallel} (273% apart)

The field splits into a fast tier {head_tail, kernel_nway, head_tail_dyn, kernel_seq} and a slow tier {sequential, nway, std_parallel} with a 273% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.2x the fastest

Fastest head_tail (1.68 us) to slowest std_parallel (13.72 us): 8.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader head_tail vs stability leader kernel_nway (+1% speed for 1.5x steadier)

head_tail is fastest (1.68 us, CV 15.0%); kernel_nway gives up 0.8% median for 1.5x lower variance (CV 9.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### kernel_seq is inconsistent: worst-20% is 1.7x its best-20%

kernel_seq's best 20% of batches run at 1.72 us but its worst 20% at 2.91 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 1679.3 ns median
- 4 variants significantly slower than baseline
- Spread: 8.17x (fastest 1679.3 ns, slowest 13722.0 ns)

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
| head_tail | 2059ns | 2020ns | 1871ns | 1959ns | 2303ns | base |
| head_tail_dyn | 2126ns | 2036ns | 1880ns | 1987ns | 2459ns | +3.28% |
| kernel_nway | 2019ns | 1938ns | 1890ns | 1939ns | 2203ns | -1.93% |
| kernel_seq | 2606ns | 2386ns | 1938ns | 2388ns | 3267ns | +26.58% |
| nway | 14251ns | 13844ns | 11438ns | 13530ns | 16740ns | +592.17% |
| sequential | 9099ns | 7890ns | 7276ns | 7975ns | 11696ns | +341.93% |
| std_parallel | 14058ns | 14207ns | 10989ns | 13720ns | 16100ns | +582.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 1793ns | 1661ns | 2010ns | base | 0.036 |
| head_tail_dyn | 1852ns | 1667ns | 2124ns | +3.29% | 0.035 |
| kernel_nway | 1752ns | 1680ns | 1877ns | -2.32% | 0.037 |
| kernel_seq | 2293ns | 1719ns | 2907ns | +27.85% | 0.028 |
| nway | 13927ns | 11149ns | 16392ns | +676.62% | 0.005 |
| sequential | 8800ns | 7046ns | 11346ns | +390.70% | 0.007 |
| std_parallel | 13663ns | 10713ns | 15693ns | +661.91% | 0.005 |

## Performance model

- Peak throughput: **0.039 Gops/s** (head_tail; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.038 | 98.9% |
| head_tail_dyn | 0.037 | 95.3% |
| kernel_nway | 0.038 | 98.1% |
| kernel_seq | 0.031 | 81.4% |
| nway | 0.005 | 12.3% |
| sequential | 0.008 | 21.8% |
| std_parallel | 0.005 | 12.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2059ns | 2059ns | base |
| head_tail_dyn | 2126ns | 2126ns | +3.28% |
| kernel_nway | 2019ns | 2019ns | -1.93% |
| kernel_seq | 2606ns | 2606ns | +26.58% |
| nway | 14251ns | 14251ns | +592.17% |
| sequential | 9099ns | 9099ns | +341.93% |
| std_parallel | 14058ns | 14058ns | +582.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 1679ns | base | --- | [1673, 1822] | --- | --- | --- | --- |
| head_tail_dyn | 1742ns | no significant difference | [-7, +77]ns | [1676, 1882] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 1693ns | no significant difference | [-114, +46]ns | [1687, 1716] | no | 0.4807 | 0.4807 | 0 |
| kernel_seq | 2040ns | +270.4ns (+16.1%) | [+26, +1113]ns | [1848, 2814] | YES | 0.0020 | 0.0013 | 0 |
| nway | 13531ns | +11374.6ns (+677.3%) | [+10911, +13750]ns | [12657, 15414] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7608ns | +5847.6ns (+348.2%) | [+5421, +8294]ns | [7188, 9967] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 13722ns | +11727.9ns (+698.4%) | [+10847, +13348]ns | [12570, 15017] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1834ns | +2.7% | -8.0% | +3.1% | +637.9% | +287.7% | +600.9% |
| 1 | 2 | 1675ns | +0.8% | +2.1% | +8.7% | +570.1% | +353.7% | +665.4% |
| 1 | 3 | 1673ns | +0.5% | +1.1% | +70.8% | +880.7% | +570.4% | +814.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.160 | ok |
| head_tail_dyn | -0.146 | ok |
| kernel_nway | -0.649 | HIGH- (thermal bounce) |
| kernel_seq | -0.202 | moderate- |
| nway | -0.462 | moderate- |
| sequential | -0.108 | ok |
| std_parallel | -0.173 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 1893.0ns | 1793.3ns | 105.6% | HIGH |
| head_tail_dyn | 1995.3ns | 1852.3ns | 107.7% | HIGH |
| kernel_nway | 1877.5ns | 1751.7ns | 107.2% | HIGH |
| kernel_seq | 2572.8ns | 2292.8ns | 112.2% | HIGH |
| nway | 14843.9ns | 13927.1ns | 106.6% | HIGH |
| sequential | 9295.5ns | 8799.8ns | 105.6% | HIGH |
| std_parallel | 13867.1ns | 13663.4ns | 101.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 1660.8-2010.2 ns)
   1660.8 |########################################
   1678.3 |#####
   1695.8 |#####
   1713.2 |
   1730.7 |
   1748.2 |
   1765.6 |
   1783.1 |
   1800.6 |##########
   1818.0 |#####
   1835.5 |##########
   1853.0 |
   1870.4 |
   1887.9 |
   1905.4 |
   1922.8 |
   1940.3 |
   1957.8 |#####
   1975.2 |
   1992.7 |
  (1 below, 1 above range)

head_tail_dyn (n=18, range 1666.6-2123.9 ns)
   1666.6 |########################################
   1689.4 |################
   1712.3 |########
   1735.2 |########
   1758.0 |
   1780.9 |
   1803.8 |
   1826.7 |########
   1849.5 |################
   1872.4 |
   1895.3 |########
   1918.1 |
   1941.0 |########
   1963.9 |
   1986.7 |
   2009.6 |
   2032.5 |
   2055.3 |
   2078.2 |
   2101.1 |
  (1 below, 3 above range)

kernel_nway (n=18, range 1680.2-1877.2 ns)
   1680.2 |########################################
   1690.1 |########################
   1699.9 |################
   1709.8 |################
   1719.6 |
   1729.5 |
   1739.3 |########
   1749.2 |
   1759.0 |
   1768.9 |
   1778.7 |########
   1788.6 |
   1798.4 |
   1808.3 |
   1818.1 |
   1828.0 |
   1837.8 |
   1847.7 |
   1857.5 |
   1867.4 |
  (2 below, 2 above range)

kernel_seq (n=18, range 1719.0-2906.9 ns)
   1719.0 |
   1778.4 |##########
   1837.8 |########################################
   1897.2 |####################
   1956.6 |
   2016.0 |
   2075.4 |
   2134.8 |##########
   2194.2 |
   2253.5 |
   2312.9 |
   2372.3 |
   2431.7 |
   2491.1 |
   2550.5 |##########
   2609.9 |
   2669.3 |##########
   2728.7 |##########
   2788.1 |
   2847.5 |####################
  (2 below, 3 above range)

nway (n=18, range 11149.2-16391.8 ns)
  11149.2 |########################################
  11411.3 |
  11673.4 |
  11935.6 |
  12197.7 |
  12459.8 |#############
  12721.9 |##########################
  12984.1 |#############
  13246.2 |#############
  13508.3 |
  13770.5 |#############
  14032.6 |#############
  14294.7 |
  14556.9 |
  14819.0 |#############
  15081.1 |
  15343.2 |##########################
  15605.4 |#############
  15867.5 |
  16129.6 |
  (1 below, 3 above range)

sequential (n=18, range 7045.5-11346.1 ns)
   7045.5 |########################################
   7260.5 |##########
   7475.6 |##############################
   7690.6 |
   7905.6 |
   8120.7 |
   8335.7 |
   8550.7 |####################
   8765.7 |
   8980.8 |
   9195.8 |
   9410.8 |
   9625.9 |
   9840.9 |##########
  10055.9 |
  10271.0 |
  10486.0 |##########
  10701.0 |
  10916.1 |##########
  11131.1 |
  (2 below, 3 above range)

std_parallel (n=18, range 10713.2-15692.7 ns)
  10713.2 |
  10962.2 |
  11211.2 |
  11460.1 |####################
  11709.1 |
  11958.1 |####################
  12207.1 |####################
  12456.0 |
  12705.0 |########################################
  12954.0 |####################
  13203.0 |####################
  13451.9 |
  13700.9 |
  13949.9 |####################
  14198.8 |
  14447.8 |########################################
  14696.8 |####################
  14945.8 |####################
  15194.7 |####################
  15443.7 |####################
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=110.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=105.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=107.3% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=22.2% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=97.3% of algo (FFI overhead may distort results)
- **nway**: bridge=105.5% of algo (FFI overhead may distort results)
- **sequential**: CV=22.1% (high variance, measurements may be unstable)
- **sequential**: bridge=113.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=97.3% of algo (FFI overhead may distort results)
