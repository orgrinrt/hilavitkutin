# append, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is an outlier: 3.2x slower than the field

nway (22.58 ms) is 3.2x the fastest (7.11 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.67)

nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.2x the fastest

Fastest kernel_nway (7.11 ms) to slowest nway (22.58 ms): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### nway is inconsistent: worst-20% is 2.7x its best-20%

nway's best 20% of batches run at 8.42 ms but its worst 20% at 22.73 ms (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 7107902.4 ns median (-28.3% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.18x (fastest 7107902.4 ns, slowest 22582840.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 2 |
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
| head_tail | 9909637ns | 9921658ns | 9803371ns | 9884521ns | 10000444ns | base |
| head_tail_dyn | 9405968ns | 9381440ns | 8783704ns | 9349357ns | 9802014ns | -5.08% |
| kernel_nway | 9037355ns | 7112220ns | 6886507ns | 7050642ns | 13092850ns | -8.80% |
| kernel_seq | 9765022ns | 9869510ns | 9247356ns | 9795382ns | 9978316ns | -1.46% |
| nway | 18424284ns | 22585348ns | 8420484ns | 18889697ns | 22728066ns | +85.92% |
| sequential | 13989723ns | 13956073ns | 13889876ns | 13934913ns | 14121860ns | +41.17% |
| std_parallel | 10784488ns | 7617943ns | 7475653ns | 7582866ns | 17241337ns | +8.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9905757ns | 9800770ns | 9995449ns | base | 1.694 |
| head_tail_dyn | 9402481ns | 8781253ns | 9797323ns | -5.08% | 1.784 |
| kernel_nway | 9033089ns | 6883573ns | 13088781ns | -8.81% | 1.857 |
| kernel_seq | 9761365ns | 9244606ns | 9973954ns | -1.46% | 1.719 |
| nway | 18421733ns | 8417812ns | 22725477ns | +85.97% | 0.911 |
| sequential | 13987942ns | 13888232ns | 14119888ns | +41.21% | 1.199 |
| std_parallel | 10780799ns | 7473248ns | 17238398ns | +8.83% | 1.556 |

## Performance model

- Peak throughput: **2.437 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.691 | 69.4% |
| head_tail_dyn | 1.789 | 73.4% |
| kernel_nway | 2.360 | 96.8% |
| kernel_seq | 1.700 | 69.8% |
| nway | 0.743 | 30.5% |
| sequential | 1.202 | 49.3% |
| std_parallel | 2.204 | 90.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9909637ns | 9909637ns | base |
| head_tail_dyn | 9405968ns | 9405968ns | -5.08% |
| kernel_nway | 9037355ns | 9037355ns | -8.80% |
| kernel_seq | 9765022ns | 9765022ns | -1.46% |
| nway | 18424284ns | 18424284ns | +85.92% |
| sequential | 13989723ns | 13989723ns | +41.17% |
| std_parallel | 10784488ns | 10784488ns | +8.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9918719ns | base | --- | [9803102, 9995449] | --- | --- | --- | --- |
| head_tail_dyn | 9378099ns | -553986.9ns (-5.6%) | [-851446, -104394]ns | [9032022, 9797323] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| kernel_nway | 7107902ns | no significant difference | [-2915192, +3152150]ns | [6902584, 13088781] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 9866743ns | no significant difference | [-493232, +149256]ns | [9443400, 9973954] | no | 0.6875 | 0.6875 | 0 |
| nway | 22582841ns | +12646209.5ns (+127.5%) | [+153779, +12747940]ns | [9956880, 22725477] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| sequential | 13954356ns | +4067189.6ns (+41.0%) | [+3981726, +4197640]ns | [13889581, 14119888] | YES (adj: no) | 0.1875 | 0.0313 | 0 |
| std_parallel | 7613547ns | no significant difference | [-2415916, +7319679]ns | [7490453, 17238398] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9936631ns | -4.6% | +31.7% | -5.0% | +127.3% | +40.7% | +41.5% |
| 1 | 2 | 9803102ns | -4.6% | -29.6% | +1.5% | +1.6% | +42.2% | -23.2% |
| 1 | 3 | 9977537ns | -6.0% | -28.8% | -0.9% | +127.8% | +40.8% | +7.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.633 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.293 | moderate- |
| kernel_nway | -0.184 | ok |
| kernel_seq | -0.237 | moderate- |
| nway | -0.667 | HIGH- (thermal bounce) |
| sequential | -0.400 | moderate- |
| std_parallel | -0.497 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 72880687.0ns | 9905756.7ns | 735.7% | HIGH |
| head_tail_dyn | 73476960.1ns | 9402481.2ns | 781.5% | HIGH |
| kernel_nway | 74311320.0ns | 9033089.2ns | 822.7% | HIGH |
| kernel_seq | 73307768.3ns | 9761365.4ns | 751.0% | HIGH |
| nway | 89030492.6ns | 18421732.8ns | 483.3% | HIGH |
| sequential | 75531496.4ns | 13987941.8ns | 540.0% | HIGH |
| std_parallel | 73070778.8ns | 10780799.3ns | 677.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 9800770.2-9995449.1 ns)
  9800770.2 |########################################
  9810504.1 |
  9820238.1 |
  9829972.0 |
  9839706.0 |
  9849439.9 |
  9859173.9 |
  9868907.8 |####################
  9878641.8 |
  9888375.7 |
  9898109.7 |
  9907843.6 |
  9917577.6 |
  9927311.5 |
  9937045.5 |
  9946779.4 |
  9956513.4 |####################
  9966247.3 |
  9975981.3 |
  9985715.2 |####################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 8781252.6-9797323.4 ns)
  8781252.6 |########################################
  8832056.1 |
  8882859.7 |
  8933663.2 |
  8984466.8 |
  9035270.3 |
  9086073.9 |
  9136877.4 |
  9187680.9 |
  9238484.5 |########################################
  9289288.0 |########################################
  9340091.6 |
  9390895.1 |
  9441698.7 |########################################
  9492502.2 |
  9543305.7 |
  9594109.3 |
  9644912.8 |########################################
  9695716.4 |
  9746519.9 |
  (0 below, 1 above range)

kernel_nway (n=6, range 6883573.0-13088781.3 ns)
  6883573.0 |########################################
  7193833.4 |
  7504093.8 |
  7814354.2 |
  8124614.7 |
  8434875.1 |
  8745135.5 |
  9055395.9 |
  9365656.3 |
  9675916.7 |
  9986177.2 |
  10296437.6 |
  10606698.0 |
  10916958.4 |
  11227218.8 |
  11537479.2 |
  11847739.6 |
  12158000.1 |
  12468260.5 |
  12778520.9 |##########
  (0 below, 1 above range)

kernel_seq (n=6, range 9244606.1-9973953.8 ns)
  9244606.1 |########################################
  9281073.5 |
  9317540.9 |
  9354008.2 |
  9390475.6 |
  9426943.0 |
  9463410.4 |
  9499877.8 |
  9536345.2 |
  9572812.5 |
  9609279.9 |########################################
  9645747.3 |
  9682214.7 |
  9718682.1 |
  9755149.5 |
  9791616.8 |########################################
  9828084.2 |
  9864551.6 |
  9901019.0 |########################################
  9937486.4 |########################################
  (0 below, 1 above range)

nway (n=6, range 8417812.3-22725477.3 ns)
  8417812.3 |#############
  9133195.6 |
  9848578.8 |
  10563962.1 |
  11279345.3 |#############
  11994728.6 |
  12710111.8 |
  13425495.1 |
  14140878.3 |
  14856261.6 |
  15571644.8 |
  16287028.1 |
  17002411.3 |
  17717794.6 |
  18433177.8 |
  19148561.1 |
  19863944.3 |
  20579327.6 |
  21294710.8 |
  22010094.1 |########################################
  (0 below, 1 above range)

sequential (n=6, range 13888231.6-14119888.2 ns)
  13888231.6 |########################################
  13899814.4 |
  13911397.3 |####################
  13922980.1 |
  13934562.9 |
  13946145.8 |
  13957728.6 |
  13969311.4 |
  13980894.2 |####################
  13992477.1 |
  14004059.9 |
  14015642.7 |
  14027225.6 |####################
  14038808.4 |
  14050391.2 |
  14061974.0 |
  14073556.9 |
  14085139.7 |
  14096722.5 |
  14108305.4 |
  (0 below, 1 above range)

std_parallel (n=6, range 7473247.6-17238398.1 ns)
  7473247.6 |########################################
  7961505.1 |
  8449762.6 |
  8938020.2 |
  9426277.7 |
  9914535.2 |
  10402792.7 |
  10891050.3 |
  11379307.8 |
  11867565.3 |
  12355822.8 |
  12844080.3 |
  13332337.9 |
  13820595.4 |##########
  14308852.9 |
  14797110.4 |
  15285368.0 |
  15773625.5 |
  16261883.0 |
  16750140.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=730.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=775.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=31.8% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=1027.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=736.9% of algo (FFI overhead may distort results)
- **nway**: CV=32.8% (high variance, measurements may be unstable)
- **nway**: bridge=386.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=540.3% of algo (FFI overhead may distort results)
- **std_parallel**: CV=46.0% (high variance, measurements may be unstable)
- **std_parallel**: bridge=933.0% of algo (FFI overhead may distort results)
