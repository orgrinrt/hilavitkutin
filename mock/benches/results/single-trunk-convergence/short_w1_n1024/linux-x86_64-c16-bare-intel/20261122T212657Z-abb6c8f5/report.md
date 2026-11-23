# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 63% faster than the next best (head_tail)

sequential (7.11 us) leads head_tail (11.60 us) by 63%, a clear separation rather than a photo finish. CV 130.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 5.5x slower than the field

std_parallel (39.25 us) is 5.5x the fastest (7.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 130.4%)

sequential wins on median (7.11 us) yet has the highest variance (CV 130.4%), while std_parallel is the steadiest (CV 12.5%, 39.25 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### std_parallel shows alternating (throttle bounce) (autocorr -0.54)

std_parallel's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.5x the fastest

Fastest sequential (7.11 us) to slowest std_parallel (39.25 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.6x its best-20%

sequential's best 20% of batches run at 6.87 us but its worst 20% at 24.68 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7106.9 ns median (-38.7% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 5.52x (fastest 7106.9 ns, slowest 39253.8 ns)

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
| Counter frequency | 2112789863 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 13522ns | 12575ns | 10425ns | 12334ns | 16852ns | base |
| head_tail_dyn | 14634ns | 14095ns | 9431ns | 13293ns | 19249ns | +8.23% |
| kernel_nway | 13642ns | 12746ns | 10583ns | 12388ns | 17051ns | +0.89% |
| kernel_seq | 17234ns | 16684ns | 13293ns | 16113ns | 20885ns | +27.45% |
| nway | 35540ns | 34165ns | 28713ns | 32834ns | 43014ns | +162.84% |
| sequential | 13301ns | 7243ns | 7006ns | 7471ns | 25195ns | -1.63% |
| std_parallel | 40678ns | 39871ns | 34415ns | 38799ns | 46626ns | +200.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12690ns | 9822ns | 15847ns | base | 0.081 |
| head_tail_dyn | 13736ns | 8948ns | 17930ns | +8.25% | 0.075 |
| kernel_nway | 12813ns | 9992ns | 15841ns | +0.97% | 0.080 |
| kernel_seq | 16104ns | 12722ns | 19379ns | +26.91% | 0.064 |
| nway | 35042ns | 28313ns | 42397ns | +176.15% | 0.029 |
| sequential | 13034ns | 6869ns | 24678ns | +2.72% | 0.079 |
| std_parallel | 39944ns | 33838ns | 45666ns | +214.78% | 0.026 |

## Performance model

- Peak throughput: **0.149 Gops/s** (sequential; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.088 | 59.2% |
| head_tail_dyn | 0.077 | 51.4% |
| kernel_nway | 0.085 | 57.1% |
| kernel_seq | 0.064 | 43.3% |
| nway | 0.030 | 20.4% |
| sequential | 0.144 | 96.7% |
| std_parallel | 0.026 | 17.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 13522ns | 13522ns | base |
| head_tail_dyn | 14634ns | 14634ns | +8.23% |
| kernel_nway | 13642ns | 13642ns | +0.89% |
| kernel_seq | 17234ns | 17234ns | +27.45% |
| nway | 35540ns | 35540ns | +162.84% |
| sequential | 13301ns | 13301ns | -1.63% |
| std_parallel | 40678ns | 40678ns | +200.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 11599ns | base | --- | [10630, 14289] | --- | --- | --- | --- |
| head_tail_dyn | 13368ns | no significant difference | [-1338, +2753]ns | [11334, 15863] | no | 0.2379 | 0.2379 | 0 |
| kernel_nway | 12039ns | no significant difference | [-11, +403]ns | [10808, 13741] | no | 0.1444 | 0.0963 | 0 |
| kernel_seq | 15878ns | no significant difference | [-1458, +6354]ns | [13965, 17424] | no | 0.2379 | 0.2379 | 0 |
| nway | 33651ns | +19965.2ns (+172.1%) | [+18615, +24245]ns | [30868, 37630] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7107ns | no significant difference | [-4014, +3373]ns | [7002, 16544] | no | 0.1444 | 0.0963 | 0 |
| std_parallel | 39254ns | +25715.7ns (+221.7%) | [+23823, +30018]ns | [35802, 42545] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11429ns | -14.5% | +0.1% | +63.9% | +182.8% | -27.9% | +279.7% |
| 1 | 2 | 10630ns | +27.0% | +1.7% | +38.4% | +211.7% | -32.9% | +229.7% |
| 1 | 3 | 15664ns | +10.1% | +0.9% | -13.2% | +118.1% | -54.6% | +153.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.258 | moderate- |
| head_tail_dyn | -0.000 | ok |
| kernel_nway | -0.238 | moderate- |
| kernel_seq | -0.064 | ok |
| nway | -0.003 | ok |
| sequential | -0.154 | ok |
| std_parallel | -0.541 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 13257.5ns | 12689.7ns | 104.5% | HIGH |
| head_tail_dyn | 14566.5ns | 13736.5ns | 106.0% | HIGH |
| kernel_nway | 13302.0ns | 12813.4ns | 103.8% | HIGH |
| kernel_seq | 16887.6ns | 16104.4ns | 104.9% | HIGH |
| nway | 35692.3ns | 35042.4ns | 101.9% | HIGH |
| sequential | 13432.5ns | 13034.4ns | 103.1% | HIGH |
| std_parallel | 40334.4ns | 39944.4ns | 101.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 9821.9-15847.1 ns)
   9821.9 |##########################
  10123.1 |#############
  10424.4 |##########################
  10725.6 |
  11026.9 |
  11328.2 |########################################
  11629.4 |#############
  11930.7 |
  12231.9 |
  12533.2 |
  12834.5 |
  13135.7 |##########################
  13437.0 |
  13738.2 |#############
  14039.5 |
  14340.8 |
  14642.0 |#############
  14943.3 |
  15244.5 |#############
  15545.8 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 8948.1-17930.1 ns)
   8948.1 |
   9397.2 |####################
   9846.3 |####################
  10295.4 |
  10744.5 |
  11193.6 |########################################
  11642.7 |
  12091.8 |########################################
  12540.9 |####################
  12990.0 |
  13439.1 |
  13888.2 |########################################
  14337.3 |####################
  14786.4 |
  15235.5 |
  15684.6 |########################################
  16133.7 |####################
  16582.8 |
  17031.9 |
  17481.0 |
  (2 below, 3 above range)

kernel_nway (n=18, range 9992.5-15840.5 ns)
   9992.5 |########################################
  10284.9 |####################
  10577.3 |####################
  10869.7 |########################################
  11162.1 |
  11454.5 |####################
  11746.9 |####################
  12039.3 |####################
  12331.7 |
  12624.1 |
  12916.5 |
  13208.9 |####################
  13501.3 |########################################
  13793.7 |####################
  14086.1 |
  14378.5 |
  14670.9 |
  14963.3 |
  15255.7 |####################
  15548.1 |
  (1 below, 3 above range)

kernel_seq (n=18, range 12721.7-19379.0 ns)
  12721.7 |#############
  13054.6 |#############
  13387.5 |#############
  13720.3 |#############
  14053.2 |#############
  14386.1 |#############
  14718.9 |#############
  15051.8 |
  15384.7 |
  15717.5 |########################################
  16050.4 |
  16383.3 |
  16716.1 |##########################
  17049.0 |
  17381.9 |
  17714.7 |#############
  18047.6 |
  18380.5 |
  18713.3 |##########################
  19046.2 |
  (1 below, 2 above range)

nway (n=18, range 28313.0-42397.3 ns)
  28313.0 |########################################
  29017.2 |####################
  29721.4 |
  30425.7 |####################
  31129.9 |########################################
  31834.1 |####################
  32538.3 |
  33242.5 |####################
  33946.7 |####################
  34651.0 |########################################
  35355.2 |
  36059.4 |
  36763.6 |
  37467.8 |########################################
  38172.0 |
  38876.3 |
  39580.5 |####################
  40284.7 |
  40988.9 |
  41693.1 |
  (1 below, 3 above range)

sequential (n=18, range 6868.8-24677.8 ns)
   6868.8 |########################################
   7759.2 |
   8649.7 |####
   9540.1 |
  10430.6 |####
  11321.0 |
  12211.5 |
  13101.9 |
  13992.4 |
  14882.8 |
  15773.3 |
  16663.7 |
  17554.2 |
  18444.6 |
  19335.1 |
  20225.5 |
  21116.0 |
  22006.4 |
  22896.9 |####
  23787.3 |
  (2 below, 4 above range)

std_parallel (n=18, range 33838.3-45666.2 ns)
  33838.3 |####################
  34429.7 |####################
  35021.1 |####################
  35612.5 |########################################
  36203.9 |
  36795.3 |
  37386.7 |####################
  37978.0 |####################
  38569.4 |####################
  39160.8 |####################
  39752.2 |
  40343.6 |####################
  40935.0 |####################
  41526.4 |####################
  42117.8 |
  42709.2 |####################
  43300.6 |####################
  43892.0 |
  44483.4 |
  45074.8 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=20.1% (high variance, measurements may be unstable)
- **head_tail**: bridge=110.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=26.2% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=112.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=106.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=104.0% of algo (FFI overhead may distort results)
- **nway**: bridge=103.7% of algo (FFI overhead may distort results)
- **sequential**: CV=71.1% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sequential**: bridge=102.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=96.6% of algo (FFI overhead may distort results)
