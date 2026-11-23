# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 40% faster than the next best (head_tail_dyn)

sequential (8.15 us) leads head_tail_dyn (11.40 us) by 40%, a clear separation rather than a photo finish. CV 96.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 8.7x slower than the field

std_parallel (71.01 us) is 8.7x the fastest (8.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 96.7%)

sequential wins on median (8.15 us) yet has the highest variance (CV 96.7%), while head_tail is the steadiest (CV 7.9%, 12.75 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, head_tail_dyn, kernel_seq, head_tail, kernel_nway} vs {nway, std_parallel} (161% apart)

The field splits into a fast tier {sequential, head_tail_dyn, kernel_seq, head_tail, kernel_nway} and a slow tier {nway, std_parallel} with a 161% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.7x the fastest

Fastest sequential (8.15 us) to slowest std_parallel (71.01 us): 8.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.5x its best-20%

sequential's best 20% of batches run at 6.82 us but its worst 20% at 23.70 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 8150.6 ns median (-36.1% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 8.71x (fastest 8150.6 ns, slowest 71009.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 8 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112783970 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 13424ns | 13316ns | 11590ns | 13091ns | 14840ns | base |
| head_tail_dyn | 13181ns | 12200ns | 11293ns | 11991ns | 15910ns | -1.81% |
| kernel_nway | 18724ns | 18603ns | 15371ns | 17976ns | 21521ns | +39.48% |
| kernel_seq | 13604ns | 13444ns | 10877ns | 12887ns | 16043ns | +1.34% |
| nway | 50780ns | 47143ns | 42646ns | 46581ns | 61145ns | +278.28% |
| sequential | 13943ns | 8310ns | 6956ns | 9437ns | 24196ns | +3.87% |
| std_parallel | 73855ns | 71237ns | 65815ns | 70305ns | 83199ns | +450.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12665ns | 11059ns | 13740ns | base | 0.020 |
| head_tail_dyn | 12446ns | 10679ns | 15139ns | -1.72% | 0.021 |
| kernel_nway | 17946ns | 14736ns | 20552ns | +41.70% | 0.014 |
| kernel_seq | 12614ns | 10144ns | 14744ns | -0.40% | 0.020 |
| nway | 50121ns | 42141ns | 60265ns | +295.75% | 0.005 |
| sequential | 13658ns | 6817ns | 23699ns | +7.85% | 0.019 |
| std_parallel | 73554ns | 65606ns | 82856ns | +480.78% | 0.003 |

## Performance model

- Peak throughput: **0.038 Gops/s** (sequential; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.020 | 53.5% |
| head_tail_dyn | 0.022 | 59.8% |
| kernel_nway | 0.014 | 38.1% |
| kernel_seq | 0.020 | 54.0% |
| nway | 0.005 | 14.6% |
| sequential | 0.031 | 83.6% |
| std_parallel | 0.004 | 9.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 13424ns | 13424ns | base |
| head_tail_dyn | 13181ns | 13181ns | -1.81% |
| kernel_nway | 18724ns | 18724ns | +39.48% |
| kernel_seq | 13604ns | 13604ns | +1.34% |
| nway | 50780ns | 50780ns | +278.28% |
| sequential | 13943ns | 13943ns | +3.87% |
| std_parallel | 73855ns | 73855ns | +450.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 12751ns | base | --- | [12090, 13379] | --- | --- | --- | --- |
| head_tail_dyn | 11395ns | no significant difference | [-1468, +1554]ns | [10919, 13791] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 17884ns | +5574.8ns (+43.7%) | [+4178, +6276]ns | [16025, 19062] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 12622ns | no significant difference | [-2087, +1562]ns | [10544, 13870] | no | 0.7210 | 0.4807 | 0 |
| nway | 46613ns | +35093.8ns (+275.2%) | [+31695, +41317]ns | [43221, 55148] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 8151ns | no significant difference | [-4910, +7471]ns | [6900, 20572] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 71009ns | +57626.7ns (+451.9%) | [+56127, +62755]ns | [69082, 74122] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 13235ns | -18.7% | +26.7% | -21.8% | +222.0% | +11.0% | +436.6% |
| 1 | 2 | 12751ns | +19.8% | +52.0% | +11.8% | +377.4% | -45.2% | +446.3% |
| 1 | 3 | 11970ns | -6.8% | +42.6% | +10.5% | +288.0% | -16.2% | +499.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.012 | ok |
| head_tail_dyn | -0.663 | HIGH- (thermal bounce) |
| kernel_nway | -0.659 | HIGH- (thermal bounce) |
| kernel_seq | -0.329 | moderate- |
| nway | -0.640 | HIGH- (thermal bounce) |
| sequential | -0.426 | moderate- |
| std_parallel | -0.585 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 13348.3ns | 12664.7ns | 105.4% | HIGH |
| head_tail_dyn | 12885.9ns | 12446.3ns | 103.5% | HIGH |
| kernel_nway | 18852.4ns | 17945.7ns | 105.1% | HIGH |
| kernel_seq | 13069.7ns | 12614.2ns | 103.6% | HIGH |
| nway | 50731.8ns | 50121.0ns | 101.2% | HIGH |
| sequential | 14157.8ns | 13658.3ns | 103.7% | HIGH |
| std_parallel | 73900.4ns | 73554.0ns | 100.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 11058.9-13740.1 ns)
  11058.9 |
  11193.0 |
  11327.1 |
  11461.1 |
  11595.2 |########################################
  11729.2 |
  11863.3 |####################
  11997.3 |
  12131.4 |####################
  12265.5 |########################################
  12399.5 |
  12533.6 |####################
  12667.6 |
  12801.7 |
  12935.8 |####################
  13069.8 |####################
  13203.9 |########################################
  13337.9 |
  13472.0 |########################################
  13606.1 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 10679.4-15139.2 ns)
  10679.4 |########################################
  10902.4 |##########################
  11125.4 |########################################
  11348.4 |#############
  11571.4 |##########################
  11794.3 |
  12017.3 |
  12240.3 |
  12463.3 |
  12686.3 |
  12909.3 |
  13132.3 |#############
  13355.3 |
  13578.3 |
  13801.2 |
  14024.2 |
  14247.2 |#############
  14470.2 |#############
  14693.2 |
  14916.2 |
  (1 below, 3 above range)

kernel_nway (n=18, range 14735.9-20551.7 ns)
  14735.9 |##########
  15026.7 |####################
  15317.4 |##########
  15608.2 |
  15899.0 |
  16189.8 |##########
  16480.6 |
  16771.4 |
  17062.2 |
  17353.0 |
  17643.8 |########################################
  17934.5 |##########
  18225.3 |
  18516.1 |
  18806.9 |####################
  19097.7 |##########
  19388.5 |##########
  19679.3 |##########
  19970.1 |
  20260.9 |
  (1 below, 2 above range)

kernel_seq (n=18, range 10144.5-14744.1 ns)
  10144.5 |########################################
  10374.5 |########################################
  10604.4 |####################
  10834.4 |
  11064.4 |
  11294.4 |
  11524.4 |####################
  11754.4 |
  11984.3 |
  12214.3 |
  12444.3 |########################################
  12674.3 |####################
  12904.3 |
  13134.2 |
  13364.2 |####################
  13594.2 |########################################
  13824.2 |####################
  14054.2 |####################
  14284.2 |####################
  14514.1 |
  (1 below, 2 above range)

nway (n=18, range 42140.9-60265.2 ns)
  42140.9 |########################################
  43047.1 |##########
  43953.3 |
  44859.6 |
  45765.8 |##############################
  46672.0 |##########
  47578.2 |
  48484.4 |##########
  49390.6 |
  50296.9 |##########
  51203.1 |
  52109.3 |
  53015.5 |
  53921.7 |##########
  54827.9 |
  55734.2 |##########
  56640.4 |
  57546.6 |
  58452.8 |
  59359.0 |##########
  (1 below, 3 above range)

sequential (n=18, range 6816.9-23699.4 ns)
   6816.9 |########################################
   7661.1 |
   8505.2 |#####
   9349.3 |
  10193.4 |
  11037.6 |
  11881.7 |#####
  12725.8 |
  13569.9 |
  14414.1 |
  15258.2 |
  16102.3 |
  16946.4 |
  17790.6 |
  18634.7 |
  19478.8 |#####
  20322.9 |#####
  21167.1 |#####
  22011.2 |#####
  22855.3 |
  (1 below, 3 above range)

std_parallel (n=18, range 65605.9-82856.4 ns)
  65605.9 |####################
  66468.5 |
  67331.0 |##########
  68193.5 |##########
  69056.0 |####################
  69918.6 |##########
  70781.1 |########################################
  71643.6 |##########
  72506.1 |
  73368.6 |
  74231.2 |
  75093.7 |##########
  75956.2 |
  76818.7 |##########
  77681.3 |##########
  78543.8 |
  79406.3 |
  80268.8 |
  81131.4 |
  81993.9 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=103.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=102.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=104.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=102.2% of algo (FFI overhead may distort results)
- **nway**: bridge=100.9% of algo (FFI overhead may distort results)
- **sequential**: CV=57.7% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **sequential**: bridge=104.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=98.2% of algo (FFI overhead may distort results)
