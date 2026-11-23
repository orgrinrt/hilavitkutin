# append, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway beats baseline by 27% (significant)

kernel_nway is -2.53 ms (27%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 2.0x slower than the field

sequential (13.92 ms) is 2.0x the fastest (6.88 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.67)

kernel_nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, std_parallel, nway, head_tail, head_tail_dyn, kernel_seq} vs {sequential} (42% apart)

The field splits into a fast tier {kernel_nway, std_parallel, nway, head_tail, head_tail_dyn, kernel_seq} and a slow tier {sequential} with a 42% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: kernel_nway** at 6879763.2 ns median (-26.7% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.02x (fastest 6879763.2 ns, slowest 13922627.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 2 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112744008 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 9490996ns | 9394359ns | 9218620ns | 9350910ns | 9837313ns | base |
| head_tail_dyn | 8820579ns | 9514776ns | 7165749ns | 8734681ns | 9776839ns | -7.06% |
| kernel_nway | 6934745ns | 6884269ns | 6834429ns | 6871911ns | 7079154ns | -26.93% |
| kernel_seq | 9697591ns | 9824294ns | 9260439ns | 9652728ns | 9983464ns | +2.18% |
| nway | 8318531ns | 8258874ns | 8131643ns | 8222349ns | 8556248ns | -12.35% |
| sequential | 13940141ns | 13924216ns | 13883206ns | 13915128ns | 14006128ns | +46.88% |
| std_parallel | 7528520ns | 7541935ns | 7445474ns | 7522664ns | 7578828ns | -20.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9487001ns | 9213847ns | 9832316ns | base | 1.768 |
| head_tail_dyn | 8816610ns | 7162926ns | 9772168ns | -7.07% | 1.903 |
| kernel_nway | 6930195ns | 6828911ns | 7074545ns | -26.95% | 2.421 |
| kernel_seq | 9693694ns | 9257853ns | 9979510ns | +2.18% | 1.731 |
| nway | 8315770ns | 8129002ns | 8553583ns | -12.35% | 2.018 |
| sequential | 13938309ns | 13881450ns | 14004204ns | +46.92% | 1.204 |
| std_parallel | 7525179ns | 7442618ns | 7575458ns | -20.68% | 2.229 |

## Performance model

- Peak throughput: **2.457 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.786 | 72.7% |
| head_tail_dyn | 1.764 | 71.8% |
| kernel_nway | 2.439 | 99.3% |
| kernel_seq | 1.708 | 69.5% |
| nway | 2.032 | 82.7% |
| sequential | 1.205 | 49.0% |
| std_parallel | 2.225 | 90.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9490996ns | 9490996ns | base |
| head_tail_dyn | 8820579ns | 8820579ns | -7.06% |
| kernel_nway | 6934745ns | 6934745ns | -26.93% |
| kernel_seq | 9697591ns | 9697591ns | +2.18% |
| nway | 8318531ns | 8318531ns | -12.35% |
| sequential | 13940141ns | 13940141ns | +46.88% |
| std_parallel | 7528520ns | 7528520ns | -20.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9391286ns | base | --- | [9237401, 9832316] | --- | --- | --- | --- |
| head_tail_dyn | 9511608ns | no significant difference | [-2071345, +275728]ns | [7166056, 9772168] | no | 0.6875 | 0.6875 | 0 |
| kernel_nway | 6879763ns | -2526070.1ns (-26.9%) | [-2981493, -2162855]ns | [6836276, 7074545] | YES | 0.0469 | 0.0313 | 0 |
| kernel_seq | 9820374ns | no significant difference | [-317691, +604762]ns | [9281197, 9979510] | no | 0.6875 | 0.6875 | 0 |
| nway | 8256116ns | -1124850.5ns (-12.0%) | [-1449207, -939637]ns | [8137610, 8553583] | YES | 0.0469 | 0.0313 | 0 |
| sequential | 13922627ns | +4505165.7ns (+48.0%) | [+4116759, +4731999]ns | [13888096, 14004204] | YES | 0.0469 | 0.0313 | 0 |
| std_parallel | 7538841ns | -1890719.0ns (-20.1%) | [-2296187, -1698559]ns | [7461239, 7575458] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9598888ns | +1.8% | -28.6% | -3.3% | -14.0% | +45.3% | -21.1% |
| 1 | 2 | 9237401ns | -22.4% | -23.4% | +6.3% | -11.9% | +51.2% | -18.4% |
| 1 | 3 | 9624714ns | -1.2% | -28.7% | +3.7% | -11.1% | +44.5% | -22.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.664 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.661 | HIGH- (thermal bounce) |
| kernel_nway | -0.667 | HIGH- (thermal bounce) |
| kernel_seq | -0.060 | ok |
| nway | -0.346 | moderate- |
| sequential | -0.435 | moderate- |
| std_parallel | -0.027 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 72500233.2ns | 9487001.0ns | 764.2% | HIGH |
| head_tail_dyn | 71090322.1ns | 8816610.4ns | 806.3% | HIGH |
| kernel_nway | 69370839.4ns | 6930194.6ns | 1001.0% | HIGH |
| kernel_seq | 72502250.9ns | 9693693.9ns | 747.9% | HIGH |
| nway | 71158255.5ns | 8315769.6ns | 855.7% | HIGH |
| sequential | 75317210.7ns | 13938309.2ns | 540.4% | HIGH |
| std_parallel | 70741724.0ns | 7525179.3ns | 940.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 9213847.1-9832315.8 ns)
  9213847.1 |########################################
  9244770.5 |########################################
  9275694.0 |
  9306617.4 |
  9337540.8 |
  9368464.3 |########################################
  9399387.7 |########################################
  9430311.2 |
  9461234.6 |
  9492158.0 |
  9523081.5 |
  9554004.9 |
  9584928.3 |
  9615851.8 |
  9646775.2 |
  9677698.7 |
  9708622.1 |
  9739545.5 |
  9770469.0 |########################################
  9801392.4 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 7162926.3-9772167.9 ns)
  7162926.3 |########################################
  7293388.4 |
  7423850.5 |
  7554312.5 |
  7684774.6 |
  7815236.7 |
  7945698.8 |
  8076160.9 |
  8206622.9 |
  8337085.0 |
  8467547.1 |
  8598009.2 |
  8728471.3 |
  8858933.3 |
  8989395.4 |
  9119857.5 |
  9250319.6 |
  9380781.7 |####################
  9511243.7 |####################
  9641705.8 |####################
  (0 below, 1 above range)

kernel_nway (n=6, range 6828910.9-7074545.2 ns)
  6828910.9 |########################################
  6841192.6 |########################################
  6853474.3 |
  6865756.0 |########################################
  6878037.8 |########################################
  6890319.5 |
  6902601.2 |
  6914882.9 |
  6927164.6 |
  6939446.3 |
  6951728.0 |
  6964009.7 |
  6976291.5 |
  6988573.2 |
  7000854.9 |
  7013136.6 |
  7025418.3 |
  7037700.0 |
  7049981.7 |
  7062263.4 |########################################
  (0 below, 1 above range)

kernel_seq (n=6, range 9257852.9-9979510.2 ns)
  9257852.9 |####################
  9293935.8 |####################
  9330018.6 |
  9366101.5 |
  9402184.4 |
  9438267.2 |
  9474350.1 |
  9510433.0 |
  9546515.8 |
  9582598.7 |
  9618681.6 |
  9654764.4 |
  9690847.3 |
  9726930.1 |
  9763013.0 |
  9799095.9 |########################################
  9835178.7 |
  9871261.6 |
  9907344.5 |
  9943427.3 |####################
  (0 below, 1 above range)

nway (n=6, range 8129001.8-8553582.6 ns)
  8129001.8 |########################################
  8150230.8 |
  8171459.9 |
  8192688.9 |
  8213918.0 |####################
  8235147.0 |
  8256376.0 |
  8277605.1 |####################
  8298834.1 |
  8320063.2 |
  8341292.2 |
  8362521.2 |
  8383750.3 |
  8404979.3 |
  8426208.4 |
  8447437.4 |
  8468666.4 |
  8489895.5 |
  8511124.5 |
  8532353.6 |####################
  (0 below, 1 above range)

sequential (n=6, range 13881449.5-14004204.5 ns)
  13881449.5 |########################################
  13887587.2 |
  13893725.0 |########################################
  13899862.8 |
  13906000.5 |########################################
  13912138.2 |
  13918276.0 |
  13924413.8 |
  13930551.5 |########################################
  13936689.2 |
  13942827.0 |
  13948964.8 |
  13955102.5 |
  13961240.2 |
  13967378.0 |
  13973515.8 |
  13979653.5 |
  13985791.2 |
  13991929.0 |
  13998066.8 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 7442618.5-7575457.8 ns)
  7442618.5 |########################################
  7449260.5 |
  7455902.4 |
  7462544.4 |
  7469186.4 |
  7475828.3 |########################################
  7482470.3 |
  7489112.3 |
  7495754.2 |
  7502396.2 |
  7509038.2 |
  7515680.1 |
  7522322.1 |
  7528964.0 |########################################
  7535606.0 |
  7542248.0 |########################################
  7548889.9 |
  7555531.9 |########################################
  7562173.9 |
  7568815.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=770.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=750.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1004.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=738.2% of algo (FFI overhead may distort results)
- **nway**: bridge=861.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=540.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=938.4% of algo (FFI overhead may distort results)
