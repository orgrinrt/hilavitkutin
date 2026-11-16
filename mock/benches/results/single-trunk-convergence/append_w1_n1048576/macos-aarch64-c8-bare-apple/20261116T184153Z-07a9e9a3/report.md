# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 6.7x slower than the field

sequential (760.02 us) is 6.7x the fastest (112.84 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail_dyn, kernel_nway) are a dead heat (<1%)

head_tail_dyn (112.84 us) and kernel_nway (112.97 us) differ by 0.12%, inside the noise, even though the wider field spreads 573.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail shows alternating (throttle bounce) (autocorr -0.53)

head_tail's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, kernel_nway, head_tail, kernel_seq, std_parallel} vs {nway, sequential} (459% apart)

The field splits into a fast tier {head_tail_dyn, kernel_nway, head_tail, kernel_seq, std_parallel} and a slow tier {nway, sequential} with a 459% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.7x the fastest

Fastest head_tail_dyn (112.84 us) to slowest sequential (760.02 us): 6.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: head_tail_dyn** at 112843.8 ns median (-0.5% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 6.74x (fastest 112843.8 ns, slowest 760018.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 1 |
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
| head_tail | 116001ns | 115056ns | 113208ns | 114821ns | 119168ns | base |
| head_tail_dyn | 114677ns | 114298ns | 113099ns | 114093ns | 116343ns | -1.14% |
| kernel_nway | 121517ns | 114531ns | 112790ns | 114338ns | 136650ns | +4.76% |
| kernel_seq | 119859ns | 115356ns | 112926ns | 114975ns | 130651ns | +3.33% |
| nway | 732558ns | 731833ns | 726165ns | 731264ns | 737694ns | +531.51% |
| sequential | 787150ns | 761756ns | 755514ns | 760541ns | 842883ns | +578.57% |
| std_parallel | 134721ns | 132281ns | 128744ns | 131541ns | 142480ns | +16.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 114334ns | 111740ns | 117293ns | base | 9.171 |
| head_tail_dyn | 113221ns | 111881ns | 114749ns | -0.97% | 9.261 |
| kernel_nway | 119617ns | 111597ns | 133998ns | +4.62% | 8.766 |
| kernel_seq | 118292ns | 111651ns | 128832ns | +3.46% | 8.864 |
| nway | 730587ns | 724621ns | 735702ns | +538.99% | 1.435 |
| sequential | 784943ns | 753476ns | 840301ns | +586.53% | 1.336 |
| std_parallel | 132990ns | 127025ns | 140632ns | +16.32% | 7.885 |

## Performance model

- Peak throughput: **9.396 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 9.248 | 98.4% |
| head_tail_dyn | 9.292 | 98.9% |
| kernel_nway | 9.281 | 98.8% |
| kernel_seq | 9.186 | 97.8% |
| nway | 1.436 | 15.3% |
| sequential | 1.380 | 14.7% |
| std_parallel | 8.026 | 85.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 116001ns | 116001ns | base |
| head_tail_dyn | 114677ns | 114677ns | -1.14% |
| kernel_nway | 121517ns | 121517ns | +4.76% |
| kernel_seq | 119859ns | 119859ns | +3.33% |
| nway | 732558ns | 732558ns | +531.51% |
| sequential | 787150ns | 787150ns | +578.57% |
| std_parallel | 134721ns | 134721ns | +16.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 113383ns | base | --- | [112973, 114021] | --- | --- | --- | --- |
| head_tail_dyn | 112844ns | no significant difference | [-1679, +650]ns | [112221, 113673] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 112975ns | no significant difference | [-1644, +1452]ns | [112010, 114538] | no | 0.9774 | 0.8145 | 0 |
| kernel_seq | 114144ns | no significant difference | [-1485, +2575]ns | [112579, 115710] | no | 0.7210 | 0.4807 | 0 |
| nway | 730017ns | +615754.2ns (+543.1%) | [+612662, +620338]ns | [727608, 733700] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 760019ns | +647056.2ns (+570.7%) | [+642221, +651046]ns | [756069, 763490] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 130646ns | +16681.3ns (+14.7%) | [+15179, +18942]ns | [128694, 132315] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 113894ns | -0.7% | -0.7% | +1.6% | +542.5% | +570.4% | +15.7% |
| 1 | 2 | 112979ns | -0.1% | -0.6% | +1.3% | +545.3% | +570.5% | +15.9% |
| 1 | 3 | 113475ns | -0.6% | +0.9% | -0.5% | +543.0% | +567.8% | +13.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.527 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.026 | ok |
| kernel_nway | -0.395 | moderate- |
| kernel_seq | -0.002 | ok |
| nway | -0.296 | moderate- |
| sequential | -0.186 | ok |
| std_parallel | -0.049 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 5867274.5ns | 114334.5ns | 5131.7% | HIGH |
| head_tail_dyn | 5867111.6ns | 113220.6ns | 5182.0% | HIGH |
| kernel_nway | 5935956.0ns | 119616.9ns | 4962.5% | HIGH |
| kernel_seq | 5878185.7ns | 118292.1ns | 4969.2% | HIGH |
| nway | 6480595.1ns | 730586.8ns | 887.0% | HIGH |
| sequential | 6549029.2ns | 784943.3ns | 834.3% | HIGH |
| std_parallel | 5892900.2ns | 132989.6ns | 4431.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 111740.3-117293.1 ns)
  111740.3 |
  112017.9 |##########
  112295.6 |
  112573.2 |
  112850.9 |########################################
  113128.5 |####################
  113406.1 |####################
  113683.8 |##########
  113961.4 |####################
  114239.0 |##########
  114516.7 |
  114794.3 |##########
  115071.9 |
  115349.6 |
  115627.2 |
  115904.9 |
  116182.5 |
  116460.1 |
  116737.8 |
  117015.4 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 111880.6-114749.3 ns)
  111880.6 |####################
  112024.0 |####################
  112167.4 |########################################
  112310.9 |####################
  112454.3 |########################################
  112597.8 |
  112741.2 |
  112884.6 |
  113028.1 |####################
  113171.5 |####################
  113314.9 |####################
  113458.4 |####################
  113601.8 |
  113745.2 |####################
  113888.7 |####################
  114032.1 |
  114175.6 |####################
  114319.0 |
  114462.4 |
  114605.9 |
  (2 below, 2 above range)

kernel_nway (n=18, range 111597.2-133997.9 ns)
  111597.2 |########################################
  112717.3 |####################
  113837.3 |####################
  114957.3 |
  116077.4 |
  117197.4 |######
  118317.4 |
  119437.5 |
  120557.5 |
  121677.5 |
  122797.6 |
  123917.6 |
  125037.7 |
  126157.7 |
  127277.7 |
  128397.8 |
  129517.8 |
  130637.8 |
  131757.9 |
  132877.9 |
  (2 below, 3 above range)

kernel_seq (n=18, range 111651.4-128831.9 ns)
  111651.4 |########################################
  112510.4 |########################################
  113369.5 |##########################
  114228.5 |########################################
  115087.5 |#############
  115946.5 |##########################
  116805.6 |
  117664.6 |
  118523.6 |#############
  119382.6 |
  120241.7 |#############
  121100.7 |
  121959.7 |
  122818.7 |
  123677.8 |
  124536.8 |
  125395.8 |
  126254.9 |
  127113.9 |
  127972.9 |
  (1 below, 1 above range)

nway (n=18, range 724620.8-735702.1 ns)
  724620.8 |
  725174.9 |
  725729.0 |####################
  726283.0 |####################
  726837.1 |####################
  727391.1 |
  727945.2 |####################
  728499.3 |########################################
  729053.3 |####################
  729607.4 |
  730161.5 |####################
  730715.5 |####################
  731269.6 |
  731823.6 |
  732377.7 |####################
  732931.8 |
  733485.8 |####################
  734039.9 |####################
  734594.0 |########################################
  735148.0 |
  (2 below, 2 above range)

sequential (n=18, range 753476.4-840301.4 ns)
  753476.4 |########################################
  757817.6 |##########################
  762158.9 |####################
  766500.1 |
  770841.4 |######
  775182.6 |
  779523.9 |
  783865.1 |#############
  788206.4 |
  792547.6 |
  796888.9 |
  801230.1 |
  805571.4 |
  809912.6 |
  814253.9 |
  818595.1 |
  822936.4 |
  827277.6 |
  831618.9 |
  835960.1 |
  (1 below, 1 above range)

std_parallel (n=18, range 127025.0-140631.9 ns)
  127025.0 |#############
  127705.3 |#############
  128385.7 |########################################
  129066.0 |
  129746.4 |##########################
  130426.7 |#############
  131107.1 |#############
  131787.4 |########################################
  132467.8 |#############
  133148.1 |#############
  133828.5 |
  134508.8 |
  135189.2 |#############
  135869.5 |
  136549.9 |
  137230.2 |
  137910.5 |
  138590.9 |
  139271.2 |
  139951.6 |
  (2 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=5170.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=5190.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=5198.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=5139.4% of algo (FFI overhead may distort results)
- **nway**: bridge=886.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=855.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=4497.6% of algo (FFI overhead may distort results)
