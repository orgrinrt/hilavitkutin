# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 5.2x slower than the field

std_parallel (74.99 us) is 5.2x the fastest (14.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_seq, head_tail) are a dead heat (<1%)

kernel_seq (14.44 us) and head_tail (14.45 us) differ by 0.08%, inside the noise, even though the wider field spreads 419.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.67)

kernel_seq's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, head_tail, head_tail_dyn, kernel_nway, sequential} vs {nway, std_parallel} (109% apart)

The field splits into a fast tier {kernel_seq, head_tail, head_tail_dyn, kernel_nway, sequential} and a slow tier {nway, std_parallel} with a 109% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.2x the fastest

Fastest kernel_seq (14.44 us) to slowest std_parallel (74.99 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader kernel_seq vs stability leader head_tail_dyn (+9% speed for 1.2x steadier)

kernel_seq is fastest (14.44 us, CV 13.2%); head_tail_dyn gives up 8.9% median for 1.2x lower variance (CV 10.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### std_parallel is inconsistent: worst-20% is 1.6x its best-20%

std_parallel's best 20% of batches run at 65.72 us but its worst 20% at 108.40 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 14440.8 ns median (-0.1% vs baseline)
- 5 variants significantly slower than baseline
- Spread: 5.19x (fastest 14440.8 ns, slowest 74985.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 15743ns | 15052ns | 13067ns | 14346ns | 19177ns | base |
| head_tail_dyn | 16672ns | 16328ns | 14496ns | 15758ns | 19130ns | +5.90% |
| kernel_nway | 22087ns | 20320ns | 17730ns | 19704ns | 27841ns | +40.30% |
| kernel_seq | 15497ns | 15167ns | 12722ns | 14646ns | 18160ns | -1.57% |
| nway | 58002ns | 54907ns | 49505ns | 54748ns | 67130ns | +268.43% |
| sequential | 27261ns | 26493ns | 23324ns | 25886ns | 31292ns | +73.16% |
| std_parallel | 83719ns | 75226ns | 65924ns | 72613ns | 109276ns | +431.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 14875ns | 12475ns | 18006ns | base | 0.275 |
| head_tail_dyn | 15694ns | 13949ns | 17372ns | +5.50% | 0.261 |
| kernel_nway | 21081ns | 17033ns | 26441ns | +41.72% | 0.194 |
| kernel_seq | 14364ns | 12094ns | 16191ns | -3.43% | 0.285 |
| nway | 57248ns | 48976ns | 66202ns | +284.86% | 0.072 |
| sequential | 26726ns | 22868ns | 30664ns | +79.67% | 0.153 |
| std_parallel | 83284ns | 65717ns | 108405ns | +459.89% | 0.049 |

## Performance model

- Peak throughput: **0.339 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.283 | 83.7% |
| head_tail_dyn | 0.260 | 76.9% |
| kernel_nway | 0.215 | 63.5% |
| kernel_seq | 0.284 | 83.7% |
| nway | 0.076 | 22.3% |
| sequential | 0.158 | 46.6% |
| std_parallel | 0.055 | 16.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 15743ns | 15743ns | base |
| head_tail_dyn | 16672ns | 16672ns | +5.90% |
| kernel_nway | 22087ns | 22087ns | +40.30% |
| kernel_seq | 15497ns | 15497ns | -1.57% |
| nway | 58002ns | 58002ns | +268.43% |
| sequential | 27261ns | 27261ns | +73.16% |
| std_parallel | 83719ns | 83719ns | +431.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 14452ns | base | --- | [12670, 14745] | --- | --- | --- | --- |
| head_tail_dyn | 15732ns | +1537.2ns (+10.6%) | [+203, +2336]ns | [14401, 16205] | YES | 0.0371 | 0.0309 | 0 |
| kernel_nway | 19042ns | +4367.0ns (+30.2%) | [+3815, +8950]ns | [18312, 23434] | YES | 0.0002 | 0.0001 | 0 |
| kernel_seq | 14441ns | no significant difference | [-700, +658]ns | [13216, 14514] | no | 0.4807 | 0.4807 | 0 |
| nway | 54162ns | +40769.0ns (+282.1%) | [+37775, +44015]ns | [53348, 60543] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 25976ns | +10987.0ns (+76.0%) | [+10233, +14518]ns | [23712, 28630] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 74986ns | +59279.3ns (+410.2%) | [+53817, +71179]ns | [67359, 84686] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 13976ns | +15.9% | +85.5% | +3.7% | +310.8% | +110.8% | +510.7% |
| 1 | 2 | 12670ns | +12.3% | +39.0% | -4.0% | +290.4% | +85.7% | +431.6% |
| 1 | 3 | 14583ns | +4.4% | +26.2% | -1.0% | +313.6% | +74.6% | +414.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.602 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.505 | HIGH- (thermal bounce) |
| kernel_nway | -0.218 | moderate- |
| kernel_seq | -0.666 | HIGH- (thermal bounce) |
| nway | -0.622 | HIGH- (thermal bounce) |
| sequential | -0.375 | moderate- |
| std_parallel | -0.447 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 14497.9ns | 14875.1ns | 97.5% | HIGH |
| head_tail_dyn | 16550.4ns | 15693.9ns | 105.5% | HIGH |
| kernel_nway | 21855.3ns | 21081.0ns | 103.7% | HIGH |
| kernel_seq | 15636.0ns | 14364.5ns | 108.9% | HIGH |
| nway | 58385.4ns | 57247.7ns | 102.0% | HIGH |
| sequential | 27782.5ns | 26725.7ns | 104.0% | HIGH |
| std_parallel | 84865.8ns | 83284.1ns | 101.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 12475.3-18005.9 ns)
  12475.3 |################################
  12751.9 |########
  13028.4 |
  13304.9 |########
  13581.4 |
  13858.0 |
  14134.5 |
  14411.0 |########################################
  14687.6 |########
  14964.1 |########
  15240.6 |
  15517.1 |
  15793.7 |
  16070.2 |
  16346.7 |########
  16623.3 |
  16899.8 |
  17176.3 |
  17452.8 |
  17729.4 |########
  (2 below, 1 above range)

head_tail_dyn (n=18, range 13949.1-17371.8 ns)
  13949.1 |
  14120.2 |########################################
  14291.4 |##########################
  14462.5 |#############
  14633.6 |
  14804.8 |
  14975.9 |#############
  15147.0 |
  15318.2 |#############
  15489.3 |
  15660.4 |
  15831.6 |
  16002.7 |##########################
  16173.9 |########################################
  16345.0 |##########################
  16516.1 |
  16687.3 |
  16858.4 |
  17029.5 |
  17200.7 |
  (1 below, 2 above range)

kernel_nway (n=18, range 17033.2-26441.4 ns)
  17033.2 |##########
  17503.6 |
  17974.0 |########################################
  18444.4 |####################
  18914.8 |
  19385.3 |####################
  19855.7 |
  20326.1 |##########
  20796.5 |
  21266.9 |
  21737.3 |
  22207.7 |
  22678.1 |
  23148.6 |##########
  23619.0 |
  24089.4 |
  24559.8 |
  25030.2 |##########
  25500.6 |##########
  25971.0 |
  (2 below, 3 above range)

kernel_seq (n=18, range 12093.8-16191.4 ns)
  12093.8 |######################
  12298.7 |
  12503.6 |
  12708.5 |
  12913.3 |
  13118.2 |
  13323.1 |
  13528.0 |
  13732.8 |
  13937.7 |#####
  14142.6 |#####
  14347.5 |########################################
  14552.4 |#####
  14757.2 |
  14962.1 |
  15167.0 |
  15371.9 |
  15576.7 |
  15781.6 |
  15986.5 |
  (1 below, 3 above range)

nway (n=18, range 48975.6-66201.8 ns)
  48975.6 |##############################
  49836.9 |
  50698.2 |
  51559.5 |
  52420.8 |##########
  53282.1 |########################################
  54143.4 |####################
  55004.8 |
  55866.1 |
  56727.4 |
  57588.7 |
  58450.0 |
  59311.3 |##########
  60172.6 |##############################
  61034.0 |
  61895.3 |
  62756.6 |
  63617.9 |
  64479.2 |
  65340.5 |
  (1 below, 3 above range)

sequential (n=18, range 22867.6-30664.1 ns)
  22867.6 |########################################
  23257.4 |####################
  23647.2 |########################################
  24037.0 |
  24426.9 |
  24816.7 |
  25206.5 |########################################
  25596.4 |####################
  25986.2 |####################
  26376.0 |
  26765.8 |####################
  27155.7 |
  27545.5 |
  27935.3 |####################
  28325.2 |####################
  28715.0 |
  29104.8 |####################
  29494.6 |########################################
  29884.5 |
  30274.3 |
  (1 below, 2 above range)

std_parallel (n=18, range 65716.7-108404.8 ns)
  65716.7 |########################################
  67851.1 |########
  69985.5 |
  72119.9 |################
  74254.3 |########
  76388.7 |########
  78523.1 |################
  80657.5 |
  82791.9 |
  84926.3 |
  87060.8 |
  89195.2 |########
  91329.6 |########
  93464.0 |
  95598.4 |########
  97732.8 |
  99867.2 |
  102001.6 |
  104136.0 |
  106270.4 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=24.8% (high variance, measurements may be unstable)
- **head_tail**: bridge=101.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=104.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=104.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=103.6% of algo (FFI overhead may distort results)
- **nway**: bridge=103.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=104.8% of algo (FFI overhead may distort results)
- **std_parallel**: CV=29.2% (high variance, measurements may be unstable)
- **std_parallel**: bridge=99.5% of algo (FFI overhead may distort results)
