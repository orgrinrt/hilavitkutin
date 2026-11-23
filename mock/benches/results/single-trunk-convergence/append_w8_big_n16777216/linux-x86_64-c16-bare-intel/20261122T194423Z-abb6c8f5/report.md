# append, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway beats baseline by 38% (significant)

nway is -3.77 ms (38%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 2.4x slower than the field

sequential (15.11 ms) is 2.4x the fastest (6.24 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.58)

nway's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: nway** at 6243393.2 ns median (-37.7% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.42x (fastest 6243393.2 ns, slowest 15109065.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 8 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112822431 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 9715329ns | 10030963ns | 8258085ns | 9907413ns | 10155826ns | base |
| head_tail_dyn | 9702671ns | 9862085ns | 9181314ns | 9673963ns | 10006411ns | -0.13% |
| kernel_nway | 7093607ns | 7032521ns | 6311065ns | 6986604ns | 7645383ns | -26.99% |
| kernel_seq | 9655529ns | 9724530ns | 9295736ns | 9603026ns | 9914180ns | -0.62% |
| nway | 6225390ns | 6246428ns | 6098316ns | 6199395ns | 6327918ns | -35.92% |
| sequential | 15055859ns | 15111048ns | 13973091ns | 14864360ns | 15884492ns | +54.97% |
| std_parallel | 6394103ns | 6378616ns | 6335488ns | 6364681ns | 6467543ns | -34.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9711319ns | 8253184ns | 10151838ns | base | 1.728 |
| head_tail_dyn | 9698497ns | 9178080ns | 10002353ns | -0.13% | 1.730 |
| kernel_nway | 7088887ns | 6305482ns | 7640419ns | -27.00% | 2.367 |
| kernel_seq | 9651614ns | 9290899ns | 9910349ns | -0.61% | 1.738 |
| nway | 6222453ns | 6095445ns | 6325019ns | -35.93% | 2.696 |
| sequential | 15053918ns | 13971586ns | 15882623ns | +55.01% | 1.114 |
| std_parallel | 6390295ns | 6330889ns | 6463806ns | -34.20% | 2.625 |

## Performance model

- Peak throughput: **2.752 Gops/s** (nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.673 | 60.8% |
| head_tail_dyn | 1.702 | 61.8% |
| kernel_nway | 2.387 | 86.7% |
| kernel_seq | 1.726 | 62.7% |
| nway | 2.687 | 97.6% |
| sequential | 1.110 | 40.3% |
| std_parallel | 2.632 | 95.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9715329ns | 9715329ns | base |
| head_tail_dyn | 9702671ns | 9702671ns | -0.13% |
| kernel_nway | 7093607ns | 7093607ns | -26.99% |
| kernel_seq | 9655529ns | 9655529ns | -0.62% |
| nway | 6225390ns | 6225390ns | -35.92% |
| sequential | 15055859ns | 15055859ns | +54.97% |
| std_parallel | 6394103ns | 6394103ns | -34.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 10026872ns | base | --- | [8955248, 10151838] | --- | --- | --- | --- |
| head_tail_dyn | 9857865ns | no significant difference | [-577900, +685821]ns | [9235274, 10002353] | no | 0.6875 | 0.6875 | 0 |
| kernel_nway | 7027920ns | -2811475.0ns (-28.0%) | [-3388470, -1667350]ns | [6598324, 7640419] | YES | 0.0469 | 0.0313 | 0 |
| kernel_seq | 9720507ns | no significant difference | [-622339, +627349]ns | [9323987, 9910349] | no | 0.6875 | 0.6875 | 0 |
| nway | 6243393ns | -3766105.6ns (-37.6%) | [-3896648, -2803846]ns | [6098946, 6325019] | YES | 0.0469 | 0.0313 | 0 |
| sequential | 15109066ns | +5115243.0ns (+51.0%) | [+4329044, +6583510]ns | [14170066, 15882623] | YES | 0.0469 | 0.0313 | 0 |
| std_parallel | 6374572ns | -3599617.9ns (-35.9%) | [-3784716, -2578739]ns | [6332507, 6463806] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 10151838ns | -2.9% | -24.7% | -4.2% | -37.7% | +50.3% | -37.2% |
| 1 | 2 | 9855651ns | +1.5% | -33.1% | +0.6% | -38.1% | +54.4% | -35.7% |
| 1 | 3 | 9126468ns | +1.2% | -23.0% | +2.2% | -31.6% | +61.0% | -29.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.037 | ok |
| head_tail_dyn | -0.278 | moderate- |
| kernel_nway | -0.439 | moderate- |
| kernel_seq | -0.374 | moderate- |
| nway | -0.582 | HIGH- (thermal bounce) |
| sequential | -0.132 | ok |
| std_parallel | -0.371 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 72815163.0ns | 9711319.4ns | 749.8% | HIGH |
| head_tail_dyn | 72153238.6ns | 9698497.2ns | 744.0% | HIGH |
| kernel_nway | 70601768.7ns | 7088887.4ns | 995.9% | HIGH |
| kernel_seq | 72308859.5ns | 9651614.5ns | 749.2% | HIGH |
| nway | 73752319.5ns | 6222452.9ns | 1185.3% | HIGH |
| sequential | 77315495.5ns | 15053918.3ns | 513.6% | HIGH |
| std_parallel | 70865308.1ns | 6390295.2ns | 1109.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 8253183.6-10151838.4 ns)
  8253183.6 |####################
  8348116.3 |
  8443049.1 |
  8537981.8 |
  8632914.6 |
  8727847.3 |
  8822780.1 |
  8917712.8 |
  9012645.5 |
  9107578.3 |
  9202511.0 |
  9297443.8 |
  9392376.5 |
  9487309.3 |
  9582242.0 |####################
  9677174.7 |
  9772107.5 |
  9867040.2 |
  9961973.0 |########################################
  10056905.7 |####################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 9178080.0-10002352.9 ns)
  9178080.0 |####################
  9219293.6 |
  9260507.3 |####################
  9301720.9 |
  9342934.6 |
  9384148.2 |
  9425361.9 |
  9466575.5 |
  9507789.2 |
  9549002.8 |
  9590216.4 |
  9631430.1 |
  9672643.7 |
  9713857.4 |
  9755071.0 |
  9796284.7 |
  9837498.3 |########################################
  9878712.0 |
  9919925.6 |
  9961139.3 |####################
  (0 below, 1 above range)

kernel_nway (n=6, range 6305482.2-7640419.2 ns)
  6305482.2 |########################################
  6372229.0 |
  6438975.9 |
  6505722.7 |
  6572469.6 |
  6639216.4 |
  6705963.3 |
  6772710.1 |
  6839457.0 |########################################
  6906203.8 |########################################
  6972950.7 |
  7039697.5 |########################################
  7106444.4 |
  7173191.2 |
  7239938.1 |
  7306684.9 |########################################
  7373431.8 |
  7440178.6 |
  7506925.5 |
  7573672.3 |
  (0 below, 1 above range)

kernel_seq (n=6, range 9290898.7-9910349.4 ns)
  9290898.7 |####################
  9321871.2 |
  9352843.8 |####################
  9383816.3 |
  9414788.8 |
  9445761.4 |
  9476733.9 |
  9507706.4 |
  9538679.0 |
  9569651.5 |####################
  9600624.0 |
  9631596.6 |
  9662569.1 |
  9693541.7 |
  9724514.2 |
  9755486.7 |
  9786459.3 |
  9817431.8 |
  9848404.3 |########################################
  9879376.9 |
  (0 below, 1 above range)

nway (n=6, range 6095445.0-6325019.2 ns)
  6095445.0 |########################################
  6106923.7 |
  6118402.4 |
  6129881.1 |
  6141359.8 |
  6152838.5 |
  6164317.3 |
  6175796.0 |
  6187274.7 |
  6198753.4 |####################
  6210232.1 |
  6221710.8 |
  6233189.5 |
  6244668.2 |
  6256146.9 |
  6267625.7 |
  6279104.4 |####################
  6290583.1 |####################
  6302061.8 |
  6313540.5 |
  (0 below, 1 above range)

sequential (n=6, range 13971586.3-15882623.1 ns)
  13971586.3 |########################################
  14067138.1 |
  14162690.0 |
  14258241.8 |
  14353793.7 |########################################
  14449345.5 |
  14544897.3 |
  14640449.2 |
  14736001.0 |########################################
  14831552.8 |
  14927104.7 |
  15022656.5 |
  15118208.4 |
  15213760.2 |
  15309312.0 |
  15404863.9 |########################################
  15500415.7 |
  15595967.5 |
  15691519.4 |########################################
  15787071.2 |
  (0 below, 1 above range)

std_parallel (n=6, range 6330888.8-6463806.2 ns)
  6330888.8 |########################################
  6337534.7 |
  6344180.5 |
  6350826.4 |
  6357472.3 |
  6364118.2 |
  6370764.0 |
  6377409.9 |
  6384055.8 |
  6390701.7 |
  6397347.5 |
  6403993.4 |
  6410639.3 |#############
  6417285.1 |#############
  6423931.0 |
  6430576.9 |
  6437222.8 |
  6443868.6 |
  6450514.5 |
  6457160.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=727.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=731.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1001.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=745.7% of algo (FFI overhead may distort results)
- **nway**: bridge=1196.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=513.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1110.9% of algo (FFI overhead may distort results)
