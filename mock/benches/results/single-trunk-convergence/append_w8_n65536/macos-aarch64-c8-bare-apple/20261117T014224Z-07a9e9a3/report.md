# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 8.6x slower than the field

std_parallel (81.64 us) is 8.6x the fastest (9.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_seq, head_tail_dyn) are a dead heat (<1%)

kernel_seq (9.55 us) and head_tail_dyn (9.59 us) differ by 0.44%, inside the noise, even though the wider field spreads 755.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### nway shows alternating (throttle bounce) (autocorr -0.64)

nway's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 8.6x the fastest

Fastest kernel_seq (9.55 us) to slowest std_parallel (81.64 us): 8.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.6x its best-20%

kernel_nway's best 20% of batches run at 20.45 us but its worst 20% at 31.97 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 9545.8 ns median (-1.0% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 8.55x (fastest 9545.8 ns, slowest 81641.6 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 10368ns | 10394ns | 9950ns | 10284ns | 10704ns | base |
| head_tail_dyn | 10390ns | 10356ns | 10103ns | 10267ns | 10717ns | +0.21% |
| kernel_nway | 28082ns | 27738ns | 22194ns | 26911ns | 32784ns | +170.85% |
| kernel_seq | 10306ns | 10300ns | 9881ns | 10212ns | 10660ns | -0.60% |
| nway | 64795ns | 64531ns | 61975ns | 63958ns | 67462ns | +524.94% |
| sequential | 56563ns | 56315ns | 55208ns | 56123ns | 57900ns | +445.54% |
| std_parallel | 83319ns | 82202ns | 76497ns | 80883ns | 90383ns | +703.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9678ns | 9482ns | 9891ns | base | 6.771 |
| head_tail_dyn | 9596ns | 9488ns | 9698ns | -0.85% | 6.830 |
| kernel_nway | 26954ns | 20449ns | 31969ns | +178.50% | 2.431 |
| kernel_seq | 9649ns | 9482ns | 9898ns | -0.31% | 6.792 |
| nway | 63899ns | 61115ns | 66567ns | +560.23% | 1.026 |
| sequential | 56057ns | 54940ns | 57151ns | +479.21% | 1.169 |
| std_parallel | 82494ns | 75568ns | 89824ns | +752.36% | 0.794 |

## Performance model

- Peak throughput: **6.912 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.800 | 98.4% |
| head_tail_dyn | 6.836 | 98.9% |
| kernel_nway | 2.440 | 35.3% |
| kernel_seq | 6.865 | 99.3% |
| nway | 1.032 | 14.9% |
| sequential | 1.173 | 17.0% |
| std_parallel | 0.803 | 11.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 10368ns | 10368ns | base |
| head_tail_dyn | 10390ns | 10390ns | +0.21% |
| kernel_nway | 28082ns | 28082ns | +170.85% |
| kernel_seq | 10306ns | 10306ns | -0.60% |
| nway | 64795ns | 64795ns | +524.94% |
| sequential | 56563ns | 56563ns | +445.54% |
| std_parallel | 83319ns | 83319ns | +703.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9638ns | base | --- | [9560, 9729] | --- | --- | --- | --- |
| head_tail_dyn | 9588ns | no significant difference | [-119, +15]ns | [9529, 9635] | no | 0.1155 | 0.0963 | 0 |
| kernel_nway | 26854ns | +17329.2ns (+179.8%) | [+14208, +19904]ns | [23596, 29383] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 9546ns | no significant difference | [-119, +50]ns | [9512, 9610] | no | 0.4807 | 0.4807 | 0 |
| nway | 63500ns | +53766.7ns (+557.9%) | [+52800, +55175]ns | [62506, 64760] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 55888ns | +46264.6ns (+480.0%) | [+45810, +46788]ns | [55379, 56467] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 81642ns | +72072.9ns (+747.8%) | [+67929, +76573]ns | [77315, 86260] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9560ns | -0.3% | +185.2% | -0.0% | +553.9% | +492.8% | +802.3% |
| 1 | 2 | 9675ns | -0.7% | +188.4% | -1.3% | +569.4% | +469.0% | +686.7% |
| 1 | 3 | 9638ns | -0.3% | +178.6% | -1.1% | +553.7% | +481.0% | +749.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.376 | moderate- |
| head_tail_dyn | -0.117 | ok |
| kernel_nway | -0.567 | HIGH- (thermal bounce) |
| kernel_seq | -0.042 | ok |
| nway | -0.639 | HIGH- (thermal bounce) |
| sequential | -0.551 | HIGH- (thermal bounce) |
| std_parallel | -0.544 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 441353.0ns | 9678.2ns | 4560.3% | HIGH |
| head_tail_dyn | 440720.8ns | 9595.8ns | 4592.8% | HIGH |
| kernel_nway | 451747.2ns | 26954.2ns | 1676.0% | HIGH |
| kernel_seq | 440739.1ns | 9648.6ns | 4567.9% | HIGH |
| nway | 487535.0ns | 63898.8ns | 763.0% | HIGH |
| sequential | 468209.9ns | 56057.4ns | 835.2% | HIGH |
| std_parallel | 502921.1ns | 82493.5ns | 609.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 9482.0-9891.0 ns)
   9482.0 |####################
   9502.4 |
   9522.9 |####################
   9543.3 |########################################
   9563.8 |########################################
   9584.2 |
   9604.7 |
   9625.1 |########################################
   9645.6 |####################
   9666.0 |####################
   9686.5 |####################
   9706.9 |
   9727.4 |
   9747.8 |####################
   9768.3 |
   9788.7 |
   9809.2 |####################
   9829.6 |
   9850.1 |
   9870.5 |####################
  (2 below, 2 above range)

head_tail_dyn (n=18, range 9487.5-9697.9 ns)
   9487.5 |
   9498.0 |####################
   9508.5 |########################################
   9519.1 |####################
   9529.6 |####################
   9540.1 |####################
   9550.6 |
   9561.1 |####################
   9571.7 |
   9582.2 |########################################
   9592.7 |
   9603.2 |####################
   9613.8 |####################
   9624.3 |####################
   9634.8 |
   9645.3 |####################
   9655.8 |
   9666.4 |
   9676.9 |
   9687.4 |
  (1 below, 4 above range)

kernel_nway (n=18, range 20448.6-31968.8 ns)
  20448.6 |
  21024.6 |
  21600.6 |####################
  22176.7 |####################
  22752.7 |
  23328.7 |########################################
  23904.7 |
  24480.7 |
  25056.7 |
  25632.7 |
  26208.7 |########################################
  26784.7 |########################################
  27360.7 |########################################
  27936.7 |
  28512.7 |
  29088.7 |####################
  29664.7 |
  30240.7 |####################
  30816.8 |########################################
  31392.8 |
  (2 below, 2 above range)

kernel_seq (n=18, range 9481.9-9897.9 ns)
   9481.9 |##########################
   9502.7 |########################################
   9523.5 |########################################
   9544.3 |#############
   9565.1 |#############
   9585.9 |##########################
   9606.7 |#############
   9627.5 |#############
   9648.3 |#############
   9669.1 |
   9689.9 |
   9710.7 |
   9731.5 |
   9752.3 |
   9773.1 |
   9793.9 |
   9814.7 |
   9835.5 |
   9856.3 |
   9877.1 |
  (1 below, 2 above range)

nway (n=18, range 61115.3-66567.3 ns)
  61115.3 |
  61387.9 |#############
  61660.5 |#############
  61933.1 |
  62205.7 |#############
  62478.3 |########################################
  62750.9 |
  63023.5 |#############
  63296.1 |
  63568.7 |#############
  63841.3 |##########################
  64113.9 |
  64386.5 |
  64659.1 |#############
  64931.7 |
  65204.3 |##########################
  65476.9 |
  65749.5 |
  66022.1 |#############
  66294.7 |
  (2 below, 2 above range)

sequential (n=18, range 54940.3-57151.4 ns)
  54940.3 |####################
  55050.8 |########################################
  55161.4 |
  55271.9 |
  55382.5 |
  55493.0 |
  55603.6 |########################################
  55714.2 |####################
  55824.7 |########################################
  55935.3 |
  56045.8 |########################################
  56156.4 |
  56266.9 |
  56377.5 |####################
  56488.1 |####################
  56598.6 |####################
  56709.2 |
  56819.7 |####################
  56930.3 |
  57040.8 |
  (2 below, 2 above range)

std_parallel (n=18, range 75568.1-89823.6 ns)
  75568.1 |########################################
  76280.8 |####################
  76993.6 |########################################
  77706.4 |####################
  78419.2 |
  79131.9 |
  79844.7 |
  80557.5 |####################
  81270.3 |########################################
  81983.1 |####################
  82695.8 |####################
  83408.6 |
  84121.4 |
  84834.2 |
  85546.9 |####################
  86259.7 |####################
  86972.5 |
  87685.3 |####################
  88398.0 |
  89110.8 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=4578.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=4589.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1683.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=4618.1% of algo (FFI overhead may distort results)
- **nway**: bridge=768.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=835.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=615.1% of algo (FFI overhead may distort results)
