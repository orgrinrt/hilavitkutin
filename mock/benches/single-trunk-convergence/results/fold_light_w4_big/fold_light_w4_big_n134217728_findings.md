# fold_light, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel beats baseline by 47% (significant)

std_parallel is -21.57 ms (47%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 3.7x slower than the field

sequential (88.58 ms) is 3.7x the fastest (23.69 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.66)

kernel_seq's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.7x the fastest

Fastest std_parallel (23.69 ms) to slowest sequential (88.58 ms): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: std_parallel** at 23685045.9 ns median (-48.1% vs baseline)
- 3 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 3.74x (fastest 23685045.9 ns, slowest 88576939.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 4 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 46124028ns | 45630602ns | 44368170ns | 45292582ns | 48249124ns | base |
| head_tail_dyn | 55477563ns | 56063360ns | 51760531ns | 54751104ns | 58425767ns | +20.28% |
| kernel_nway | 25721177ns | 25178799ns | 22606449ns | 24654114ns | 28879135ns | -44.23% |
| kernel_seq | 84966843ns | 83414378ns | 80749170ns | 82622378ns | 90592376ns | +84.21% |
| nway | 29342011ns | 29071675ns | 23594124ns | 28345613ns | 33710551ns | -36.38% |
| sequential | 90097928ns | 88587551ns | 81450502ns | 87005897ns | 99059688ns | +95.34% |
| std_parallel | 23962100ns | 23690574ns | 21832379ns | 23174118ns | 26208936ns | -48.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 46117857ns | 44360426ns | 48244390ns | base | 2.910 |
| head_tail_dyn | 55471212ns | 51755868ns | 58417848ns | +20.28% | 2.420 |
| kernel_nway | 25714010ns | 22601850ns | 28868442ns | -44.24% | 5.220 |
| kernel_seq | 84957442ns | 80744955ns | 90577995ns | +84.22% | 1.580 |
| nway | 29334691ns | 23589907ns | 33706825ns | -36.39% | 4.575 |
| sequential | 90091115ns | 81446856ns | 99053451ns | +95.35% | 1.490 |
| std_parallel | 23955302ns | 21829199ns | 26197057ns | -48.06% | 5.603 |

## Performance model

- Peak throughput: **6.149 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.942 | 47.8% |
| head_tail_dyn | 2.394 | 38.9% |
| kernel_nway | 5.332 | 86.7% |
| kernel_seq | 1.609 | 26.2% |
| nway | 4.619 | 75.1% |
| sequential | 1.515 | 24.6% |
| std_parallel | 5.667 | 92.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 46124028ns | 46124028ns | base |
| head_tail_dyn | 55477563ns | 55477563ns | +20.28% |
| kernel_nway | 25721177ns | 25721177ns | -44.23% |
| kernel_seq | 84966843ns | 84966843ns | +84.21% |
| nway | 29342011ns | 29342011ns | -36.38% |
| sequential | 90097928ns | 90097928ns | +95.34% |
| std_parallel | 23962100ns | 23962100ns | -48.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 45623032ns | base | --- | [44486149, 48244390] | --- | --- | --- | --- |
| head_tail_dyn | 56058923ns | +9639441.4ns (+21.1%) | [+5005387, +13415235]ns | [51936864, 58417848] | YES | 0.0313 | 0.0313 | 0 |
| kernel_nway | 25174172ns | -20911590.4ns (-45.8%) | [-23545361, -16754590]ns | [23099415, 28868442] | YES | 0.0313 | 0.0313 | 0 |
| kernel_seq | 83406779ns | +38459074.9ns (+84.3%) | [+33000709, +45058969]ns | [80887551, 90577995] | YES | 0.0313 | 0.0313 | 0 |
| nway | 29057528ns | -18632327.2ns (-40.8%) | [-20937847, -10779324]ns | [25239720, 33706825] | YES | 0.0313 | 0.0313 | 0 |
| sequential | 88576939ns | +41162559.2ns (+90.2%) | [+36739519, +54017696]ns | [82642956, 99053451] | YES | 0.0313 | 0.0313 | 0 |
| std_parallel | 23685046ns | -21565080.8ns (-47.3%) | [-25353433, -19569152]ns | [21983804, 26197057] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 45980580ns | +13.0% | -37.8% | +88.3% | -40.5% | +84.9% | -43.0% |
| 1 | 2 | 47886842ns | +18.1% | -46.8% | +68.9% | -43.7% | +88.8% | -51.7% |
| 1 | 3 | 44486149ns | +30.2% | -48.1% | +96.5% | -24.2% | +113.2% | -49.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.539 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.062 | ok |
| kernel_nway | -0.004 | ok |
| kernel_seq | -0.657 | HIGH- (thermal bounce) |
| nway | -0.198 | ok |
| sequential | -0.002 | ok |
| std_parallel | -0.092 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 46259409.3ns | 46117857.3ns | 100.3% | HIGH |
| head_tail_dyn | 55218849.3ns | 55471211.7ns | 99.5% | HIGH |
| kernel_nway | 25521195.2ns | 25714010.1ns | 99.3% | HIGH |
| kernel_seq | 85743046.8ns | 84957441.5ns | 100.9% | HIGH |
| nway | 29139745.5ns | 29334691.2ns | 99.3% | HIGH |
| sequential | 87373551.7ns | 90091115.3ns | 97.0% | HIGH |
| std_parallel | 26238973.6ns | 23955302.1ns | 109.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 44360426.3-48244390.0 ns)
  44360426.3 |####################
  44554624.5 |####################
  44748822.7 |
  44943020.9 |
  45137219.0 |
  45331417.2 |
  45525615.4 |########################################
  45719813.6 |
  45914011.8 |
  46108210.0 |
  46302408.1 |####################
  46496606.3 |
  46690804.5 |
  46885002.7 |
  47079200.9 |
  47273399.1 |
  47467597.3 |
  47661795.4 |
  47855993.6 |
  48050191.8 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 51755868.1-58417848.0 ns)
  51755868.1 |########################################
  52088967.1 |########################################
  52422066.1 |
  52755165.1 |
  53088264.1 |
  53421363.1 |
  53754462.1 |
  54087561.1 |
  54420660.1 |########################################
  54753759.1 |
  55086858.0 |
  55419957.0 |
  55753056.0 |
  56086155.0 |
  56419254.0 |
  56752353.0 |
  57085452.0 |########################################
  57418551.0 |
  57751650.0 |
  58084749.0 |########################################
  (0 below, 1 above range)

kernel_nway (n=6, range 22601850.1-28868442.4 ns)
  22601850.1 |########################################
  22915179.7 |
  23228509.3 |
  23541839.0 |########################################
  23855168.6 |
  24168498.2 |
  24481827.8 |
  24795157.4 |########################################
  25108487.0 |########################################
  25421816.7 |
  25735146.3 |########################################
  26048475.9 |
  26361805.5 |
  26675135.1 |
  26988464.7 |
  27301794.4 |
  27615124.0 |
  27928453.6 |
  28241783.2 |
  28555112.8 |
  (0 below, 1 above range)

kernel_seq (n=6, range 80744954.7-90577994.9 ns)
  80744954.7 |########################################
  81236606.7 |
  81728258.7 |####################
  82219910.7 |
  82711562.7 |
  83203214.8 |
  83694866.8 |
  84186518.8 |
  84678170.8 |####################
  85169822.8 |
  85661474.8 |
  86153126.8 |
  86644778.8 |
  87136430.8 |
  87628082.8 |####################
  88119734.9 |
  88611386.9 |
  89103038.9 |
  89594690.9 |
  90086342.9 |
  (0 below, 1 above range)

nway (n=6, range 23589907.3-33706825.2 ns)
  23589907.3 |########################################
  24095753.2 |
  24601599.1 |
  25107445.0 |
  25613290.9 |
  26119136.8 |
  26624982.7 |########################################
  27130828.6 |
  27636674.5 |########################################
  28142520.4 |
  28648366.3 |
  29154212.2 |
  29660058.1 |
  30165904.0 |########################################
  30671749.9 |
  31177595.8 |
  31683441.7 |
  32189287.6 |
  32695133.5 |########################################
  33200979.4 |
  (0 below, 1 above range)

sequential (n=6, range 81446855.6-99053450.8 ns)
  81446855.6 |####################
  82327185.4 |
  83207515.1 |####################
  84087844.9 |
  84968174.6 |
  85848504.4 |
  86728834.1 |
  87609163.9 |
  88489493.7 |########################################
  89369823.4 |
  90250153.2 |
  91130482.9 |
  92010812.7 |####################
  92891142.4 |
  93771472.2 |
  94651802.0 |
  95532131.7 |
  96412461.5 |
  97292791.2 |
  98173121.0 |
  (0 below, 1 above range)

std_parallel (n=6, range 21829199.0-26197056.5 ns)
  21829199.0 |########################################
  22047591.9 |########################################
  22265984.8 |
  22484377.6 |
  22702770.5 |
  22921163.4 |
  23139556.3 |########################################
  23357949.1 |
  23576342.0 |
  23794734.9 |
  24013127.8 |########################################
  24231520.7 |
  24449913.5 |
  24668306.4 |
  24886699.3 |########################################
  25105092.2 |
  25323485.0 |
  25541877.9 |
  25760270.8 |
  25978663.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=98.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.6% of algo (FFI overhead may distort results)
- **nway**: bridge=98.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=96.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=108.0% of algo (FFI overhead may distort results)
