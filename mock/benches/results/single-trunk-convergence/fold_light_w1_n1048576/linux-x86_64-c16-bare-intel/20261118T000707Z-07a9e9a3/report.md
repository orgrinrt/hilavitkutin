# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 2.7x slower than the field

std_parallel (284.15 us) is 2.7x the fastest (106.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, head_tail_dyn, sequential, nway, head_tail, kernel_nway} vs {std_parallel} (129% apart)

The field splits into a fast tier {kernel_seq, head_tail_dyn, sequential, nway, head_tail, kernel_nway} and a slow tier {std_parallel} with a 129% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader kernel_seq vs stability leader sequential (+1% speed for 15.5x steadier)

kernel_seq is fastest (106.17 us, CV 47.8%); sequential gives up 1.4% median for 15.5x lower variance (CV 3.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### std_parallel is inconsistent: worst-20% is 2.8x its best-20%

std_parallel's best 20% of batches run at 208.11 us but its worst 20% at 575.15 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 106172.6 ns median (-7.6% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.68x (fastest 106172.6 ns, slowest 284147.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 1 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 131769ns | 115108ns | 104316ns | 111418ns | 176022ns | base |
| head_tail_dyn | 127667ns | 107562ns | 103954ns | 106330ns | 171529ns | -3.11% |
| kernel_nway | 148767ns | 124341ns | 103653ns | 120261ns | 214082ns | +12.90% |
| kernel_seq | 128946ns | 106556ns | 101748ns | 105380ns | 177894ns | -2.14% |
| nway | 149775ns | 111089ns | 108843ns | 110465ns | 229208ns | +13.67% |
| sequential | 108898ns | 107942ns | 104066ns | 107730ns | 113065ns | -17.36% |
| std_parallel | 364732ns | 285014ns | 208526ns | 273872ns | 579126ns | +176.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 131172ns | 104169ns | 174759ns | base | 7.994 |
| head_tail_dyn | 127056ns | 103703ns | 170199ns | -3.14% | 8.253 |
| kernel_nway | 148007ns | 103415ns | 212707ns | +12.83% | 7.085 |
| kernel_seq | 128385ns | 101547ns | 176726ns | -2.12% | 8.167 |
| nway | 149459ns | 108675ns | 228597ns | +13.94% | 7.016 |
| sequential | 108600ns | 103814ns | 112729ns | -17.21% | 9.655 |
| std_parallel | 362736ns | 208115ns | 575146ns | +176.54% | 2.891 |

## Performance model

- Peak throughput: **10.326 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 9.128 | 88.4% |
| head_tail_dyn | 9.776 | 94.7% |
| kernel_nway | 8.451 | 81.8% |
| kernel_seq | 9.876 | 95.6% |
| nway | 9.454 | 91.6% |
| sequential | 9.742 | 94.3% |
| std_parallel | 3.690 | 35.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 131769ns | 131769ns | base |
| head_tail_dyn | 127667ns | 127667ns | -3.11% |
| kernel_nway | 148767ns | 148767ns | +12.90% |
| kernel_seq | 128946ns | 128946ns | -2.14% |
| nway | 149775ns | 149775ns | +13.67% |
| sequential | 108898ns | 108898ns | -17.36% |
| std_parallel | 364732ns | 364732ns | +176.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 114875ns | base | --- | [105166, 129998] | --- | --- | --- | --- |
| head_tail_dyn | 107265ns | no significant difference | [-17291, +480]ns | [104509, 114366] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 124081ns | no significant difference | [-10615, +49273]ns | [107337, 180837] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 106173ns | no significant difference | [-21361, +995]ns | [103006, 113282] | no | 0.1444 | 0.0963 | 0 |
| nway | 110913ns | +5006.8ns (+4.4%) | [+34, +6057]ns | [109310, 137234] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| sequential | 107639ns | no significant difference | [-18963, +110]ns | [106733, 111040] | no | 0.1444 | 0.0963 | 0 |
| std_parallel | 284147ns | +143632.2ns (+125.0%) | [+109594, +355793]ns | [212406, 467886] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 106021ns | -1.0% | +96.2% | +0.9% | +4.7% | +4.9% | +114.2% |
| 1 | 2 | 122782ns | -11.6% | -10.0% | -16.6% | -10.2% | -12.1% | +271.3% |
| 1 | 3 | 105578ns | +5.2% | -1.1% | +2.2% | +3.7% | -0.1% | +223.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.666 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.007 | ok |
| kernel_nway | -0.138 | ok |
| kernel_seq | -0.651 | HIGH- (thermal bounce) |
| nway | -0.001 | ok |
| sequential | -0.003 | ok |
| std_parallel | -0.500 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 133269.7ns | 131171.5ns | 101.6% | HIGH |
| head_tail_dyn | 128448.9ns | 127056.0ns | 101.1% | HIGH |
| kernel_nway | 152089.3ns | 148007.1ns | 102.8% | HIGH |
| kernel_seq | 131070.2ns | 128385.3ns | 102.1% | HIGH |
| nway | 153628.2ns | 149458.6ns | 102.8% | HIGH |
| sequential | 110869.7ns | 108599.6ns | 102.1% | HIGH |
| std_parallel | 371089.2ns | 362735.6ns | 102.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 104168.8-174758.6 ns)
  104168.8 |########################################
  107698.3 |#####
  111227.8 |
  114757.2 |
  118286.7 |###########
  121816.2 |###########
  125345.7 |
  128875.2 |
  132404.7 |
  135934.2 |#####
  139463.7 |
  142993.2 |#####
  146522.7 |
  150052.2 |
  153581.7 |#####
  157111.2 |
  160640.7 |
  164170.2 |
  167699.6 |
  171229.1 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 103703.1-170198.9 ns)
  103703.1 |########################################
  107027.9 |###############
  110352.6 |#####
  113677.4 |
  117002.2 |#####
  120327.0 |
  123651.8 |
  126976.6 |
  130301.4 |
  133626.2 |
  136951.0 |
  140275.8 |
  143600.5 |
  146925.3 |
  150250.1 |
  153574.9 |#####
  156899.7 |
  160224.5 |
  163549.3 |#####
  166874.1 |
  (1 below, 2 above range)

kernel_nway (n=18, range 103415.3-212707.2 ns)
  103415.3 |########################################
  108879.9 |##############################
  114344.5 |
  119809.1 |
  125273.7 |
  130738.3 |##########
  136202.9 |##########
  141667.5 |
  147132.1 |##########
  152596.7 |
  158061.3 |
  163525.9 |
  168990.5 |##########
  174455.1 |
  179919.7 |
  185384.3 |
  190848.9 |
  196313.4 |
  201778.0 |##########
  207242.6 |##########
  (2 below, 3 above range)

kernel_seq (n=18, range 101546.7-176725.9 ns)
  101546.7 |########################################
  105305.6 |###############
  109064.6 |#####
  112823.6 |
  116582.5 |#####
  120341.5 |
  124100.4 |
  127859.4 |
  131618.4 |
  135377.3 |
  139136.3 |
  142895.3 |
  146654.2 |
  150413.2 |
  154172.1 |
  157931.1 |
  161690.1 |
  165449.0 |#####
  169208.0 |#####
  172967.0 |
  (1 below, 2 above range)

nway (n=18, range 108674.7-228597.1 ns)
  108674.7 |########################################
  114670.8 |
  120666.9 |
  126663.0 |
  132659.1 |###
  138655.3 |###
  144651.4 |###
  150647.5 |
  156643.6 |
  162639.8 |
  168635.9 |
  174632.0 |
  180628.1 |
  186624.2 |
  192620.4 |
  198616.5 |
  204612.6 |
  210608.7 |
  216604.8 |
  222601.0 |
  (1 below, 3 above range)

sequential (n=18, range 103813.7-112728.6 ns)
  103813.7 |#############
  104259.4 |#############
  104705.2 |
  105150.9 |
  105596.7 |
  106042.4 |
  106488.2 |########################################
  106933.9 |#############
  107379.6 |##########################
  107825.4 |##########################
  108271.1 |
  108716.9 |#############
  109162.6 |
  109608.4 |
  110054.1 |
  110499.9 |
  110945.6 |##########################
  111391.4 |
  111837.1 |
  112282.9 |#############
  (1 below, 3 above range)

std_parallel (n=18, range 208114.6-575145.6 ns)
  208114.6 |########################################
  226466.2 |########
  244817.7 |########
  263169.3 |########
  281520.8 |########
  299872.4 |
  318223.9 |
  336575.5 |########
  354927.0 |
  373278.6 |
  391630.1 |
  409981.7 |########
  428333.2 |########
  446684.8 |
  465036.3 |
  483387.9 |########
  501739.4 |
  520091.0 |
  538442.5 |########
  556794.1 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=32.7% (high variance, measurements may be unstable)
- **head_tail**: bridge=101.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=34.7% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=99.9% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=33.6% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=100.7% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=39.5% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=99.7% of algo (FFI overhead may distort results)
- **nway**: CV=51.1% (high variance, measurements may be unstable)
- **nway**: bridge=100.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=102.8% of algo (FFI overhead may distort results)
- **std_parallel**: CV=46.0% (high variance, measurements may be unstable)
- **std_parallel**: bridge=113.5% of algo (FFI overhead may distort results)
