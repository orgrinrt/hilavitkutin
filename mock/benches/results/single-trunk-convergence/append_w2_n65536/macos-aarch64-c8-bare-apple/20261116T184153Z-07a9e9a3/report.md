# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is an outlier: 6.2x slower than the field

nway (60.37 us) is 6.2x the fastest (9.75 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_seq, head_tail) are a dead heat (<1%)

kernel_seq (9.75 us) and head_tail (9.78 us) differ by 0.32%, inside the noise, even though the wider field spreads 519.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### std_parallel shows alternating (throttle bounce) (autocorr -0.61)

std_parallel's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, head_tail, head_tail_dyn, kernel_nway} vs {std_parallel, sequential, nway} (239% apart)

The field splits into a fast tier {kernel_seq, head_tail, head_tail_dyn, kernel_nway} and a slow tier {std_parallel, sequential, nway} with a 239% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.2x the fastest

Fastest kernel_seq (9.75 us) to slowest nway (60.37 us): 6.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 9745.9 ns median (-0.3% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 6.19x (fastest 9745.9 ns, slowest 60368.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 10358ns | 10333ns | 9889ns | 10231ns | 10783ns | base |
| head_tail_dyn | 10690ns | 10560ns | 9994ns | 10457ns | 11388ns | +3.21% |
| kernel_nway | 10888ns | 10850ns | 9922ns | 10750ns | 11577ns | +5.12% |
| kernel_seq | 10543ns | 10492ns | 10003ns | 10476ns | 10913ns | +1.78% |
| nway | 60497ns | 61010ns | 55151ns | 60747ns | 62793ns | +484.06% |
| sequential | 56572ns | 56515ns | 55917ns | 56370ns | 57203ns | +446.18% |
| std_parallel | 34681ns | 34777ns | 32013ns | 34274ns | 36626ns | +234.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9860ns | 9457ns | 10222ns | base | 6.647 |
| head_tail_dyn | 9953ns | 9517ns | 10406ns | +0.94% | 6.585 |
| kernel_nway | 10091ns | 9349ns | 10745ns | +2.34% | 6.495 |
| kernel_seq | 9922ns | 9606ns | 10330ns | +0.63% | 6.605 |
| nway | 59706ns | 54528ns | 61919ns | +505.55% | 1.098 |
| sequential | 56170ns | 55481ns | 56812ns | +469.69% | 1.167 |
| std_parallel | 33868ns | 31303ns | 36001ns | +243.49% | 1.935 |

## Performance model

- Peak throughput: **7.010 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.703 | 95.6% |
| head_tail_dyn | 6.627 | 94.5% |
| kernel_nway | 6.569 | 93.7% |
| kernel_seq | 6.725 | 95.9% |
| nway | 1.086 | 15.5% |
| sequential | 1.171 | 16.7% |
| std_parallel | 1.936 | 27.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 10358ns | 10358ns | base |
| head_tail_dyn | 10690ns | 10690ns | +3.21% |
| kernel_nway | 10888ns | 10888ns | +5.12% |
| kernel_seq | 10543ns | 10543ns | +1.78% |
| nway | 60497ns | 60497ns | +484.06% |
| sequential | 56572ns | 56572ns | +446.18% |
| std_parallel | 34681ns | 34681ns | +234.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9777ns | base | --- | [9650, 10142] | --- | --- | --- | --- |
| head_tail_dyn | 9890ns | no significant difference | [-135, +325]ns | [9681, 10144] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 9977ns | no significant difference | [-121, +575]ns | [9662, 10485] | no | 0.7210 | 0.4807 | 0 |
| kernel_seq | 9746ns | no significant difference | [-171, +185]ns | [9646, 10129] | no | 0.8145 | 0.8145 | 0 |
| nway | 60369ns | +50458.3ns (+516.1%) | [+49369, +51117]ns | [59362, 61171] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 55988ns | +46170.8ns (+472.2%) | [+45915, +46427]ns | [55758, 56573] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 33844ns | +23879.2ns (+244.2%) | [+22738, +25302]ns | [32462, 35152] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9848ns | +3.0% | +3.9% | +0.3% | +506.3% | +469.6% | +250.6% |
| 1 | 2 | 10052ns | -3.5% | +3.4% | -1.8% | +494.2% | +462.8% | +230.4% |
| 1 | 3 | 9660ns | +1.4% | +1.0% | +0.5% | +533.9% | +476.5% | +253.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.514 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.287 | moderate- |
| kernel_nway | -0.328 | moderate- |
| kernel_seq | -0.148 | ok |
| nway | -0.160 | ok |
| sequential | -0.525 | HIGH- (thermal bounce) |
| std_parallel | -0.607 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 444614.3ns | 9859.7ns | 4509.4% | HIGH |
| head_tail_dyn | 444396.3ns | 9952.5ns | 4465.2% | HIGH |
| kernel_nway | 437150.0ns | 10090.5ns | 4332.3% | HIGH |
| kernel_seq | 445384.0ns | 9922.2ns | 4488.8% | HIGH |
| nway | 483847.2ns | 59706.0ns | 810.4% | HIGH |
| sequential | 469391.7ns | 56170.1ns | 835.7% | HIGH |
| std_parallel | 458480.1ns | 33867.6ns | 1353.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 9457.0-10221.5 ns)
   9457.0 |
   9495.2 |
   9533.4 |
   9571.6 |####################
   9609.9 |####################
   9648.1 |########################################
   9686.3 |########################################
   9724.6 |####################
   9762.8 |
   9801.0 |####################
   9839.2 |
   9877.5 |
   9915.7 |####################
   9953.9 |####################
   9992.2 |
  10030.4 |
  10068.6 |
  10106.8 |####################
  10145.1 |####################
  10183.3 |########################################
  (2 below, 2 above range)

head_tail_dyn (n=18, range 9516.7-10405.6 ns)
   9516.7 |########################################
   9561.1 |####################
   9605.6 |####################
   9650.0 |
   9694.4 |####################
   9738.9 |####################
   9783.3 |
   9827.8 |########################################
   9872.2 |####################
   9916.7 |########################################
   9961.1 |
  10005.6 |
  10050.0 |####################
  10094.5 |
  10138.9 |
  10183.3 |####################
  10227.8 |####################
  10272.2 |
  10316.7 |
  10361.1 |####################
  (1 below, 2 above range)

kernel_nway (n=18, range 9348.6-10745.1 ns)
   9348.6 |
   9418.4 |########################################
   9488.3 |
   9558.1 |########################################
   9627.9 |
   9697.7 |####################
   9767.6 |
   9837.4 |####################
   9907.2 |########################################
   9977.0 |####################
  10046.9 |
  10116.7 |####################
  10186.5 |
  10256.3 |####################
  10326.2 |
  10396.0 |####################
  10465.8 |
  10535.7 |
  10605.5 |####################
  10675.3 |########################################
  (1 below, 2 above range)

kernel_seq (n=18, range 9605.5-10329.9 ns)
   9605.5 |##############################
   9641.7 |########################################
   9678.0 |
   9714.2 |##########
   9750.4 |##########
   9786.6 |
   9822.8 |
   9859.0 |
   9895.3 |
   9931.5 |
   9967.7 |
  10003.9 |####################
  10040.1 |
  10076.3 |##########
  10112.6 |
  10148.8 |##########
  10185.0 |##########
  10221.2 |
  10257.4 |
  10293.6 |
  (1 below, 3 above range)

nway (n=18, range 54527.8-61918.8 ns)
  54527.8 |
  54897.3 |
  55266.9 |
  55636.4 |
  56006.0 |####################
  56375.5 |
  56745.1 |
  57114.6 |
  57484.2 |
  57853.7 |
  58223.3 |
  58592.8 |####################
  58962.4 |########################################
  59331.9 |####################
  59701.5 |########################################
  60071.0 |####################
  60440.6 |########################################
  60810.1 |########################################
  61179.7 |########################################
  61549.2 |
  (1 below, 3 above range)

sequential (n=18, range 55480.6-56811.8 ns)
  55480.6 |
  55547.1 |
  55613.7 |##########
  55680.3 |####################
  55746.8 |##########
  55813.4 |
  55879.9 |##############################
  55946.5 |
  56013.1 |##########
  56079.6 |
  56146.2 |##########
  56212.8 |
  56279.3 |
  56345.9 |
  56412.4 |
  56479.0 |
  56545.6 |########################################
  56612.1 |##########
  56678.7 |
  56745.3 |
  (2 below, 2 above range)

std_parallel (n=18, range 31302.8-36000.7 ns)
  31302.8 |########################################
  31537.7 |####################
  31772.6 |####################
  32007.5 |
  32242.4 |
  32477.2 |
  32712.1 |
  32947.0 |####################
  33181.9 |####################
  33416.8 |
  33651.7 |########################################
  33886.6 |########################################
  34121.5 |
  34356.4 |####################
  34591.3 |
  34826.2 |####################
  35061.1 |####################
  35296.0 |
  35530.9 |
  35765.8 |####################
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=4550.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=4494.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=4379.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=4561.5% of algo (FFI overhead may distort results)
- **nway**: bridge=792.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=838.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1346.5% of algo (FFI overhead may distort results)
