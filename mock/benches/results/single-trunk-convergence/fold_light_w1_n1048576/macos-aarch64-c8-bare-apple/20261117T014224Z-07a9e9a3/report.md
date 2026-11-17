# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (head_tail, head_tail_dyn) are a dead heat (<1%)

head_tail (92.11 us) and head_tail_dyn (92.60 us) differ by 0.52%, inside the noise, even though the wider field spreads 19.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### std_parallel shows alternating (throttle bounce) (autocorr -0.65)

std_parallel's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (92.11 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (head_tail) is the fastest** at 92114.6 ns median
- 5 variants significantly slower than baseline
- Spread: 1.19x (fastest 92114.6 ns, slowest 109945.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 1 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 93132ns | 92294ns | 89926ns | 91919ns | 96553ns | base |
| head_tail_dyn | 92426ns | 92777ns | 90114ns | 92180ns | 93951ns | -0.76% |
| kernel_nway | 97331ns | 97485ns | 94003ns | 96695ns | 99948ns | +4.51% |
| kernel_seq | 94579ns | 94615ns | 93926ns | 94464ns | 95077ns | +1.55% |
| nway | 103281ns | 103277ns | 102942ns | 103200ns | 103570ns | +10.90% |
| sequential | 98539ns | 97494ns | 96849ns | 97440ns | 101031ns | +5.81% |
| std_parallel | 109689ns | 110177ns | 106711ns | 109577ns | 111347ns | +17.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 92758ns | 89738ns | 95899ns | base | 11.304 |
| head_tail_dyn | 92241ns | 89925ns | 93760ns | -0.56% | 11.368 |
| kernel_nway | 96886ns | 93821ns | 99408ns | +4.45% | 10.823 |
| kernel_seq | 94368ns | 93744ns | 94889ns | +1.74% | 11.112 |
| nway | 103070ns | 102735ns | 103358ns | +11.12% | 10.173 |
| sequential | 98322ns | 96661ns | 100811ns | +6.00% | 10.665 |
| std_parallel | 109448ns | 106493ns | 111078ns | +17.99% | 9.581 |

## Performance model

- Peak throughput: **11.685 Gops/s** (head_tail; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 11.383 | 97.4% |
| head_tail_dyn | 11.324 | 96.9% |
| kernel_nway | 10.821 | 92.6% |
| kernel_seq | 11.109 | 95.1% |
| nway | 10.174 | 87.1% |
| sequential | 10.781 | 92.3% |
| std_parallel | 9.537 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 93132ns | 93132ns | base |
| head_tail_dyn | 92426ns | 92426ns | -0.76% |
| kernel_nway | 97331ns | 97331ns | +4.51% |
| kernel_seq | 94579ns | 94579ns | +1.55% |
| nway | 103281ns | 103281ns | +10.90% |
| sequential | 98539ns | 98539ns | +5.81% |
| std_parallel | 109689ns | 109689ns | +17.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 92115ns | base | --- | [91156, 93404] | --- | --- | --- | --- |
| head_tail_dyn | 92598ns | no significant difference | [-2179, +2192]ns | [91204, 93481] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 96906ns | +4679.2ns (+5.1%) | [+2204, +5983]ns | [95017, 98408] | YES | 0.0003 | 0.0001 | 0 |
| kernel_seq | 94390ns | +2689.6ns (+2.9%) | [+598, +3381]ns | [93981, 94715] | YES (adj: no) | 0.1155 | 0.0963 | 0 |
| nway | 103062ns | +11081.2ns (+12.0%) | [+9458, +12077]ns | [102862, 103227] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 97260ns | +5116.7ns (+5.6%) | [+3877, +8954]ns | [96869, 99946] | YES | 0.0020 | 0.0013 | 0 |
| std_parallel | 109946ns | +17235.4ns (+18.7%) | [+15079, +19233]ns | [108631, 110631] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 91823ns | +0.6% | +3.0% | +2.8% | +12.3% | +5.3% | +20.4% |
| 1 | 2 | 95792ns | -5.4% | +2.9% | -2.0% | +7.6% | +1.1% | +12.6% |
| 1 | 3 | 90504ns | +3.3% | +6.7% | +4.7% | +13.7% | +11.3% | +21.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.628 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.561 | HIGH- (thermal bounce) |
| kernel_nway | -0.503 | HIGH- (thermal bounce) |
| kernel_seq | -0.589 | HIGH- (thermal bounce) |
| nway | -0.003 | ok |
| sequential | -0.141 | ok |
| std_parallel | -0.651 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 93803.7ns | 92757.6ns | 101.1% | HIGH |
| head_tail_dyn | 92888.2ns | 92240.7ns | 100.7% | HIGH |
| kernel_nway | 97498.6ns | 96886.1ns | 100.6% | HIGH |
| kernel_seq | 94352.1ns | 94367.8ns | 100.0% | HIGH |
| nway | 103466.2ns | 103069.9ns | 100.4% | HIGH |
| sequential | 98375.7ns | 98322.4ns | 100.1% | HIGH |
| std_parallel | 109752.3ns | 109447.7ns | 100.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 89737.5-95899.3 ns)
  89737.5 |
  90045.6 |#############
  90353.7 |
  90661.8 |#############
  90969.9 |##########################
  91278.0 |#############
  91586.0 |#############
  91894.1 |#############
  92202.2 |########################################
  92510.3 |
  92818.4 |#############
  93126.5 |
  93434.6 |
  93742.7 |#############
  94050.8 |
  94358.9 |
  94667.0 |
  94975.0 |
  95283.1 |
  95591.2 |#############
  (2 below, 3 above range)

head_tail_dyn (n=18, range 89925.0-93759.7 ns)
  89925.0 |#############
  90116.7 |
  90308.5 |#############
  90500.2 |
  90691.9 |##########################
  90883.7 |
  91075.4 |#############
  91267.2 |
  91458.9 |#############
  91650.6 |
  91842.4 |
  92034.1 |#############
  92225.8 |
  92417.6 |#############
  92609.3 |#############
  92801.0 |
  92992.8 |##########################
  93184.5 |#############
  93376.2 |
  93568.0 |########################################
  (1 below, 2 above range)

kernel_nway (n=18, range 93820.8-99408.3 ns)
  93820.8 |
  94100.2 |####################
  94379.6 |
  94659.0 |########################################
  94938.3 |####################
  95217.7 |
  95497.1 |####################
  95776.5 |
  96055.8 |####################
  96335.2 |
  96614.6 |####################
  96894.0 |####################
  97173.3 |####################
  97452.7 |
  97732.1 |
  98011.5 |####################
  98290.8 |########################################
  98570.2 |####################
  98849.6 |
  99129.0 |
  (2 below, 3 above range)

kernel_seq (n=18, range 93744.5-94888.9 ns)
  93744.5 |####################
  93801.7 |####################
  93858.9 |
  93916.1 |####################
  93973.4 |####################
  94030.6 |
  94087.8 |
  94145.0 |
  94202.2 |####################
  94259.5 |####################
  94316.7 |
  94373.9 |########################################
  94431.1 |####################
  94488.3 |####################
  94545.6 |
  94602.8 |
  94660.0 |####################
  94717.2 |########################################
  94774.4 |
  94831.7 |
  (2 below, 3 above range)

nway (n=18, range 102734.7-103358.3 ns)
  102734.7 |
  102765.9 |####################
  102797.1 |####################
  102828.3 |####################
  102859.5 |####################
  102890.6 |
  102921.8 |
  102953.0 |
  102984.2 |########################################
  103015.4 |
  103046.5 |########################################
  103077.7 |####################
  103108.9 |
  103140.1 |
  103171.3 |########################################
  103202.4 |
  103233.6 |
  103264.8 |####################
  103296.0 |####################
  103327.2 |
  (2 below, 3 above range)

sequential (n=18, range 96661.1-100811.1 ns)
  96661.1 |########################################
  96868.6 |########################################
  97076.1 |
  97283.6 |
  97491.1 |####################
  97698.6 |
  97906.1 |
  98113.6 |
  98321.1 |
  98528.6 |##########
  98736.1 |
  98943.6 |
  99151.1 |
  99358.6 |
  99566.1 |
  99773.6 |##########
  99981.1 |
  100188.6 |##########
  100396.1 |##########
  100603.6 |
  (1 below, 3 above range)

std_parallel (n=18, range 106493.1-111078.5 ns)
  106493.1 |####################
  106722.3 |####################
  106951.6 |
  107180.9 |
  107410.1 |
  107639.4 |
  107868.7 |####################
  108098.0 |
  108327.2 |####################
  108556.5 |
  108785.8 |########################################
  109015.0 |
  109244.3 |####################
  109473.6 |
  109702.8 |####################
  109932.1 |####################
  110161.4 |########################################
  110390.7 |####################
  110619.9 |####################
  110849.2 |####################
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=101.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.9% of algo (FFI overhead may distort results)
- **nway**: bridge=100.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.2% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=99.9% of algo (FFI overhead may distort results)
