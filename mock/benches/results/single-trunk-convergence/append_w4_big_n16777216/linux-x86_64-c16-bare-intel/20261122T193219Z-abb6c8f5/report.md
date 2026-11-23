# append, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway beats baseline by 35% (significant)

kernel_nway is -3.29 ms (35%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 2.5x slower than the field

sequential (15.67 ms) is 2.5x the fastest (6.21 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, std_parallel, nway, head_tail, kernel_seq, head_tail_dyn} vs {sequential} (60% apart)

The field splits into a fast tier {kernel_nway, std_parallel, nway, head_tail, kernel_seq, head_tail_dyn} and a slow tier {sequential} with a 60% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader kernel_nway vs stability leader std_parallel (+10% speed for 3.7x steadier)

kernel_nway is fastest (6.21 ms, CV 3.5%); std_parallel gives up 9.9% median for 3.7x lower variance (CV 1.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### nway is inconsistent: worst-20% is 2.6x its best-20%

nway's best 20% of batches run at 6.64 ms but its worst 20% at 17.18 ms (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 6206657.7 ns median (-33.5% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.53x (fastest 6206657.7 ns, slowest 15672224.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 4 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112325871 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 9541111ns | 9333831ns | 9291825ns | 9323303ns | 9992466ns | base |
| head_tail_dyn | 9625299ns | 9823369ns | 9046558ns | 9570042ns | 9997555ns | +0.88% |
| kernel_nway | 6221259ns | 6212471ns | 5941640ns | 6146905ns | 6472599ns | -34.80% |
| kernel_seq | 9566425ns | 9564945ns | 9147720ns | 9453638ns | 9944956ns | +0.27% |
| nway | 10412193ns | 7399651ns | 6640831ns | 7156363ns | 17181617ns | +9.13% |
| sequential | 15238998ns | 15673843ns | 13883542ns | 15115042ns | 16102662ns | +59.72% |
| std_parallel | 6806480ns | 6823887ns | 6681032ns | 6807909ns | 6867061ns | -28.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9537254ns | 9286697ns | 9988672ns | base | 1.759 |
| head_tail_dyn | 9621351ns | 9041539ns | 9993679ns | +0.88% | 1.744 |
| kernel_nway | 6216600ns | 5938041ns | 6468014ns | -34.82% | 2.699 |
| kernel_seq | 9562511ns | 9144792ns | 9940091ns | +0.26% | 1.754 |
| nway | 10409199ns | 6638077ns | 17178771ns | +9.14% | 1.612 |
| sequential | 15237147ns | 13881738ns | 16100858ns | +59.76% | 1.101 |
| std_parallel | 6802653ns | 6675785ns | 6864129ns | -28.67% | 2.466 |

## Performance model

- Peak throughput: **2.825 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.798 | 63.6% |
| head_tail_dyn | 1.709 | 60.5% |
| kernel_nway | 2.703 | 95.7% |
| kernel_seq | 1.755 | 62.1% |
| nway | 2.268 | 80.3% |
| sequential | 1.071 | 37.9% |
| std_parallel | 2.460 | 87.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9541111ns | 9541111ns | base |
| head_tail_dyn | 9625299ns | 9625299ns | +0.88% |
| kernel_nway | 6221259ns | 6221259ns | -34.80% |
| kernel_seq | 9566425ns | 9566425ns | +0.27% |
| nway | 10412193ns | 10412193ns | +9.13% |
| sequential | 15238998ns | 15238998ns | +59.72% |
| std_parallel | 6806480ns | 6806480ns | -28.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9329928ns | base | --- | [9293162, 9988672] | --- | --- | --- | --- |
| head_tail_dyn | 9819414ns | no significant difference | [-242885, +490169]ns | [9050961, 9993679] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 6206658ns | -3286038.9ns (-35.2%) | [-3850093, -2825832]ns | [5975127, 6468014] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| kernel_seq | 9561122ns | no significant difference | [-428296, +527725]ns | [9186319, 9940091] | no | 1.0000 | 1.0000 | 0 |
| nway | 7396384ns | no significant difference | [-3336230, +7849526]ns | [6652441, 17178771] | no | 1.0000 | 0.6875 | 0 |
| sequential | 15672225ns | +5830009.8ns (+62.5%) | [+4644512, +6625157]ns | [13938357, 16100858] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 6820293ns | -2524941.5ns (-27.1%) | [-3242679, -2436181]ns | [6723539, 6864129] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9988672ns | +0.1% | -38.5% | -4.0% | -33.4% | +58.4% | -32.5% |
| 1 | 2 | 9293845ns | -2.6% | -30.4% | -0.6% | -20.4% | +50.0% | -26.4% |
| 1 | 3 | 9329245ns | +5.3% | -35.2% | +5.7% | +84.1% | +71.0% | -26.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.193 | ok |
| head_tail_dyn | -0.647 | HIGH- (thermal bounce) |
| kernel_nway | -0.636 | HIGH- (thermal bounce) |
| kernel_seq | -0.543 | HIGH- (thermal bounce) |
| nway | -0.132 | ok |
| sequential | -0.664 | HIGH- (thermal bounce) |
| std_parallel | -0.267 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 1/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 71937957.1ns | 9537253.9ns | 754.3% | HIGH |
| head_tail_dyn | 72158934.8ns | 9621351.2ns | 750.0% | HIGH |
| kernel_nway | 70685758.6ns | 6216599.5ns | 1137.0% | HIGH |
| kernel_seq | 72100550.9ns | 9562510.6ns | 754.0% | HIGH |
| nway | 77474818.8ns | 10409199.0ns | 744.3% | HIGH |
| sequential | 77401972.7ns | 15237146.8ns | 508.0% | HIGH |
| std_parallel | 70356997.2ns | 6802653.4ns | 1034.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 9286696.8-9988671.6 ns)
  9286696.8 |########################################
  9321795.5 |
  9356894.3 |#############
  9391993.0 |
  9427091.8 |
  9462190.5 |
  9497289.2 |
  9532388.0 |
  9567486.7 |
  9602585.5 |
  9637684.2 |
  9672782.9 |
  9707881.7 |
  9742980.4 |
  9778079.2 |
  9813177.9 |
  9848276.6 |
  9883375.4 |
  9918474.1 |#############
  9953572.9 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 9041538.6-9993679.4 ns)
  9041538.6 |########################################
  9089145.6 |
  9136752.7 |
  9184359.7 |
  9231966.8 |
  9279573.8 |
  9327180.9 |
  9374787.9 |
  9422394.9 |
  9470002.0 |
  9517609.0 |
  9565216.1 |
  9612823.1 |
  9660430.2 |
  9708037.2 |
  9755644.2 |####################
  9803251.3 |####################
  9850858.3 |
  9898465.4 |
  9946072.4 |####################
  (0 below, 1 above range)

kernel_nway (n=6, range 5938041.4-6468013.5 ns)
  5938041.4 |########################################
  5964540.0 |
  5991038.6 |########################################
  6017537.2 |
  6044035.8 |
  6070534.4 |########################################
  6097033.0 |
  6123531.7 |
  6150030.3 |
  6176528.9 |
  6203027.5 |
  6229526.1 |
  6256024.7 |
  6282523.3 |
  6309021.9 |
  6335520.5 |########################################
  6362019.1 |
  6388517.7 |
  6415016.3 |########################################
  6441514.9 |
  (0 below, 1 above range)

kernel_seq (n=6, range 9144791.8-9940091.1 ns)
  9144791.8 |########################################
  9184556.8 |
  9224321.7 |########################################
  9264086.7 |
  9303851.7 |########################################
  9343616.6 |
  9383381.6 |
  9423146.5 |
  9462911.5 |
  9502676.5 |
  9542441.4 |
  9582206.4 |
  9621971.4 |
  9661736.3 |
  9701501.3 |
  9741266.2 |
  9781031.2 |########################################
  9820796.2 |
  9860561.1 |
  9900326.1 |########################################
  (0 below, 1 above range)

nway (n=6, range 6638077.1-17178771.4 ns)
  6638077.1 |########################################
  7165111.8 |########################################
  7692146.5 |
  8219181.2 |
  8746215.9 |
  9273250.7 |
  9800285.4 |
  10327320.1 |
  10854354.8 |
  11381389.5 |
  11908424.2 |
  12435458.9 |
  12962493.7 |
  13489528.4 |####################
  14016563.1 |
  14543597.8 |
  15070632.5 |
  15597667.2 |
  16124701.9 |
  16651736.6 |
  (0 below, 1 above range)

sequential (n=6, range 13881737.9-16100858.1 ns)
  13881737.9 |########################################
  13992693.9 |########################################
  14103649.9 |
  14214605.9 |
  14325561.9 |
  14436517.9 |
  14547474.0 |
  14658430.0 |
  14769386.0 |
  14880342.0 |
  14991298.0 |
  15102254.0 |
  15213210.0 |
  15324166.0 |
  15435122.0 |
  15546078.1 |########################################
  15657034.1 |########################################
  15767990.1 |
  15878946.1 |
  15989902.1 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 6675784.8-6864129.0 ns)
  6675784.8 |########################################
  6685202.0 |
  6694619.2 |
  6704036.4 |
  6713453.6 |
  6722870.8 |
  6732288.1 |
  6741705.3 |
  6751122.5 |
  6760539.7 |
  6769956.9 |########################################
  6779374.1 |
  6788791.3 |
  6798208.5 |
  6807625.7 |########################################
  6817043.0 |########################################
  6826460.2 |
  6835877.4 |
  6845294.6 |########################################
  6854711.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=770.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=735.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1116.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=753.8% of algo (FFI overhead may distort results)
- **nway**: CV=49.4% (high variance, measurements may be unstable)
- **nway**: bridge=1012.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=499.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1033.9% of algo (FFI overhead may distort results)
