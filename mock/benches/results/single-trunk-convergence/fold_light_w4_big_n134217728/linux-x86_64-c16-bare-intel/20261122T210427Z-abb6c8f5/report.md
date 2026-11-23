# fold_light, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel dominates: 35% faster than the next best (head_tail)

std_parallel (15.73 ms) leads head_tail (21.25 ms) by 35%, a clear separation rather than a photo finish. CV 49.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway is an outlier: 2.9x slower than the field

kernel_nway (46.15 ms) is 2.9x the fastest (15.73 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.66)

nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### nway is inconsistent: worst-20% is 3.2x its best-20%

nway's best 20% of batches run at 15.10 ms but its worst 20% at 47.60 ms (3.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 15726981.0 ns median (-26.0% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 2.93x (fastest 15726981.0 ns, slowest 46145609.1 ns)

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
| Counter frequency | 2112786031 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 20868914ns | 21260771ns | 19626535ns | 20716848ns | 21718202ns | base |
| head_tail_dyn | 42813784ns | 28578045ns | 26674514ns | 27976761ns | 73138954ns | +105.16% |
| kernel_nway | 38962217ns | 46172616ns | 15080476ns | 41342749ns | 47332289ns | +86.70% |
| kernel_seq | 30843124ns | 30847787ns | 30752982ns | 30824120ns | 30916702ns | +47.79% |
| nway | 32531402ns | 34847604ns | 15104888ns | 28288275ns | 47609347ns | +55.88% |
| sequential | 28207778ns | 27895602ns | 26071038ns | 27324157ns | 30601580ns | +35.17% |
| std_parallel | 20499861ns | 15730158ns | 15586134ns | 15690923ns | 30170132ns | -1.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 20862742ns | 19622096ns | 21712355ns | base | 6.433 |
| head_tail_dyn | 42806068ns | 26672675ns | 73131911ns | +105.18% | 3.135 |
| kernel_nway | 38947606ns | 15077389ns | 47321188ns | +86.68% | 3.446 |
| kernel_seq | 30837434ns | 30752042ns | 30906680ns | +47.81% | 4.352 |
| nway | 32526532ns | 15102654ns | 47601055ns | +55.91% | 4.126 |
| sequential | 28206628ns | 26069736ns | 30600566ns | +35.20% | 4.758 |
| std_parallel | 20494913ns | 15575049ns | 30165158ns | -1.76% | 6.549 |

## Performance model

- Peak throughput: **8.902 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.316 | 70.9% |
| head_tail_dyn | 4.698 | 52.8% |
| kernel_nway | 2.909 | 32.7% |
| kernel_seq | 4.352 | 48.9% |
| nway | 3.852 | 43.3% |
| sequential | 4.812 | 54.1% |
| std_parallel | 8.534 | 95.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 20868914ns | 20868914ns | base |
| head_tail_dyn | 42813784ns | 42813784ns | +105.16% |
| kernel_nway | 38962217ns | 38962217ns | +86.70% |
| kernel_seq | 30843124ns | 30843124ns | +47.79% |
| nway | 32531402ns | 32531402ns | +55.88% |
| sequential | 28207778ns | 28207778ns | +35.17% |
| std_parallel | 20499861ns | 20499861ns | -1.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 21251647ns | base | --- | [19624225, 21712355] | --- | --- | --- | --- |
| head_tail_dyn | 28570287ns | +8946061.5ns (+42.1%) | [+5003651, +51880264]ns | [26716006, 73131911] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| kernel_nway | 46145609ns | +24903907.0ns (+117.2%) | [+2124373, +27226310]ns | [23376020, 47321188] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| kernel_seq | 30841995ns | +9573000.8ns (+45.0%) | [+9094597, +11256477]ns | [30763626, 30906680] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| nway | 34843550ns | no significant difference | [-4489234, +26177548]ns | [15134991, 47601055] | no | 0.6875 | 0.6875 | 0 |
| sequential | 27894485ns | +6182129.9ns (+29.1%) | [+4873185, +10976341]ns | [26124832, 30600566] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 15726981ns | no significant difference | [-5830908, +8624664]ns | [15592599, 30165158] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 21712355ns | +23.0% | +114.7% | +41.9% | +97.5% | +28.5% | -9.1% |
| 1 | 2 | 19624225ns | +45.6% | +138.7% | +57.4% | -22.9% | +55.9% | -19.9% |
| 1 | 3 | 21251647ns | +244.1% | +10.0% | +45.0% | +86.2% | +22.9% | +22.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.637 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.147 | ok |
| kernel_nway | -0.172 | ok |
| kernel_seq | -0.631 | HIGH- (thermal bounce) |
| nway | -0.659 | HIGH- (thermal bounce) |
| sequential | -0.564 | HIGH- (thermal bounce) |
| std_parallel | -0.423 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 22958865.3ns | 20862742.3ns | 110.0% | HIGH |
| head_tail_dyn | 37282354.2ns | 42806067.8ns | 87.1% | HIGH |
| kernel_nway | 33202107.1ns | 38947605.9ns | 85.2% | HIGH |
| kernel_seq | 30895618.5ns | 30837433.8ns | 100.2% | HIGH |
| nway | 29941692.7ns | 32526532.0ns | 92.1% | HIGH |
| sequential | 28163090.4ns | 28206627.8ns | 99.8% | HIGH |
| std_parallel | 20867650.2ns | 20494912.6ns | 101.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 19622096.1-21712354.9 ns)
  19622096.1 |########################################
  19726609.0 |
  19831122.0 |
  19935634.9 |
  20040147.9 |
  20144660.8 |
  20249173.8 |
  20353686.7 |
  20458199.6 |
  20562712.6 |
  20667225.5 |
  20771738.5 |
  20876251.4 |
  20980764.4 |
  21085277.3 |####################
  21189790.2 |
  21294303.2 |
  21398816.1 |####################
  21503329.1 |
  21607842.0 |####################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 26672674.8-73131911.2 ns)
  26672674.8 |########################################
  28995636.6 |#############
  31318598.4 |
  33641560.3 |#############
  35964522.1 |
  38287483.9 |
  40610445.7 |
  42933407.5 |
  45256369.3 |
  47579331.2 |
  49902293.0 |
  52225254.8 |
  54548216.6 |
  56871178.4 |
  59194140.2 |
  61517102.1 |
  63840063.9 |
  66163025.7 |
  68485987.5 |
  70808949.3 |
  (0 below, 1 above range)

kernel_nway (n=6, range 15077389.1-47321188.2 ns)
  15077389.1 |#############
  16689579.1 |
  18301769.0 |
  19913959.0 |
  21526148.9 |
  23138338.9 |
  24750528.8 |
  26362718.8 |
  27974908.7 |
  29587098.7 |
  31199288.6 |#############
  32811478.6 |
  34423668.6 |
  36035858.5 |
  37648048.5 |
  39260238.4 |
  40872428.4 |
  42484618.3 |
  44096808.3 |
  45708998.2 |########################################
  (0 below, 1 above range)

kernel_seq (n=6, range 30752041.9-30906680.2 ns)
  30752041.9 |########################################
  30759773.8 |
  30767505.7 |########################################
  30775237.6 |
  30782969.6 |
  30790701.5 |
  30798433.4 |
  30806165.3 |
  30813897.2 |
  30821629.1 |
  30829361.1 |
  30837093.0 |########################################
  30844824.9 |########################################
  30852556.8 |
  30860288.7 |
  30868020.6 |
  30875752.5 |
  30883484.5 |
  30891216.4 |########################################
  30898948.3 |
  (0 below, 1 above range)

nway (n=6, range 15102654.0-47601055.2 ns)
  15102654.0 |########################################
  16727574.1 |
  18352494.1 |
  19977414.2 |
  21602334.2 |
  23227254.3 |
  24852174.4 |
  26477094.4 |
  28102014.5 |
  29726934.6 |####################
  31351854.6 |
  32976774.7 |
  34601694.8 |
  36226614.8 |
  37851534.9 |####################
  39476454.9 |
  41101375.0 |
  42726295.1 |
  44351215.1 |
  45976135.2 |####################
  (0 below, 1 above range)

sequential (n=6, range 26069736.1-30600566.2 ns)
  26069736.1 |########################################
  26296277.6 |
  26522819.1 |
  26749360.6 |
  26975902.1 |
  27202443.6 |
  27428985.1 |####################
  27655526.6 |
  27882068.1 |
  28108609.6 |####################
  28335151.1 |
  28561692.7 |
  28788234.2 |
  29014775.7 |
  29241317.2 |
  29467858.7 |
  29694400.2 |
  29920941.7 |
  30147483.2 |
  30374024.7 |####################
  (0 below, 1 above range)

std_parallel (n=6, range 15575049.1-30165158.0 ns)
  15575049.1 |########################################
  16304554.5 |
  17034060.0 |
  17763565.4 |
  18493070.9 |
  19222576.3 |
  19952081.8 |
  20681587.2 |
  21411092.7 |
  22140598.1 |
  22870103.6 |
  23599609.0 |##########
  24329114.4 |
  25058619.9 |
  25788125.3 |
  26517630.8 |
  27247136.2 |
  27976641.7 |
  28706147.1 |
  29435652.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=99.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=71.1% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=102.7% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=30.9% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **kernel_nway**: bridge=82.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.3% of algo (FFI overhead may distort results)
- **nway**: CV=41.7% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **nway**: bridge=100.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.0% of algo (FFI overhead may distort results)
- **std_parallel**: CV=37.8% (high variance, measurements may be unstable)
- **std_parallel**: bridge=108.8% of algo (FFI overhead may distort results)
