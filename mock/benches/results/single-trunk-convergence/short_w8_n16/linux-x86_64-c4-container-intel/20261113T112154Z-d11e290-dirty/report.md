# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 173% faster than the next best (kernel_seq)

sequential (7.92 us) leads kernel_seq (21.63 us) by 173%, a clear separation rather than a photo finish. CV 17.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 71% (significant)

sequential is -22.04 us (71%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 23.8x slower than the field

std_parallel (188.50 us) is 23.8x the fastest (7.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.54)

nway's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 23.8x the fastest

Fastest sequential (7.92 us) to slowest std_parallel (188.50 us): 23.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 2.5x its best-20%

kernel_nway's best 20% of batches run at 32.97 us but its worst 20% at 83.99 us (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7923.7 ns median (-74.5% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 23.79x (fastest 7923.7 ns, slowest 188503.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 8 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100242360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 31426ns | 31672ns | 24847ns | 29648ns | 37382ns | base |
| head_tail_dyn | 29078ns | 29037ns | 21894ns | 27864ns | 34492ns | -7.47% |
| kernel_nway | 62991ns | 67059ns | 33432ns | 58471ns | 84552ns | +100.44% |
| kernel_seq | 22524ns | 22089ns | 19593ns | 21783ns | 25102ns | -28.33% |
| nway | 81234ns | 78118ns | 72208ns | 77352ns | 91569ns | +158.49% |
| sequential | 8721ns | 8183ns | 7218ns | 8037ns | 10498ns | -72.25% |
| std_parallel | 226924ns | 189250ns | 153013ns | 187011ns | 323749ns | +622.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 30898ns | 24488ns | 36754ns | base | 0.001 |
| head_tail_dyn | 28599ns | 21539ns | 33957ns | -7.44% | 0.001 |
| kernel_nway | 62500ns | 32974ns | 83989ns | +102.28% | 0.000 |
| kernel_seq | 22065ns | 19162ns | 24546ns | -28.59% | 0.001 |
| nway | 80826ns | 71840ns | 91125ns | +161.59% | 0.000 |
| sequential | 8429ns | 6997ns | 10179ns | -72.72% | 0.002 |
| std_parallel | 226196ns | 152399ns | 322854ns | +632.07% | 0.000 |

## Performance model

- Peak throughput: **0.002 Gops/s** (sequential; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.001 | 22.6% |
| head_tail_dyn | 0.001 | 24.5% |
| kernel_nway | 0.000 | 10.5% |
| kernel_seq | 0.001 | 32.3% |
| nway | 0.000 | 9.0% |
| sequential | 0.002 | 88.3% |
| std_parallel | 0.000 | 3.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 31426ns | 31426ns | base |
| head_tail_dyn | 29078ns | 29078ns | -7.47% |
| kernel_nway | 62991ns | 62991ns | +100.44% |
| kernel_seq | 22524ns | 22524ns | -28.33% |
| nway | 81234ns | 81234ns | +158.49% |
| sequential | 8721ns | 8721ns | -72.25% |
| std_parallel | 226924ns | 226924ns | +622.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 31023ns | base | --- | [26537, 32740] | --- | --- | --- | --- |
| head_tail_dyn | 28613ns | no significant difference | [-7757, +3260]ns | [25072, 31641] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 66638ns | +34647.4ns (+111.7%) | [+8356, +54880]ns | [41280, 79283] | YES | 0.0090 | 0.0075 | 0 |
| kernel_seq | 21632ns | -9042.0ns (-29.1%) | [-11099, -3405]ns | [20847, 23746] | YES | 0.0000 | 0.0000 | 0 |
| nway | 77667ns | +46439.1ns (+149.7%) | [+42541, +59262]ns | [75080, 84149] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7924ns | -22044.1ns (-71.1%) | [-24860, -18858]ns | [7404, 9671] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 188503ns | +156685.9ns (+505.1%) | [+134968, +223896]ns | [166403, 257497] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 35392ns | -37.2% | +2.1% | -44.7% | +115.2% | -72.3% | +627.6% |
| 1 | 2 | 31186ns | +10.1% | +113.7% | -25.8% | +180.6% | -77.5% | +423.3% |
| 1 | 3 | 25102ns | +11.4% | +232.5% | -9.4% | +223.1% | -69.1% | +613.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.007 | ok |
| head_tail_dyn | -0.517 | HIGH- (thermal bounce) |
| kernel_nway | -0.018 | ok |
| kernel_seq | -0.232 | moderate- |
| nway | -0.541 | HIGH- (thermal bounce) |
| sequential | -0.332 | moderate- |
| std_parallel | -0.265 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 33268.4ns | 30898.3ns | 107.7% | HIGH |
| head_tail_dyn | 29040.9ns | 28599.1ns | 101.5% | HIGH |
| kernel_nway | 63162.2ns | 62499.6ns | 101.1% | HIGH |
| kernel_seq | 22634.6ns | 22065.0ns | 102.6% | HIGH |
| nway | 86256.6ns | 80825.8ns | 106.7% | HIGH |
| sequential | 8870.7ns | 8429.0ns | 105.2% | HIGH |
| std_parallel | 199553.4ns | 226195.6ns | 88.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 24487.6-36753.9 ns)
  24487.6 |####################
  25100.9 |########################################
  25714.2 |
  26327.6 |
  26940.9 |####################
  27554.2 |####################
  28167.5 |
  28780.8 |
  29394.1 |####################
  30007.4 |
  30620.8 |####################
  31234.1 |########################################
  31847.4 |####################
  32460.7 |########################################
  33074.0 |####################
  33687.3 |
  34300.7 |
  34914.0 |
  35527.3 |
  36140.6 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 21539.4-33956.8 ns)
  21539.4 |#############
  22160.3 |#############
  22781.2 |
  23402.0 |
  24022.9 |
  24643.8 |#############
  25264.6 |#############
  25885.5 |
  26506.4 |
  27127.2 |##########################
  27748.1 |#############
  28369.0 |#############
  28989.9 |
  29610.7 |
  30231.6 |########################################
  30852.5 |
  31473.3 |
  32094.2 |#############
  32715.1 |
  33335.9 |
  (2 below, 4 above range)

kernel_nway (n=18, range 32973.8-83989.1 ns)
  32973.8 |########################################
  35524.6 |####################
  38075.4 |####################
  40626.1 |####################
  43176.9 |
  45727.7 |
  48278.4 |
  50829.2 |
  53379.9 |
  55930.7 |####################
  58481.5 |
  61032.2 |####################
  63583.0 |####################
  66133.8 |####################
  68684.5 |
  71235.3 |####################
  73786.0 |####################
  76336.8 |####################
  78887.6 |####################
  81438.3 |########################################
  (1 below, 2 above range)

kernel_seq (n=18, range 19162.4-24546.3 ns)
  19162.4 |########################################
  19431.6 |
  19700.8 |####################
  19970.0 |
  20239.2 |
  20508.4 |####################
  20777.6 |
  21046.8 |########################################
  21316.0 |####################
  21585.2 |########################################
  21854.4 |
  22123.6 |####################
  22392.8 |####################
  22662.0 |
  22931.2 |
  23200.3 |
  23469.5 |####################
  23738.7 |########################################
  24007.9 |####################
  24277.1 |####################
  (1 below, 1 above range)

nway (n=18, range 71840.0-91124.7 ns)
  71840.0 |#############
  72804.3 |##########################
  73768.5 |#############
  74732.7 |#############
  75697.0 |
  76661.2 |########################################
  77625.4 |#############
  78589.7 |#############
  79553.9 |
  80518.1 |#############
  81482.4 |
  82446.6 |
  83410.8 |#############
  84375.1 |#############
  85339.3 |
  86303.5 |
  87267.8 |#############
  88232.0 |
  89196.2 |
  90160.5 |
  (1 below, 3 above range)

sequential (n=18, range 6997.0-10178.8 ns)
   6997.0 |##############################
   7156.1 |
   7315.2 |
   7474.3 |
   7633.4 |########################################
   7792.5 |
   7951.6 |##########
   8110.7 |##########
   8269.7 |##########
   8428.8 |
   8587.9 |
   8747.0 |
   8906.1 |
   9065.2 |
   9224.3 |
   9383.4 |
   9542.4 |####################
   9701.5 |####################
   9860.6 |
  10019.7 |
  (2 below, 2 above range)

std_parallel (n=18, range 152398.7-322854.4 ns)
  152398.7 |########################################
  160921.5 |########################################
  169444.2 |#############
  177967.0 |
  186489.8 |##########################
  195012.6 |
  203535.4 |
  212058.2 |
  220581.0 |
  229103.8 |##########################
  237626.5 |
  246149.3 |
  254672.1 |##########################
  263194.9 |
  271717.7 |##########################
  280240.5 |
  288763.3 |
  297286.1 |
  305808.9 |
  314331.6 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=100.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.3% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=32.2% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=104.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=103.3% of algo (FFI overhead may distort results)
- **nway**: bridge=107.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=109.9% of algo (FFI overhead may distort results)
- **std_parallel**: CV=37.7% (high variance, measurements may be unstable)
- **std_parallel**: bridge=106.8% of algo (FFI overhead may distort results)
