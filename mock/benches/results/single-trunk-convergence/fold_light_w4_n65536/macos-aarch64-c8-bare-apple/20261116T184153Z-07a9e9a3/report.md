# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 6.7x slower than the field

std_parallel (58.19 us) is 6.7x the fastest (8.75 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {kernel_seq, head_tail, head_tail_dyn, sequential, kernel_nway} vs {nway, std_parallel} (119% apart)

The field splits into a fast tier {kernel_seq, head_tail, head_tail_dyn, sequential, kernel_nway} and a slow tier {nway, std_parallel} with a 119% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.7x the fastest

Fastest kernel_seq (8.75 us) to slowest std_parallel (58.19 us): 6.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 8745.9 ns median (-5.6% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 6.65x (fastest 8745.9 ns, slowest 58191.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 4 |
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
| head_tail | 10014ns | 9825ns | 9015ns | 9729ns | 10942ns | base |
| head_tail_dyn | 13157ns | 13144ns | 12600ns | 13062ns | 13580ns | +31.39% |
| kernel_nway | 19013ns | 19181ns | 14769ns | 18801ns | 21453ns | +89.86% |
| kernel_seq | 9255ns | 9158ns | 8786ns | 9097ns | 9726ns | -7.58% |
| nway | 41380ns | 41194ns | 36007ns | 40578ns | 45268ns | +313.21% |
| sequential | 16075ns | 15967ns | 15660ns | 15905ns | 16538ns | +60.52% |
| std_parallel | 59452ns | 58608ns | 52794ns | 57300ns | 66010ns | +493.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9427ns | 8465ns | 10325ns | base | 6.952 |
| head_tail_dyn | 12610ns | 12129ns | 13006ns | +33.76% | 5.197 |
| kernel_nway | 18368ns | 14132ns | 20822ns | +94.85% | 3.568 |
| kernel_seq | 8841ns | 8453ns | 9246ns | -6.22% | 7.413 |
| nway | 40807ns | 35424ns | 44701ns | +332.87% | 1.606 |
| sequential | 15893ns | 15476ns | 16349ns | +68.59% | 4.124 |
| std_parallel | 59033ns | 52371ns | 65572ns | +526.21% | 1.110 |

## Performance model

- Peak throughput: **7.753 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.072 | 91.2% |
| head_tail_dyn | 5.215 | 67.3% |
| kernel_nway | 3.534 | 45.6% |
| kernel_seq | 7.493 | 96.6% |
| nway | 1.614 | 20.8% |
| sequential | 4.151 | 53.5% |
| std_parallel | 1.126 | 14.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 10014ns | 10014ns | base |
| head_tail_dyn | 13157ns | 13157ns | +31.39% |
| kernel_nway | 19013ns | 19013ns | +89.86% |
| kernel_seq | 9255ns | 9255ns | -7.58% |
| nway | 41380ns | 41380ns | +313.21% |
| sequential | 16075ns | 16075ns | +60.52% |
| std_parallel | 59452ns | 59452ns | +493.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9267ns | base | --- | [8952, 9856] | --- | --- | --- | --- |
| head_tail_dyn | 12567ns | +3381.3ns (+36.5%) | [+2669, +3725]ns | [12410, 12769] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 18546ns | +8608.4ns (+92.9%) | [+8044, +10650]ns | [17533, 19904] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 8746ns | -460.4ns (-5.0%) | [-981, -229]ns | [8581, 8979] | YES | 0.0075 | 0.0075 | 0 |
| nway | 40600ns | +31347.9ns (+338.3%) | [+29475, +32169]ns | [38827, 42204] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 15790ns | +6556.2ns (+70.8%) | [+6198, +6750]ns | [15667, 15894] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 58192ns | +49197.9ns (+530.9%) | [+45738, +51279]ns | [54079, 61117] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9802ns | +30.8% | +94.0% | -8.9% | +315.3% | +61.9% | +476.1% |
| 1 | 2 | 9096ns | +38.7% | +103.9% | -5.1% | +344.0% | +73.1% | +539.8% |
| 1 | 3 | 8994ns | +38.0% | +94.8% | -3.9% | +345.0% | +74.8% | +549.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.105 | ok |
| head_tail_dyn | -0.000 | ok |
| kernel_nway | -0.029 | ok |
| kernel_seq | -0.181 | ok |
| nway | -0.001 | ok |
| sequential | -0.095 | ok |
| std_parallel | -0.102 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 90663.0ns | 9427.1ns | 961.7% | HIGH |
| head_tail_dyn | 94280.8ns | 12609.9ns | 747.7% | HIGH |
| kernel_nway | 108471.1ns | 18368.3ns | 590.5% | HIGH |
| kernel_seq | 93502.1ns | 8841.0ns | 1057.6% | HIGH |
| nway | 111995.1ns | 40806.7ns | 274.5% | HIGH |
| sequential | 94296.3ns | 15892.8ns | 593.3% | HIGH |
| std_parallel | 117962.3ns | 59032.9ns | 199.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 8465.3-10325.0 ns)
   8465.3 |
   8558.3 |
   8651.2 |
   8744.2 |####################
   8837.2 |####################
   8930.2 |########################################
   9023.2 |########################################
   9116.2 |####################
   9209.2 |
   9302.1 |########################################
   9395.1 |
   9488.1 |####################
   9581.1 |####################
   9674.1 |
   9767.1 |
   9860.1 |
   9953.1 |
  10046.0 |####################
  10139.0 |
  10232.0 |####################
  (2 below, 3 above range)

head_tail_dyn (n=18, range 12129.2-13006.2 ns)
  12129.2 |
  12173.1 |
  12216.9 |####################
  12260.8 |
  12304.6 |####################
  12348.5 |####################
  12392.3 |
  12436.2 |########################################
  12480.0 |
  12523.9 |########################################
  12567.7 |########################################
  12611.6 |####################
  12655.4 |
  12699.3 |####################
  12743.1 |
  12787.0 |####################
  12830.8 |
  12874.7 |
  12918.5 |
  12962.4 |
  (2 below, 4 above range)

kernel_nway (n=18, range 14131.9-20821.5 ns)
  14131.9 |
  14466.4 |
  14800.9 |
  15135.4 |
  15469.9 |
  15804.3 |
  16138.8 |#############
  16473.3 |
  16807.8 |#############
  17142.3 |#############
  17476.7 |
  17811.2 |########################################
  18145.7 |#############
  18480.2 |
  18814.7 |##########################
  19149.1 |#############
  19483.6 |
  19818.1 |##########################
  20152.6 |#############
  20487.1 |
  (2 below, 3 above range)

kernel_seq (n=18, range 8452.8-9245.8 ns)
   8452.8 |
   8492.4 |####################
   8532.1 |########################################
   8571.7 |########################################
   8611.4 |
   8651.0 |####################
   8690.7 |####################
   8730.3 |
   8770.0 |####################
   8809.6 |
   8849.3 |
   8889.0 |########################################
   8928.6 |####################
   8968.3 |
   9007.9 |####################
   9047.6 |
   9087.2 |####################
   9126.9 |
   9166.5 |####################
   9206.2 |
  (2 below, 2 above range)

nway (n=18, range 35423.6-44701.4 ns)
  35423.6 |
  35887.5 |
  36351.4 |##########################
  36815.3 |
  37279.2 |
  37743.1 |##########################
  38207.0 |
  38670.9 |
  39134.7 |#############
  39598.6 |#############
  40062.5 |##########################
  40526.4 |#############
  40990.3 |#############
  41454.2 |
  41918.1 |########################################
  42382.0 |
  42845.8 |
  43309.7 |#############
  43773.6 |
  44237.5 |
  (1 below, 3 above range)

sequential (n=18, range 15476.4-16349.3 ns)
  15476.4 |
  15520.0 |
  15563.7 |####################
  15607.3 |##########
  15651.0 |##########
  15694.6 |##########
  15738.2 |##########
  15781.9 |########################################
  15825.5 |##########
  15869.2 |
  15912.8 |##########
  15956.5 |
  16000.1 |
  16043.8 |
  16087.4 |
  16131.1 |
  16174.7 |
  16218.4 |
  16262.0 |
  16305.7 |
  (2 below, 4 above range)

std_parallel (n=18, range 52370.8-65572.2 ns)
  52370.8 |##########
  53030.9 |##########
  53691.0 |##############################
  54351.0 |
  55011.1 |
  55671.2 |
  56331.2 |
  56991.3 |
  57651.4 |########################################
  58311.4 |####################
  58971.5 |##########
  59631.6 |
  60291.7 |
  60951.7 |
  61611.8 |
  62271.9 |##########
  62931.9 |
  63592.0 |
  64252.1 |
  64912.1 |##########
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=976.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=748.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=597.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1061.7% of algo (FFI overhead may distort results)
- **nway**: bridge=278.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=599.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=201.2% of algo (FFI overhead may distort results)
