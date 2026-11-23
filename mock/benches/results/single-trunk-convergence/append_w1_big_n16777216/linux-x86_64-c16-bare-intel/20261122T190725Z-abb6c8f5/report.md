# append, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (head_tail, kernel_nway) are a dead heat (<1%)

head_tail (9.87 ms) and kernel_nway (9.87 ms) differ by 0.02%, inside the noise, even though the wider field spreads 49.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### sequential shows alternating (throttle bounce) (autocorr -0.57)

sequential's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (9.87 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, kernel_nway, head_tail_dyn, kernel_seq} vs {std_parallel, sequential, nway} (39% apart)

The field splits into a fast tier {head_tail, kernel_nway, head_tail_dyn, kernel_seq} and a slow tier {std_parallel, sequential, nway} with a 39% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader head_tail vs stability leader head_tail_dyn (+1% speed for 4.9x steadier)

head_tail is fastest (9.87 ms, CV 3.4%); head_tail_dyn gives up 0.6% median for 4.9x lower variance (CV 0.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (head_tail) is the fastest** at 9866191.7 ns median
- 3 variants significantly slower than baseline
- Spread: 1.49x (fastest 9866191.7 ns, slowest 14732430.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 1 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112813859 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 9679216ns | 9868934ns | 9050462ns | 9712483ns | 9943692ns | base |
| head_tail_dyn | 9908946ns | 9932933ns | 9759606ns | 9924532ns | 9960236ns | +2.37% |
| kernel_nway | 9734035ns | 9871814ns | 9091218ns | 9799861ns | 9956704ns | +0.57% |
| kernel_seq | 9695710ns | 9938903ns | 9143694ns | 9674952ns | 10002855ns | +0.17% |
| nway | 14687516ns | 14734808ns | 14477391ns | 14683042ns | 14799290ns | +51.74% |
| sequential | 14537851ns | 14182852ns | 13948632ns | 14139930ns | 15429342ns | +50.20% |
| std_parallel | 12980090ns | 13800310ns | 10685380ns | 12876006ns | 14283571ns | +34.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9675426ns | 9046008ns | 9938771ns | base | 1.734 |
| head_tail_dyn | 9904921ns | 9756778ns | 9956319ns | +2.37% | 1.694 |
| kernel_nway | 9729904ns | 9087846ns | 9952777ns | +0.56% | 1.724 |
| kernel_seq | 9691425ns | 9140599ns | 9997307ns | +0.17% | 1.731 |
| nway | 14685067ns | 14475058ns | 14796700ns | +51.78% | 1.142 |
| sequential | 14536045ns | 13946993ns | 15427558ns | +50.24% | 1.154 |
| std_parallel | 12976455ns | 10682483ns | 14279174ns | +34.12% | 1.293 |

## Performance model

- Peak throughput: **1.855 Gops/s** (head_tail; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.700 | 91.7% |
| head_tail_dyn | 1.690 | 91.1% |
| kernel_nway | 1.700 | 91.7% |
| kernel_seq | 1.689 | 91.0% |
| nway | 1.139 | 61.4% |
| sequential | 1.183 | 63.8% |
| std_parallel | 1.216 | 65.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9679216ns | 9679216ns | base |
| head_tail_dyn | 9908946ns | 9908946ns | +2.37% |
| kernel_nway | 9734035ns | 9734035ns | +0.57% |
| kernel_seq | 9695710ns | 9695710ns | +0.17% |
| nway | 14687516ns | 14687516ns | +51.74% |
| sequential | 14537851ns | 14537851ns | +50.20% |
| std_parallel | 12980090ns | 12980090ns | +34.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9866192ns | base | --- | [9221315, 9938771] | --- | --- | --- | --- |
| head_tail_dyn | 9927826ns | no significant difference | [-79545, +713506]ns | [9830618, 9956319] | no | 0.6875 | 0.6875 | 0 |
| kernel_nway | 9867709ns | no significant difference | [-531337, +731462]ns | [9369226, 9952777] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 9935696ns | no significant difference | [-759290, +769240]ns | [9141273, 9997307] | no | 0.6875 | 0.6875 | 0 |
| nway | 14732430ns | +4836984.6ns (+49.0%) | [+4625508, +5566430]ns | [14526071, 14796700] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| sequential | 14180993ns | +4504032.2ns (+45.7%) | [+4115980, +5961846]ns | [13999586, 15427558] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 13796763ns | +4090756.8ns (+41.5%) | [+987236, +4825094]ns | [10853428, 14279174] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9679227ns | +2.7% | +2.7% | +2.9% | +52.2% | +44.7% | +44.3% |
| 1 | 2 | 9446488ns | +5.0% | +4.5% | +5.6% | +56.6% | +60.5% | +34.4% |
| 1 | 3 | 9900563ns | -0.4% | -5.4% | -7.7% | +46.7% | +45.8% | +24.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.508 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.029 | ok |
| kernel_nway | -0.107 | ok |
| kernel_seq | -0.178 | ok |
| nway | -0.312 | moderate- |
| sequential | -0.574 | HIGH- (thermal bounce) |
| std_parallel | -0.052 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 72955801.6ns | 9675426.0ns | 754.0% | HIGH |
| head_tail_dyn | 72858855.2ns | 9904920.8ns | 735.6% | HIGH |
| kernel_nway | 72213722.4ns | 9729903.9ns | 742.2% | HIGH |
| kernel_seq | 72428847.0ns | 9691425.0ns | 747.3% | HIGH |
| nway | 77461371.9ns | 14685066.7ns | 527.5% | HIGH |
| sequential | 76335812.2ns | 14536045.5ns | 525.1% | HIGH |
| std_parallel | 71993000.4ns | 12976454.9ns | 554.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 9046008.1-9938770.9 ns)
  9046008.1 |########################################
  9090646.2 |
  9135284.4 |
  9179922.5 |
  9224560.7 |
  9269198.8 |
  9313836.9 |
  9358475.1 |########################################
  9403113.2 |
  9447751.3 |
  9492389.5 |
  9537027.6 |
  9581665.8 |
  9626303.9 |
  9670942.0 |
  9715580.2 |
  9760218.3 |
  9804856.4 |########################################
  9849494.6 |########################################
  9894132.7 |########################################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 9756777.5-9956318.6 ns)
  9756777.5 |########################################
  9766754.6 |
  9776731.6 |
  9786708.7 |
  9796685.7 |
  9806662.8 |
  9816639.8 |
  9826616.9 |
  9836593.9 |
  9846571.0 |
  9856548.1 |
  9866525.1 |
  9876502.2 |
  9886479.2 |
  9896456.3 |########################################
  9906433.3 |
  9916410.4 |########################################
  9926387.4 |########################################
  9936364.5 |########################################
  9946341.5 |
  (0 below, 1 above range)

kernel_nway (n=6, range 9087846.1-9952776.9 ns)
  9087846.1 |####################
  9131092.6 |
  9174339.2 |
  9217585.7 |
  9260832.3 |
  9304078.8 |
  9347325.4 |
  9390571.9 |
  9433818.4 |
  9477065.0 |
  9520311.5 |
  9563558.1 |
  9606804.6 |
  9650051.2 |####################
  9693297.7 |
  9736544.2 |
  9779790.8 |####################
  9823037.3 |
  9866283.9 |
  9909530.4 |########################################
  (0 below, 1 above range)

kernel_seq (n=6, range 9140598.6-9997306.6 ns)
  9140598.6 |########################################
  9183434.0 |
  9226269.4 |
  9269104.8 |
  9311940.2 |
  9354775.6 |
  9397611.0 |
  9440446.4 |
  9483281.8 |
  9526117.2 |
  9568952.6 |
  9611788.0 |
  9654623.4 |
  9697458.8 |
  9740294.2 |
  9783129.6 |
  9825965.0 |
  9868800.4 |
  9911635.8 |########################################
  9954471.2 |####################
  (0 below, 1 above range)

nway (n=6, range 14475057.8-14796699.6 ns)
  14475057.8 |########################################
  14491139.9 |
  14507222.0 |
  14523304.1 |
  14539386.2 |
  14555468.2 |
  14571550.3 |########################################
  14587632.4 |
  14603714.5 |
  14619796.6 |
  14635878.7 |
  14651960.8 |
  14668042.9 |
  14684125.0 |
  14700207.1 |
  14716289.2 |########################################
  14732371.2 |########################################
  14748453.3 |########################################
  14764535.4 |
  14780617.5 |
  (0 below, 1 above range)

sequential (n=6, range 13946993.3-15427557.6 ns)
  13946993.3 |####################
  14021021.5 |########################################
  14095049.7 |
  14169077.9 |
  14243106.2 |####################
  14317134.4 |
  14391162.6 |
  14465190.8 |
  14539219.0 |
  14613247.2 |
  14687275.5 |
  14761303.7 |####################
  14835331.9 |
  14909360.1 |
  14983388.3 |
  15057416.5 |
  15131444.7 |
  15205473.0 |
  15279501.2 |
  15353529.4 |
  (0 below, 1 above range)

std_parallel (n=6, range 10682483.3-14279173.9 ns)
  10682483.3 |########################################
  10862317.8 |########################################
  11042152.4 |
  11221986.9 |
  11401821.4 |
  11581655.9 |
  11761490.5 |
  11941325.0 |
  12121159.5 |
  12300994.0 |
  12480828.6 |
  12660663.1 |
  12840497.6 |
  13020332.2 |
  13200166.7 |
  13380001.2 |
  13559835.7 |########################################
  13739670.3 |########################################
  13919504.8 |
  14099339.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=736.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=733.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=732.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=729.6% of algo (FFI overhead may distort results)
- **nway**: bridge=525.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=535.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=520.6% of algo (FFI overhead may distort results)
