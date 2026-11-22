# fold_heavy, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 3.2x slower than the field

std_parallel (612.89 us) is 3.2x the fastest (192.91 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_seq, head_tail) are a dead heat (<1%)

kernel_seq (192.91 us) and head_tail (193.60 us) differ by 0.36%, inside the noise, even though the wider field spreads 217.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### nway shows alternating (throttle bounce) (autocorr -0.67)

nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, head_tail, kernel_nway, head_tail_dyn, sequential, nway} vs {std_parallel} (153% apart)

The field splits into a fast tier {kernel_seq, head_tail, kernel_nway, head_tail_dyn, sequential, nway} and a slow tier {std_parallel} with a 153% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.2x the fastest

Fastest kernel_seq (192.91 us) to slowest std_parallel (612.89 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader kernel_seq vs stability leader sequential (+1% speed for 22.0x steadier)

kernel_seq is fastest (192.91 us, CV 15.2%); sequential gives up 1.3% median for 22.0x lower variance (CV 0.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### nway is inconsistent: worst-20% is 1.7x its best-20%

nway's best 20% of batches run at 199.57 us but its worst 20% at 329.30 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 192911.8 ns median (-0.4% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 3.18x (fastest 192911.8 ns, slowest 612890.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 1 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 206428ns | 194120ns | 191811ns | 193845ns | 232610ns | base |
| head_tail_dyn | 211750ns | 194183ns | 190715ns | 194123ns | 248709ns | +2.58% |
| kernel_nway | 205268ns | 194233ns | 193163ns | 194409ns | 227609ns | -0.56% |
| kernel_seq | 206782ns | 193372ns | 191687ns | 193283ns | 234578ns | +0.17% |
| nway | 261141ns | 242804ns | 199739ns | 235760ns | 329914ns | +26.50% |
| sequential | 195202ns | 195508ns | 193348ns | 195086ns | 196302ns | -5.44% |
| std_parallel | 622975ns | 614754ns | 521706ns | 602726ns | 703983ns | +201.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 205346ns | 191411ns | 230350ns | base | 0.319 |
| head_tail_dyn | 210770ns | 190270ns | 246700ns | +2.64% | 0.311 |
| kernel_nway | 204316ns | 192714ns | 225775ns | -0.50% | 0.321 |
| kernel_seq | 205835ns | 191194ns | 232748ns | +0.24% | 0.318 |
| nway | 260823ns | 199569ns | 329303ns | +27.02% | 0.251 |
| sequential | 195030ns | 193182ns | 196124ns | -5.02% | 0.336 |
| std_parallel | 620318ns | 519810ns | 700868ns | +202.08% | 0.106 |

## Performance model

- Peak throughput: **0.344 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.339 | 98.3% |
| head_tail_dyn | 0.338 | 98.2% |
| kernel_nway | 0.338 | 98.2% |
| kernel_seq | 0.340 | 98.6% |
| nway | 0.270 | 78.4% |
| sequential | 0.335 | 97.4% |
| std_parallel | 0.107 | 31.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 206428ns | 206428ns | base |
| head_tail_dyn | 211750ns | 211750ns | +2.58% |
| kernel_nway | 205268ns | 205268ns | -0.56% |
| kernel_seq | 206782ns | 206782ns | +0.17% |
| nway | 261141ns | 261141ns | +26.50% |
| sequential | 195202ns | 195202ns | -5.44% |
| std_parallel | 622975ns | 622975ns | +201.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 193604ns | base | --- | [193028, 194801] | --- | --- | --- | --- |
| head_tail_dyn | 193741ns | no significant difference | [-1868, +2277]ns | [192924, 211902] | no | 0.9774 | 0.8145 | 0 |
| kernel_nway | 193670ns | no significant difference | [-623, +1984]ns | [193350, 195827] | no | 0.9774 | 0.8145 | 0 |
| kernel_seq | 192912ns | no significant difference | [-1386, +1417]ns | [192110, 194872] | no | 1.0000 | 1.0000 | 0 |
| nway | 242635ns | +48898.5ns (+25.3%) | [+37760, +51145]ns | [231669, 242898] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 195346ns | no significant difference | [-366, +1963]ns | [194394, 195493] | no | 0.9614 | 0.4807 | 0 |
| std_parallel | 612891ns | +404113.7ns (+208.7%) | [+383367, +425950]ns | [583750, 626721] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 193725ns | -0.8% | +0.5% | -0.9% | +25.3% | +0.9% | +201.3% |
| 1 | 2 | 193465ns | +0.3% | -0.1% | -0.2% | +3.8% | +1.0% | +223.2% |
| 1 | 3 | 192987ns | +9.6% | +0.8% | +0.8% | +25.7% | +0.2% | +220.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.019 | ok |
| head_tail_dyn | -0.118 | ok |
| kernel_nway | -0.665 | HIGH- (thermal bounce) |
| kernel_seq | -0.002 | ok |
| nway | -0.667 | HIGH- (thermal bounce) |
| sequential | -0.165 | ok |
| std_parallel | -0.277 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 2/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 208178.6ns | 205345.7ns | 101.4% | HIGH |
| head_tail_dyn | 211557.0ns | 210770.1ns | 100.4% | HIGH |
| kernel_nway | 205827.2ns | 204315.6ns | 100.7% | HIGH |
| kernel_seq | 208292.2ns | 205834.8ns | 101.2% | HIGH |
| nway | 263282.4ns | 260822.5ns | 100.9% | HIGH |
| sequential | 195251.5ns | 195029.7ns | 100.1% | HIGH |
| std_parallel | 632517.7ns | 620317.6ns | 102.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 191410.7-230350.2 ns)
  191410.7 |########################################
  193357.7 |########################################
  195304.7 |#############
  197251.6 |
  199198.6 |
  201145.6 |
  203092.6 |
  205039.5 |
  206986.5 |
  208933.5 |
  210880.4 |
  212827.4 |
  214774.4 |
  216721.4 |
  218668.3 |
  220615.3 |
  222562.3 |
  224509.3 |
  226456.2 |
  228403.2 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 190269.5-246700.0 ns)
  190269.5 |#################
  193091.0 |########################################
  195912.5 |#####
  198734.1 |
  201555.6 |
  204377.1 |
  207198.6 |
  210020.2 |
  212841.7 |
  215663.2 |
  218484.8 |
  221306.3 |
  224127.8 |
  226949.3 |#####
  229770.9 |
  232592.4 |
  235413.9 |
  238235.4 |
  241057.0 |#####
  243878.5 |
  (2 below, 3 above range)

kernel_nway (n=18, range 192714.0-225775.1 ns)
  192714.0 |########################################
  194367.0 |#################
  196020.1 |
  197673.1 |
  199326.2 |
  200979.2 |
  202632.3 |
  204285.4 |
  205938.4 |
  207591.5 |
  209244.5 |####
  210897.6 |
  212550.6 |
  214203.7 |
  215856.8 |
  217509.8 |
  219162.9 |
  220815.9 |
  222469.0 |
  224122.0 |
  (1 below, 3 above range)

kernel_seq (n=18, range 191194.4-232747.8 ns)
  191194.4 |########################################
  193272.1 |#################
  195349.8 |####
  197427.4 |
  199505.1 |
  201582.8 |
  203660.4 |
  205738.1 |
  207815.8 |
  209893.4 |
  211971.1 |
  214048.8 |
  216126.4 |
  218204.1 |
  220281.8 |
  222359.4 |
  224437.1 |
  226514.8 |
  228592.4 |
  230670.1 |
  (1 below, 3 above range)

nway (n=18, range 199569.0-329303.4 ns)
  199569.0 |############
  206055.8 |
  212542.5 |
  219029.2 |####
  225515.9 |
  232002.6 |
  238489.4 |########################################
  244976.1 |
  251462.8 |
  257949.5 |
  264436.2 |
  270923.0 |
  277409.7 |
  283896.4 |
  290383.1 |
  296869.8 |
  303356.6 |
  309843.3 |
  316330.0 |
  322816.7 |
  (1 below, 3 above range)

sequential (n=18, range 193182.2-196124.4 ns)
  193182.2 |####################
  193329.3 |
  193476.4 |##########
  193623.6 |
  193770.7 |
  193917.8 |
  194064.9 |
  194212.0 |
  194359.1 |
  194506.2 |
  194653.3 |
  194800.4 |
  194947.5 |
  195094.6 |##########
  195241.7 |########################################
  195388.8 |##############################
  195536.0 |########################################
  195683.1 |
  195830.2 |
  195977.3 |
  (2 below, 1 above range)

std_parallel (n=18, range 519810.1-700868.0 ns)
  519810.1 |
  528863.0 |
  537915.9 |
  546968.8 |
  556021.7 |
  565074.6 |
  574127.5 |########################################
  583180.4 |#############
  592233.3 |##########################
  601286.2 |
  610339.1 |########################################
  619392.0 |##########################
  628444.9 |##########################
  637497.8 |
  646550.7 |
  655603.6 |#############
  664656.5 |
  673709.4 |
  682762.3 |#############
  691815.2 |
  (2 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=99.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.8% of algo (FFI overhead may distort results)
- **nway**: CV=30.1% (high variance, measurements may be unstable)
- **nway**: bridge=100.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.2% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=101.7% of algo (FFI overhead may distort results)
