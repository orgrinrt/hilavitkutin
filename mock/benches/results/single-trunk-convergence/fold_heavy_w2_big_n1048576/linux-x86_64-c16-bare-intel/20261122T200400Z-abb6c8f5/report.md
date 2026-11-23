# fold_heavy, frames of tens of milliseconds, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 2.2x slower than the field

std_parallel (3.93 ms) is 2.2x the fastest (1.77 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.60)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, nway, kernel_nway, head_tail} vs {sequential, kernel_seq, std_parallel} (51% apart)

The field splits into a fast tier {head_tail_dyn, nway, kernel_nway, head_tail} and a slow tier {sequential, kernel_seq, std_parallel} with a 51% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### head_tail is inconsistent: worst-20% is 2.4x its best-20%

head_tail's best 20% of batches run at 1.65 ms but its worst 20% at 3.97 ms (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 1771853.6 ns median (-12.3% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.22x (fastest 1771853.6 ns, slowest 3930625.0 ns)

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
| Counter frequency | 2112775013 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 2561380ns | 2027873ns | 1653444ns | 1920837ns | 3976164ns | base |
| head_tail_dyn | 1777717ns | 1777991ns | 1736673ns | 1764434ns | 1818164ns | -30.60% |
| kernel_nway | 1817724ns | 1842763ns | 1664375ns | 1793176ns | 1931221ns | -29.03% |
| kernel_seq | 3163060ns | 3158847ns | 3124684ns | 3149384ns | 3202762ns | +23.49% |
| nway | 1894665ns | 1803367ns | 1684004ns | 1775985ns | 2178015ns | -26.03% |
| sequential | 3048846ns | 3054094ns | 3016783ns | 3045641ns | 3069687ns | +19.03% |
| std_parallel | 4065565ns | 3935342ns | 3564923ns | 3820956ns | 4682798ns | +58.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2556394ns | 1652473ns | 3970319ns | base | 0.410 |
| head_tail_dyn | 1772789ns | 1735678ns | 1810500ns | -30.65% | 0.591 |
| kernel_nway | 1811968ns | 1663453ns | 1920743ns | -29.12% | 0.579 |
| kernel_seq | 3158243ns | 3123758ns | 3193770ns | +23.54% | 0.332 |
| nway | 1893063ns | 1683367ns | 2176469ns | -25.95% | 0.554 |
| sequential | 3048387ns | 3016406ns | 3069274ns | +19.25% | 0.344 |
| std_parallel | 4062386ns | 3562891ns | 4680346ns | +58.91% | 0.258 |

## Performance model

- Peak throughput: **0.635 Gops/s** (head_tail; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.519 | 81.8% |
| head_tail_dyn | 0.592 | 93.3% |
| kernel_nway | 0.571 | 89.9% |
| kernel_seq | 0.332 | 52.4% |
| nway | 0.582 | 91.8% |
| sequential | 0.343 | 54.1% |
| std_parallel | 0.267 | 42.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2561380ns | 2561380ns | base |
| head_tail_dyn | 1777717ns | 1777717ns | -30.60% |
| kernel_nway | 1817724ns | 1817724ns | -29.03% |
| kernel_seq | 3163060ns | 3163060ns | +23.49% |
| nway | 1894665ns | 1894665ns | -26.03% |
| sequential | 3048846ns | 3048846ns | +19.03% |
| std_parallel | 4065565ns | 4065565ns | +58.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2020787ns | base | --- | [1678078, 3970319] | --- | --- | --- | --- |
| head_tail_dyn | 1771854ns | no significant difference | [-2198465, +57935]ns | [1736013, 1810500] | no | 0.6875 | 0.6875 | 0 |
| kernel_nway | 1837639ns | no significant difference | [-2184571, +63274]ns | [1677521, 1920743] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 3154229ns | no significant difference | [-817079, +1449640]ns | [3126729, 3193770] | no | 0.4375 | 0.2188 | 0 |
| nway | 1800769ns | no significant difference | [-2210729, +165539]ns | [1701952, 2176469] | no | 0.6875 | 0.6875 | 0 |
| sequential | 3053580ns | no significant difference | [-908016, +1363368]ns | [3022308, 3069274] | no | 0.4375 | 0.2188 | 0 |
| std_parallel | 3930625ns | +1651688.1ns (+81.7%) | [+130547, +2735740]ns | [3576188, 4680346] | YES (adj: no) | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1774365ns | -0.4% | +3.6% | +78.3% | +5.8% | +71.1% | +154.2% |
| 1 | 2 | 2479032ns | -27.3% | -27.5% | +27.5% | -18.4% | +22.6% | +55.5% |
| 1 | 3 | 3415786ns | -48.8% | -47.3% | -7.8% | -47.9% | -10.1% | +11.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.004 | ok |
| head_tail_dyn | -0.603 | HIGH- (thermal bounce) |
| kernel_nway | -0.185 | ok |
| kernel_seq | -0.089 | ok |
| nway | -0.558 | HIGH- (thermal bounce) |
| sequential | -0.096 | ok |
| std_parallel | -0.142 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 2089199.7ns | 2556394.5ns | 81.7% | HIGH |
| head_tail_dyn | 1789828.8ns | 1772788.9ns | 101.0% | HIGH |
| kernel_nway | 1982634.7ns | 1811967.8ns | 109.4% | HIGH |
| kernel_seq | 3180996.4ns | 3158242.5ns | 100.7% | HIGH |
| nway | 2107163.0ns | 1893063.5ns | 111.3% | HIGH |
| sequential | 3043995.2ns | 3048387.2ns | 99.9% | HIGH |
| std_parallel | 4532833.5ns | 4062386.3ns | 111.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1652473.1-3970318.6 ns)
  1652473.1 |########################################
  1768365.4 |
  1884257.6 |####################
  2000149.9 |
  2116042.2 |####################
  2231934.5 |
  2347826.7 |
  2463719.0 |
  2579611.3 |
  2695503.6 |
  2811395.8 |####################
  2927288.1 |
  3043180.4 |
  3159072.6 |
  3274964.9 |
  3390857.2 |
  3506749.5 |
  3622641.7 |
  3738534.0 |
  3854426.3 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 1735677.9-1810500.3 ns)
  1735677.9 |########################################
  1739419.0 |
  1743160.1 |
  1746901.3 |
  1750642.4 |
  1754383.5 |
  1758124.6 |
  1761865.7 |####################
  1765606.9 |
  1769348.0 |
  1773089.1 |
  1776830.2 |####################
  1780571.3 |
  1784312.5 |
  1788053.6 |
  1791794.7 |
  1795535.8 |####################
  1799276.9 |
  1803018.1 |
  1806759.2 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1663453.0-1920743.4 ns)
  1663453.0 |########################################
  1676317.5 |
  1689182.0 |########################################
  1702046.6 |
  1714911.1 |
  1727775.6 |
  1740640.1 |
  1753504.6 |
  1766369.2 |########################################
  1779233.7 |
  1792098.2 |
  1804962.7 |
  1817827.2 |
  1830691.8 |
  1843556.3 |
  1856420.8 |
  1869285.3 |
  1882149.8 |
  1895014.4 |########################################
  1907878.9 |########################################
  (0 below, 1 above range)

kernel_seq (n=6, range 3123757.7-3193770.1 ns)
  3123757.7 |########################################
  3127258.3 |########################################
  3130758.9 |########################################
  3134259.6 |
  3137760.2 |
  3141260.8 |
  3144761.4 |
  3148262.0 |
  3151762.7 |
  3155263.3 |
  3158763.9 |
  3162264.5 |
  3165765.1 |
  3169265.8 |
  3172766.4 |
  3176267.0 |########################################
  3179767.6 |
  3183268.2 |
  3186768.9 |
  3190269.5 |########################################
  (0 below, 1 above range)

nway (n=6, range 1683366.9-2176469.0 ns)
  1683366.9 |########################################
  1708022.0 |########################################
  1732677.1 |
  1757332.2 |########################################
  1781987.3 |
  1806642.4 |
  1831297.5 |########################################
  1855952.7 |
  1880607.8 |
  1905262.9 |
  1929918.0 |
  1954573.1 |
  1979228.2 |########################################
  2003883.3 |
  2028538.4 |
  2053193.5 |
  2077848.6 |
  2102503.7 |
  2127158.8 |
  2151813.9 |
  (0 below, 1 above range)

sequential (n=6, range 3016405.6-3069274.2 ns)
  3016405.6 |########################################
  3019049.0 |
  3021692.5 |
  3024335.9 |
  3026979.3 |########################################
  3029622.8 |
  3032266.2 |
  3034909.6 |
  3037553.1 |
  3040196.5 |
  3042839.9 |
  3045483.4 |
  3048126.8 |
  3050770.2 |########################################
  3053413.7 |########################################
  3056057.1 |
  3058700.5 |
  3061344.0 |
  3063987.4 |########################################
  3066630.8 |
  (0 below, 1 above range)

std_parallel (n=6, range 3562890.6-4680345.8 ns)
  3562890.6 |########################################
  3618763.4 |
  3674636.1 |
  3730508.9 |
  3786381.6 |####################
  3842254.4 |
  3898127.2 |
  3953999.9 |
  4009872.7 |####################
  4065745.5 |
  4121618.2 |####################
  4177491.0 |
  4233363.8 |
  4289236.5 |
  4345109.3 |
  4400982.0 |
  4456854.8 |
  4512727.6 |
  4568600.3 |
  4624473.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=47.4% (high variance, measurements may be unstable)
- **head_tail**: bridge=99.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=103.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.5% of algo (FFI overhead may distort results)
- **nway**: bridge=110.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=103.4% of algo (FFI overhead may distort results)
