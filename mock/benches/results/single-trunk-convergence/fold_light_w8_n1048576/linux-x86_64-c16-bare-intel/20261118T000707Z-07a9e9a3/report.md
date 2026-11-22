# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 25% faster than the next best (head_tail)

kernel_nway (82.67 us) leads head_tail (103.39 us) by 25%, a clear separation rather than a photo finish. CV 99.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn is an outlier: 2.2x slower than the field

head_tail_dyn (184.80 us) is 2.2x the fastest (82.67 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.67)

sequential's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### kernel_nway is inconsistent: worst-20% is 5.6x its best-20%

kernel_nway's best 20% of batches run at 34.04 us but its worst 20% at 190.60 us (5.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 82670.8 ns median (-20.0% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 2.24x (fastest 82670.8 ns, slowest 184796.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 8 |
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
| head_tail | 179764ns | 103599ns | 84926ns | 98937ns | 348424ns | base |
| head_tail_dyn | 191321ns | 185489ns | 164861ns | 181452ns | 219355ns | +6.43% |
| kernel_nway | 106898ns | 83618ns | 34262ns | 74186ns | 192284ns | -40.53% |
| kernel_seq | 137089ns | 106587ns | 102577ns | 106671ns | 199971ns | -23.74% |
| nway | 131462ns | 121736ns | 71774ns | 111103ns | 191845ns | -26.87% |
| sequential | 107977ns | 108185ns | 104255ns | 107291ns | 110868ns | -39.93% |
| std_parallel | 178697ns | 178869ns | 112272ns | 168135ns | 227753ns | -0.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 178930ns | 84754ns | 346503ns | base | 5.860 |
| head_tail_dyn | 190701ns | 164484ns | 218356ns | +6.58% | 5.499 |
| kernel_nway | 105926ns | 34037ns | 190602ns | -40.80% | 9.899 |
| kernel_seq | 136455ns | 102377ns | 198802ns | -23.74% | 7.684 |
| nway | 130922ns | 71511ns | 191018ns | -26.83% | 8.009 |
| sequential | 107725ns | 103992ns | 110617ns | -39.79% | 9.734 |
| std_parallel | 177837ns | 111902ns | 226250ns | -0.61% | 5.896 |

## Performance model

- Peak throughput: **30.807 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 10.142 | 32.9% |
| head_tail_dyn | 5.674 | 18.4% |
| kernel_nway | 12.684 | 41.2% |
| kernel_seq | 9.862 | 32.0% |
| nway | 8.647 | 28.1% |
| sequential | 9.714 | 31.5% |
| std_parallel | 5.885 | 19.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 179764ns | 179764ns | base |
| head_tail_dyn | 191321ns | 191321ns | +6.43% |
| kernel_nway | 106898ns | 106898ns | -40.53% |
| kernel_seq | 137089ns | 137089ns | -23.74% |
| nway | 131462ns | 131462ns | -26.87% |
| sequential | 107977ns | 107977ns | -39.93% |
| std_parallel | 178697ns | 178697ns | -0.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 103394ns | base | --- | [94546, 150775] | --- | --- | --- | --- |
| head_tail_dyn | 184797ns | +62883.0ns (+60.8%) | [+31703, +98933]ns | [174162, 199710] | YES (adj: no) | 0.0927 | 0.0309 | 0 |
| kernel_nway | 82671ns | no significant difference | [-81134, +2552]ns | [63609, 97025] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 106327ns | +17076.8ns (+16.5%) | [+3129, +24254]ns | [105143, 126668] | YES | 0.0452 | 0.0075 | 0 |
| nway | 121264ns | no significant difference | [-44692, +21376]ns | [97489, 129703] | no | 0.8145 | 0.8145 | 0 |
| sequential | 107945ns | no significant difference | [-42170, +13840]ns | [106136, 109074] | no | 0.2855 | 0.2379 | 0 |
| std_parallel | 178186ns | +65074.7ns (+62.9%) | [+10791, +94294]ns | [154635, 200413] | YES (adj: no) | 0.1925 | 0.0963 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 103672ns | +59.6% | +70.7% | +5.2% | +8.5% | +3.2% | +49.1% |
| 1 | 2 | 85302ns | +116.6% | -25.4% | +20.7% | +26.0% | +28.0% | +118.5% |
| 1 | 3 | 188786ns | +5.8% | -51.4% | -43.7% | -32.5% | -43.3% | -13.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.271 | moderate- |
| head_tail_dyn | -0.003 | ok |
| kernel_nway | -0.319 | moderate- |
| kernel_seq | -0.528 | HIGH- (thermal bounce) |
| nway | -0.323 | moderate- |
| sequential | -0.666 | HIGH- (thermal bounce) |
| std_parallel | -0.627 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 179490.6ns | 178930.1ns | 100.3% | HIGH |
| head_tail_dyn | 193015.4ns | 190701.4ns | 101.2% | HIGH |
| kernel_nway | 108079.2ns | 105925.7ns | 102.0% | HIGH |
| kernel_seq | 140345.6ns | 136454.8ns | 102.9% | HIGH |
| nway | 131125.7ns | 130922.1ns | 100.2% | HIGH |
| sequential | 111353.4ns | 107725.3ns | 103.4% | HIGH |
| std_parallel | 178940.6ns | 177836.9ns | 100.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 84753.8-346503.3 ns)
  84753.8 |#################################
  97841.3 |########################################
  110928.8 |
  124016.3 |
  137103.7 |
  150191.2 |
  163278.7 |
  176366.2 |
  189453.6 |######
  202541.1 |
  215628.6 |
  228716.1 |######
  241803.5 |
  254891.0 |
  267978.5 |
  281066.0 |######
  294153.4 |
  307240.9 |
  320328.4 |
  333415.9 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 164484.5-218356.2 ns)
  164484.5 |########################################
  167178.1 |
  169871.7 |
  172565.3 |
  175258.8 |##########
  177952.4 |
  180646.0 |##########
  183339.6 |########################################
  186033.2 |
  188726.8 |
  191420.4 |
  194113.9 |
  196807.5 |##########
  199501.1 |########################################
  202194.7 |
  204888.3 |
  207581.9 |
  210275.4 |
  212969.0 |
  215662.6 |
  (1 below, 2 above range)

kernel_nway (n=18, range 34037.3-190602.1 ns)
  34037.3 |####################
  41865.6 |##########
  49693.8 |
  57522.1 |
  65350.3 |
  73178.5 |########################################
  81006.8 |##############################
  88835.0 |
  96663.2 |##############################
  104491.5 |
  112319.7 |
  120148.0 |
  127976.2 |
  135804.4 |
  143632.7 |
  151460.9 |
  159289.2 |
  167117.4 |
  174945.6 |
  182773.9 |
  (2 below, 3 above range)

kernel_seq (n=18, range 102376.9-198801.6 ns)
  102376.9 |########################################
  107198.1 |##########
  112019.4 |
  116840.6 |#####
  121661.8 |
  126483.1 |
  131304.3 |#####
  136125.6 |
  140946.8 |
  145768.0 |
  150589.3 |
  155410.5 |
  160231.7 |
  165053.0 |#####
  169874.2 |
  174695.4 |
  179516.7 |
  184337.9 |
  189159.2 |
  193980.4 |
  (2 below, 3 above range)

nway (n=18, range 71510.6-191018.4 ns)
  71510.6 |##########
  77486.0 |##########
  83461.4 |##########
  89436.8 |##########
  95412.2 |##########
  101387.6 |##########
  107362.9 |
  113338.3 |
  119313.7 |########################################
  125289.1 |##############################
  131264.5 |####################
  137239.9 |
  143215.3 |
  149190.6 |
  155166.0 |
  161141.4 |
  167116.8 |##########
  173092.2 |
  179067.6 |
  185043.0 |
  (1 below, 1 above range)

sequential (n=18, range 103991.6-110617.4 ns)
  103991.6 |
  104322.9 |
  104654.2 |
  104985.4 |
  105316.7 |##########
  105648.0 |
  105979.3 |########################################
  106310.6 |##########
  106641.9 |
  106973.2 |
  107304.5 |
  107635.8 |##########
  107967.1 |##############################
  108298.4 |##########
  108629.7 |
  108961.0 |
  109292.3 |##########
  109623.6 |
  109954.9 |##########
  110286.2 |##########
  (2 below, 2 above range)

std_parallel (n=18, range 111901.6-226250.2 ns)
  111901.6 |#############
  117619.1 |#############
  123336.5 |
  129053.9 |
  134771.3 |#############
  140488.8 |
  146206.2 |#############
  151923.6 |
  157641.1 |#############
  163358.5 |#############
  169075.9 |#############
  174793.3 |########################################
  180510.8 |
  186228.2 |#############
  191945.6 |#############
  197663.1 |
  203380.5 |#############
  209097.9 |#############
  214815.3 |
  220532.8 |#############
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=94.3% (high variance, measurements may be unstable)
- **head_tail**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **head_tail**: bridge=100.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.2% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=77.8% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 5.6x (possible bimodal distribution)
- **kernel_nway**: bridge=100.2% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=42.7% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=99.8% of algo (FFI overhead may distort results)
- **nway**: CV=62.8% (high variance, measurements may be unstable)
- **nway**: bridge=101.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=102.4% of algo (FFI overhead may distort results)
- **std_parallel**: CV=24.6% (high variance, measurements may be unstable)
- **std_parallel**: bridge=101.4% of algo (FFI overhead may distort results)
