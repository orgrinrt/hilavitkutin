# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 62% faster than the next best (head_tail_dyn)

kernel_seq (2.54 us) leads head_tail_dyn (4.11 us) by 62%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 40% (significant)

kernel_seq is -1.65 us (40%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 12.5x slower than the field

std_parallel (31.66 us) is 12.5x the fastest (2.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.58)

kernel_seq's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 12.5x the fastest

Fastest kernel_seq (2.54 us) to slowest std_parallel (31.66 us): 12.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2540.8 ns median (-39.0% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 12.46x (fastest 2540.8 ns, slowest 31655.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 2 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 4939ns | 4766ns | 4584ns | 4734ns | 5424ns | base |
| head_tail_dyn | 4697ns | 4706ns | 4512ns | 4681ns | 4815ns | -4.90% |
| kernel_nway | 4852ns | 4714ns | 4567ns | 4701ns | 5219ns | -1.77% |
| kernel_seq | 3016ns | 2988ns | 2746ns | 2960ns | 3234ns | -38.94% |
| nway | 18437ns | 18388ns | 18060ns | 18302ns | 18827ns | +273.28% |
| sequential | 10497ns | 10375ns | 10228ns | 10361ns | 10834ns | +112.52% |
| std_parallel | 32711ns | 32058ns | 30669ns | 31710ns | 35234ns | +562.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4329ns | 3971ns | 4821ns | base | 0.015 |
| head_tail_dyn | 4091ns | 3916ns | 4203ns | -5.49% | 0.016 |
| kernel_nway | 4257ns | 4000ns | 4622ns | -1.66% | 0.015 |
| kernel_seq | 2563ns | 2451ns | 2690ns | -40.80% | 0.025 |
| nway | 17907ns | 17527ns | 18289ns | +313.65% | 0.004 |
| sequential | 10354ns | 10090ns | 10690ns | +139.18% | 0.006 |
| std_parallel | 32323ns | 30276ns | 34841ns | +646.65% | 0.002 |

## Performance model

- Peak throughput: **0.026 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.015 | 58.8% |
| head_tail_dyn | 0.016 | 59.7% |
| kernel_nway | 0.015 | 59.3% |
| kernel_seq | 0.025 | 96.4% |
| nway | 0.004 | 13.7% |
| sequential | 0.006 | 23.9% |
| std_parallel | 0.002 | 7.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4939ns | 4939ns | base |
| head_tail_dyn | 4697ns | 4697ns | -4.90% |
| kernel_nway | 4852ns | 4852ns | -1.77% |
| kernel_seq | 3016ns | 3016ns | -38.94% |
| nway | 18437ns | 18437ns | +273.28% |
| sequential | 10497ns | 10497ns | +112.52% |
| std_parallel | 32711ns | 32711ns | +562.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4164ns | base | --- | [4038, 4682] | --- | --- | --- | --- |
| head_tail_dyn | 4105ns | no significant difference | [-579, +58]ns | [4036, 4170] | no | 0.1155 | 0.0963 | 0 |
| kernel_nway | 4132ns | no significant difference | [-154, +118]ns | [4051, 4330] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 2541ns | -1653.3ns (-39.7%) | [-2041, -1497]ns | [2478, 2577] | YES | 0.0000 | 0.0000 | 0 |
| nway | 17856ns | +13479.9ns (+323.7%) | [+13359, +13756]ns | [17652, 18062] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10232ns | +6063.3ns (+145.6%) | [+5825, +6229]ns | [10168, 10358] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 31656ns | +27635.0ns (+663.6%) | [+26543, +28627]ns | [30800, 33556] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4152ns | -0.2% | -0.3% | -39.4% | +336.4% | +145.2% | +646.6% |
| 1 | 2 | 4038ns | -1.2% | +2.3% | -36.8% | +336.4% | +153.2% | +681.2% |
| 1 | 3 | 4776ns | -14.6% | -10.1% | -47.0% | +274.2% | +116.3% | +602.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.256 | moderate- |
| head_tail_dyn | -0.546 | HIGH- (thermal bounce) |
| kernel_nway | -0.193 | ok |
| kernel_seq | -0.577 | HIGH- (thermal bounce) |
| nway | -0.502 | HIGH- (thermal bounce) |
| sequential | -0.043 | ok |
| std_parallel | -0.066 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 97015.2ns | 4329.1ns | 2241.0% | HIGH |
| head_tail_dyn | 98153.7ns | 4091.4ns | 2399.0% | HIGH |
| kernel_nway | 98490.5ns | 4257.3ns | 2313.4% | HIGH |
| kernel_seq | 93009.8ns | 2563.0ns | 3629.0% | HIGH |
| nway | 92331.4ns | 17907.0ns | 515.6% | HIGH |
| sequential | 91767.7ns | 10354.4ns | 886.3% | HIGH |
| std_parallel | 103666.1ns | 32323.1ns | 320.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 3970.5-4820.8 ns)
   3970.5 |########################################
   4013.0 |########################################
   4055.6 |####################
   4098.1 |####################
   4140.6 |####################
   4183.1 |########################################
   4225.6 |####################
   4268.1 |
   4310.7 |
   4353.2 |
   4395.7 |
   4438.2 |
   4480.7 |
   4523.2 |
   4565.7 |
   4608.3 |
   4650.8 |####################
   4693.3 |####################
   4735.8 |########################################
   4778.3 |####################
  (2 below, 1 above range)

head_tail_dyn (n=18, range 3916.1-4203.3 ns)
   3916.1 |####################
   3930.5 |
   3944.8 |
   3959.2 |
   3973.5 |
   3987.9 |
   4002.3 |
   4016.6 |########################################
   4031.0 |####################
   4045.4 |####################
   4059.7 |####################
   4074.1 |####################
   4088.4 |
   4102.8 |
   4117.2 |####################
   4131.5 |########################################
   4145.9 |####################
   4160.2 |
   4174.6 |####################
   4189.0 |####################
  (2 below, 3 above range)

kernel_nway (n=18, range 4000.0-4622.0 ns)
   4000.0 |##########
   4031.1 |##############################
   4062.2 |##########
   4093.3 |##########
   4124.4 |########################################
   4155.5 |
   4186.6 |
   4217.7 |##########
   4248.8 |
   4279.9 |
   4311.0 |
   4342.1 |
   4373.2 |
   4404.3 |
   4435.4 |
   4466.5 |##########
   4497.6 |##########
   4528.7 |
   4559.8 |
   4590.9 |
  (2 below, 3 above range)

kernel_seq (n=18, range 2450.6-2690.3 ns)
   2450.6 |########################################
   2462.6 |####################
   2474.5 |####################
   2486.5 |
   2498.5 |####################
   2510.5 |####################
   2522.5 |
   2534.5 |########################################
   2546.5 |####################
   2558.4 |########################################
   2570.4 |
   2582.4 |####################
   2594.4 |
   2606.4 |####################
   2618.4 |
   2630.4 |
   2642.3 |
   2654.3 |
   2666.3 |####################
   2678.3 |
  (2 below, 2 above range)

nway (n=18, range 17526.7-18289.2 ns)
  17526.7 |
  17564.8 |####################
  17602.9 |########################################
  17641.0 |####################
  17679.2 |####################
  17717.3 |
  17755.4 |
  17793.5 |########################################
  17831.7 |
  17869.8 |####################
  17907.9 |########################################
  17946.0 |
  17984.2 |
  18022.3 |####################
  18060.4 |####################
  18098.5 |
  18136.7 |
  18174.8 |####################
  18212.9 |
  18251.0 |
  (2 below, 3 above range)

sequential (n=18, range 10090.0-10690.0 ns)
  10090.0 |
  10120.0 |
  10150.0 |########################################
  10180.0 |########################
  10210.0 |
  10240.0 |########
  10270.0 |########
  10300.0 |
  10330.0 |################
  10360.0 |########
  10390.0 |########
  10420.0 |
  10450.0 |
  10480.0 |
  10510.0 |
  10540.0 |
  10570.0 |
  10600.0 |
  10630.0 |
  10660.0 |########
  (1 below, 2 above range)

std_parallel (n=18, range 30276.1-34841.4 ns)
  30276.1 |########################################
  30504.4 |#############
  30732.6 |#############
  30960.9 |
  31189.2 |##########################
  31417.4 |#############
  31645.7 |##########################
  31874.0 |
  32102.2 |#############
  32330.5 |
  32558.8 |
  32787.0 |
  33015.3 |#############
  33243.5 |
  33471.8 |
  33700.1 |
  33928.3 |##########################
  34156.6 |#############
  34384.9 |
  34613.1 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=2346.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2406.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2406.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3645.5% of algo (FFI overhead may distort results)
- **nway**: bridge=519.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=896.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=327.9% of algo (FFI overhead may distort results)
