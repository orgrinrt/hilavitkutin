# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail dominates: 26% faster than the next best (head_tail_dyn)

head_tail (29.49 us) leads head_tail_dyn (37.15 us) by 26%, a clear separation rather than a photo finish. CV 115.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 3.9x slower than the field

std_parallel (114.68 us) is 3.9x the fastest (29.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.67)

kernel_nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (29.49 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.9x the fastest

Fastest head_tail (29.49 us) to slowest std_parallel (114.68 us): 3.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail_dyn is inconsistent: worst-20% is 4.5x its best-20%

head_tail_dyn's best 20% of batches run at 28.41 us but its worst 20% at 128.20 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 29493.5 ns median
- 6 variants significantly slower than baseline
- Spread: 3.89x (fastest 29493.5 ns, slowest 114678.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 1 |
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
| head_tail | 49376ns | 29681ns | 23736ns | 30201ns | 90958ns | base |
| head_tail_dyn | 69068ns | 37410ns | 28614ns | 41892ns | 130058ns | +39.88% |
| kernel_nway | 59039ns | 39211ns | 28738ns | 35086ns | 110120ns | +19.57% |
| kernel_seq | 56002ns | 40336ns | 28887ns | 38415ns | 95940ns | +13.42% |
| nway | 100073ns | 68099ns | 56760ns | 67214ns | 171020ns | +102.68% |
| sequential | 57102ns | 57070ns | 56865ns | 57008ns | 57362ns | +15.65% |
| std_parallel | 138491ns | 115067ns | 54604ns | 107596ns | 226778ns | +180.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 48781ns | 23574ns | 89540ns | base | 1.343 |
| head_tail_dyn | 68257ns | 28406ns | 128199ns | +39.93% | 0.960 |
| kernel_nway | 58345ns | 28548ns | 108481ns | +19.61% | 1.123 |
| kernel_seq | 55379ns | 28672ns | 94502ns | +13.53% | 1.183 |
| nway | 99673ns | 56587ns | 170173ns | +104.33% | 0.658 |
| sequential | 56923ns | 56683ns | 57179ns | +16.69% | 1.151 |
| std_parallel | 137615ns | 54379ns | 225106ns | +182.11% | 0.476 |

## Performance model

- Peak throughput: **2.780 Gops/s** (head_tail; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.222 | 79.9% |
| head_tail_dyn | 1.764 | 63.5% |
| kernel_nway | 1.683 | 60.5% |
| kernel_seq | 1.632 | 58.7% |
| nway | 0.965 | 34.7% |
| sequential | 1.152 | 41.4% |
| std_parallel | 0.571 | 20.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 49376ns | 49376ns | base |
| head_tail_dyn | 69068ns | 69068ns | +39.88% |
| kernel_nway | 59039ns | 59039ns | +19.57% |
| kernel_seq | 56002ns | 56002ns | +13.42% |
| nway | 100073ns | 100073ns | +102.68% |
| sequential | 57102ns | 57102ns | +15.65% |
| std_parallel | 138491ns | 138491ns | +180.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 29494ns | base | --- | [28342, 52927] | --- | --- | --- | --- |
| head_tail_dyn | 37148ns | +10065.5ns (+34.1%) | [+4732, +27114]ns | [32241, 84915] | YES | 0.0113 | 0.0075 | 0 |
| kernel_nway | 38938ns | +4906.3ns (+16.6%) | [+1020, +10610]ns | [29671, 46100] | YES | 0.0113 | 0.0075 | 0 |
| kernel_seq | 40151ns | +12309.5ns (+41.7%) | [+3527, +16060]ns | [36496, 47070] | YES | 0.0371 | 0.0309 | 0 |
| nway | 67920ns | +38762.6ns (+131.4%) | [+33500, +66708]ns | [67109, 116305] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 56888ns | +27403.0ns (+92.9%) | [+4245, +29119]ns | [56749, 56973] | YES (adj: no) | 0.0963 | 0.0963 | 0 |
| std_parallel | 114678ns | +86961.5ns (+294.8%) | [+41990, +138537]ns | [68345, 198116] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 24940ns | +116.6% | +23.9% | +62.4% | +130.3% | +128.0% | +262.2% |
| 1 | 2 | 52927ns | -37.0% | -21.5% | -45.2% | +33.2% | +7.3% | +265.8% |
| 1 | 3 | 29328ns | +75.5% | +6.8% | +40.4% | +129.9% | +94.2% | +155.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.652 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.658 | HIGH- (thermal bounce) |
| kernel_nway | -0.666 | HIGH- (thermal bounce) |
| kernel_seq | -0.665 | HIGH- (thermal bounce) |
| nway | -0.310 | moderate- |
| sequential | -0.327 | moderate- |
| std_parallel | -0.657 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 427643.9ns | 48780.5ns | 876.7% | HIGH |
| head_tail_dyn | 517375.8ns | 68257.0ns | 758.0% | HIGH |
| kernel_nway | 451905.2ns | 58345.2ns | 774.5% | HIGH |
| kernel_seq | 397781.5ns | 55378.9ns | 718.3% | HIGH |
| nway | 418170.5ns | 99672.7ns | 419.5% | HIGH |
| sequential | 296272.5ns | 56923.0ns | 520.5% | HIGH |
| std_parallel | 568185.8ns | 137615.3ns | 412.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 23573.7-89539.8 ns)
  23573.7 |#############
  26872.0 |########################################
  30170.3 |######
  33468.6 |
  36766.9 |######
  40065.2 |######
  43363.5 |
  46661.8 |
  49960.1 |
  53258.4 |
  56556.7 |
  59855.0 |
  63153.3 |######
  66451.7 |
  69750.0 |
  73048.3 |######
  76346.6 |
  79644.9 |
  82943.2 |
  86241.5 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 28406.3-128199.0 ns)
  28406.3 |########################################
  33395.9 |########################################
  38385.5 |
  43375.2 |
  48364.8 |
  53354.5 |
  58344.1 |
  63333.7 |##########
  68323.4 |####################
  73313.0 |
  78302.7 |
  83292.3 |
  88281.9 |
  93271.6 |##########
  98261.2 |
  103250.8 |
  108240.5 |
  113230.1 |
  118219.8 |
  123209.4 |
  (2 below, 4 above range)

kernel_nway (n=18, range 28548.3-108481.3 ns)
  28548.3 |########################################
  32545.0 |######
  36541.6 |######
  40538.3 |##########################
  44534.9 |
  48531.6 |######
  52528.2 |
  56524.9 |
  60521.5 |
  64518.2 |
  68514.8 |
  72511.5 |
  76508.1 |
  80504.8 |
  84501.4 |
  88498.0 |
  92494.7 |
  96491.3 |
  100488.0 |
  104484.6 |
  (1 below, 4 above range)

kernel_seq (n=18, range 28671.7-94502.3 ns)
  28671.7 |#################
  31963.2 |#####
  35254.7 |
  38546.3 |########################################
  41837.8 |#####
  45129.3 |
  48420.9 |
  51712.4 |#####
  55003.9 |
  58295.5 |#####
  61587.0 |
  64878.5 |
  68170.1 |
  71461.6 |
  74753.1 |
  78044.7 |
  81336.2 |
  84627.7 |
  87919.3 |
  91210.8 |
  (1 below, 3 above range)

nway (n=18, range 56587.3-170172.8 ns)
  56587.3 |################
  62266.6 |########################################
  67945.9 |########################
  73625.1 |
  79304.4 |
  84983.7 |
  90663.0 |
  96342.2 |########
  102021.5 |
  107700.8 |
  113380.1 |
  119059.3 |
  124738.6 |
  130417.9 |########
  136097.1 |
  141776.4 |
  147455.7 |
  153135.0 |
  158814.2 |########
  164493.5 |
  (2 below, 3 above range)

sequential (n=18, range 56683.0-57179.2 ns)
  56683.0 |########################################
  56707.8 |#############
  56732.6 |
  56757.5 |#############
  56782.3 |#############
  56807.1 |
  56831.9 |#############
  56856.7 |#############
  56881.5 |#############
  56906.3 |#############
  56931.1 |
  56955.9 |########################################
  56980.7 |##########################
  57005.5 |#############
  57030.3 |
  57055.1 |
  57079.9 |
  57104.7 |
  57129.5 |
  57154.3 |
  (1 below, 1 above range)

std_parallel (n=18, range 54379.3-225106.0 ns)
  54379.3 |##########################
  62915.6 |########################################
  71451.9 |#############
  79988.3 |#############
  88524.6 |
  97061.0 |#############
  105597.3 |
  114133.6 |
  122670.0 |#############
  131206.3 |
  139742.7 |
  148279.0 |
  156815.3 |
  165351.7 |
  173888.0 |
  182424.4 |##########################
  190960.7 |##########################
  199497.0 |#############
  208033.4 |#############
  216569.7 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=69.9% (high variance, measurements may be unstable)
- **head_tail**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **head_tail**: bridge=1230.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=70.6% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **head_tail_dyn**: bridge=1168.3% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=76.0% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **kernel_nway**: bridge=988.2% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=69.2% (high variance, measurements may be unstable)
- **kernel_seq**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **kernel_seq**: bridge=895.6% of algo (FFI overhead may distort results)
- **nway**: CV=56.2% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **nway**: bridge=464.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=520.5% of algo (FFI overhead may distort results)
- **std_parallel**: CV=54.1% (high variance, measurements may be unstable)
- **std_parallel**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **std_parallel**: bridge=405.2% of algo (FFI overhead may distort results)
