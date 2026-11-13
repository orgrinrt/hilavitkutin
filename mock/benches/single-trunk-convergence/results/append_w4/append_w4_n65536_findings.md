# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 2.2x slower than the field

std_parallel (125.77 us) is 2.2x the fastest (58.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, head_tail_dyn, head_tail, kernel_nway, sequential} vs {nway, std_parallel} (39% apart)

The field splits into a fast tier {kernel_seq, head_tail_dyn, head_tail, kernel_nway, sequential} and a slow tier {nway, std_parallel} with a 39% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### sequential is inconsistent: worst-20% is 1.9x its best-20%

sequential's best 20% of batches run at 72.30 us but its worst 20% at 137.36 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 58347.0 ns median (-12.8% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.16x (fastest 58347.0 ns, slowest 125772.4 ns)

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
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 68772ns | 67515ns | 57494ns | 65049ns | 79997ns | base |
| head_tail_dyn | 64920ns | 62325ns | 56135ns | 61104ns | 75034ns | -5.60% |
| kernel_nway | 79433ns | 70492ns | 61050ns | 70178ns | 102507ns | +15.50% |
| kernel_seq | 63326ns | 59558ns | 51484ns | 58089ns | 77101ns | -7.92% |
| nway | 112552ns | 107504ns | 87244ns | 101892ns | 141196ns | +63.66% |
| sequential | 96249ns | 77258ns | 72652ns | 76283ns | 137997ns | +39.95% |
| std_parallel | 125796ns | 126809ns | 99579ns | 122758ns | 143460ns | +82.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 68172ns | 57033ns | 79399ns | base | 0.961 |
| head_tail_dyn | 64268ns | 55683ns | 74305ns | -5.73% | 1.020 |
| kernel_nway | 78650ns | 60520ns | 101559ns | +15.37% | 0.833 |
| kernel_seq | 62643ns | 51010ns | 76576ns | -8.11% | 1.046 |
| nway | 111967ns | 86783ns | 140657ns | +64.24% | 0.585 |
| sequential | 95762ns | 72304ns | 137361ns | +40.47% | 0.684 |
| std_parallel | 124639ns | 98898ns | 141924ns | +82.83% | 0.526 |

## Performance model

- Peak throughput: **1.285 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.979 | 76.2% |
| head_tail_dyn | 1.064 | 82.8% |
| kernel_nway | 0.936 | 72.9% |
| kernel_seq | 1.123 | 87.4% |
| nway | 0.613 | 47.7% |
| sequential | 0.853 | 66.4% |
| std_parallel | 0.521 | 40.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 68772ns | 68772ns | base |
| head_tail_dyn | 64920ns | 64920ns | -5.60% |
| kernel_nway | 79433ns | 79433ns | +15.50% |
| kernel_seq | 63326ns | 63326ns | -7.92% |
| nway | 112552ns | 112552ns | +63.66% |
| sequential | 96249ns | 96249ns | +39.95% |
| std_parallel | 125796ns | 125796ns | +82.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 66922ns | base | --- | [59977, 74418] | --- | --- | --- | --- |
| head_tail_dyn | 61595ns | no significant difference | [-11895, +4064]ns | [58379, 65567] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 70006ns | no significant difference | [-5061, +11968]ns | [66487, 75594] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 58347ns | -5155.9ns (-7.7%) | [-7491, -1575]ns | [55787, 67135] | YES | 0.0463 | 0.0309 | 0 |
| nway | 106971ns | +43959.0ns (+65.7%) | [+22966, +56568]ns | [93683, 124173] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 76815ns | +10353.3ns (+15.5%) | [+2439, +71050]ns | [73243, 134743] | YES | 0.0463 | 0.0309 | 0 |
| std_parallel | 125772ns | +58936.4ns (+88.1%) | [+43332, +68468]ns | [117375, 130720] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 64471ns | -1.2% | -0.8% | -10.4% | +105.3% | +12.4% | +102.9% |
| 1 | 2 | 59394ns | +3.7% | +28.0% | -6.1% | +82.4% | +131.4% | +104.3% |
| 1 | 3 | 80176ns | -28.3% | -9.5% | -3.0% | +16.2% | -4.8% | +51.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.316 | moderate- |
| head_tail_dyn | -0.024 | ok |
| kernel_nway | -0.350 | moderate- |
| kernel_seq | -0.215 | moderate- |
| nway | -0.011 | ok |
| sequential | -0.665 | HIGH- (thermal bounce) |
| std_parallel | -0.156 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 414128.5ns | 68172.0ns | 607.5% | HIGH |
| head_tail_dyn | 444154.1ns | 64267.6ns | 691.1% | HIGH |
| kernel_nway | 432909.8ns | 78649.9ns | 550.4% | HIGH |
| kernel_seq | 398359.8ns | 62643.1ns | 635.9% | HIGH |
| nway | 465402.8ns | 111967.1ns | 415.7% | HIGH |
| sequential | 419929.3ns | 95762.5ns | 438.5% | HIGH |
| std_parallel | 519245.8ns | 124639.3ns | 416.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 57032.6-79399.4 ns)
  57032.6 |####################
  58150.9 |########################################
  59269.3 |########################################
  60387.6 |
  61505.9 |
  62624.3 |####################
  63742.6 |
  64861.0 |####################
  65979.3 |####################
  67097.6 |####################
  68216.0 |####################
  69334.3 |####################
  70452.7 |
  71571.0 |
  72689.4 |
  73807.7 |########################################
  74926.0 |
  76044.4 |
  77162.7 |
  78281.1 |
  (1 below, 4 above range)

head_tail_dyn (n=18, range 55682.7-74304.6 ns)
  55682.7 |#############
  56613.8 |##########################
  57544.9 |##########################
  58475.9 |
  59407.0 |
  60338.1 |##########################
  61269.2 |########################################
  62200.3 |
  63131.4 |#############
  64062.5 |#############
  64993.6 |
  65924.7 |
  66855.8 |#############
  67786.9 |#############
  68718.0 |#############
  69649.1 |
  70580.2 |
  71511.3 |
  72442.4 |
  73373.5 |
  (1 below, 2 above range)

kernel_nway (n=18, range 60520.5-101559.3 ns)
  60520.5 |##########################
  62572.4 |
  64624.4 |##########################
  66676.3 |##########################
  68728.3 |########################################
  70780.2 |
  72832.1 |########################################
  74884.1 |#############
  76936.0 |#############
  78988.0 |##########################
  81039.9 |
  83091.8 |
  85143.8 |
  87195.7 |
  89247.6 |
  91299.6 |
  93351.5 |
  95403.5 |
  97455.4 |
  99507.3 |
  (1 below, 1 above range)

kernel_seq (n=18, range 51010.3-76575.8 ns)
  51010.3 |################
  52288.6 |########
  53566.8 |########
  54845.1 |
  56123.4 |
  57401.7 |########################################
  58679.9 |################
  59958.2 |########
  61236.5 |
  62514.8 |
  63793.0 |
  65071.3 |
  66349.6 |
  67627.9 |
  68906.1 |
  70184.4 |
  71462.7 |
  72741.0 |########
  74019.2 |########
  75297.5 |
  (1 below, 3 above range)

nway (n=18, range 86782.9-140656.8 ns)
  86782.9 |####################
  89476.6 |####################
  92170.3 |########################################
  94864.0 |####################
  97557.7 |
  100251.4 |####################
  102945.1 |
  105638.8 |########################################
  108332.4 |####################
  111026.1 |####################
  113719.8 |
  116413.5 |
  119107.2 |
  121800.9 |####################
  124494.6 |
  127188.3 |####################
  129882.0 |
  132575.7 |####################
  135269.4 |
  137963.1 |####################
  (2 below, 2 above range)

sequential (n=18, range 72304.3-137360.7 ns)
  72304.3 |########################################
  75557.1 |####################
  78809.9 |#############
  82062.7 |
  85315.5 |
  88568.4 |
  91821.2 |
  95074.0 |
  98326.8 |
  101579.6 |
  104832.5 |
  108085.3 |
  111338.1 |
  114590.9 |
  117843.7 |
  121096.6 |
  124349.4 |
  127602.2 |
  130855.0 |
  134107.8 |####################
  (1 below, 3 above range)

std_parallel (n=18, range 98898.0-141923.9 ns)
  98898.0 |####################
  101049.3 |####################
  103200.6 |####################
  105351.9 |
  107503.2 |
  109654.5 |
  111805.8 |
  113957.1 |
  116108.4 |########################################
  118259.7 |
  120411.0 |
  122562.2 |########################################
  124713.5 |########################################
  126864.8 |########################################
  129016.1 |####################
  131167.4 |########################################
  133318.7 |
  135470.0 |
  137621.3 |
  139772.6 |####################
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=617.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=710.4% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=44.2% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=615.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=667.7% of algo (FFI overhead may distort results)
- **nway**: CV=21.0% (high variance, measurements may be unstable)
- **nway**: bridge=430.0% of algo (FFI overhead may distort results)
- **sequential**: CV=30.8% (high variance, measurements may be unstable)
- **sequential**: bridge=519.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=400.8% of algo (FFI overhead may distort results)
