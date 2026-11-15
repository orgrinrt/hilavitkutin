# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 3.1x slower than the field

std_parallel (185.82 us) is 3.1x the fastest (60.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.65)

kernel_nway's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.1x the fastest

Fastest head_tail_dyn (60.61 us) to slowest std_parallel (185.82 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 2.1x its best-20%

sequential's best 20% of batches run at 74.22 us but its worst 20% at 159.30 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 60606.6 ns median (-14.4% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 3.07x (fastest 60606.6 ns, slowest 185822.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100242360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 69950ns | 71387ns | 59792ns | 68623ns | 77020ns | base |
| head_tail_dyn | 63868ns | 61260ns | 55172ns | 60705ns | 72962ns | -8.69% |
| kernel_nway | 84112ns | 85153ns | 74728ns | 82633ns | 91024ns | +20.25% |
| kernel_seq | 67619ns | 63604ns | 54697ns | 62621ns | 81579ns | -3.33% |
| nway | 135416ns | 140298ns | 99780ns | 131886ns | 158529ns | +93.59% |
| sequential | 111044ns | 96043ns | 74630ns | 90565ns | 159968ns | +58.75% |
| std_parallel | 185358ns | 186621ns | 153142ns | 174349ns | 217980ns | +164.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 69290ns | 59041ns | 76306ns | base | 0.946 |
| head_tail_dyn | 63108ns | 54686ns | 72045ns | -8.92% | 1.038 |
| kernel_nway | 83328ns | 74105ns | 90485ns | +20.26% | 0.786 |
| kernel_seq | 66768ns | 54212ns | 80863ns | -3.64% | 0.982 |
| nway | 134589ns | 99235ns | 157317ns | +94.24% | 0.487 |
| sequential | 110498ns | 74219ns | 159304ns | +59.47% | 0.593 |
| std_parallel | 184376ns | 152315ns | 216735ns | +166.09% | 0.355 |

## Performance model

- Peak throughput: **1.209 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.926 | 76.6% |
| head_tail_dyn | 1.081 | 89.4% |
| kernel_nway | 0.777 | 64.3% |
| kernel_seq | 1.048 | 86.7% |
| nway | 0.471 | 38.9% |
| sequential | 0.687 | 56.8% |
| std_parallel | 0.353 | 29.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 69950ns | 69950ns | base |
| head_tail_dyn | 63868ns | 63868ns | -8.69% |
| kernel_nway | 84112ns | 84112ns | +20.25% |
| kernel_seq | 67619ns | 67619ns | -3.33% |
| nway | 135416ns | 135416ns | +93.59% |
| sequential | 111044ns | 111044ns | +58.75% |
| std_parallel | 185358ns | 185358ns | +164.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 70782ns | base | --- | [64840, 73910] | --- | --- | --- | --- |
| head_tail_dyn | 60607ns | -8031.5ns (-11.3%) | [-13303, -596]ns | [58266, 65671] | YES | 0.0371 | 0.0309 | 0 |
| kernel_nway | 84364ns | +14061.0ns (+19.9%) | [+9942, +17719]ns | [78550, 88502] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 62532ns | no significant difference | [-12528, +6088]ns | [57023, 75382] | no | 0.4807 | 0.4807 | 0 |
| nway | 139267ns | +67980.1ns (+96.0%) | [+47943, +82849]ns | [120020, 145701] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 95370ns | +32884.1ns (+46.5%) | [+5167, +78971]ns | [78533, 149664] | YES | 0.0002 | 0.0001 | 0 |
| std_parallel | 185822ns | +116765.8ns (+165.0%) | [+93675, +127844]ns | [160446, 195898] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 62945ns | +2.3% | +24.1% | +25.8% | +143.9% | +51.5% | +198.1% |
| 1 | 2 | 73602ns | -20.6% | +18.9% | -16.8% | +45.8% | +6.1% | +176.0% |
| 1 | 3 | 69099ns | -11.0% | +15.1% | -19.2% | +101.5% | +129.0% | +127.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.446 | moderate- |
| head_tail_dyn | -0.510 | HIGH- (thermal bounce) |
| kernel_nway | -0.653 | HIGH- (thermal bounce) |
| kernel_seq | -0.058 | ok |
| nway | -0.606 | HIGH- (thermal bounce) |
| sequential | -0.296 | moderate- |
| std_parallel | -0.385 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 468054.9ns | 69289.5ns | 675.5% | HIGH |
| head_tail_dyn | 416253.7ns | 63108.5ns | 659.6% | HIGH |
| kernel_nway | 463042.0ns | 83327.7ns | 555.7% | HIGH |
| kernel_seq | 405550.6ns | 66768.4ns | 607.4% | HIGH |
| nway | 510632.4ns | 134589.4ns | 379.4% | HIGH |
| sequential | 449023.8ns | 110497.5ns | 406.4% | HIGH |
| std_parallel | 539366.5ns | 184375.9ns | 292.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 59040.7-76306.0 ns)
  59040.7 |
  59904.0 |
  60767.3 |
  61630.5 |####################
  62493.8 |
  63357.1 |########################################
  64220.3 |
  65083.6 |####################
  65946.8 |####################
  66810.1 |####################
  67673.4 |
  68536.6 |
  69399.9 |####################
  70263.2 |
  71126.4 |########################################
  71989.7 |####################
  72852.9 |####################
  73716.2 |
  74579.5 |####################
  75442.7 |####################
  (2 below, 3 above range)

head_tail_dyn (n=18, range 54685.6-72044.8 ns)
  54685.6 |#############
  55553.6 |##########################
  56421.5 |
  57289.5 |#############
  58157.5 |########################################
  59025.4 |#############
  59893.4 |
  60761.3 |
  61629.3 |#############
  62497.2 |
  63365.2 |#############
  64233.2 |#############
  65101.1 |#############
  65969.1 |##########################
  66837.0 |
  67705.0 |
  68572.9 |
  69440.9 |
  70308.9 |#############
  71176.8 |
  (1 below, 2 above range)

kernel_nway (n=18, range 74105.1-90485.3 ns)
  74105.1 |
  74924.1 |
  75743.1 |
  76562.1 |########################################
  77381.1 |####################
  78200.2 |########################################
  79019.2 |####################
  79838.2 |####################
  80657.2 |
  81476.2 |
  82295.2 |
  83114.2 |####################
  83933.2 |
  84752.3 |########################################
  85571.3 |
  86390.3 |####################
  87209.3 |
  88028.3 |########################################
  88847.3 |
  89666.3 |########################################
  (1 below, 2 above range)

kernel_seq (n=18, range 54211.9-80863.1 ns)
  54211.9 |#############
  55544.5 |########################################
  56877.0 |#############
  58209.6 |#############
  59542.1 |
  60874.7 |##########################
  62207.3 |
  63539.8 |#############
  64872.4 |#############
  66204.9 |
  67537.5 |
  68870.1 |
  70202.6 |
  71535.2 |
  72867.7 |
  74200.3 |##########################
  75532.9 |##########################
  76865.4 |
  78198.0 |
  79530.5 |
  (1 below, 3 above range)

nway (n=18, range 99235.4-157317.1 ns)
  99235.4 |
  102139.5 |####################
  105043.5 |####################
  107947.6 |########################################
  110851.7 |
  113755.8 |
  116659.9 |
  119564.0 |
  122468.1 |
  125372.2 |
  128276.2 |####################
  131180.3 |####################
  134084.4 |####################
  136988.5 |########################################
  139892.6 |####################
  142796.7 |########################################
  145700.8 |####################
  148604.9 |
  151509.0 |####################
  154413.0 |
  (1 below, 3 above range)

sequential (n=18, range 74219.2-159303.9 ns)
  74219.2 |########################################
  78473.4 |########################################
  82727.6 |#############
  86981.9 |
  91236.1 |
  95490.4 |
  99744.6 |
  103998.8 |#############
  108253.1 |#############
  112507.3 |#############
  116761.6 |
  121015.8 |
  125270.0 |
  129524.3 |
  133778.5 |
  138032.8 |
  142287.0 |
  146541.2 |##########################
  150795.5 |#############
  155049.7 |
  (2 below, 3 above range)

std_parallel (n=18, range 152314.8-216734.7 ns)
  152314.8 |####################
  155535.8 |####################
  158756.8 |########################################
  161977.8 |
  165198.8 |########################################
  168419.8 |
  171640.8 |
  174861.8 |
  178082.8 |
  181303.8 |
  184524.8 |########################################
  187745.7 |########################################
  190966.7 |####################
  194187.7 |
  197408.7 |####################
  200629.7 |
  203850.7 |
  207071.7 |####################
  210292.7 |
  213513.7 |####################
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=623.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=677.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=552.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=638.7% of algo (FFI overhead may distort results)
- **nway**: bridge=347.9% of algo (FFI overhead may distort results)
- **sequential**: CV=33.3% (high variance, measurements may be unstable)
- **sequential**: bridge=483.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=286.5% of algo (FFI overhead may distort results)
