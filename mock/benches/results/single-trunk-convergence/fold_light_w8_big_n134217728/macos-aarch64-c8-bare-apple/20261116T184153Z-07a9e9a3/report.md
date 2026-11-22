# fold_light, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (head_tail, std_parallel) are a dead heat (<1%)

head_tail (12.80 ms) and std_parallel (12.86 ms) differ by 0.47%, inside the noise, even though the wider field spreads 52.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.67)

kernel_seq's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (12.80 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, std_parallel, nway, kernel_nway, sequential, kernel_seq} vs {head_tail_dyn} (27% apart)

The field splits into a fast tier {head_tail, std_parallel, nway, kernel_nway, sequential, kernel_seq} and a slow tier {head_tail_dyn} with a 27% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (head_tail) is the fastest** at 12803708.3 ns median
- 4 variants significantly slower than baseline
- Spread: 1.52x (fastest 12803708.3 ns, slowest 19460708.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 8 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 12643894ns | 12807349ns | 11586531ns | 12557215ns | 13302594ns | base |
| head_tail_dyn | 19389333ns | 19464552ns | 18207125ns | 19100694ns | 20413396ns | +53.35% |
| kernel_nway | 13718967ns | 13790620ns | 12529875ns | 13524344ns | 14605448ns | +8.50% |
| kernel_seq | 15715913ns | 15381484ns | 15101729ns | 15331181ns | 16600104ns | +24.30% |
| nway | 12994814ns | 12926177ns | 11962875ns | 12617410ns | 14076891ns | +2.78% |
| sequential | 15218894ns | 15344807ns | 14013417ns | 15104483ns | 15993250ns | +20.37% |
| std_parallel | 13201545ns | 12866984ns | 12381729ns | 12836448ns | 14159099ns | +4.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12640196ns | 11583521ns | 13298661ns | base | 10.618 |
| head_tail_dyn | 19385021ns | 18202646ns | 20408630ns | +53.36% | 6.924 |
| kernel_nway | 13714465ns | 12525219ns | 14600505ns | +8.50% | 9.787 |
| kernel_seq | 15712649ns | 15097646ns | 16596354ns | +24.31% | 8.542 |
| nway | 12990819ns | 11958562ns | 14073078ns | +2.77% | 10.332 |
| sequential | 15216155ns | 14010281ns | 15990453ns | +20.38% | 8.821 |
| std_parallel | 13198538ns | 12378677ns | 14156161ns | +4.42% | 10.169 |

## Performance model

- Peak throughput: **11.587 Gops/s** (head_tail; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 10.483 | 90.5% |
| head_tail_dyn | 6.897 | 59.5% |
| kernel_nway | 9.736 | 84.0% |
| kernel_seq | 8.727 | 75.3% |
| nway | 10.387 | 89.6% |
| sequential | 8.748 | 75.5% |
| std_parallel | 10.434 | 90.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 12643894ns | 12643894ns | base |
| head_tail_dyn | 19389333ns | 19389333ns | +53.35% |
| kernel_nway | 13718967ns | 13718967ns | +8.50% |
| kernel_seq | 15715913ns | 15715913ns | +24.30% |
| nway | 12994814ns | 12994814ns | +2.78% |
| sequential | 15218894ns | 15218894ns | +20.37% |
| std_parallel | 13201545ns | 13201545ns | +4.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 12803708ns | base | --- | [11818219, 13298661] | --- | --- | --- | --- |
| head_tail_dyn | 19460708ns | +6776765.7ns (+52.9%) | [+5552266, +7905443]ns | [18285724, 20408630] | YES | 0.0469 | 0.0313 | 0 |
| kernel_nway | 13786365ns | +1149140.6ns (+9.0%) | [+475370, +1598297]ns | [12756526, 14600505] | YES | 0.0469 | 0.0313 | 0 |
| kernel_seq | 15378807ns | +3056713.5ns (+23.9%) | [+2268000, +3892646]ns | [15162786, 16596354] | YES | 0.0469 | 0.0313 | 0 |
| nway | 12921812ns | no significant difference | [-826141, +1604068]ns | [11977568, 14073078] | no | 1.0000 | 1.0000 | 0 |
| sequential | 15342083ns | +2588760.4ns (+20.2%) | [+1662552, +3476563]ns | [14315927, 15990453] | YES | 0.0469 | 0.0313 | 0 |
| std_parallel | 12863859ns | no significant difference | [-524609, +1742990]ns | [12575594, 14156161] | no | 0.8250 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 12281156ns | +52.6% | +3.9% | +24.5% | +4.8% | +18.3% | +9.1% |
| 1 | 2 | 12703708ns | +60.7% | +9.0% | +30.6% | +8.0% | +19.1% | -0.8% |
| 1 | 3 | 12935724ns | +46.9% | +12.4% | +17.9% | -4.3% | +23.6% | +5.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.018 | ok |
| head_tail_dyn | -0.653 | HIGH- (thermal bounce) |
| kernel_nway | -0.012 | ok |
| kernel_seq | -0.666 | HIGH- (thermal bounce) |
| nway | -0.581 | HIGH- (thermal bounce) |
| sequential | -0.006 | ok |
| std_parallel | -0.641 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 11612767.4ns | 12640196.2ns | 91.9% | HIGH |
| head_tail_dyn | 19129772.6ns | 19385020.8ns | 98.7% | HIGH |
| kernel_nway | 12687816.0ns | 13714465.3ns | 92.5% | HIGH |
| kernel_seq | 14281583.3ns | 15712649.3ns | 90.9% | HIGH |
| nway | 13857404.5ns | 12990819.4ns | 106.7% | HIGH |
| sequential | 15388300.4ns | 15216154.5ns | 101.1% | HIGH |
| std_parallel | 13239046.8ns | 13198538.2ns | 100.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 11583520.8-13298661.4 ns)
  11583520.8 |########################################
  11669277.8 |
  11755034.9 |
  11840791.9 |
  11926548.9 |
  12012306.0 |########################################
  12098063.0 |
  12183820.0 |
  12269577.1 |
  12355334.1 |
  12441091.1 |
  12526848.2 |
  12612605.2 |########################################
  12698362.2 |
  12784119.3 |
  12869876.3 |
  12955633.3 |########################################
  13041390.4 |
  13127147.4 |
  13212904.4 |########################################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 18202645.8-20408630.2 ns)
  18202645.8 |########################################
  18312945.0 |########################################
  18423244.2 |
  18533543.5 |
  18643842.7 |
  18754141.9 |
  18864441.1 |
  18974740.3 |
  19085039.6 |########################################
  19195338.8 |
  19305638.0 |
  19415937.2 |
  19526236.4 |
  19636535.7 |
  19746834.9 |########################################
  19857134.1 |
  19967433.3 |
  20077732.5 |########################################
  20188031.8 |
  20298331.0 |
  (0 below, 1 above range)

kernel_nway (n=6, range 12525218.8-14600505.2 ns)
  12525218.8 |########################################
  12628983.1 |
  12732747.4 |
  12836511.8 |
  12940276.1 |########################################
  13044040.4 |
  13147804.7 |########################################
  13251569.0 |
  13355333.4 |
  13459097.7 |
  13562862.0 |
  13666626.3 |
  13770390.6 |
  13874155.0 |
  13977919.3 |
  14081683.6 |
  14185447.9 |
  14289212.2 |########################################
  14392976.6 |
  14496740.9 |########################################
  (0 below, 1 above range)

kernel_seq (n=6, range 15097645.8-16596354.1 ns)
  15097645.8 |########################################
  15172581.2 |########################################
  15247516.6 |########################################
  15322452.1 |
  15397387.5 |
  15472322.9 |########################################
  15547258.3 |
  15622193.7 |
  15697129.1 |
  15772064.6 |
  15847000.0 |
  15921935.4 |
  15996870.8 |
  16071806.2 |
  16146741.6 |########################################
  16221677.1 |
  16296612.5 |
  16371547.9 |
  16446483.3 |
  16521418.7 |
  (0 below, 1 above range)

nway (n=6, range 11958562.5-14073078.2 ns)
  11958562.5 |########################################
  12064288.3 |
  12170014.1 |
  12275739.8 |
  12381465.6 |
  12487191.4 |
  12592917.2 |
  12698643.0 |
  12804368.8 |####################
  12910094.5 |
  13015820.3 |####################
  13121546.1 |
  13227271.9 |
  13332997.7 |
  13438723.5 |
  13544449.2 |
  13650175.0 |####################
  13755900.8 |
  13861626.6 |
  13967352.4 |
  (0 below, 1 above range)

sequential (n=6, range 14010281.2-15990453.2 ns)
  14010281.2 |########################################
  14109289.8 |
  14208298.4 |
  14307307.0 |
  14406315.6 |
  14505324.2 |
  14604332.8 |########################################
  14703341.4 |
  14802350.0 |
  14901358.6 |
  15000367.2 |########################################
  15099375.8 |
  15198384.4 |
  15297393.0 |
  15396401.6 |
  15495410.2 |
  15594418.8 |########################################
  15693427.4 |
  15792436.0 |########################################
  15891444.6 |
  (0 below, 1 above range)

std_parallel (n=6, range 12378677.1-14156161.4 ns)
  12378677.1 |####################
  12467551.3 |
  12556425.5 |
  12645299.8 |
  12734174.0 |########################################
  12823048.2 |####################
  12911922.4 |
  13000796.6 |
  13089670.8 |
  13178545.1 |
  13267419.3 |
  13356293.5 |
  13445167.7 |
  13534041.9 |
  13622916.1 |
  13711790.4 |
  13800664.6 |####################
  13889538.8 |
  13978413.0 |
  14067287.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=90.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=98.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=92.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=91.5% of algo (FFI overhead may distort results)
- **nway**: bridge=102.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=103.0% of algo (FFI overhead may distort results)
