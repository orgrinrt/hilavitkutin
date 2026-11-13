# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 18% faster than the next best (nway)

kernel_nway (156.54 us) leads nway (184.07 us) by 18%, a clear separation rather than a photo finish. CV 14.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn is an outlier: 2.2x slower than the field

head_tail_dyn (347.12 us) is 2.2x the fastest (156.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.66)

kernel_seq's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, nway, head_tail, sequential, std_parallel} vs {kernel_seq, head_tail_dyn} (50% apart)

The field splits into a fast tier {kernel_nway, nway, head_tail, sequential, std_parallel} and a slow tier {kernel_seq, head_tail_dyn} with a 50% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### kernel_seq is inconsistent: worst-20% is 1.7x its best-20%

kernel_seq's best 20% of batches run at 209.25 us but its worst 20% at 352.65 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 156542.8 ns median (-16.0% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 2.22x (fastest 156542.8 ns, slowest 347120.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 8 |
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
| head_tail | 183843ns | 186957ns | 138796ns | 174515ns | 220359ns | base |
| head_tail_dyn | 351250ns | 348328ns | 289599ns | 334797ns | 406754ns | +91.06% |
| kernel_nway | 151466ns | 157598ns | 115602ns | 148110ns | 174432ns | -17.61% |
| kernel_seq | 300135ns | 331933ns | 209690ns | 294240ns | 354201ns | +63.26% |
| nway | 187426ns | 184924ns | 165749ns | 183456ns | 204220ns | +1.95% |
| sequential | 215655ns | 209340ns | 172977ns | 195325ns | 267489ns | +17.30% |
| std_parallel | 219847ns | 222395ns | 181692ns | 209373ns | 254635ns | +19.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 183156ns | 138347ns | 219687ns | base | 5.725 |
| head_tail_dyn | 349760ns | 288632ns | 404662ns | +90.96% | 2.998 |
| kernel_nway | 150320ns | 114836ns | 172603ns | -17.93% | 6.976 |
| kernel_seq | 298899ns | 209254ns | 352648ns | +63.19% | 3.508 |
| nway | 186829ns | 165197ns | 203598ns | +2.01% | 5.612 |
| sequential | 214667ns | 172139ns | 266325ns | +17.20% | 4.885 |
| std_parallel | 218726ns | 180747ns | 253622ns | +19.42% | 4.794 |

## Performance model

- Peak throughput: **9.131 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 5.628 | 61.6% |
| head_tail_dyn | 3.021 | 33.1% |
| kernel_nway | 6.698 | 73.4% |
| kernel_seq | 3.170 | 34.7% |
| nway | 5.696 | 62.4% |
| sequential | 5.035 | 55.1% |
| std_parallel | 4.750 | 52.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 183843ns | 183843ns | base |
| head_tail_dyn | 351250ns | 351250ns | +91.06% |
| kernel_nway | 151466ns | 151466ns | -17.61% |
| kernel_seq | 300135ns | 300135ns | +63.26% |
| nway | 187426ns | 187426ns | +1.95% |
| sequential | 215655ns | 215655ns | +17.30% |
| std_parallel | 219847ns | 219847ns | +19.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 186310ns | base | --- | [153040, 211216] | --- | --- | --- | --- |
| head_tail_dyn | 347120ns | +165863.1ns (+89.0%) | [+149175, +180029]ns | [312576, 391830] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 156543ns | no significant difference | [-58265, +8697]ns | [135802, 165268] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 330802ns | +133445.1ns (+71.6%) | [+51352, +169048]ns | [226206, 344177] | YES | 0.0003 | 0.0001 | 0 |
| nway | 184075ns | no significant difference | [-34167, +43223]ns | [177774, 196710] | no | 0.8145 | 0.8145 | 0 |
| sequential | 208275ns | +26864.9ns (+14.4%) | [+22973, +39453]ns | [176381, 236297] | YES | 0.0003 | 0.0001 | 0 |
| std_parallel | 220732ns | +34285.0ns (+18.4%) | [+19436, +47279]ns | [191298, 234458] | YES | 0.0020 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 147356ns | +99.4% | +12.9% | +127.4% | +33.8% | +17.3% | +29.0% |
| 1 | 2 | 186310ns | +86.3% | -35.8% | +17.5% | +1.9% | +11.8% | +14.6% |
| 1 | 3 | 212675ns | +88.5% | -26.2% | +62.7% | -16.4% | +22.8% | +10.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.008 | ok |
| head_tail_dyn | -0.000 | ok |
| kernel_nway | -0.643 | HIGH- (thermal bounce) |
| kernel_seq | -0.663 | HIGH- (thermal bounce) |
| nway | -0.012 | ok |
| sequential | -0.009 | ok |
| std_parallel | -0.001 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 180084.9ns | 183155.5ns | 98.3% | HIGH |
| head_tail_dyn | 347744.9ns | 349760.2ns | 99.4% | HIGH |
| kernel_nway | 148149.7ns | 150319.8ns | 98.6% | HIGH |
| kernel_seq | 298510.0ns | 298899.4ns | 99.9% | HIGH |
| nway | 189429.8ns | 186829.1ns | 101.4% | HIGH |
| sequential | 220286.1ns | 214667.3ns | 102.6% | HIGH |
| std_parallel | 217694.0ns | 218726.2ns | 99.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 138347.4-219687.1 ns)
  138347.4 |
  142414.4 |#############
  146481.4 |##########################
  150548.4 |
  154615.4 |#############
  158682.3 |
  162749.3 |
  166816.3 |##########################
  170883.3 |
  174950.3 |
  179017.2 |#############
  183084.2 |
  187151.2 |##########################
  191218.2 |
  195285.2 |
  199352.1 |
  203419.1 |
  207486.1 |##########################
  211553.1 |########################################
  215620.1 |#############
  (2 below, 1 above range)

head_tail_dyn (n=18, range 288632.4-404662.2 ns)
  288632.4 |####################
  294433.9 |####################
  300235.4 |####################
  306036.8 |
  311838.3 |####################
  317639.8 |
  323441.3 |
  329242.8 |
  335044.3 |####################
  340845.8 |########################################
  346647.3 |####################
  352448.8 |########################################
  358250.3 |
  364051.8 |
  369853.3 |
  375654.8 |
  381456.3 |
  387257.8 |####################
  393059.3 |########################################
  398860.8 |
  (2 below, 3 above range)

kernel_nway (n=18, range 114836.3-172603.1 ns)
  114836.3 |##########
  117724.6 |
  120613.0 |##########
  123501.3 |
  126389.7 |##########
  129278.0 |
  132166.3 |
  135054.7 |
  137943.0 |
  140831.4 |##########
  143719.7 |##########
  146608.0 |
  149496.4 |
  152384.7 |
  155273.1 |########################################
  158161.4 |
  161049.7 |##########
  163938.1 |####################
  166826.4 |##########
  169714.8 |####################
  (2 below, 1 above range)

kernel_seq (n=18, range 209253.5-352647.7 ns)
  209253.5 |##########################
  216423.2 |#############
  223592.9 |##########################
  230762.7 |
  237932.4 |
  245102.1 |
  252271.8 |
  259441.5 |
  266611.2 |
  273780.9 |
  280950.6 |
  288120.3 |
  295290.0 |#############
  302459.7 |
  309629.4 |
  316799.1 |
  323968.9 |##########################
  331138.6 |########################################
  338308.3 |#############
  345478.0 |########################################
  (1 below, 2 above range)

nway (n=18, range 165196.7-203598.3 ns)
  165196.7 |
  167116.7 |
  169036.8 |
  170956.9 |##########
  172877.0 |####################
  174797.1 |
  176717.2 |
  178637.2 |
  180557.3 |########################################
  182477.4 |
  184397.5 |##########
  186317.6 |
  188237.7 |
  190157.7 |##########
  192077.8 |##########
  193997.9 |
  195918.0 |####################
  197838.1 |##########
  199758.2 |
  201678.2 |##########
  (2 below, 2 above range)

sequential (n=18, range 172139.3-266324.5 ns)
  172139.3 |########################################
  176848.6 |##########
  181557.9 |##########
  186267.1 |##########
  190976.4 |
  195685.6 |
  200394.9 |
  205104.1 |####################
  209813.4 |
  214522.7 |####################
  219231.9 |
  223941.2 |
  228650.4 |
  233359.7 |##########
  238069.0 |
  242778.2 |
  247487.5 |##########
  252196.7 |##########
  256906.0 |
  261615.3 |
  (1 below, 3 above range)

std_parallel (n=18, range 180747.1-253622.2 ns)
  180747.1 |#############
  184390.9 |
  188034.6 |########################################
  191678.4 |#############
  195322.1 |#############
  198965.9 |
  202609.6 |#############
  206253.4 |
  209897.1 |
  213540.9 |
  217184.7 |#############
  220828.4 |#############
  224472.2 |#############
  228115.9 |
  231759.7 |########################################
  235403.4 |
  239047.2 |
  242690.9 |
  246334.7 |
  249978.5 |#############
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=98.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=98.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=96.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.5% of algo (FFI overhead may distort results)
- **nway**: bridge=105.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=101.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=91.8% of algo (FFI overhead may distort results)
