# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 6.7x slower than the field

sequential (744.22 us) is 6.7x the fastest (110.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_nway, head_tail) are a dead heat (<1%)

kernel_nway (110.70 us) and head_tail (110.70 us) differ by 0.01%, inside the noise, even though the wider field spreads 572.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### nway shows alternating (throttle bounce) (autocorr -0.67)

nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, head_tail, kernel_seq, head_tail_dyn, std_parallel} vs {nway, sequential} (450% apart)

The field splits into a fast tier {kernel_nway, head_tail, kernel_seq, head_tail_dyn, std_parallel} and a slow tier {nway, sequential} with a 450% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.7x the fastest

Fastest kernel_nway (110.70 us) to slowest sequential (744.22 us): 6.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 110695.9 ns median (-0.0% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 6.72x (fastest 110695.9 ns, slowest 744222.9 ns)

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
| head_tail | 112769ns | 112433ns | 110938ns | 112106ns | 114681ns | base |
| head_tail_dyn | 115438ns | 112950ns | 111439ns | 112537ns | 121788ns | +2.37% |
| kernel_nway | 112506ns | 112188ns | 111006ns | 112002ns | 114011ns | -0.23% |
| kernel_seq | 113514ns | 113135ns | 111249ns | 112650ns | 115944ns | +0.66% |
| nway | 720996ns | 718144ns | 711522ns | 717799ns | 730529ns | +539.36% |
| sequential | 744837ns | 745735ns | 735589ns | 744670ns | 749712ns | +560.50% |
| std_parallel | 132364ns | 132204ns | 126243ns | 130638ns | 138015ns | +17.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 111299ns | 110112ns | 113015ns | base | 9.421 |
| head_tail_dyn | 113942ns | 110222ns | 119802ns | +2.37% | 9.203 |
| kernel_nway | 111123ns | 110057ns | 112535ns | -0.16% | 9.436 |
| kernel_seq | 111710ns | 109951ns | 114174ns | +0.37% | 9.387 |
| nway | 719490ns | 710414ns | 728931ns | +546.45% | 1.457 |
| sequential | 743244ns | 733918ns | 748042ns | +567.79% | 1.411 |
| std_parallel | 130779ns | 125353ns | 136103ns | +17.50% | 8.018 |

## Performance model

- Peak throughput: **9.537 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 9.472 | 99.3% |
| head_tail_dyn | 9.403 | 98.6% |
| kernel_nway | 9.473 | 99.3% |
| kernel_seq | 9.471 | 99.3% |
| nway | 1.464 | 15.3% |
| sequential | 1.409 | 14.8% |
| std_parallel | 8.050 | 84.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 112769ns | 112769ns | base |
| head_tail_dyn | 115438ns | 115438ns | +2.37% |
| kernel_nway | 112506ns | 112506ns | -0.23% |
| kernel_seq | 113514ns | 113514ns | +0.66% |
| nway | 720996ns | 720996ns | +539.36% |
| sequential | 744837ns | 744837ns | +560.50% |
| std_parallel | 132364ns | 132364ns | +17.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 110704ns | base | --- | [110319, 111723] | --- | --- | --- | --- |
| head_tail_dyn | 111512ns | no significant difference | [-154, +1198]ns | [110858, 112627] | no | 0.3568 | 0.2379 | 0 |
| kernel_nway | 110696ns | no significant difference | [-935, +381]ns | [110331, 111071] | no | 0.5768 | 0.4807 | 0 |
| kernel_seq | 110715ns | no significant difference | [-433, +708]ns | [110288, 112473] | no | 1.0000 | 1.0000 | 0 |
| nway | 716442ns | +606227.1ns (+547.6%) | [+602929, +610283]ns | [715217, 721029] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 744223ns | +633308.3ns (+572.1%) | [+629181, +634333]ns | [742019, 745519] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 130258ns | +18537.5ns (+16.7%) | [+16592, +21954]ns | [126608, 132917] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 110456ns | +0.4% | -0.0% | +0.9% | +547.9% | +574.4% | +15.4% |
| 1 | 2 | 111342ns | +0.8% | -0.5% | -0.6% | +545.4% | +568.7% | +18.5% |
| 1 | 3 | 110762ns | +1.1% | -0.0% | +0.2% | +546.1% | +571.4% | +17.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.589 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.305 | moderate- |
| kernel_nway | -0.287 | moderate- |
| kernel_seq | -0.446 | moderate- |
| nway | -0.667 | HIGH- (thermal bounce) |
| sequential | -0.055 | ok |
| std_parallel | -0.411 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 0/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 5709555.1ns | 111299.1ns | 5129.9% | HIGH |
| head_tail_dyn | 5757430.1ns | 113941.7ns | 5053.0% | HIGH |
| kernel_nway | 5720696.3ns | 111122.9ns | 5148.1% | HIGH |
| kernel_seq | 5730907.9ns | 111709.7ns | 5130.2% | HIGH |
| nway | 6336354.9ns | 719490.0ns | 880.7% | HIGH |
| sequential | 6322201.2ns | 743244.2ns | 850.6% | HIGH |
| std_parallel | 5745959.5ns | 130779.2ns | 4393.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 110112.5-113015.3 ns)
  110112.5 |#############
  110257.6 |########################################
  110402.8 |#############
  110547.9 |##########################
  110693.1 |########################################
  110838.2 |
  110983.3 |
  111128.5 |
  111273.6 |
  111418.7 |
  111563.9 |#############
  111709.0 |#############
  111854.2 |#############
  111999.3 |
  112144.4 |
  112289.6 |
  112434.7 |
  112579.9 |
  112725.0 |
  112870.1 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 110222.2-119802.1 ns)
  110222.2 |##############################
  110701.2 |##############################
  111180.2 |##########
  111659.2 |########################################
  112138.2 |
  112617.2 |
  113096.2 |##########
  113575.2 |##########
  114054.2 |##########
  114533.1 |
  115012.1 |
  115491.1 |
  115970.1 |
  116449.1 |
  116928.1 |##########
  117407.1 |
  117886.1 |
  118365.1 |
  118844.1 |
  119323.1 |
  (2 below, 1 above range)

kernel_nway (n=18, range 110057.0-112535.4 ns)
  110057.0 |#############
  110180.9 |##########################
  110304.8 |########################################
  110428.7 |#############
  110552.7 |
  110676.6 |########################################
  110800.5 |#############
  110924.4 |#############
  111048.3 |#############
  111172.3 |
  111296.2 |
  111420.1 |#############
  111544.0 |
  111668.0 |
  111791.9 |#############
  111915.8 |
  112039.7 |
  112163.6 |#############
  112287.6 |
  112411.5 |
  (1 below, 1 above range)

kernel_seq (n=18, range 109951.4-114174.3 ns)
  109951.4 |#############
  110162.5 |########################################
  110373.7 |########################################
  110584.8 |
  110796.0 |#############
  111007.1 |
  111218.3 |
  111429.4 |##########################
  111640.6 |
  111851.7 |
  112062.9 |
  112274.0 |#############
  112485.1 |#############
  112696.3 |#############
  112907.4 |
  113118.6 |
  113329.7 |
  113540.9 |
  113752.0 |
  113963.2 |#############
  (2 below, 2 above range)

nway (n=18, range 710413.9-728930.5 ns)
  710413.9 |####################
  711339.7 |
  712265.5 |
  713191.4 |
  714117.2 |##########
  715043.0 |########################################
  715968.9 |
  716894.7 |##########
  717820.5 |
  718746.4 |
  719672.2 |####################
  720598.0 |##############################
  721523.9 |
  722449.7 |
  723375.5 |
  724301.4 |
  725227.2 |
  726153.0 |
  727078.9 |
  728004.7 |##########
  (2 below, 2 above range)

sequential (n=18, range 733918.1-748042.4 ns)
  733918.1 |##########
  734624.3 |
  735330.5 |
  736036.7 |
  736742.9 |##########
  737449.1 |
  738155.4 |##########
  738861.6 |
  739567.8 |
  740274.0 |
  740980.2 |##########
  741686.4 |
  742392.6 |##########
  743098.9 |##########
  743805.1 |##############################
  744511.3 |##########
  745217.5 |########################################
  745923.7 |##########
  746629.9 |
  747336.2 |
  (1 below, 2 above range)

std_parallel (n=18, range 125352.8-136102.8 ns)
  125352.8 |
  125890.3 |########################################
  126427.8 |########################################
  126965.3 |
  127502.8 |
  128040.3 |####################
  128577.8 |
  129115.3 |####################
  129652.8 |####################
  130190.3 |####################
  130727.8 |
  131265.3 |
  131802.8 |########################################
  132340.3 |####################
  132877.8 |####################
  133415.3 |
  133952.8 |
  134490.3 |
  135027.8 |
  135565.3 |########################################
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=5145.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=5126.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=5141.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=5147.1% of algo (FFI overhead may distort results)
- **nway**: bridge=882.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=848.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=4390.0% of algo (FFI overhead may distort results)
