# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 254% faster than the next best (head_tail_dyn)

sequential (8.12 us) leads head_tail_dyn (28.73 us) by 254%, a clear separation rather than a photo finish. CV 39.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 76% (significant)

sequential is -26.05 us (76%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 7.3x slower than the field

std_parallel (59.66 us) is 7.3x the fastest (8.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {head_tail_dyn, kernel_nway, kernel_seq, head_tail, nway, std_parallel} (254% apart)

The field splits into a fast tier {sequential} and a slow tier {head_tail_dyn, kernel_nway, kernel_seq, head_tail, nway, std_parallel} with a 254% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.3x the fastest

Fastest sequential (8.12 us) to slowest std_parallel (59.66 us): 7.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### std_parallel is inconsistent: worst-20% is 4.7x its best-20%

std_parallel's best 20% of batches run at 50.58 us but its worst 20% at 237.56 us (4.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 8125.0 ns median (-76.2% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 7.34x (fastest 8125.0 ns, slowest 59660.1 ns)

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
| Counter frequency | 2100342298 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 34219ns | 34598ns | 24936ns | 32582ns | 41317ns | base |
| head_tail_dyn | 35641ns | 29137ns | 25661ns | 28336ns | 51588ns | +4.15% |
| kernel_nway | 32793ns | 30306ns | 19657ns | 27252ns | 47671ns | -4.17% |
| kernel_seq | 33047ns | 31048ns | 21624ns | 28869ns | 45026ns | -3.42% |
| nway | 62070ns | 49663ns | 35341ns | 47101ns | 97889ns | +81.39% |
| sequential | 10440ns | 8366ns | 7237ns | 8797ns | 14507ns | -69.49% |
| std_parallel | 118446ns | 60079ns | 50991ns | 61159ns | 238103ns | +246.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 33734ns | 24473ns | 40849ns | base | 0.000 |
| head_tail_dyn | 35188ns | 25232ns | 51054ns | +4.31% | 0.000 |
| kernel_nway | 32283ns | 19288ns | 47074ns | -4.30% | 0.000 |
| kernel_seq | 32524ns | 21100ns | 44521ns | -3.59% | 0.000 |
| nway | 61620ns | 35005ns | 97336ns | +82.66% | 0.000 |
| sequential | 10165ns | 7013ns | 14176ns | -69.87% | 0.002 |
| std_parallel | 117930ns | 50576ns | 237556ns | +249.58% | 0.000 |

## Performance model

- Peak throughput: **0.002 Gops/s** (sequential; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.000 | 20.5% |
| head_tail_dyn | 0.001 | 24.4% |
| kernel_nway | 0.001 | 23.5% |
| kernel_seq | 0.001 | 23.1% |
| nway | 0.000 | 14.2% |
| sequential | 0.002 | 86.3% |
| std_parallel | 0.000 | 11.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 34219ns | 34219ns | base |
| head_tail_dyn | 35641ns | 35641ns | +4.15% |
| kernel_nway | 32793ns | 32793ns | -4.17% |
| kernel_seq | 33047ns | 33047ns | -3.42% |
| nway | 62070ns | 62070ns | +81.39% |
| sequential | 10440ns | 10440ns | -69.49% |
| std_parallel | 118446ns | 118446ns | +246.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 34177ns | base | --- | [29018, 39907] | --- | --- | --- | --- |
| head_tail_dyn | 28729ns | no significant difference | [-8041, +5035]ns | [26510, 31778] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 29818ns | no significant difference | [-11741, +3004]ns | [21653, 36923] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 30335ns | no significant difference | [-5603, +2221]ns | [26187, 39174] | no | 0.2855 | 0.2379 | 0 |
| nway | 49235ns | +18326.6ns (+53.6%) | [+3959, +33967]ns | [36822, 63981] | YES | 0.0026 | 0.0013 | 0 |
| sequential | 8125ns | -26053.4ns (-76.2%) | [-30023, -16188]ns | [7469, 13674] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 59660ns | +25483.0ns (+74.6%) | [+21625, +105050]ns | [56271, 143673] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 25491ns | +12.7% | +33.2% | -13.2% | +139.5% | -44.8% | +463.6% |
| 1 | 2 | 40409ns | +9.3% | -14.9% | +8.4% | +56.3% | -80.4% | +47.3% |
| 1 | 3 | 34360ns | -23.4% | -40.8% | -14.4% | +4.9% | -79.5% | +63.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.434 | moderate- |
| head_tail_dyn | -0.656 | HIGH- (thermal bounce) |
| kernel_nway | -0.184 | ok |
| kernel_seq | -0.594 | HIGH- (thermal bounce) |
| nway | -0.208 | moderate- |
| sequential | -0.105 | ok |
| std_parallel | -0.148 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 40409.6ns | 33734.4ns | 119.8% | HIGH |
| head_tail_dyn | 36695.3ns | 35187.8ns | 104.3% | HIGH |
| kernel_nway | 31416.2ns | 32283.1ns | 97.3% | HIGH |
| kernel_seq | 43069.9ns | 32524.3ns | 132.4% | HIGH |
| nway | 70186.0ns | 61619.9ns | 113.9% | HIGH |
| sequential | 10423.4ns | 10164.7ns | 102.5% | HIGH |
| std_parallel | 101332.1ns | 117930.1ns | 85.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 24472.8-40849.2 ns)
  24472.8 |##########################
  25291.6 |#############
  26110.4 |
  26929.3 |
  27748.1 |#############
  28566.9 |
  29385.7 |#############
  30204.6 |
  31023.4 |
  31842.2 |##########################
  32661.0 |
  33479.8 |#############
  34298.7 |##########################
  35117.5 |
  35936.3 |
  36755.1 |#############
  37573.9 |
  38392.8 |
  39211.6 |#############
  40030.4 |########################################
  (1 below, 2 above range)

head_tail_dyn (n=18, range 25232.1-51053.6 ns)
  25232.1 |########################################
  26523.2 |########
  27814.3 |########################
  29105.3 |################
  30396.4 |########
  31687.5 |########
  32978.6 |
  34269.6 |
  35560.7 |
  36851.8 |
  38142.9 |
  39433.9 |########
  40725.0 |
  42016.1 |
  43307.2 |
  44598.2 |
  45889.3 |
  47180.4 |########
  48471.5 |
  49762.5 |
  (1 below, 2 above range)

kernel_nway (n=18, range 19288.3-47074.1 ns)
  19288.3 |#############
  20677.6 |########################################
  22066.9 |
  23456.2 |#############
  24845.5 |
  26234.8 |#############
  27624.1 |#############
  29013.3 |
  30402.6 |##########################
  31791.9 |
  33181.2 |#############
  34570.5 |#############
  35959.8 |
  37349.1 |
  38738.4 |##########################
  40127.6 |
  41516.9 |#############
  42906.2 |
  44295.5 |
  45684.8 |
  (2 below, 2 above range)

kernel_seq (n=18, range 21100.4-44520.9 ns)
  21100.4 |##########################
  22271.4 |#############
  23442.5 |#############
  24613.5 |
  25784.5 |
  26955.5 |#############
  28126.6 |##########################
  29297.6 |#############
  30468.6 |########################################
  31639.6 |
  32810.7 |
  33981.7 |
  35152.7 |
  36323.7 |
  37494.8 |#############
  38665.8 |
  39836.8 |#############
  41007.9 |
  42178.9 |#############
  43349.9 |#############
  (1 below, 2 above range)

nway (n=18, range 35004.8-97336.3 ns)
  35004.8 |########################################
  38121.4 |
  41238.0 |
  44354.5 |########################
  47471.1 |
  50587.7 |########
  53704.3 |
  56820.8 |########
  59937.4 |################
  63054.0 |########
  66170.6 |
  69287.1 |
  72403.7 |########
  75520.3 |
  78636.8 |
  81753.4 |
  84870.0 |
  87986.6 |
  91103.1 |
  94219.7 |
  (1 below, 3 above range)

sequential (n=18, range 7013.3-14176.2 ns)
   7013.3 |########################################
   7371.5 |##########################
   7729.6 |##########################
   8087.8 |#############
   8445.9 |
   8804.0 |
   9162.2 |#############
   9520.3 |
   9878.5 |
  10236.6 |
  10594.8 |
  10952.9 |
  11311.0 |
  11669.2 |
  12027.3 |
  12385.5 |
  12743.6 |
  13101.8 |
  13459.9 |########################################
  13818.0 |##########################
  (2 below, 2 above range)

std_parallel (n=18, range 50576.1-237555.6 ns)
  50576.1 |########################################
  59925.1 |#####
  69274.0 |#####
  78623.0 |
  87972.0 |
  97321.0 |
  106669.9 |
  116018.9 |
  125367.9 |#####
  134716.9 |
  144065.8 |
  153414.8 |#####
  162763.8 |##########
  172112.8 |
  181461.7 |
  190810.7 |#####
  200159.7 |
  209508.6 |
  218857.6 |
  228206.6 |
  (2 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=51.1% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=101.2% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=41.0% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=99.3% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=29.8% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=96.8% of algo (FFI overhead may distort results)
- **nway**: CV=52.0% (high variance, measurements may be unstable)
- **nway**: bridge=99.0% of algo (FFI overhead may distort results)
- **sequential**: CV=31.4% (high variance, measurements may be unstable)
- **sequential**: bridge=107.4% of algo (FFI overhead may distort results)
- **std_parallel**: CV=109.1% (high variance, measurements may be unstable)
- **std_parallel**: worst_20/best_20 = 4.7x (possible bimodal distribution)
- **std_parallel**: bridge=93.1% of algo (FFI overhead may distort results)
