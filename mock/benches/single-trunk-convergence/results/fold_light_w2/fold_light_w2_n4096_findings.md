# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 212% faster than the next best (kernel_seq)

sequential (8.79 us) leads kernel_seq (27.42 us) by 212%, a clear separation rather than a photo finish. CV 65.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 71% (significant)

sequential is -22.32 us (71%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 7.2x slower than the field

std_parallel (63.57 us) is 7.2x the fastest (8.79 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 65.1%)

sequential wins on median (8.79 us) yet has the highest variance (CV 65.1%), while kernel_seq is the steadiest (CV 8.1%, 27.42 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail shows alternating (throttle bounce) (autocorr -0.60)

head_tail's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {kernel_seq, kernel_nway, head_tail_dyn, head_tail, nway, std_parallel} (212% apart)

The field splits into a fast tier {sequential} and a slow tier {kernel_seq, kernel_nway, head_tail_dyn, head_tail, nway, std_parallel} with a 212% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.2x the fastest

Fastest sequential (8.79 us) to slowest std_parallel (63.57 us): 7.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 1.9x its best-20%

sequential's best 20% of batches run at 8.20 us but its worst 20% at 15.25 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 8788.5 ns median (-71.9% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 7.23x (fastest 8788.5 ns, slowest 63574.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100342298 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 33082ns | 31747ns | 24989ns | 29911ns | 41886ns | base |
| head_tail_dyn | 30832ns | 29524ns | 25727ns | 29053ns | 36054ns | -6.80% |
| kernel_nway | 29990ns | 28023ns | 25475ns | 27813ns | 35513ns | -9.35% |
| kernel_seq | 28430ns | 28140ns | 25991ns | 27577ns | 30929ns | -14.06% |
| nway | 44049ns | 43527ns | 36666ns | 42146ns | 50596ns | +33.15% |
| sequential | 11069ns | 9068ns | 8452ns | 8911ns | 15615ns | -66.54% |
| std_parallel | 67693ns | 64355ns | 52724ns | 61518ns | 84439ns | +104.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 32536ns | 24526ns | 41241ns | base | 0.126 |
| head_tail_dyn | 30277ns | 25355ns | 35403ns | -6.94% | 0.135 |
| kernel_nway | 29471ns | 24914ns | 34976ns | -9.42% | 0.139 |
| kernel_seq | 27768ns | 25399ns | 30239ns | -14.66% | 0.148 |
| nway | 43666ns | 36316ns | 50187ns | +34.21% | 0.094 |
| sequential | 10770ns | 8196ns | 15249ns | -66.90% | 0.380 |
| std_parallel | 67128ns | 52154ns | 83927ns | +106.32% | 0.061 |

## Performance model

- Peak throughput: **0.500 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.131 | 26.2% |
| head_tail_dyn | 0.141 | 28.2% |
| kernel_nway | 0.148 | 29.7% |
| kernel_seq | 0.149 | 29.9% |
| nway | 0.095 | 19.0% |
| sequential | 0.466 | 93.3% |
| std_parallel | 0.064 | 12.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 33082ns | 33082ns | base |
| head_tail_dyn | 30832ns | 30832ns | -6.80% |
| kernel_nway | 29990ns | 29990ns | -9.35% |
| kernel_seq | 28430ns | 28430ns | -14.06% |
| nway | 44049ns | 44049ns | +33.15% |
| sequential | 11069ns | 11069ns | -66.54% |
| std_parallel | 67693ns | 67693ns | +104.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 31312ns | base | --- | [26437, 34766] | --- | --- | --- | --- |
| head_tail_dyn | 29111ns | no significant difference | [-4030, +1805]ns | [27299, 32512] | no | 0.9774 | 0.8145 | 0 |
| kernel_nway | 27609ns | no significant difference | [-6296, +2683]ns | [26862, 30404] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 27417ns | -3683.1ns (-11.8%) | [-6941, -521]ns | [26063, 28621] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| nway | 43203ns | +11983.7ns (+38.3%) | [+9173, +16372]ns | [38887, 46773] | YES | 0.0003 | 0.0001 | 0 |
| sequential | 8788ns | -22320.7ns (-71.3%) | [-26144, -16672]ns | [8280, 9519] | YES | 0.0003 | 0.0001 | 0 |
| std_parallel | 63574ns | +31773.2ns (+101.5%) | [+26585, +39099]ns | [56220, 69677] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 28736ns | -10.6% | -5.8% | -9.3% | +70.3% | -63.8% | +90.1% |
| 1 | 2 | 34883ns | -2.2% | -22.4% | -19.1% | +26.5% | -76.4% | +104.3% |
| 1 | 3 | 25926ns | +12.3% | +8.7% | +3.6% | +44.0% | -66.7% | +145.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.604 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.558 | HIGH- (thermal bounce) |
| kernel_nway | -0.174 | ok |
| kernel_seq | -0.579 | HIGH- (thermal bounce) |
| nway | -0.007 | ok |
| sequential | -0.271 | moderate- |
| std_parallel | -0.474 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 33336.5ns | 32536.4ns | 102.5% | HIGH |
| head_tail_dyn | 31466.5ns | 30277.3ns | 103.9% | HIGH |
| kernel_nway | 28348.3ns | 29470.7ns | 96.2% | HIGH |
| kernel_seq | 29625.4ns | 27768.0ns | 106.7% | HIGH |
| nway | 46819.4ns | 43666.2ns | 107.2% | HIGH |
| sequential | 9726.0ns | 10770.3ns | 90.3% | HIGH |
| std_parallel | 63783.6ns | 67127.9ns | 95.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 24525.7-41241.4 ns)
  24525.7 |########################################
  25361.5 |#############
  26197.3 |#############
  27033.1 |
  27868.9 |#############
  28704.7 |#############
  29540.4 |#############
  30376.2 |
  31212.0 |
  32047.8 |#############
  32883.6 |########################################
  33719.4 |
  34555.1 |
  35390.9 |#############
  36226.7 |#############
  37062.5 |##########################
  37898.3 |
  38734.1 |
  39569.8 |
  40405.6 |
  (1 below, 1 above range)

head_tail_dyn (n=18, range 25354.8-35403.3 ns)
  25354.8 |########################################
  25857.2 |
  26359.6 |#############
  26862.1 |
  27364.5 |#############
  27866.9 |##########################
  28369.3 |
  28871.8 |##########################
  29374.2 |
  29876.6 |#############
  30379.1 |
  30881.5 |#############
  31383.9 |
  31886.3 |#############
  32388.8 |
  32891.2 |##########################
  33393.6 |
  33896.0 |
  34398.5 |
  34900.9 |#############
  (1 below, 2 above range)

kernel_nway (n=18, range 24914.1-34975.6 ns)
  24914.1 |
  25417.2 |
  25920.3 |##########################
  26423.3 |##########################
  26926.4 |########################################
  27429.5 |#############
  27932.6 |##########################
  28435.6 |
  28938.7 |
  29441.8 |
  29944.8 |#############
  30447.9 |##########################
  30951.0 |
  31454.1 |
  31957.1 |
  32460.2 |
  32963.3 |
  33466.4 |#############
  33969.4 |
  34472.5 |
  (2 below, 2 above range)

kernel_seq (n=18, range 25399.0-30238.5 ns)
  25399.0 |#############
  25641.0 |#############
  25883.0 |########################################
  26125.0 |#############
  26366.9 |
  26608.9 |
  26850.9 |
  27092.9 |##########################
  27334.8 |#############
  27576.8 |
  27818.8 |##########################
  28060.7 |
  28302.7 |#############
  28544.7 |
  28786.7 |#############
  29028.6 |
  29270.6 |
  29512.6 |##########################
  29754.6 |
  29996.5 |
  (1 below, 2 above range)

nway (n=18, range 36316.2-50186.9 ns)
  36316.2 |
  37009.7 |########################################
  37703.3 |####################
  38396.8 |
  39090.3 |
  39783.9 |####################
  40477.4 |####################
  41170.9 |
  41864.5 |
  42558.0 |########################################
  43251.5 |####################
  43945.1 |
  44638.6 |####################
  45332.1 |
  46025.7 |########################################
  46719.2 |####################
  47412.7 |
  48106.3 |
  48799.8 |
  49493.3 |
  (2 below, 4 above range)

sequential (n=18, range 8195.8-15248.5 ns)
   8195.8 |########################################
   8548.5 |###########
   8901.1 |#####
   9253.7 |#################
   9606.4 |
   9959.0 |
  10311.6 |
  10664.3 |
  11016.9 |#####
  11369.5 |
  11722.2 |
  12074.8 |
  12427.5 |#####
  12780.1 |
  13132.7 |
  13485.4 |
  13838.0 |
  14190.6 |
  14543.3 |
  14895.9 |
  (1 below, 2 above range)

std_parallel (n=18, range 52153.5-83926.7 ns)
  52153.5 |########################################
  53742.2 |####################
  55330.8 |########################################
  56919.5 |
  58508.1 |####################
  60096.8 |####################
  61685.5 |
  63274.1 |########################################
  64862.8 |####################
  66451.4 |####################
  68040.1 |####################
  69628.8 |####################
  71217.4 |
  72806.1 |####################
  74394.8 |
  75983.4 |
  77572.1 |####################
  79160.7 |
  80749.4 |
  82338.1 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=29.2% (high variance, measurements may be unstable)
- **head_tail**: bridge=100.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=105.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=101.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=106.4% of algo (FFI overhead may distort results)
- **nway**: bridge=95.3% of algo (FFI overhead may distort results)
- **sequential**: CV=53.1% (high variance, measurements may be unstable)
- **sequential**: bridge=99.1% of algo (FFI overhead may distort results)
- **std_parallel**: CV=23.8% (high variance, measurements may be unstable)
- **std_parallel**: bridge=98.9% of algo (FFI overhead may distort results)
