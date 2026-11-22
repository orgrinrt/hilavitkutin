# fold_heavy, frames of tens of milliseconds, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail is fastest but the noisiest (CV 27.7%)

head_tail wins on median (1.76 ms) yet has the highest variance (CV 27.7%), while sequential is the steadiest (CV 0.5%, 3.02 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential shows alternating (throttle bounce) (autocorr -0.63)

sequential's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (1.76 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, head_tail_dyn, kernel_nway, nway} vs {std_parallel, sequential, kernel_seq} (40% apart)

The field splits into a fast tier {head_tail, head_tail_dyn, kernel_nway, nway} and a slow tier {std_parallel, sequential, kernel_seq} with a 40% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### std_parallel is inconsistent: worst-20% is 2.4x its best-20%

std_parallel's best 20% of batches run at 1.43 ms but its worst 20% at 3.44 ms (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 1759492.1 ns median
- 2 variants significantly slower than baseline
- Spread: 1.80x (fastest 1759492.1 ns, slowest 3166847.3 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 2032385ns | 1764402ns | 1667654ns | 1753236ns | 2633476ns | base |
| head_tail_dyn | 1828508ns | 1809226ns | 1731745ns | 1785781ns | 1940979ns | -10.03% |
| kernel_nway | 1805137ns | 1813622ns | 1549721ns | 1750221ns | 2015218ns | -11.18% |
| kernel_seq | 3176205ns | 3172297ns | 3134963ns | 3163865ns | 3215334ns | +56.28% |
| nway | 2144579ns | 2053110ns | 1551474ns | 1974096ns | 2696857ns | +5.52% |
| sequential | 3025255ns | 3018887ns | 3009352ns | 3016385ns | 3046512ns | +48.85% |
| std_parallel | 2651623ns | 2875190ns | 1430579ns | 2526732ns | 3449480ns | +30.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2028038ns | 1666799ns | 2627450ns | base | 0.517 |
| head_tail_dyn | 1820539ns | 1730782ns | 1927264ns | -10.23% | 0.576 |
| kernel_nway | 1800267ns | 1545142ns | 2008307ns | -11.23% | 0.582 |
| kernel_seq | 3170947ns | 3134138ns | 3206601ns | +56.36% | 0.331 |
| nway | 2143460ns | 1550911ns | 2695362ns | +5.69% | 0.489 |
| sequential | 3024836ns | 3008970ns | 3046088ns | +49.15% | 0.347 |
| std_parallel | 2648158ns | 1426148ns | 3444741ns | +30.58% | 0.396 |

## Performance model

- Peak throughput: **0.735 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.596 | 81.1% |
| head_tail_dyn | 0.582 | 79.2% |
| kernel_nway | 0.580 | 78.8% |
| kernel_seq | 0.331 | 45.0% |
| nway | 0.511 | 69.5% |
| sequential | 0.347 | 47.2% |
| std_parallel | 0.365 | 49.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2032385ns | 2032385ns | base |
| head_tail_dyn | 1828508ns | 1828508ns | -10.03% |
| kernel_nway | 1805137ns | 1805137ns | -11.18% |
| kernel_seq | 3176205ns | 3176205ns | +56.28% |
| nway | 2144579ns | 2144579ns | +5.52% |
| sequential | 3025255ns | 3025255ns | +48.85% |
| std_parallel | 2651623ns | 2651623ns | +30.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 1759492ns | base | --- | [1697171, 2627450] | --- | --- | --- | --- |
| head_tail_dyn | 1800291ns | no significant difference | [-751679, +103233]ns | [1734062, 1927264] | no | 0.6875 | 0.6875 | 0 |
| kernel_nway | 1809010ns | no significant difference | [-621288, +99884]ns | [1583484, 2008307] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 3166847ns | +1398446.1ns (+79.5%) | [+579151, +1451131]ns | [3139393, 3206601] | YES (adj: no) | 0.1875 | 0.0313 | 0 |
| nway | 2051824ns | no significant difference | [-572679, +831921]ns | [1683195, 2695362] | no | 0.6875 | 0.6875 | 0 |
| sequential | 3018477ns | +1268567.5ns (+72.1%) | [+387009, +1334818]ns | [3009942, 3046088] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| std_parallel | 2872425ns | no significant difference | [-114333, +1540303]ns | [1627309, 3444741] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2402919ns | -21.5% | -23.7% | +31.7% | -23.8% | +25.6% | +43.4% |
| 1 | 2 | 1711270ns | +6.0% | +5.8% | +84.2% | +11.3% | +78.0% | +34.1% |
| 1 | 3 | 1969924ns | -10.7% | -10.9% | +62.3% | +36.8% | +52.8% | +12.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.411 | moderate- |
| head_tail_dyn | -0.004 | ok |
| kernel_nway | -0.036 | ok |
| kernel_seq | -0.324 | moderate- |
| nway | -0.124 | ok |
| sequential | -0.633 | HIGH- (thermal bounce) |
| std_parallel | -0.131 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 2217594.6ns | 2028037.7ns | 109.3% | HIGH |
| head_tail_dyn | 1880292.4ns | 1820538.9ns | 103.3% | HIGH |
| kernel_nway | 1850387.7ns | 1800266.8ns | 102.8% | HIGH |
| kernel_seq | 3241773.2ns | 3170947.3ns | 102.2% | HIGH |
| nway | 2441290.8ns | 2143460.5ns | 113.9% | HIGH |
| sequential | 3030390.1ns | 3024835.8ns | 100.2% | HIGH |
| std_parallel | 2538321.1ns | 2648158.4ns | 95.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1666799.1-2627450.0 ns)
  1666799.1 |####################
  1714831.6 |########################################
  1762864.2 |####################
  1810896.7 |
  1858929.3 |
  1906961.8 |
  1954994.4 |
  2003026.9 |
  2051059.4 |
  2099092.0 |
  2147124.5 |
  2195157.1 |####################
  2243189.6 |
  2291222.2 |
  2339254.7 |
  2387287.2 |
  2435319.8 |
  2483352.3 |
  2531384.9 |
  2579417.4 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 1730781.6-1927263.8 ns)
  1730781.6 |########################################
  1740605.7 |
  1750429.8 |
  1760253.9 |
  1770078.0 |
  1779902.1 |####################
  1789726.2 |
  1799550.4 |
  1809374.5 |####################
  1819198.6 |
  1829022.7 |
  1838846.8 |
  1848670.9 |
  1858495.0 |
  1868319.1 |
  1878143.2 |
  1887967.3 |####################
  1897791.4 |
  1907615.5 |
  1917439.6 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1545142.5-2008306.7 ns)
  1545142.5 |########################################
  1568300.7 |
  1591458.9 |
  1614617.1 |########################################
  1637775.3 |
  1660933.6 |
  1684091.8 |
  1707250.0 |########################################
  1730408.2 |
  1753566.4 |
  1776724.6 |
  1799882.8 |
  1823041.0 |
  1846199.2 |
  1869357.4 |########################################
  1892515.6 |########################################
  1915673.9 |
  1938832.1 |
  1961990.3 |
  1985148.5 |
  (0 below, 1 above range)

kernel_seq (n=6, range 3134138.1-3206601.5 ns)
  3134138.1 |########################################
  3137761.3 |
  3141384.4 |########################################
  3145007.6 |
  3148630.8 |
  3152253.9 |
  3155877.1 |
  3159500.3 |########################################
  3163123.4 |
  3166746.6 |
  3170369.8 |########################################
  3173992.9 |
  3177616.1 |
  3181239.3 |########################################
  3184862.4 |
  3188485.6 |
  3192108.8 |
  3195731.9 |
  3199355.1 |
  3202978.3 |
  (0 below, 1 above range)

nway (n=6, range 1550911.3-2695362.3 ns)
  1550911.3 |########################################
  1608133.9 |
  1665356.4 |
  1722578.9 |
  1779801.5 |########################################
  1837024.1 |
  1894246.6 |
  1951469.1 |########################################
  2008691.7 |
  2065914.2 |########################################
  2123136.8 |
  2180359.3 |
  2237581.9 |########################################
  2294804.5 |
  2352027.0 |
  2409249.5 |
  2466472.1 |
  2523694.6 |
  2580917.2 |
  2638139.8 |
  (0 below, 1 above range)

sequential (n=6, range 3008969.7-3046088.0 ns)
  3008969.7 |########################################
  3010825.6 |########################################
  3012681.5 |
  3014537.4 |
  3016393.4 |########################################
  3018249.3 |########################################
  3020105.2 |
  3021961.1 |
  3023817.0 |
  3025672.9 |
  3027528.8 |
  3029384.7 |
  3031240.7 |
  3033096.6 |
  3034952.5 |
  3036808.4 |
  3038664.3 |
  3040520.2 |
  3042376.1 |
  3044232.0 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 1426147.7-3444741.2 ns)
  1426147.7 |########################################
  1527077.4 |
  1628007.1 |
  1728936.7 |########################################
  1829866.4 |
  1930796.1 |
  2031725.8 |
  2132655.4 |
  2233585.1 |
  2334514.8 |
  2435444.5 |
  2536374.1 |########################################
  2637303.8 |
  2738233.5 |
  2839163.2 |
  2940092.8 |
  3041022.5 |
  3141952.2 |########################################
  3242881.9 |
  3343811.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=24.1% (high variance, measurements may be unstable)
- **head_tail**: bridge=111.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=96.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.2% of algo (FFI overhead may distort results)
- **nway**: CV=23.4% (high variance, measurements may be unstable)
- **nway**: bridge=114.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.2% of algo (FFI overhead may distort results)
- **std_parallel**: CV=29.7% (high variance, measurements may be unstable)
- **std_parallel**: bridge=78.4% of algo (FFI overhead may distort results)
