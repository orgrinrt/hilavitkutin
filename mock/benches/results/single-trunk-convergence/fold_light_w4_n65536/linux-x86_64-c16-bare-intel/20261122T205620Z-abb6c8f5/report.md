# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 25% faster than the next best (head_tail)

kernel_nway (21.61 us) leads head_tail (26.95 us) by 25%, a clear separation rather than a photo finish. CV 16.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 4.2x slower than the field

std_parallel (91.49 us) is 4.2x the fastest (21.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.64)

nway's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, head_tail, sequential, kernel_seq, head_tail_dyn, nway} vs {std_parallel} (81% apart)

The field splits into a fast tier {kernel_nway, head_tail, sequential, kernel_seq, head_tail_dyn, nway} and a slow tier {std_parallel} with a 81% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.2x the fastest

Fastest kernel_nway (21.61 us) to slowest std_parallel (91.49 us): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.7x its best-20%

sequential's best 20% of batches run at 11.96 us but its worst 20% at 44.09 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 21613.5 ns median (-19.8% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 4.23x (fastest 21613.5 ns, slowest 91486.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112780093 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 28283ns | 27593ns | 24927ns | 27131ns | 31688ns | base |
| head_tail_dyn | 39790ns | 38434ns | 34582ns | 37859ns | 45291ns | +40.69% |
| kernel_nway | 23396ns | 22292ns | 19513ns | 21972ns | 27473ns | -17.28% |
| kernel_seq | 37611ns | 36928ns | 30067ns | 35613ns | 44379ns | +32.98% |
| nway | 52308ns | 51074ns | 44757ns | 49434ns | 60394ns | +84.95% |
| sequential | 27360ns | 27514ns | 12115ns | 20884ns | 44696ns | -3.26% |
| std_parallel | 91458ns | 92141ns | 81803ns | 89993ns | 98485ns | +223.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 27393ns | 24299ns | 30512ns | base | 2.392 |
| head_tail_dyn | 39075ns | 34074ns | 44136ns | +42.65% | 1.677 |
| kernel_nway | 22503ns | 18683ns | 26189ns | -17.85% | 2.912 |
| kernel_seq | 36625ns | 29494ns | 42711ns | +33.70% | 1.789 |
| nway | 51669ns | 44284ns | 59544ns | +88.62% | 1.268 |
| sequential | 26999ns | 11959ns | 44094ns | -1.44% | 2.427 |
| std_parallel | 90695ns | 81299ns | 97490ns | +231.09% | 0.723 |

## Performance model

- Peak throughput: **5.480 Gops/s** (sequential; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.432 | 44.4% |
| head_tail_dyn | 1.729 | 31.6% |
| kernel_nway | 3.032 | 55.3% |
| kernel_seq | 1.808 | 33.0% |
| nway | 1.297 | 23.7% |
| sequential | 2.414 | 44.1% |
| std_parallel | 0.716 | 13.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 28283ns | 28283ns | base |
| head_tail_dyn | 39790ns | 39790ns | +40.69% |
| kernel_nway | 23396ns | 23396ns | -17.28% |
| kernel_seq | 37611ns | 37611ns | +32.98% |
| nway | 52308ns | 52308ns | +84.95% |
| sequential | 27360ns | 27360ns | -3.26% |
| std_parallel | 91458ns | 91458ns | +223.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 26950ns | base | --- | [25332, 28267] | --- | --- | --- | --- |
| head_tail_dyn | 37905ns | +11061.6ns (+41.0%) | [+9140, +13016]ns | [36057, 41303] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 21614ns | -5187.4ns (-19.2%) | [-6998, -3585]ns | [19974, 24177] | YES | 0.0002 | 0.0001 | 0 |
| kernel_seq | 36241ns | +9737.9ns (+36.1%) | [+5102, +11453]ns | [32929, 40389] | YES | 0.0000 | 0.0000 | 0 |
| nway | 50531ns | +20210.7ns (+75.0%) | [+18560, +29823]ns | [46938, 55310] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 27145ns | no significant difference | [-13083, +11194]ns | [12180, 37787] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 91486ns | +63343.8ns (+235.0%) | [+59561, +65029]ns | [86516, 92964] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 29885ns | +30.9% | -17.2% | +17.2% | +62.8% | -57.6% | +211.1% |
| 1 | 2 | 25190ns | +47.3% | -14.2% | +32.5% | +137.8% | +24.2% | +261.1% |
| 1 | 3 | 26728ns | +42.9% | -26.6% | +48.5% | +71.2% | +29.9% | +223.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.377 | moderate- |
| head_tail_dyn | -0.523 | HIGH- (thermal bounce) |
| kernel_nway | -0.011 | ok |
| kernel_seq | -0.330 | moderate- |
| nway | -0.641 | HIGH- (thermal bounce) |
| sequential | -0.091 | ok |
| std_parallel | -0.030 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 28168.7ns | 27393.1ns | 102.8% | HIGH |
| head_tail_dyn | 40085.2ns | 39075.0ns | 102.6% | HIGH |
| kernel_nway | 23499.3ns | 22502.8ns | 104.4% | HIGH |
| kernel_seq | 36993.6ns | 36624.9ns | 101.0% | HIGH |
| nway | 54131.4ns | 51668.8ns | 104.8% | HIGH |
| sequential | 27623.1ns | 26998.5ns | 102.3% | HIGH |
| std_parallel | 91716.5ns | 90695.0ns | 101.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 24299.0-30512.1 ns)
  24299.0 |#############
  24609.6 |
  24920.3 |##########################
  25230.9 |##########################
  25541.6 |
  25852.2 |
  26162.9 |#############
  26473.6 |##########################
  26784.2 |
  27094.9 |########################################
  27405.5 |#############
  27716.2 |
  28026.8 |
  28337.5 |
  28648.2 |
  28958.8 |#############
  29269.5 |
  29580.1 |#############
  29890.8 |#############
  30201.4 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 34074.2-44135.7 ns)
  34074.2 |##########################
  34577.3 |
  35080.3 |##########################
  35583.4 |
  36086.5 |#############
  36589.6 |#############
  37092.6 |
  37595.7 |########################################
  38098.8 |#############
  38601.9 |
  39104.9 |
  39608.0 |
  40111.1 |#############
  40614.2 |#############
  41117.2 |
  41620.3 |#############
  42123.4 |
  42626.4 |
  43129.5 |
  43632.6 |#############
  (1 below, 3 above range)

kernel_nway (n=18, range 18682.6-26189.4 ns)
  18682.6 |
  19058.0 |#############
  19433.3 |
  19808.6 |########################################
  20184.0 |
  20559.3 |
  20934.7 |#############
  21310.0 |########################################
  21685.3 |
  22060.7 |
  22436.0 |#############
  22811.3 |
  23186.7 |#############
  23562.0 |#############
  23937.3 |
  24312.7 |#############
  24688.0 |##########################
  25063.4 |#############
  25438.7 |
  25814.0 |
  (2 below, 1 above range)

kernel_seq (n=18, range 29494.1-42710.7 ns)
  29494.1 |
  30154.9 |#############
  30815.7 |
  31476.6 |##########################
  32137.4 |
  32798.2 |
  33459.1 |#############
  34119.9 |#############
  34780.7 |#############
  35441.6 |
  36102.4 |########################################
  36763.2 |
  37424.1 |
  38084.9 |#############
  38745.7 |
  39406.6 |#############
  40067.4 |
  40728.2 |#############
  41389.1 |#############
  42049.9 |
  (2 below, 3 above range)

nway (n=18, range 44283.8-59543.8 ns)
  44283.8 |################
  45046.8 |
  45809.8 |
  46572.8 |########################################
  47335.8 |
  48098.8 |
  48861.8 |
  49624.8 |########
  50387.8 |################
  51150.8 |
  51913.8 |
  52676.8 |################
  53439.8 |
  54202.8 |
  54965.8 |
  55728.8 |
  56491.8 |
  57254.8 |################
  58017.8 |
  58780.8 |
  (1 below, 3 above range)

sequential (n=18, range 11959.4-44094.1 ns)
  11959.4 |########################################
  13566.1 |
  15172.8 |
  16779.6 |
  18386.3 |
  19993.1 |
  21599.8 |
  23206.5 |######
  24813.3 |
  26420.0 |
  28026.7 |
  29633.5 |######
  31240.2 |
  32847.0 |######
  34453.7 |######
  36060.4 |######
  37667.2 |######
  39273.9 |######
  40880.6 |######
  42487.4 |######
  (2 below, 1 above range)

std_parallel (n=18, range 81298.9-97490.4 ns)
  81298.9 |
  82108.4 |
  82918.0 |##########################
  83727.6 |#############
  84537.2 |
  85346.7 |
  86156.3 |##########################
  86965.9 |
  87775.5 |
  88585.0 |
  89394.6 |#############
  90204.2 |#############
  91013.8 |########################################
  91823.4 |##########################
  92632.9 |#############
  93442.5 |#############
  94252.1 |
  95061.7 |
  95871.2 |
  96680.8 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=102.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=104.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.2% of algo (FFI overhead may distort results)
- **nway**: bridge=100.7% of algo (FFI overhead may distort results)
- **sequential**: CV=56.1% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **sequential**: bridge=103.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=99.4% of algo (FFI overhead may distort results)
