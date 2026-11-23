# fold_heavy, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 3.1x slower than the field

std_parallel (590.95 us) is 3.1x the fastest (193.64 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_nway, head_tail) are a dead heat (<1%)

kernel_nway (193.64 us) and head_tail (193.74 us) differ by 0.05%, inside the noise, even though the wider field spreads 205.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### nway shows alternating (throttle bounce) (autocorr -0.67)

nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, head_tail, head_tail_dyn, sequential, kernel_seq, nway} vs {std_parallel} (176% apart)

The field splits into a fast tier {kernel_nway, head_tail, head_tail_dyn, sequential, kernel_seq, nway} and a slow tier {std_parallel} with a 176% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.1x the fastest

Fastest kernel_nway (193.64 us) to slowest std_parallel (590.95 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader kernel_nway vs stability leader sequential (+1% speed for 26.4x steadier)

kernel_nway is fastest (193.64 us, CV 14.5%); sequential gives up 0.9% median for 26.4x lower variance (CV 0.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: kernel_nway** at 193643.3 ns median (-0.0% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 3.05x (fastest 193643.3 ns, slowest 590946.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 1 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112765630 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 204911ns | 194191ns | 192519ns | 194048ns | 227402ns | base |
| head_tail_dyn | 210921ns | 195367ns | 192640ns | 196225ns | 242105ns | +2.93% |
| kernel_nway | 206206ns | 194166ns | 190671ns | 193710ns | 232718ns | +0.63% |
| kernel_seq | 213387ns | 196991ns | 192175ns | 196968ns | 248621ns | +4.14% |
| nway | 234112ns | 214667ns | 199424ns | 213200ns | 282825ns | +14.25% |
| sequential | 195014ns | 195541ns | 193298ns | 194978ns | 195926ns | -4.83% |
| std_parallel | 633085ns | 593975ns | 528590ns | 585173ns | 757199ns | +208.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 204012ns | 192176ns | 225521ns | base | 0.321 |
| head_tail_dyn | 210056ns | 192252ns | 240586ns | +2.96% | 0.312 |
| kernel_nway | 205297ns | 190455ns | 230789ns | +0.63% | 0.319 |
| kernel_seq | 212316ns | 191913ns | 246488ns | +4.07% | 0.309 |
| nway | 233642ns | 199261ns | 281779ns | +14.52% | 0.280 |
| sequential | 194840ns | 193129ns | 195751ns | -4.50% | 0.336 |
| std_parallel | 629918ns | 526603ns | 753350ns | +208.77% | 0.104 |

## Performance model

- Peak throughput: **0.344 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.338 | 98.3% |
| head_tail_dyn | 0.337 | 97.9% |
| kernel_nway | 0.338 | 98.4% |
| kernel_seq | 0.334 | 97.1% |
| nway | 0.306 | 88.8% |
| sequential | 0.335 | 97.5% |
| std_parallel | 0.111 | 32.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 204911ns | 204911ns | base |
| head_tail_dyn | 210921ns | 210921ns | +2.93% |
| kernel_nway | 206206ns | 206206ns | +0.63% |
| kernel_seq | 213387ns | 213387ns | +4.14% |
| nway | 234112ns | 234112ns | +14.25% |
| sequential | 195014ns | 195014ns | -4.83% |
| std_parallel | 633085ns | 633085ns | +208.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 193738ns | base | --- | [193027, 195866] | --- | --- | --- | --- |
| head_tail_dyn | 194630ns | no significant difference | [-279, +9006]ns | [193282, 215150] | no | 0.1444 | 0.0963 | 0 |
| kernel_nway | 193643ns | no significant difference | [-2337, +1400]ns | [192811, 194272] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 196214ns | +1476.2ns (+0.8%) | [+57, +15135]ns | [193699, 229033] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| nway | 214448ns | +9572.5ns (+4.9%) | [+7037, +48548]ns | [200536, 244550] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 195365ns | no significant difference | [-604, +1896]ns | [193814, 195518] | no | 0.5768 | 0.4807 | 0 |
| std_parallel | 590946ns | +397183.2ns (+205.0%) | [+361956, +445807]ns | [554775, 638533] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 193453ns | -0.1% | +0.2% | +13.2% | +4.4% | +1.0% | +185.6% |
| 1 | 2 | 194825ns | -0.5% | -0.7% | +0.1% | +25.7% | +0.4% | +212.5% |
| 1 | 3 | 194012ns | +10.9% | -0.2% | +1.0% | +3.1% | -0.4% | +229.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.557 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.150 | ok |
| kernel_nway | -0.125 | ok |
| kernel_seq | -0.187 | ok |
| nway | -0.666 | HIGH- (thermal bounce) |
| sequential | -0.200 | moderate- |
| std_parallel | -0.020 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 0/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 208401.6ns | 204011.7ns | 102.2% | HIGH |
| head_tail_dyn | 213239.0ns | 210055.6ns | 101.5% | HIGH |
| kernel_nway | 206254.3ns | 205296.9ns | 100.5% | HIGH |
| kernel_seq | 211300.8ns | 212315.5ns | 99.5% | HIGH |
| nway | 235946.0ns | 233642.1ns | 101.0% | HIGH |
| sequential | 195165.0ns | 194839.7ns | 100.2% | HIGH |
| std_parallel | 641782.0ns | 629917.7ns | 101.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 192176.5-225520.9 ns)
  192176.5 |########################################
  193843.7 |##########
  195510.9 |##########
  197178.1 |#####
  198845.4 |
  200512.6 |
  202179.8 |
  203847.0 |
  205514.2 |
  207181.5 |
  208848.7 |
  210515.9 |
  212183.1 |
  213850.3 |
  215517.6 |
  217184.8 |
  218852.0 |
  220519.2 |
  222186.4 |
  223853.7 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 192252.3-240586.2 ns)
  192252.3 |########################################
  194669.0 |##########
  197085.7 |
  199502.4 |
  201919.1 |
  204335.8 |
  206752.5 |#####
  209169.2 |#####
  211585.9 |
  214002.6 |
  216419.3 |
  218835.9 |#####
  221252.6 |
  223669.3 |
  226086.0 |
  228502.7 |
  230919.4 |#####
  233336.1 |
  235752.8 |
  238169.5 |
  (1 below, 3 above range)

kernel_nway (n=18, range 190455.1-230789.2 ns)
  190455.1 |#############
  192471.8 |########################################
  194488.5 |########
  196505.2 |
  198522.0 |
  200538.7 |
  202555.4 |
  204572.1 |
  206588.8 |
  208605.5 |
  210622.2 |
  212638.9 |
  214655.6 |
  216672.3 |
  218689.0 |
  220705.7 |
  222722.4 |
  224739.1 |
  226755.8 |
  228772.5 |
  (1 below, 3 above range)

kernel_seq (n=18, range 191912.8-246487.6 ns)
  191912.8 |########################################
  194641.6 |########################
  197370.3 |########
  200099.1 |
  202827.8 |
  205556.5 |########
  208285.3 |
  211014.0 |
  213742.8 |
  216471.5 |
  219200.2 |
  221929.0 |
  224657.7 |
  227386.5 |########
  230115.2 |########
  232843.9 |
  235572.7 |########
  238301.4 |########
  241030.2 |
  243758.9 |
  (2 below, 2 above range)

nway (n=18, range 199261.1-281778.9 ns)
  199261.1 |########################################
  203387.0 |######
  207512.8 |
  211638.7 |
  215764.6 |
  219890.5 |
  224016.4 |######
  228142.3 |
  232268.2 |
  236394.1 |
  240520.0 |####################
  244645.9 |#############
  248771.8 |
  252897.7 |
  257023.5 |
  261149.4 |
  265275.3 |
  269401.2 |
  273527.1 |
  277653.0 |######
  (2 below, 2 above range)

sequential (n=18, range 193129.0-195750.8 ns)
  193129.0 |##########
  193260.1 |####################
  193391.2 |
  193522.3 |##########
  193653.4 |
  193784.5 |
  193915.6 |
  194046.6 |##########
  194177.7 |
  194308.8 |
  194439.9 |
  194571.0 |
  194702.1 |
  194833.2 |
  194964.2 |
  195095.3 |
  195226.4 |####################
  195357.5 |########################################
  195488.6 |####################
  195619.7 |####################
  (1 below, 2 above range)

std_parallel (n=18, range 526603.2-753349.5 ns)
  526603.2 |####################
  537940.5 |########################################
  549277.8 |########################################
  560615.1 |####################
  571952.4 |####################
  583289.8 |####################
  594627.1 |####################
  605964.4 |
  617301.7 |####################
  628639.0 |########################################
  639976.3 |####################
  651313.7 |
  662651.0 |
  673988.3 |
  685325.6 |####################
  696662.9 |
  708000.3 |
  719337.6 |
  730674.9 |
  742012.2 |####################
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=100.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=99.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.7% of algo (FFI overhead may distort results)
- **nway**: bridge=101.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=103.4% of algo (FFI overhead may distort results)
