# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 5.4x slower than the field

std_parallel (76.94 us) is 5.4x the fastest (14.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 88.2%)

sequential wins on median (14.12 us) yet has the highest variance (CV 88.2%), while kernel_nway is the steadiest (CV 14.1%, 14.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### std_parallel shows alternating (throttle bounce) (autocorr -0.64)

std_parallel's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.4x the fastest

Fastest sequential (14.12 us) to slowest std_parallel (76.94 us): 5.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader sequential vs stability leader kernel_nway (+1% speed for 6.3x steadier)

sequential is fastest (14.12 us, CV 88.2%); kernel_nway gives up 1.1% median for 6.3x lower variance (CV 14.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### sequential is inconsistent: worst-20% is 3.9x its best-20%

sequential's best 20% of batches run at 9.80 us but its worst 20% at 37.75 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 14123.7 ns median (-13.3% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 5.45x (fastest 14123.7 ns, slowest 76937.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112846685 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 18662ns | 17012ns | 14148ns | 16414ns | 24292ns | base |
| head_tail_dyn | 17421ns | 16411ns | 13704ns | 16177ns | 21146ns | -6.65% |
| kernel_nway | 15537ns | 14938ns | 13152ns | 14397ns | 18440ns | -16.75% |
| kernel_seq | 15460ns | 14812ns | 11842ns | 14380ns | 18890ns | -17.16% |
| nway | 45501ns | 41840ns | 40094ns | 41527ns | 54165ns | +143.81% |
| sequential | 21304ns | 14330ns | 9952ns | 13726ns | 38347ns | +14.16% |
| std_parallel | 75631ns | 78022ns | 56266ns | 74690ns | 86726ns | +305.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 17637ns | 13686ns | 22397ns | base | 0.232 |
| head_tail_dyn | 16554ns | 13260ns | 19905ns | -6.14% | 0.247 |
| kernel_nway | 14702ns | 12640ns | 17077ns | -16.64% | 0.279 |
| kernel_seq | 14594ns | 11329ns | 17288ns | -17.25% | 0.281 |
| nway | 44953ns | 39610ns | 53488ns | +154.88% | 0.091 |
| sequential | 20981ns | 9802ns | 37747ns | +18.96% | 0.195 |
| std_parallel | 74941ns | 55942ns | 85738ns | +324.91% | 0.055 |

## Performance model

- Peak throughput: **0.418 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.252 | 60.2% |
| head_tail_dyn | 0.262 | 62.8% |
| kernel_nway | 0.287 | 68.7% |
| kernel_seq | 0.285 | 68.2% |
| nway | 0.099 | 23.7% |
| sequential | 0.290 | 69.4% |
| std_parallel | 0.053 | 12.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 18662ns | 18662ns | base |
| head_tail_dyn | 17421ns | 17421ns | -6.65% |
| kernel_nway | 15537ns | 15537ns | -16.75% |
| kernel_seq | 15460ns | 15460ns | -17.16% |
| nway | 45501ns | 45501ns | +143.81% |
| sequential | 21304ns | 21304ns | +14.16% |
| std_parallel | 75631ns | 75631ns | +305.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 16283ns | base | --- | [15075, 18837] | --- | --- | --- | --- |
| head_tail_dyn | 15614ns | no significant difference | [-1432, +325]ns | [14848, 17254] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 14272ns | -1960.9ns (-12.0%) | [-2945, -988]ns | [12853, 15255] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 14373ns | -2521.2ns (-15.5%) | [-3877, -714]ns | [13068, 15854] | YES | 0.0113 | 0.0075 | 0 |
| nway | 41351ns | +25588.4ns (+157.2%) | [+24322, +27962]ns | [40001, 44974] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 14124ns | no significant difference | [-4965, +15671]ns | [9963, 33268] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 76937ns | +59811.7ns (+367.3%) | [+53659, +62222]ns | [70522, 81309] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 18141ns | -10.0% | -25.7% | -29.4% | +130.7% | -9.3% | +332.5% |
| 1 | 2 | 14642ns | +1.4% | -4.2% | +4.4% | +188.1% | -9.8% | +413.2% |
| 1 | 3 | 16936ns | -6.6% | -12.0% | -15.1% | +136.7% | -7.2% | +359.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.590 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.592 | HIGH- (thermal bounce) |
| kernel_nway | -0.012 | ok |
| kernel_seq | -0.406 | moderate- |
| nway | -0.261 | moderate- |
| sequential | -0.637 | HIGH- (thermal bounce) |
| std_parallel | -0.644 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 82061.4ns | 17636.8ns | 465.3% | HIGH |
| head_tail_dyn | 72533.0ns | 16553.9ns | 438.2% | HIGH |
| kernel_nway | 71623.0ns | 14701.8ns | 487.2% | HIGH |
| kernel_seq | 69682.7ns | 14593.9ns | 477.5% | HIGH |
| nway | 91535.3ns | 44952.6ns | 203.6% | HIGH |
| sequential | 52332.5ns | 20980.7ns | 249.4% | HIGH |
| std_parallel | 113416.6ns | 74941.3ns | 151.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 13686.0-22397.0 ns)
  13686.0 |####################
  14121.5 |########################################
  14557.1 |####################
  14992.6 |########################################
  15428.2 |####################
  15863.7 |########################################
  16299.3 |####################
  16734.8 |
  17170.4 |####################
  17605.9 |####################
  18041.5 |
  18477.0 |
  18912.6 |
  19348.1 |####################
  19783.7 |########################################
  20219.2 |
  20654.8 |
  21090.3 |
  21525.9 |
  21961.4 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 13259.9-19905.3 ns)
  13259.9 |
  13592.1 |####################
  13924.4 |
  14256.7 |####################
  14589.0 |####################
  14921.2 |########################################
  15253.5 |########################################
  15585.8 |####################
  15918.0 |####################
  16250.3 |
  16582.6 |####################
  16914.8 |####################
  17247.1 |####################
  17579.4 |####################
  17911.7 |
  18243.9 |
  18576.2 |
  18908.5 |
  19240.7 |
  19573.0 |
  (2 below, 3 above range)

kernel_nway (n=18, range 12640.5-17077.2 ns)
  12640.5 |########################################
  12862.3 |##########
  13084.1 |
  13306.0 |##########
  13527.8 |
  13749.6 |
  13971.5 |####################
  14193.3 |##########
  14415.2 |
  14637.0 |##########
  14858.8 |####################
  15080.7 |
  15302.5 |##########
  15524.3 |
  15746.2 |
  15968.0 |
  16189.8 |
  16411.7 |
  16633.5 |
  16855.3 |##########
  (1 below, 3 above range)

kernel_seq (n=18, range 11328.8-17288.2 ns)
  11328.8 |
  11626.8 |
  11924.8 |####################
  12222.7 |
  12520.7 |########################################
  12818.7 |
  13116.7 |
  13414.6 |########################################
  13712.6 |########################################
  14010.6 |
  14308.5 |
  14606.5 |####################
  14904.5 |########################################
  15202.4 |####################
  15500.4 |
  15798.4 |
  16096.4 |####################
  16394.3 |
  16692.3 |
  16990.3 |
  (2 below, 4 above range)

nway (n=18, range 39610.1-53488.0 ns)
  39610.1 |########################################
  40304.0 |####################
  40997.9 |####################
  41691.8 |
  42385.7 |####################
  43079.6 |
  43773.5 |
  44467.4 |##########
  45161.3 |##########
  45855.1 |##########
  46549.0 |
  47242.9 |
  47936.8 |
  48630.7 |
  49324.6 |##########
  50018.5 |
  50712.4 |
  51406.3 |
  52100.2 |##########
  52794.1 |
  (2 below, 1 above range)

sequential (n=18, range 9802.0-37747.3 ns)
   9802.0 |########################################
  11199.2 |#####
  12596.5 |
  13993.8 |
  15391.0 |#####
  16788.3 |
  18185.6 |
  19582.8 |#####
  20980.1 |#####
  22377.4 |
  23774.6 |
  25171.9 |
  26569.2 |
  27966.4 |
  29363.7 |
  30761.0 |
  32158.2 |#####
  33555.5 |
  34952.8 |#####
  36350.0 |#####
  (1 below, 3 above range)

std_parallel (n=18, range 55942.1-85738.2 ns)
  55942.1 |
  57431.9 |
  58921.7 |
  60411.5 |####################
  61901.3 |
  63391.1 |####################
  64880.9 |
  66370.7 |
  67860.5 |
  69350.3 |####################
  70840.2 |####################
  72330.0 |####################
  73819.8 |
  75309.6 |########################################
  76799.4 |####################
  78289.2 |####################
  79779.0 |########################################
  81268.8 |####################
  82758.6 |
  84248.4 |####################
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=26.4% (high variance, measurements may be unstable)
- **head_tail**: bridge=512.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=437.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=488.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=472.2% of algo (FFI overhead may distort results)
- **nway**: CV=21.8% (high variance, measurements may be unstable)
- **nway**: bridge=210.6% of algo (FFI overhead may distort results)
- **sequential**: CV=59.4% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **sequential**: bridge=258.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=157.2% of algo (FFI overhead may distort results)
