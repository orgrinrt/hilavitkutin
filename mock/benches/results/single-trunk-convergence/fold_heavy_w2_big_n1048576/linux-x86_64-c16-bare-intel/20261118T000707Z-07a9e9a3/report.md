# fold_heavy, frames of tens of milliseconds, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 2.2x slower than the field

std_parallel (3.97 ms) is 2.2x the fastest (1.77 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail, kernel_nway) are a dead heat (<1%)

head_tail (1.77 ms) and kernel_nway (1.78 ms) differ by 0.74%, inside the noise, even though the wider field spreads 124.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### nway shows alternating (throttle bounce) (autocorr -0.66)

nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (1.77 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### head_tail_dyn is inconsistent: worst-20% is 1.6x its best-20%

head_tail_dyn's best 20% of batches run at 1.75 ms but its worst 20% at 2.79 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 1770697.5 ns median
- 4 variants significantly slower than baseline
- Spread: 2.24x (fastest 1770697.5 ns, slowest 3966942.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, frames of tens of milliseconds, W = 2 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 1841650ns | 1776470ns | 1670183ns | 1755185ns | 2057080ns | base |
| head_tail_dyn | 2256129ns | 2151880ns | 1752040ns | 2064946ns | 2794949ns | +22.51% |
| kernel_nway | 1897244ns | 1792256ns | 1521126ns | 1748053ns | 2309088ns | +3.02% |
| kernel_seq | 3731279ns | 3182952ns | 3129319ns | 3166121ns | 4879996ns | +102.61% |
| nway | 1812296ns | 1839438ns | 1644120ns | 1786510ns | 1935063ns | -1.59% |
| sequential | 3044342ns | 3047405ns | 3023355ns | 3040982ns | 3059874ns | +65.31% |
| std_parallel | 3908838ns | 3968334ns | 3658884ns | 3918942ns | 4018659ns | +112.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 1835953ns | 1669274ns | 2047446ns | base | 0.571 |
| head_tail_dyn | 2251986ns | 1749478ns | 2791637ns | +22.66% | 0.466 |
| kernel_nway | 1892900ns | 1520533ns | 2305590ns | +3.10% | 0.554 |
| kernel_seq | 3726355ns | 3128506ns | 4875778ns | +102.97% | 0.281 |
| nway | 1811190ns | 1643469ns | 1933870ns | -1.35% | 0.579 |
| sequential | 3043907ns | 3022876ns | 3059392ns | +65.79% | 0.344 |
| std_parallel | 3907175ns | 3656847ns | 4017259ns | +112.81% | 0.268 |

## Performance model

- Peak throughput: **0.690 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.592 | 85.9% |
| head_tail_dyn | 0.488 | 70.8% |
| kernel_nway | 0.588 | 85.2% |
| kernel_seq | 0.330 | 47.9% |
| nway | 0.571 | 82.7% |
| sequential | 0.344 | 49.9% |
| std_parallel | 0.264 | 38.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 1841650ns | 1841650ns | base |
| head_tail_dyn | 2256129ns | 2256129ns | +22.51% |
| kernel_nway | 1897244ns | 1897244ns | +3.02% |
| kernel_seq | 3731279ns | 3731279ns | +102.61% |
| nway | 1812296ns | 1812296ns | -1.59% |
| sequential | 3044342ns | 3044342ns | +65.31% |
| std_parallel | 3908838ns | 3908838ns | +112.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 1770698ns | base | --- | [1689716, 2047446] | --- | --- | --- | --- |
| head_tail_dyn | 2148003ns | +374502.9ns (+21.2%) | [+29191, +844404]ns | [1816318, 2791637] | YES | 0.0469 | 0.0313 | 0 |
| kernel_nway | 1783766ns | no significant difference | [-266469, +509394]ns | [1589342, 2305590] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 3173888ns | +1374021.8ns (+77.6%) | [+1158160, +3139024]ns | [3129399, 4875778] | YES | 0.0469 | 0.0313 | 0 |
| nway | 1837953ns | no significant difference | [-140651, +68292]ns | [1661747, 1933870] | no | 1.0000 | 1.0000 | 0 |
| sequential | 3047010ns | +1272294.6ns (+71.9%) | [+988158, +1363408]ns | [3025319, 3059392] | YES | 0.0469 | 0.0313 | 0 |
| std_parallel | 3966943ns | +2101176.3ns (+118.7%) | [+1826964, +2285526]ns | [3737325, 4017259] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1695983ns | +40.4% | +21.4% | +85.5% | +4.0% | +80.4% | +130.6% |
| 1 | 2 | 1995238ns | +26.8% | -12.6% | +58.3% | -5.2% | +51.6% | +91.6% |
| 1 | 3 | 1816638ns | +1.6% | +3.3% | +168.4% | -2.2% | +67.7% | +119.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.560 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.297 | moderate- |
| kernel_nway | -0.444 | moderate- |
| kernel_seq | -0.163 | ok |
| nway | -0.661 | HIGH- (thermal bounce) |
| sequential | -0.581 | HIGH- (thermal bounce) |
| std_parallel | -0.522 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 1939138.1ns | 1835953.1ns | 105.6% | HIGH |
| head_tail_dyn | 2355901.7ns | 2251985.8ns | 104.6% | HIGH |
| kernel_nway | 2033556.2ns | 1892899.6ns | 107.4% | HIGH |
| kernel_seq | 3201598.9ns | 3726354.9ns | 85.9% | HIGH |
| nway | 1899292.3ns | 1811190.1ns | 104.9% | HIGH |
| sequential | 3039576.5ns | 3043906.7ns | 99.9% | HIGH |
| std_parallel | 3866221.6ns | 3907175.4ns | 99.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1669273.8-2047446.1 ns)
  1669273.8 |####################
  1688182.4 |
  1707091.0 |########################################
  1725999.7 |
  1744908.3 |
  1763816.9 |
  1782725.5 |
  1801634.1 |####################
  1820542.7 |
  1839451.4 |
  1858360.0 |
  1877268.6 |
  1896177.2 |
  1915085.8 |####################
  1933994.4 |
  1952903.1 |
  1971811.7 |
  1990720.3 |
  2009628.9 |
  2028537.5 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 1749477.9-2791636.8 ns)
  1749477.9 |########################################
  1801585.8 |
  1853693.8 |########################################
  1905801.7 |########################################
  1957909.7 |
  2010017.6 |
  2062125.6 |
  2114233.5 |
  2166341.4 |
  2218449.4 |
  2270557.3 |
  2322665.3 |########################################
  2374773.2 |########################################
  2426881.2 |
  2478989.1 |
  2531097.0 |
  2583205.0 |
  2635312.9 |
  2687420.9 |
  2739528.8 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1520533.0-2305590.4 ns)
  1520533.0 |########################################
  1559785.9 |
  1599038.7 |
  1638291.6 |########################################
  1677544.5 |
  1716797.4 |########################################
  1756050.2 |
  1795303.1 |########################################
  1834556.0 |
  1873808.8 |
  1913061.7 |
  1952314.6 |
  1991567.4 |
  2030820.3 |
  2070073.2 |
  2109326.0 |
  2148578.9 |
  2187831.8 |
  2227084.7 |########################################
  2266337.5 |
  (0 below, 1 above range)

kernel_seq (n=6, range 3128506.5-4875777.8 ns)
  3128506.5 |########################################
  3215870.1 |##########
  3303233.6 |
  3390597.2 |
  3477960.8 |
  3565324.3 |
  3652687.9 |
  3740051.5 |
  3827415.0 |
  3914778.6 |
  4002142.2 |
  4089505.7 |
  4176869.3 |
  4264232.9 |
  4351596.4 |
  4438960.0 |
  4526323.6 |
  4613687.1 |
  4701050.7 |
  4788414.3 |
  (0 below, 1 above range)

nway (n=6, range 1643468.9-1933870.1 ns)
  1643468.9 |########################################
  1657989.0 |
  1672509.0 |########################################
  1687029.1 |
  1701549.1 |
  1716069.2 |
  1730589.3 |
  1745109.3 |
  1759629.4 |
  1774149.4 |
  1788669.5 |
  1803189.6 |
  1817709.6 |########################################
  1832229.7 |
  1846749.7 |########################################
  1861269.8 |
  1875789.9 |
  1890309.9 |
  1904830.0 |########################################
  1919350.0 |
  (0 below, 1 above range)

sequential (n=6, range 3022875.7-3059391.7 ns)
  3022875.7 |########################################
  3024701.5 |
  3026527.3 |########################################
  3028353.1 |
  3030178.9 |
  3032004.7 |
  3033830.5 |
  3035656.3 |
  3037482.1 |
  3039307.9 |
  3041133.7 |
  3042959.5 |
  3044785.3 |########################################
  3046611.1 |########################################
  3048436.9 |
  3050262.7 |
  3052088.5 |
  3053914.3 |########################################
  3055740.1 |
  3057565.9 |
  (0 below, 1 above range)

std_parallel (n=6, range 3656846.6-4017258.8 ns)
  3656846.6 |########################################
  3674867.2 |
  3692887.8 |
  3710908.4 |
  3728929.0 |
  3746949.6 |
  3764970.2 |
  3782990.9 |
  3801011.5 |########################################
  3819032.1 |
  3837052.7 |
  3855073.3 |
  3873093.9 |
  3891114.5 |
  3909135.1 |
  3927155.7 |
  3945176.3 |########################################
  3963196.9 |
  3981217.5 |########################################
  3999238.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=105.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=21.2% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=108.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=109.3% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=33.6% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=99.7% of algo (FFI overhead may distort results)
- **nway**: bridge=104.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=97.1% of algo (FFI overhead may distort results)
