# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 65% faster than the next best (head_tail_dyn)

kernel_seq (2.50 us) leads head_tail_dyn (4.13 us) by 65%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 40% (significant)

kernel_seq is -1.72 us (40%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 21.5x slower than the field

std_parallel (53.57 us) is 21.5x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 21.5x the fastest

Fastest kernel_seq (2.50 us) to slowest std_parallel (53.57 us): 21.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2496.7 ns median (-41.3% vs baseline)
- 2 variants significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 21.46x (fastest 2496.7 ns, slowest 53567.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 4 |
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
| head_tail | 5020ns | 4930ns | 4618ns | 4868ns | 5449ns | base |
| head_tail_dyn | 4742ns | 4728ns | 4617ns | 4711ns | 4852ns | -5.52% |
| kernel_nway | 10498ns | 9921ns | 9312ns | 9814ns | 12118ns | +109.14% |
| kernel_seq | 2835ns | 2788ns | 2668ns | 2779ns | 3003ns | -43.52% |
| nway | 34450ns | 33920ns | 30712ns | 33358ns | 37958ns | +586.32% |
| sequential | 10594ns | 10316ns | 10282ns | 10320ns | 11162ns | +111.06% |
| std_parallel | 54415ns | 53945ns | 50656ns | 53326ns | 57929ns | +984.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4407ns | 4004ns | 4857ns | base | 0.015 |
| head_tail_dyn | 4136ns | 4012ns | 4243ns | -6.16% | 0.015 |
| kernel_nway | 9882ns | 8724ns | 11482ns | +124.21% | 0.006 |
| kernel_seq | 2524ns | 2447ns | 2616ns | -42.72% | 0.025 |
| nway | 33921ns | 30214ns | 37431ns | +669.66% | 0.002 |
| sequential | 10449ns | 10145ns | 11006ns | +137.09% | 0.006 |
| std_parallel | 54017ns | 50272ns | 57525ns | +1125.64% | 0.001 |

## Performance model

- Peak throughput: **0.026 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.015 | 57.5% |
| head_tail_dyn | 0.015 | 59.2% |
| kernel_nway | 0.007 | 26.3% |
| kernel_seq | 0.026 | 98.0% |
| nway | 0.002 | 7.3% |
| sequential | 0.006 | 24.0% |
| std_parallel | 0.001 | 4.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 5020ns | 5020ns | base |
| head_tail_dyn | 4742ns | 4742ns | -5.52% |
| kernel_nway | 10498ns | 10498ns | +109.14% |
| kernel_seq | 2835ns | 2835ns | -43.52% |
| nway | 34450ns | 34450ns | +586.32% |
| sequential | 10594ns | 10594ns | +111.06% |
| std_parallel | 54415ns | 54415ns | +984.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4255ns | base | --- | [4145, 4642] | --- | --- | --- | --- |
| head_tail_dyn | 4131ns | -196.7ns (-4.6%) | [-508, -22]ns | [4067, 4186] | YES (adj: no) | 0.0963 | 0.0963 | 0 |
| kernel_nway | 9304ns | +4920.9ns (+115.6%) | [+4573, +5528]ns | [8966, 9771] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2497ns | -1719.2ns (-40.4%) | [-2100, -1628]ns | [2474, 2538] | YES | 0.0000 | 0.0000 | 0 |
| nway | 33406ns | +29151.7ns (+685.1%) | [+27490, +29493]ns | [31598, 33890] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10175ns | +6032.5ns (+141.8%) | [+5864, +6268]ns | [10153, 10407] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 53568ns | +49177.5ns (+1155.8%) | [+47503, +50394]ns | [52150, 54843] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4143ns | +0.0% | +123.6% | -40.5% | +748.0% | +145.1% | +1213.9% |
| 1 | 2 | 4335ns | -5.6% | +123.7% | -42.0% | +648.7% | +134.3% | +1155.3% |
| 1 | 3 | 4642ns | -10.6% | +95.1% | -45.7% | +618.5% | +139.2% | +1010.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.012 | ok |
| head_tail_dyn | -0.659 | HIGH- (thermal bounce) |
| kernel_nway | -0.600 | HIGH- (thermal bounce) |
| kernel_seq | -0.121 | ok |
| nway | -0.383 | moderate- |
| sequential | -0.166 | ok |
| std_parallel | -0.163 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 0/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 97136.1ns | 4407.2ns | 2204.0% | HIGH |
| head_tail_dyn | 96658.4ns | 4135.6ns | 2337.2% | HIGH |
| kernel_nway | 118955.4ns | 9881.6ns | 1203.8% | HIGH |
| kernel_seq | 89948.2ns | 2524.3ns | 3563.4% | HIGH |
| nway | 111069.2ns | 33920.7ns | 327.4% | HIGH |
| sequential | 91450.5ns | 10449.2ns | 875.2% | HIGH |
| std_parallel | 114544.4ns | 54016.8ns | 212.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4004.4-4857.0 ns)
   4004.4 |##########################
   4047.1 |
   4089.7 |##########################
   4132.3 |#############
   4174.9 |########################################
   4217.6 |
   4260.2 |#############
   4302.8 |
   4345.4 |#############
   4388.1 |
   4430.7 |#############
   4473.3 |
   4516.0 |#############
   4558.6 |
   4601.2 |
   4643.8 |
   4686.5 |
   4729.1 |#############
   4771.7 |
   4814.3 |
  (1 below, 4 above range)

head_tail_dyn (n=18, range 4011.7-4242.5 ns)
   4011.7 |
   4023.2 |####################
   4034.8 |########################################
   4046.3 |####################
   4057.8 |####################
   4069.4 |
   4080.9 |
   4092.5 |####################
   4104.0 |####################
   4115.5 |####################
   4127.1 |
   4138.6 |####################
   4150.2 |####################
   4161.7 |####################
   4173.3 |
   4184.8 |########################################
   4196.3 |
   4207.9 |####################
   4219.4 |
   4231.0 |
  (1 below, 3 above range)

kernel_nway (n=18, range 8724.4-11482.2 ns)
   8724.4 |####################
   8862.3 |##############################
   9000.2 |
   9138.1 |##########
   9276.0 |########################################
   9413.9 |##########
   9551.8 |##########
   9689.7 |
   9827.5 |##########
   9965.4 |
  10103.3 |
  10241.2 |
  10379.1 |
  10517.0 |
  10654.9 |
  10792.8 |
  10930.7 |
  11068.5 |
  11206.4 |
  11344.3 |##########
  (1 below, 3 above range)

kernel_seq (n=18, range 2446.7-2616.1 ns)
   2446.7 |##########################
   2455.1 |
   2463.6 |
   2472.1 |########################################
   2480.6 |
   2489.0 |##########################
   2497.5 |##########################
   2506.0 |
   2514.4 |
   2522.9 |
   2531.4 |##########################
   2539.9 |#############
   2548.3 |
   2556.8 |##########################
   2565.3 |
   2573.8 |
   2582.2 |
   2590.7 |
   2599.2 |
   2607.6 |
  (2 below, 2 above range)

nway (n=18, range 30214.5-37431.4 ns)
  30214.5 |################
  30575.3 |
  30936.2 |
  31297.0 |################
  31657.9 |########
  32018.7 |
  32379.5 |
  32740.4 |
  33101.2 |########################################
  33462.1 |########
  33822.9 |################
  34183.8 |
  34544.6 |
  34905.5 |########
  35266.3 |
  35627.2 |
  35988.0 |
  36348.9 |
  36709.7 |########
  37070.6 |
  (1 below, 2 above range)

sequential (n=18, range 10145.0-11006.4 ns)
  10145.0 |########################################
  10188.1 |
  10231.1 |
  10274.2 |####
  10317.3 |####
  10360.3 |
  10403.4 |
  10446.5 |####
  10489.6 |
  10532.6 |
  10575.7 |
  10618.8 |
  10661.8 |
  10704.9 |
  10748.0 |
  10791.0 |
  10834.1 |
  10877.2 |
  10920.2 |
  10963.3 |
  (1 below, 4 above range)

std_parallel (n=18, range 50272.2-57524.7 ns)
  50272.2 |#############
  50634.9 |#############
  50997.5 |#############
  51360.1 |
  51722.7 |#############
  52085.4 |
  52448.0 |########################################
  52810.6 |
  53173.2 |#############
  53535.9 |#############
  53898.5 |##########################
  54261.1 |
  54623.7 |##########################
  54986.4 |
  55349.0 |
  55711.6 |#############
  56074.2 |
  56436.9 |
  56799.5 |
  57162.1 |#############
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=2305.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2326.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1256.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3606.2% of algo (FFI overhead may distort results)
- **nway**: bridge=330.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=901.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=213.1% of algo (FFI overhead may distort results)
