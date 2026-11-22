# fold_heavy, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 90% faster than the next best (head_tail_dyn)

sequential (18.87 us) leads head_tail_dyn (35.77 us) by 90%, a clear separation rather than a photo finish. CV 120.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn beats baseline by 29% (significant)

head_tail_dyn is -14.15 us (29%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 5.0x slower than the field

std_parallel (94.63 us) is 5.0x the fastest (18.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 120.0%)

sequential wins on median (18.87 us) yet has the highest variance (CV 120.0%), while std_parallel is the steadiest (CV 10.4%, 94.63 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.66)

kernel_nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.0x the fastest

Fastest sequential (18.87 us) to slowest std_parallel (94.63 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.5x its best-20%

sequential's best 20% of batches run at 18.52 us but its worst 20% at 65.00 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 18865.2 ns median (-61.9% vs baseline)
- 4 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 5.02x (fastest 18865.2 ns, slowest 94634.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 2 |
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
| head_tail | 50343ns | 50129ns | 28856ns | 48594ns | 63709ns | base |
| head_tail_dyn | 38162ns | 36529ns | 29201ns | 35010ns | 47372ns | -24.19% |
| kernel_nway | 42989ns | 36775ns | 25442ns | 35250ns | 63372ns | -14.61% |
| kernel_seq | 44003ns | 44366ns | 33864ns | 42394ns | 51485ns | -12.59% |
| nway | 81436ns | 75033ns | 63811ns | 73102ns | 102749ns | +61.76% |
| sequential | 35272ns | 19027ns | 18660ns | 20630ns | 65539ns | -29.94% |
| std_parallel | 97013ns | 95398ns | 86761ns | 92435ns | 109007ns | +92.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 49425ns | 28267ns | 62321ns | base | 0.083 |
| head_tail_dyn | 37255ns | 28680ns | 45854ns | -24.62% | 0.110 |
| kernel_nway | 42153ns | 25058ns | 61951ns | -14.71% | 0.097 |
| kernel_seq | 43062ns | 33396ns | 49751ns | -12.87% | 0.095 |
| nway | 80813ns | 63352ns | 101937ns | +63.51% | 0.051 |
| sequential | 34989ns | 18520ns | 65002ns | -29.21% | 0.117 |
| std_parallel | 96014ns | 85979ns | 107553ns | +94.26% | 0.043 |

## Performance model

- Peak throughput: **0.221 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.083 | 37.4% |
| head_tail_dyn | 0.115 | 51.8% |
| kernel_nway | 0.113 | 51.1% |
| kernel_seq | 0.094 | 42.4% |
| nway | 0.055 | 24.9% |
| sequential | 0.217 | 98.2% |
| std_parallel | 0.043 | 19.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 50343ns | 50343ns | base |
| head_tail_dyn | 38162ns | 38162ns | -24.19% |
| kernel_nway | 42989ns | 42989ns | -14.61% |
| kernel_seq | 44003ns | 44003ns | -12.59% |
| nway | 81436ns | 81436ns | +61.76% |
| sequential | 35272ns | 35272ns | -29.94% |
| std_parallel | 97013ns | 97013ns | +92.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 49455ns | base | --- | [46009, 53087] | --- | --- | --- | --- |
| head_tail_dyn | 35768ns | -14153.4ns (-28.6%) | [-18323, -3798]ns | [32476, 39309] | YES | 0.0463 | 0.0309 | 0 |
| kernel_nway | 36251ns | -4691.5ns (-9.5%) | [-18983, -895]ns | [29483, 47500] | YES | 0.0463 | 0.0309 | 0 |
| kernel_seq | 43692ns | no significant difference | [-9596, +934]ns | [39348, 45718] | no | 0.0963 | 0.0963 | 0 |
| nway | 74465ns | +31868.0ns (+64.4%) | [+22156, +37357]ns | [69194, 82192] | YES | 0.0004 | 0.0001 | 0 |
| sequential | 18865ns | -15635.8ns (-31.6%) | [-31477, -481]ns | [18705, 51454] | YES (adj: no) | 0.0963 | 0.0963 | 0 |
| std_parallel | 94635ns | +48790.4ns (+98.7%) | [+40243, +53540]ns | [88104, 99055] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 50298ns | -33.4% | -40.8% | -17.2% | +51.9% | -53.1% | +84.6% |
| 1 | 2 | 52354ns | -26.6% | -13.0% | -14.4% | +55.6% | -64.2% | +81.1% |
| 1 | 3 | 40686ns | -10.9% | -24.0% | -0.5% | +67.0% | -6.8% | +122.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.270 | moderate- |
| head_tail_dyn | -0.456 | moderate- |
| kernel_nway | -0.664 | HIGH- (thermal bounce) |
| kernel_seq | -0.623 | HIGH- (thermal bounce) |
| nway | -0.410 | moderate- |
| sequential | -0.322 | moderate- |
| std_parallel | -0.475 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 48598.5ns | 49425.0ns | 98.3% | HIGH |
| head_tail_dyn | 38540.6ns | 37254.8ns | 103.5% | HIGH |
| kernel_nway | 38715.5ns | 42152.5ns | 91.8% | HIGH |
| kernel_seq | 43155.0ns | 43061.7ns | 100.2% | HIGH |
| nway | 81891.7ns | 80813.3ns | 101.3% | HIGH |
| sequential | 35599.6ns | 34989.3ns | 101.7% | HIGH |
| std_parallel | 98632.5ns | 96014.1ns | 102.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 28267.2-62321.4 ns)
  28267.2 |
  29969.9 |
  31672.6 |
  33375.3 |
  35078.0 |#############
  36780.8 |
  38483.5 |
  40186.2 |
  41888.9 |
  43591.6 |#############
  45294.3 |##########################
  46997.0 |##########################
  48699.7 |########################################
  50402.5 |##########################
  52105.2 |
  53807.9 |##########################
  55510.6 |
  57213.3 |
  58916.0 |
  60618.7 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 28680.5-45853.9 ns)
  28680.5 |########################################
  29539.1 |
  30397.8 |####################
  31256.5 |
  32115.1 |########################################
  32973.8 |########################################
  33832.5 |
  34691.2 |####################
  35549.8 |####################
  36408.5 |####################
  37267.2 |####################
  38125.8 |####################
  38984.5 |####################
  39843.2 |
  40701.9 |
  41560.5 |
  42419.2 |
  43277.9 |
  44136.5 |########################################
  44995.2 |
  (1 below, 2 above range)

kernel_nway (n=18, range 25057.7-61950.7 ns)
  25057.7 |
  26902.3 |####################
  28747.0 |########################################
  30591.6 |####################
  32436.3 |
  34280.9 |
  36125.6 |
  37970.2 |
  39814.9 |##########
  41659.5 |
  43504.2 |##########
  45348.8 |####################
  47193.5 |##########
  49038.1 |##########
  50882.8 |
  52727.4 |##########
  54572.1 |
  56416.7 |
  58261.4 |
  60106.0 |
  (1 below, 2 above range)

kernel_seq (n=18, range 33395.9-49750.9 ns)
  33395.9 |
  34213.7 |
  35031.4 |
  35849.2 |
  36666.9 |#############
  37484.7 |#############
  38302.4 |##########################
  39120.2 |#############
  39937.9 |#############
  40755.7 |#############
  41573.4 |
  42391.1 |#############
  43208.9 |
  44026.6 |########################################
  44844.4 |#############
  45662.1 |
  46479.9 |#############
  47297.6 |#############
  48115.4 |#############
  48933.1 |
  (1 below, 2 above range)

nway (n=18, range 63352.5-101937.3 ns)
  63352.5 |
  65281.7 |####################
  67211.0 |########################################
  69140.2 |########################################
  71069.4 |####################
  72998.7 |####################
  74927.9 |####################
  76857.2 |####################
  78786.4 |########################################
  80715.6 |
  82644.9 |####################
  84574.1 |
  86503.4 |
  88432.6 |
  90361.9 |
  92291.1 |
  94220.3 |
  96149.6 |####################
  98078.8 |
  100008.1 |
  (2 below, 3 above range)

sequential (n=18, range 18520.0-65002.1 ns)
  18520.0 |########################################
  20844.1 |
  23168.2 |#####
  25492.3 |
  27816.4 |#####
  30140.5 |
  32464.6 |
  34788.7 |
  37112.8 |
  39436.9 |
  41761.0 |
  44085.1 |
  46409.2 |
  48733.3 |
  51057.4 |#####
  53381.5 |#####
  55705.6 |
  58029.7 |#####
  60353.8 |
  62677.9 |#####
  (2 below, 2 above range)

std_parallel (n=18, range 85978.9-107553.0 ns)
  85978.9 |########################################
  87057.6 |
  88136.3 |
  89215.0 |##########
  90293.7 |##########
  91372.4 |##########
  92451.1 |
  93529.8 |##########
  94608.5 |##############################
  95687.2 |
  96765.9 |##########
  97844.6 |
  98923.3 |
  100002.0 |##########
  101080.7 |
  102159.4 |##########
  103238.1 |
  104316.8 |
  105395.5 |
  106474.2 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=26.3% (high variance, measurements may be unstable)
- **head_tail**: bridge=99.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=20.8% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=98.7% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=43.6% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=95.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.4% of algo (FFI overhead may distort results)
- **nway**: CV=22.6% (high variance, measurements may be unstable)
- **nway**: bridge=104.3% of algo (FFI overhead may distort results)
- **sequential**: CV=64.7% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **sequential**: bridge=108.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=99.6% of algo (FFI overhead may distort results)
