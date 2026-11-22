# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 10% faster than the next best (kernel_nway)

kernel_seq (10.23 us) leads kernel_nway (11.26 us) by 10%, a clear separation rather than a photo finish. CV 12.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 4.7x slower than the field

std_parallel (48.41 us) is 4.7x the fastest (10.23 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, kernel_nway, head_tail, head_tail_dyn, sequential} vs {nway, std_parallel} (123% apart)

The field splits into a fast tier {kernel_seq, kernel_nway, head_tail, head_tail_dyn, sequential} and a slow tier {nway, std_parallel} with a 123% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.7x the fastest

Fastest kernel_seq (10.23 us) to slowest std_parallel (48.41 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.5x its best-20%

sequential's best 20% of batches run at 6.69 us but its worst 20% at 23.67 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 10231.9 ns median (-12.0% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.73x (fastest 10231.9 ns, slowest 48413.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 2 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 12765ns | 12351ns | 11200ns | 12095ns | 14552ns | base |
| head_tail_dyn | 13292ns | 14060ns | 10014ns | 13111ns | 15203ns | +4.13% |
| kernel_nway | 12456ns | 11934ns | 10565ns | 11697ns | 14540ns | -2.42% |
| kernel_seq | 11584ns | 11108ns | 10127ns | 10916ns | 13314ns | -9.25% |
| nway | 38064ns | 37964ns | 32182ns | 36428ns | 43460ns | +198.19% |
| sequential | 15358ns | 17109ns | 6822ns | 12337ns | 24159ns | +20.32% |
| std_parallel | 48788ns | 48908ns | 39730ns | 47792ns | 54812ns | +282.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12037ns | 10603ns | 13656ns | base | 0.005 |
| head_tail_dyn | 12485ns | 9389ns | 14270ns | +3.72% | 0.005 |
| kernel_nway | 11759ns | 10052ns | 13629ns | -2.31% | 0.005 |
| kernel_seq | 10812ns | 9525ns | 12348ns | -10.18% | 0.006 |
| nway | 37510ns | 31746ns | 42793ns | +211.63% | 0.002 |
| sequential | 15055ns | 6691ns | 23669ns | +25.08% | 0.004 |
| std_parallel | 48263ns | 39379ns | 54140ns | +300.97% | 0.001 |

## Performance model

- Peak throughput: **0.010 Gops/s** (sequential; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.006 | 57.6% |
| head_tail_dyn | 0.005 | 50.5% |
| kernel_nway | 0.006 | 59.4% |
| kernel_seq | 0.006 | 65.4% |
| nway | 0.002 | 17.9% |
| sequential | 0.004 | 39.9% |
| std_parallel | 0.001 | 13.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 12765ns | 12765ns | base |
| head_tail_dyn | 13292ns | 13292ns | +4.13% |
| kernel_nway | 12456ns | 12456ns | -2.42% |
| kernel_seq | 11584ns | 11584ns | -9.25% |
| nway | 38064ns | 38064ns | +198.19% |
| sequential | 15358ns | 15358ns | +20.32% |
| std_parallel | 48788ns | 48788ns | +282.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 11625ns | base | --- | [11195, 12700] | --- | --- | --- | --- |
| head_tail_dyn | 13238ns | no significant difference | [-1308, +2068]ns | [10944, 13863] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 11265ns | -465.9ns (-4.0%) | [-697, -166]ns | [10777, 12200] | YES | 0.0113 | 0.0075 | 0 |
| kernel_seq | 10232ns | -1263.4ns (-10.9%) | [-1457, -1087]ns | [9933, 11680] | YES | 0.0026 | 0.0013 | 0 |
| nway | 37423ns | +24458.7ns (+210.4%) | [+23634, +26691]ns | [34797, 39350] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 16787ns | no significant difference | [-3933, +8782]ns | [6951, 21975] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 48414ns | +36091.7ns (+310.5%) | [+34181, +39212]ns | [45684, 53483] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 13418ns | +3.1% | -4.5% | -13.0% | +190.4% | +30.3% | +255.1% |
| 1 | 2 | 11560ns | -14.8% | -1.8% | -10.4% | +225.8% | +14.7% | +363.9% |
| 1 | 3 | 11195ns | +20.8% | -4.1% | -11.3% | +190.7% | +19.7% | +331.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.087 | ok |
| head_tail_dyn | -0.663 | HIGH- (thermal bounce) |
| kernel_nway | -0.035 | ok |
| kernel_seq | -0.054 | ok |
| nway | -0.070 | ok |
| sequential | -0.184 | ok |
| std_parallel | -0.660 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 12465.5ns | 12036.6ns | 103.6% | HIGH |
| head_tail_dyn | 13058.3ns | 12484.7ns | 104.6% | HIGH |
| kernel_nway | 12082.6ns | 11758.9ns | 102.8% | HIGH |
| kernel_seq | 11443.2ns | 10811.7ns | 105.8% | HIGH |
| nway | 38002.0ns | 37509.6ns | 101.3% | HIGH |
| sequential | 15400.5ns | 15054.9ns | 102.3% | HIGH |
| std_parallel | 45559.6ns | 48263.0ns | 94.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 10603.2-13655.9 ns)
  10603.2 |#############
  10755.8 |#############
  10908.4 |
  11061.1 |#############
  11213.7 |########################################
  11366.4 |
  11519.0 |#############
  11671.6 |#############
  11824.3 |##########################
  11976.9 |#############
  12129.5 |
  12282.2 |
  12434.8 |
  12587.4 |
  12740.1 |
  12892.7 |
  13045.4 |
  13198.0 |#############
  13350.6 |
  13503.3 |##########################
  (2 below, 2 above range)

head_tail_dyn (n=18, range 9389.0-14270.3 ns)
   9389.0 |
   9633.1 |#############
   9877.2 |#############
  10121.2 |
  10365.3 |
  10609.4 |#############
  10853.4 |#############
  11097.5 |
  11341.5 |
  11585.6 |
  11829.7 |
  12073.7 |
  12317.8 |#############
  12561.9 |
  12805.9 |#############
  13050.0 |##########################
  13294.0 |
  13538.1 |########################################
  13782.2 |#############
  14026.2 |#############
  (2 below, 3 above range)

kernel_nway (n=18, range 10052.4-13628.9 ns)
  10052.4 |#############
  10231.3 |
  10410.1 |#############
  10588.9 |##########################
  10767.7 |#############
  10946.5 |##########################
  11125.4 |#############
  11304.2 |########################################
  11483.0 |
  11661.8 |
  11840.6 |#############
  12019.5 |
  12198.3 |
  12377.1 |
  12555.9 |#############
  12734.8 |
  12913.6 |#############
  13092.4 |#############
  13271.2 |
  13450.0 |
  (1 below, 2 above range)

kernel_seq (n=18, range 9525.1-12347.6 ns)
   9525.1 |
   9666.2 |
   9807.3 |########################################
   9948.4 |########################################
  10089.6 |#############
  10230.7 |#############
  10371.8 |
  10512.9 |
  10654.1 |#############
  10795.2 |
  10936.3 |#############
  11077.4 |
  11218.6 |
  11359.7 |
  11500.8 |#############
  11641.9 |##########################
  11783.1 |
  11924.2 |#############
  12065.3 |
  12206.4 |
  (2 below, 2 above range)

nway (n=18, range 31745.7-42792.6 ns)
  31745.7 |
  32298.1 |##############################
  32850.4 |
  33402.8 |
  33955.1 |
  34507.4 |####################
  35059.8 |##########
  35612.1 |##########
  36164.5 |
  36716.8 |
  37269.2 |########################################
  37821.5 |##########
  38373.8 |
  38926.2 |
  39478.5 |
  40030.9 |##########
  40583.2 |##########
  41135.5 |##########
  41687.9 |
  42240.2 |
  (1 below, 2 above range)

sequential (n=18, range 6690.8-23668.7 ns)
   6690.8 |########################################
   7539.7 |
   8388.6 |
   9237.5 |
  10086.3 |
  10935.2 |
  11784.1 |
  12633.0 |
  13481.9 |#####
  14330.8 |
  15179.7 |
  16028.6 |
  16877.5 |
  17726.4 |
  18575.3 |
  19424.2 |###########
  20273.1 |#####
  21122.0 |
  21970.9 |###########
  22819.8 |#####
  (1 below, 3 above range)

std_parallel (n=18, range 39379.2-54139.8 ns)
  39379.2 |
  40117.2 |
  40855.3 |
  41593.3 |
  42331.3 |##########
  43069.4 |
  43807.4 |##########
  44545.4 |
  45283.5 |####################
  46021.5 |
  46759.5 |
  47497.6 |##############################
  48235.6 |
  48973.6 |##############################
  49711.7 |
  50449.7 |
  51187.7 |
  51925.8 |
  52663.8 |
  53401.8 |########################################
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=104.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=103.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=106.3% of algo (FFI overhead may distort results)
- **nway**: bridge=102.0% of algo (FFI overhead may distort results)
- **sequential**: CV=51.2% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **sequential**: bridge=102.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=97.4% of algo (FFI overhead may distort results)
