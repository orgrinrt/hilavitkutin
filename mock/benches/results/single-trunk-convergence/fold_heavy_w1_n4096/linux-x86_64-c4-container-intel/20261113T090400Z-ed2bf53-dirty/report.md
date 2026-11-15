# fold_heavy, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 50% faster than the next best (kernel_nway)

sequential (25.35 us) leads kernel_nway (38.13 us) by 50%, a clear separation rather than a photo finish. CV 33.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 38% (significant)

sequential is -16.18 us (38%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 2.5x slower than the field

std_parallel (63.03 us) is 2.5x the fastest (25.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {kernel_nway, head_tail, nway, kernel_seq, head_tail_dyn, std_parallel} (50% apart)

The field splits into a fast tier {sequential} and a slow tier {kernel_nway, head_tail, nway, kernel_seq, head_tail_dyn, std_parallel} with a 50% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### std_parallel is inconsistent: worst-20% is 2.1x its best-20%

std_parallel's best 20% of batches run at 48.57 us but its worst 20% at 100.74 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 25345.2 ns median (-39.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.49x (fastest 25345.2 ns, slowest 63034.7 ns)

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
| Counter frequency | 2100350026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 48503ns | 42784ns | 38082ns | 43016ns | 61944ns | base |
| head_tail_dyn | 53198ns | 56703ns | 38211ns | 52960ns | 61049ns | +9.68% |
| kernel_nway | 42058ns | 38899ns | 35248ns | 38782ns | 50379ns | -13.29% |
| kernel_seq | 49307ns | 49853ns | 37029ns | 48081ns | 57285ns | +1.66% |
| nway | 52018ns | 48273ns | 44442ns | 47677ns | 62318ns | +7.25% |
| sequential | 30461ns | 25677ns | 23896ns | 25224ns | 41599ns | -37.20% |
| std_parallel | 72419ns | 63488ns | 48948ns | 60997ns | 101289ns | +49.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 47973ns | 37638ns | 61431ns | base | 0.085 |
| head_tail_dyn | 52579ns | 37602ns | 60253ns | +9.60% | 0.078 |
| kernel_nway | 41494ns | 34669ns | 49803ns | -13.51% | 0.099 |
| kernel_seq | 48805ns | 36640ns | 56851ns | +1.73% | 0.084 |
| nway | 51395ns | 44094ns | 61199ns | +7.13% | 0.080 |
| sequential | 30083ns | 23610ns | 41069ns | -37.29% | 0.136 |
| std_parallel | 71828ns | 48567ns | 100740ns | +49.73% | 0.057 |

## Performance model

- Peak throughput: **0.173 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.097 | 56.0% |
| head_tail_dyn | 0.073 | 42.0% |
| kernel_nway | 0.107 | 61.9% |
| kernel_seq | 0.083 | 47.8% |
| nway | 0.085 | 49.2% |
| sequential | 0.162 | 93.2% |
| std_parallel | 0.065 | 37.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 48503ns | 48503ns | base |
| head_tail_dyn | 53198ns | 53198ns | +9.68% |
| kernel_nway | 42058ns | 42058ns | -13.29% |
| kernel_seq | 49307ns | 49307ns | +1.66% |
| nway | 52018ns | 52018ns | +7.25% |
| sequential | 30461ns | 30461ns | -37.20% |
| std_parallel | 72419ns | 72419ns | +49.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 42135ns | base | --- | [38998, 59160] | --- | --- | --- | --- |
| head_tail_dyn | 56193ns | no significant difference | [-14182, +19239]ns | [47998, 57741] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 38132ns | no significant difference | [-18679, +4406]ns | [37290, 46780] | no | 0.4758 | 0.2379 | 0 |
| kernel_seq | 49389ns | no significant difference | [-3094, +8019]ns | [46577, 53660] | no | 1.0000 | 1.0000 | 0 |
| nway | 47939ns | no significant difference | [-11676, +12334]ns | [45418, 52767] | no | 0.5768 | 0.4807 | 0 |
| sequential | 25345ns | -16176.0ns (-38.4%) | [-30780, -11206]ns | [24213, 34118] | YES | 0.0452 | 0.0075 | 0 |
| std_parallel | 63035ns | +16380.7ns (+38.9%) | [+4898, +31042]ns | [54872, 75457] | YES (adj: no) | 0.0927 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 39622ns | +42.1% | +24.3% | +23.7% | +19.0% | +7.9% | +50.6% |
| 1 | 2 | 59615ns | -31.3% | -41.3% | -3.4% | -23.6% | -59.6% | -4.7% |
| 1 | 3 | 40288ns | +46.4% | -6.7% | -2.8% | +35.7% | -37.4% | +87.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.666 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.654 | HIGH- (thermal bounce) |
| kernel_nway | -0.274 | moderate- |
| kernel_seq | -0.476 | moderate- |
| nway | -0.268 | moderate- |
| sequential | -0.199 | ok |
| std_parallel | -0.255 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 48284.4ns | 47972.9ns | 100.6% | HIGH |
| head_tail_dyn | 52185.4ns | 52579.2ns | 99.3% | HIGH |
| kernel_nway | 41980.3ns | 41493.6ns | 101.2% | HIGH |
| kernel_seq | 48479.6ns | 48804.8ns | 99.3% | HIGH |
| nway | 52514.3ns | 51395.5ns | 102.2% | HIGH |
| sequential | 30047.7ns | 30083.4ns | 99.9% | HIGH |
| std_parallel | 64667.3ns | 71828.5ns | 90.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 37638.2-61430.7 ns)
  37638.2 |########################################
  38827.9 |#############
  40017.5 |#############
  41207.1 |##########################
  42396.7 |##########################
  43586.3 |
  44776.0 |
  45965.6 |
  47155.2 |
  48344.8 |
  49534.5 |
  50724.1 |
  51913.7 |
  53103.3 |
  54292.9 |
  55482.6 |#############
  56672.2 |
  57861.8 |
  59051.4 |########################################
  60241.1 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 37601.8-60252.8 ns)
  37601.8 |
  38734.3 |
  39866.9 |##########
  40999.4 |##########
  42132.0 |
  43264.5 |
  44397.1 |##########
  45529.6 |
  46662.2 |
  47794.8 |
  48927.3 |##########
  50059.9 |
  51192.4 |
  52325.0 |
  53457.5 |##########
  54590.1 |##########
  55722.6 |########################################
  56855.2 |##########
  57987.7 |####################
  59120.3 |##########
  (2 below, 2 above range)

kernel_nway (n=18, range 34668.6-49803.3 ns)
  34668.6 |##############################
  35425.3 |
  36182.1 |
  36938.8 |########################################
  37695.5 |##########
  38452.3 |##########
  39209.0 |##########
  39965.8 |
  40722.5 |
  41479.2 |
  42236.0 |
  42992.7 |
  43749.4 |##########
  44506.2 |
  45262.9 |
  46019.7 |
  46776.4 |##########
  47533.1 |##########
  48289.9 |##########
  49046.6 |##########
  (1 below, 2 above range)

kernel_seq (n=18, range 36640.4-56851.2 ns)
  36640.4 |########
  37651.0 |########
  38661.5 |
  39672.0 |########
  40682.6 |
  41693.1 |
  42703.7 |
  43714.2 |
  44724.7 |
  45735.3 |########
  46745.8 |################
  47756.4 |
  48766.9 |########################################
  49777.4 |
  50788.0 |
  51798.5 |########
  52809.0 |
  53819.6 |
  54830.1 |########
  55840.7 |########
  (1 below, 3 above range)

nway (n=18, range 44094.1-61199.1 ns)
  44094.1 |##########################
  44949.4 |########################################
  45804.6 |
  46659.9 |##########################
  47515.1 |
  48370.4 |#############
  49225.6 |
  50080.9 |#############
  50936.1 |#############
  51791.4 |#############
  52646.6 |#############
  53501.9 |#############
  54357.1 |
  55212.4 |#############
  56067.6 |
  56922.9 |
  57778.1 |
  58633.4 |
  59488.6 |
  60343.9 |
  (2 below, 2 above range)

sequential (n=18, range 23610.0-41068.9 ns)
  23610.0 |########################################
  24483.0 |########################
  25355.9 |########################
  26228.9 |
  27101.8 |
  27974.8 |########
  28847.7 |
  29720.6 |
  30593.6 |
  31466.5 |
  32339.5 |
  33212.4 |
  34085.4 |
  34958.3 |
  35831.2 |
  36704.2 |
  37577.1 |
  38450.1 |
  39323.0 |
  40196.0 |########
  (1 below, 4 above range)

std_parallel (n=18, range 48566.7-100740.3 ns)
  48566.7 |####################
  51175.3 |####################
  53784.0 |########################################
  56392.7 |####################
  59001.4 |####################
  61610.1 |########################################
  64218.8 |####################
  66827.4 |####################
  69436.1 |
  72044.8 |####################
  74653.5 |####################
  77262.2 |
  79870.8 |####################
  82479.5 |####################
  85088.2 |
  87696.9 |
  90305.6 |####################
  92914.2 |
  95522.9 |
  98131.6 |
  (2 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=21.7% (high variance, measurements may be unstable)
- **head_tail**: bridge=106.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=98.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=101.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=96.7% of algo (FFI overhead may distort results)
- **nway**: bridge=102.5% of algo (FFI overhead may distort results)
- **sequential**: CV=28.4% (high variance, measurements may be unstable)
- **sequential**: bridge=100.7% of algo (FFI overhead may distort results)
- **std_parallel**: CV=45.9% (high variance, measurements may be unstable)
- **std_parallel**: bridge=102.0% of algo (FFI overhead may distort results)
