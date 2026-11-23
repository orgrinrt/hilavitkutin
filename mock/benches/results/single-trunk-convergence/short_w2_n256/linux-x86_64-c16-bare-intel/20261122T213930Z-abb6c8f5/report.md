# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 37% faster than the next best (kernel_seq)

sequential (6.94 us) leads kernel_seq (9.49 us) by 37%, a clear separation rather than a photo finish. CV 123.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 6.9x slower than the field

std_parallel (47.73 us) is 6.9x the fastest (6.94 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 123.5%)

sequential wins on median (6.94 us) yet has the highest variance (CV 123.5%), while head_tail is the steadiest (CV 8.7%, 11.49 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.64)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, kernel_seq, head_tail, head_tail_dyn, kernel_nway} vs {nway, std_parallel} (132% apart)

The field splits into a fast tier {sequential, kernel_seq, head_tail, head_tail_dyn, kernel_nway} and a slow tier {nway, std_parallel} with a 132% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.9x the fastest

Fastest sequential (6.94 us) to slowest std_parallel (47.73 us): 6.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.7x its best-20%

sequential's best 20% of batches run at 6.72 us but its worst 20% at 24.69 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 6938.1 ns median (-39.6% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 6.88x (fastest 6938.1 ns, slowest 47726.2 ns)

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
| Counter frequency | 2112817100 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 12278ns | 12121ns | 10747ns | 11880ns | 13639ns | base |
| head_tail_dyn | 14272ns | 13656ns | 12393ns | 13535ns | 16318ns | +16.25% |
| kernel_nway | 15442ns | 15042ns | 12271ns | 14669ns | 18187ns | +25.77% |
| kernel_seq | 13266ns | 10386ns | 8811ns | 10233ns | 20044ns | +8.05% |
| nway | 36177ns | 33848ns | 30724ns | 33729ns | 42576ns | +194.66% |
| sequential | 13703ns | 7078ns | 6849ns | 8302ns | 25230ns | +11.61% |
| std_parallel | 45645ns | 48277ns | 34236ns | 45555ns | 51483ns | +271.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 11586ns | 10221ns | 12715ns | base | 0.022 |
| head_tail_dyn | 13528ns | 11804ns | 15409ns | +16.76% | 0.019 |
| kernel_nway | 14615ns | 11728ns | 17059ns | +26.15% | 0.018 |
| kernel_seq | 12394ns | 8313ns | 18749ns | +6.98% | 0.021 |
| nway | 35670ns | 30319ns | 41940ns | +207.89% | 0.007 |
| sequential | 13419ns | 6716ns | 24692ns | +15.82% | 0.019 |
| std_parallel | 45078ns | 33903ns | 50711ns | +289.09% | 0.006 |

## Performance model

- Peak throughput: **0.038 Gops/s** (sequential; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.022 | 58.5% |
| head_tail_dyn | 0.020 | 51.8% |
| kernel_nway | 0.018 | 46.7% |
| kernel_seq | 0.027 | 70.8% |
| nway | 0.008 | 20.1% |
| sequential | 0.037 | 96.8% |
| std_parallel | 0.005 | 14.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 12278ns | 12278ns | base |
| head_tail_dyn | 14272ns | 14272ns | +16.25% |
| kernel_nway | 15442ns | 15442ns | +25.77% |
| kernel_seq | 13266ns | 13266ns | +8.05% |
| nway | 36177ns | 36177ns | +194.66% |
| sequential | 13703ns | 13703ns | +11.61% |
| std_parallel | 45645ns | 45645ns | +271.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 11488ns | base | --- | [10835, 11845] | --- | --- | --- | --- |
| head_tail_dyn | 12960ns | +1722.8ns (+15.0%) | [+739, +2688]ns | [12419, 14334] | YES | 0.0020 | 0.0013 | 0 |
| kernel_nway | 14383ns | +2389.3ns (+20.8%) | [+1378, +4367]ns | [13435, 16001] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 9487ns | no significant difference | [-3054, +6063]ns | [9056, 16383] | no | 0.2855 | 0.2379 | 0 |
| nway | 33403ns | +22166.5ns (+193.0%) | [+21140, +26740]ns | [31975, 38704] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 6938ns | no significant difference | [-4416, +9063]ns | [6788, 21169] | no | 0.4807 | 0.4807 | 0 |
| std_parallel | 47726ns | +35439.1ns (+308.5%) | [+30906, +37919]ns | [41534, 49292] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 10728ns | +14.9% | +52.5% | +78.6% | +216.9% | +17.7% | +357.3% |
| 1 | 2 | 11441ns | +34.1% | +8.2% | -18.5% | +185.8% | -39.4% | +322.7% |
| 1 | 3 | 12369ns | +4.8% | +16.3% | -26.8% | +186.2% | -44.8% | +263.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.004 | ok |
| head_tail_dyn | -0.640 | HIGH- (thermal bounce) |
| kernel_nway | -0.502 | HIGH- (thermal bounce) |
| kernel_seq | -0.154 | ok |
| nway | -0.487 | moderate- |
| sequential | -0.157 | ok |
| std_parallel | -0.084 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 12198.0ns | 11585.6ns | 105.3% | HIGH |
| head_tail_dyn | 14000.3ns | 13527.7ns | 103.5% | HIGH |
| kernel_nway | 15214.6ns | 14615.2ns | 104.1% | HIGH |
| kernel_seq | 12969.4ns | 12394.2ns | 104.6% | HIGH |
| nway | 36953.3ns | 35670.4ns | 103.6% | HIGH |
| sequential | 14026.1ns | 13419.0ns | 104.5% | HIGH |
| std_parallel | 46228.5ns | 45078.2ns | 102.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 10221.0-12714.9 ns)
  10221.0 |#############
  10345.7 |#############
  10470.4 |
  10595.1 |
  10719.8 |##########################
  10844.5 |#############
  10969.2 |
  11093.9 |
  11218.6 |
  11343.3 |########################################
  11468.0 |#############
  11592.7 |#############
  11717.4 |##########################
  11842.1 |#############
  11966.8 |
  12091.5 |
  12216.2 |
  12340.8 |
  12465.5 |
  12590.2 |
  (1 below, 4 above range)

head_tail_dyn (n=18, range 11804.0-15408.5 ns)
  11804.0 |
  11984.2 |
  12164.4 |##############################
  12344.6 |########################################
  12524.9 |
  12705.1 |##########
  12885.3 |
  13065.6 |##########
  13245.8 |
  13426.0 |
  13606.2 |##########
  13786.5 |##########
  13966.7 |
  14146.9 |##########
  14327.1 |##########
  14507.4 |
  14687.6 |
  14867.8 |##########
  15048.0 |
  15228.3 |
  (1 below, 3 above range)

kernel_nway (n=18, range 11727.9-17059.2 ns)
  11727.9 |
  11994.4 |##########################
  12261.0 |
  12527.6 |#############
  12794.1 |
  13060.7 |#############
  13327.3 |
  13593.8 |#############
  13860.4 |##########################
  14127.0 |#############
  14393.5 |##########################
  14660.1 |#############
  14926.7 |
  15193.2 |
  15459.8 |
  15726.4 |
  15992.9 |########################################
  16259.5 |
  16526.1 |#############
  16792.6 |
  (1 below, 2 above range)

kernel_seq (n=18, range 8313.3-18748.6 ns)
   8313.3 |####################
   8835.1 |########################################
   9356.8 |####################
   9878.6 |
  10400.4 |
  10922.1 |####################
  11443.9 |
  11965.7 |
  12487.4 |
  13009.2 |
  13531.0 |
  14052.7 |
  14574.5 |
  15096.3 |
  15618.0 |##########
  16139.8 |
  16661.6 |##########
  17183.3 |
  17705.1 |
  18226.9 |
  (2 below, 4 above range)

nway (n=18, range 30319.4-41939.7 ns)
  30319.4 |########################################
  30900.4 |
  31481.4 |##########################
  32062.4 |#############
  32643.4 |#############
  33224.4 |##########################
  33805.5 |
  34386.5 |
  34967.5 |#############
  35548.5 |
  36129.5 |
  36710.5 |
  37291.6 |#############
  37872.6 |#############
  38453.6 |
  39034.6 |#############
  39615.6 |
  40196.6 |
  40777.7 |#############
  41358.7 |
  (1 below, 3 above range)

sequential (n=18, range 6715.6-24692.4 ns)
   6715.6 |########################################
   7614.4 |
   8513.3 |
   9412.1 |
  10311.0 |
  11209.8 |
  12108.6 |
  13007.5 |
  13906.3 |
  14805.1 |
  15704.0 |
  16602.8 |
  17501.7 |####
  18400.5 |
  19299.3 |
  20198.2 |
  21097.0 |########
  21995.9 |####
  22894.7 |
  23793.5 |
  (2 below, 3 above range)

std_parallel (n=18, range 33902.8-50711.4 ns)
  33902.8 |
  34743.2 |
  35583.6 |####################
  36424.1 |
  37264.5 |
  38104.9 |####################
  38945.4 |
  39785.8 |####################
  40626.2 |
  41466.7 |
  42307.1 |####################
  43147.5 |####################
  43988.0 |
  44828.4 |
  45668.8 |
  46509.3 |####################
  47349.7 |########################################
  48190.1 |########################################
  49030.6 |########################################
  49871.0 |########################################
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=105.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=104.2% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=37.7% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=108.9% of algo (FFI overhead may distort results)
- **nway**: bridge=102.8% of algo (FFI overhead may distort results)
- **sequential**: CV=63.9% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **sequential**: bridge=103.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=101.0% of algo (FFI overhead may distort results)
