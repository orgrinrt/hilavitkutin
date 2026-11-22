# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 10% faster than the next best (std_parallel)

kernel_nway (82.04 us) leads std_parallel (90.28 us) by 10%, a clear separation rather than a photo finish. CV 4.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 27% (significant)

kernel_nway is -29.69 us (27%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 9.1x slower than the field

sequential (746.66 us) is 9.1x the fastest (82.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.67)

kernel_nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, std_parallel, kernel_seq, head_tail, head_tail_dyn} vs {nway, sequential} (255% apart)

The field splits into a fast tier {kernel_nway, std_parallel, kernel_seq, head_tail, head_tail_dyn} and a slow tier {nway, sequential} with a 255% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 9.1x the fastest

Fastest kernel_nway (82.04 us) to slowest sequential (746.66 us): 9.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 82041.6 ns median (-26.8% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 9.10x (fastest 82041.6 ns, slowest 746664.6 ns)

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
| head_tail | 116024ns | 113517ns | 111686ns | 113241ns | 122367ns | base |
| head_tail_dyn | 114831ns | 114502ns | 111560ns | 113664ns | 118218ns | -1.03% |
| kernel_nway | 85773ns | 84240ns | 82721ns | 84084ns | 89833ns | -26.07% |
| kernel_seq | 113775ns | 112140ns | 111206ns | 112125ns | 117533ns | -1.94% |
| nway | 404433ns | 403977ns | 393379ns | 401213ns | 414790ns | +248.58% |
| sequential | 747420ns | 748394ns | 738981ns | 745953ns | 753840ns | +544.19% |
| std_parallel | 93251ns | 93100ns | 87385ns | 91889ns | 98228ns | -19.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 114449ns | 110243ns | 120717ns | base | 9.162 |
| head_tail_dyn | 112972ns | 110315ns | 116083ns | -1.29% | 9.282 |
| kernel_nway | 83380ns | 80924ns | 86831ns | -27.15% | 12.576 |
| kernel_seq | 112186ns | 110274ns | 115540ns | -1.98% | 9.347 |
| nway | 401210ns | 390783ns | 411947ns | +250.56% | 2.614 |
| sequential | 745975ns | 737992ns | 752219ns | +551.80% | 1.406 |
| std_parallel | 91065ns | 85850ns | 95911ns | -20.43% | 11.515 |

## Performance model

- Peak throughput: **12.958 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 9.362 | 72.2% |
| head_tail_dyn | 9.301 | 71.8% |
| kernel_nway | 12.781 | 98.6% |
| kernel_seq | 9.475 | 73.1% |
| nway | 2.623 | 20.2% |
| sequential | 1.404 | 10.8% |
| std_parallel | 11.615 | 89.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 116024ns | 116024ns | base |
| head_tail_dyn | 114831ns | 114831ns | -1.03% |
| kernel_nway | 85773ns | 85773ns | -26.07% |
| kernel_seq | 113775ns | 113775ns | -1.94% |
| nway | 404433ns | 404433ns | +248.58% |
| sequential | 747420ns | 747420ns | +544.19% |
| std_parallel | 93251ns | 93251ns | -19.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 112008ns | base | --- | [110825, 115046] | --- | --- | --- | --- |
| head_tail_dyn | 112733ns | no significant difference | [-3325, +2258]ns | [110804, 113656] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 82042ns | -29691.7ns (-26.5%) | [-31704, -28519]ns | [81738, 83225] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 110669ns | no significant difference | [-2725, +69]ns | [110383, 111835] | no | 0.2855 | 0.2379 | 0 |
| nway | 399706ns | +286116.7ns (+255.4%) | [+281558, +295238]ns | [394871, 406615] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 746665ns | +632985.4ns (+565.1%) | [+628867, +636600]ns | [741873, 749152] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 90281ns | -22887.5ns (-20.4%) | [-25533, -19748]ns | [87942, 93052] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 112165ns | +1.2% | -27.0% | +2.2% | +252.1% | +563.4% | -21.0% |
| 1 | 2 | 113256ns | -1.3% | -27.0% | -2.5% | +252.6% | +559.3% | -20.1% |
| 1 | 3 | 111398ns | +1.7% | -26.5% | -0.8% | +271.1% | +570.1% | -16.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.554 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.664 | HIGH- (thermal bounce) |
| kernel_nway | -0.666 | HIGH- (thermal bounce) |
| kernel_seq | -0.179 | ok |
| nway | -0.057 | ok |
| sequential | -0.216 | moderate- |
| std_parallel | -0.001 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 5773317.1ns | 114448.6ns | 5044.5% | HIGH |
| head_tail_dyn | 5742471.7ns | 112972.0ns | 5083.1% | HIGH |
| kernel_nway | 5709922.2ns | 83379.9ns | 6848.1% | HIGH |
| kernel_seq | 5742851.4ns | 112185.6ns | 5119.1% | HIGH |
| nway | 6066201.2ns | 401210.2ns | 1512.0% | HIGH |
| sequential | 6334629.2ns | 745975.2ns | 849.2% | HIGH |
| std_parallel | 5776038.9ns | 91064.6ns | 6342.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 110243.1-120716.7 ns)
  110243.1 |########################################
  110766.7 |########################################
  111290.4 |#############
  111814.1 |
  112337.8 |#############
  112861.5 |#############
  113385.1 |#############
  113908.8 |#############
  114432.5 |
  114956.2 |
  115479.9 |#############
  116003.5 |#############
  116527.2 |
  117050.9 |#############
  117574.6 |
  118098.3 |
  118621.9 |
  119145.6 |
  119669.3 |
  120193.0 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 110315.3-116083.3 ns)
  110315.3 |##########################
  110603.7 |########################################
  110892.1 |#############
  111180.5 |#############
  111468.9 |
  111757.3 |
  112045.7 |
  112334.1 |#############
  112622.5 |#############
  112910.9 |##########################
  113199.3 |
  113487.7 |#############
  113776.1 |#############
  114064.5 |
  114352.9 |
  114641.3 |
  114929.7 |
  115218.1 |
  115506.5 |#############
  115794.9 |
  (1 below, 3 above range)

kernel_nway (n=18, range 80923.6-86831.2 ns)
  80923.6 |
  81219.0 |##########################
  81514.4 |########################################
  81809.7 |########################################
  82105.1 |##########################
  82400.5 |
  82695.9 |
  82991.3 |##########################
  83286.7 |
  83582.0 |
  83877.4 |#############
  84172.8 |
  84468.2 |
  84763.6 |
  85059.0 |#############
  85354.3 |
  85649.7 |
  85945.1 |
  86240.5 |
  86535.9 |#############
  (2 below, 1 above range)

kernel_seq (n=18, range 110273.6-115540.3 ns)
  110273.6 |########################################
  110536.9 |########################
  110800.3 |################
  111063.6 |
  111326.9 |########
  111590.3 |
  111853.6 |
  112116.9 |########
  112380.3 |
  112643.6 |
  112906.9 |
  113170.3 |
  113433.6 |
  113696.9 |
  113960.3 |
  114223.6 |
  114486.9 |
  114750.3 |
  115013.6 |
  115276.9 |
  (2 below, 4 above range)

nway (n=18, range 390783.3-411947.2 ns)
  390783.3 |####################
  391841.5 |########################################
  392899.7 |
  393957.9 |
  395016.1 |
  396074.3 |####################
  397132.5 |########################################
  398190.7 |####################
  399248.9 |########################################
  400307.1 |####################
  401365.3 |
  402423.5 |
  403481.7 |
  404539.8 |####################
  405598.0 |
  406656.2 |
  407714.4 |
  408772.6 |####################
  409830.8 |
  410889.0 |
  (2 below, 4 above range)

sequential (n=18, range 737991.7-752219.4 ns)
  737991.7 |####################
  738703.1 |####################
  739414.4 |####################
  740125.8 |
  740837.2 |####################
  741548.6 |
  742260.0 |####################
  742971.4 |####################
  743682.8 |####################
  744394.2 |
  745105.5 |
  745816.9 |
  746528.3 |########################################
  747239.7 |####################
  747951.1 |####################
  748662.5 |####################
  749373.9 |########################################
  750085.3 |
  750796.7 |####################
  751508.0 |
  (1 below, 2 above range)

std_parallel (n=18, range 85850.0-95911.1 ns)
  85850.0 |####################
  86353.1 |
  86856.1 |####################
  87359.2 |########################################
  87862.2 |####################
  88365.3 |
  88868.3 |
  89371.4 |########################################
  89874.4 |####################
  90377.5 |########################################
  90880.6 |
  91383.6 |
  91886.7 |####################
  92389.7 |####################
  92892.8 |
  93395.8 |####################
  93898.9 |
  94401.9 |
  94905.0 |####################
  95408.1 |####################
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=5113.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=5060.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=6922.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=5147.0% of algo (FFI overhead may distort results)
- **nway**: bridge=1507.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=846.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=6316.1% of algo (FFI overhead may distort results)
