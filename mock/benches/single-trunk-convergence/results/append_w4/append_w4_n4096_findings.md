# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 14% faster than the next best (kernel_seq)

sequential (19.20 us) leads kernel_seq (21.90 us) by 14%, a clear separation rather than a photo finish. CV 22.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 44% (significant)

sequential is -12.23 us (44%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 5.0x slower than the field

std_parallel (96.16 us) is 5.0x the fastest (19.20 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.65)

sequential's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.0x the fastest

Fastest sequential (19.20 us) to slowest std_parallel (96.16 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_seq is inconsistent: worst-20% is 3.7x its best-20%

kernel_seq's best 20% of batches run at 20.27 us but its worst 20% at 75.22 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 19203.3 ns median (-30.2% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 5.01x (fastest 19203.3 ns, slowest 96156.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 30035ns | 28388ns | 24295ns | 28067ns | 35858ns | base |
| head_tail_dyn | 27427ns | 26739ns | 21084ns | 25708ns | 33176ns | -8.69% |
| kernel_nway | 42206ns | 41963ns | 36459ns | 40974ns | 46929ns | +40.52% |
| kernel_seq | 39951ns | 22580ns | 20659ns | 22381ns | 75952ns | +33.01% |
| nway | 62983ns | 58345ns | 54478ns | 58048ns | 74639ns | +109.70% |
| sequential | 17720ns | 19585ns | 11258ns | 17207ns | 21719ns | -41.00% |
| std_parallel | 96630ns | 96668ns | 81295ns | 93389ns | 109160ns | +221.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 29439ns | 23927ns | 35259ns | base | 0.139 |
| head_tail_dyn | 26868ns | 20505ns | 32505ns | -8.73% | 0.152 |
| kernel_nway | 41569ns | 36039ns | 46109ns | +41.20% | 0.099 |
| kernel_seq | 39404ns | 20273ns | 75218ns | +33.85% | 0.104 |
| nway | 62490ns | 54056ns | 74012ns | +112.27% | 0.066 |
| sequential | 17220ns | 11008ns | 21175ns | -41.51% | 0.238 |
| std_parallel | 96011ns | 80831ns | 108321ns | +226.14% | 0.043 |

## Performance model

- Peak throughput: **0.372 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.149 | 40.0% |
| head_tail_dyn | 0.157 | 42.1% |
| kernel_nway | 0.099 | 26.5% |
| kernel_seq | 0.187 | 50.3% |
| nway | 0.071 | 19.0% |
| sequential | 0.213 | 57.3% |
| std_parallel | 0.043 | 11.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 30035ns | 30035ns | base |
| head_tail_dyn | 27427ns | 27427ns | -8.69% |
| kernel_nway | 42206ns | 42206ns | +40.52% |
| kernel_seq | 39951ns | 39951ns | +33.01% |
| nway | 62983ns | 62983ns | +109.70% |
| sequential | 17720ns | 17720ns | -41.00% |
| std_parallel | 96630ns | 96630ns | +221.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 27494ns | base | --- | [26555, 30823] | --- | --- | --- | --- |
| head_tail_dyn | 26170ns | no significant difference | [-4628, +869]ns | [23669, 29877] | no | 0.0963 | 0.0963 | 0 |
| kernel_nway | 41522ns | +11625.3ns (+42.3%) | [+8666, +17106]ns | [38423, 44574] | YES | 0.0002 | 0.0001 | 0 |
| kernel_seq | 21902ns | -5544.9ns (-20.2%) | [-7529, -2596]ns | [20958, 26216] | YES | 0.0090 | 0.0075 | 0 |
| nway | 57959ns | +31593.5ns (+114.9%) | [+28704, +35744]ns | [55832, 66958] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 19203ns | -12226.7ns (-44.5%) | [-14150, -9478]ns | [11792, 20379] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 96156ns | +63338.1ns (+230.4%) | [+59976, +73295]ns | [86662, 103354] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 30823ns | -15.1% | +35.4% | -33.9% | +80.8% | -36.5% | +212.0% |
| 1 | 2 | 25482ns | -17.2% | +75.3% | -15.0% | +121.1% | -56.8% | +231.0% |
| 1 | 3 | 27312ns | +17.8% | +40.7% | +9.8% | +156.3% | -21.6% | +284.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.388 | moderate- |
| head_tail_dyn | -0.472 | moderate- |
| kernel_nway | -0.480 | moderate- |
| kernel_seq | -0.102 | ok |
| nway | -0.145 | ok |
| sequential | -0.649 | HIGH- (thermal bounce) |
| std_parallel | -0.547 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 51172.3ns | 29438.7ns | 173.8% | HIGH |
| head_tail_dyn | 48848.6ns | 26867.5ns | 181.8% | HIGH |
| kernel_nway | 61845.7ns | 41568.7ns | 148.8% | HIGH |
| kernel_seq | 46403.4ns | 39404.2ns | 117.8% | HIGH |
| nway | 84945.9ns | 62489.7ns | 135.9% | HIGH |
| sequential | 39480.0ns | 17220.0ns | 229.3% | HIGH |
| std_parallel | 120739.4ns | 96010.7ns | 125.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 23927.3-35258.9 ns)
  23927.3 |
  24493.9 |#############
  25060.4 |
  25627.0 |
  26193.6 |##########################
  26760.2 |########################################
  27326.8 |##########################
  27893.4 |
  28459.9 |#############
  29026.5 |#############
  29593.1 |
  30159.7 |#############
  30726.3 |#############
  31292.9 |
  31859.4 |
  32426.0 |##########################
  32992.6 |
  33559.2 |
  34125.8 |
  34692.4 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 20504.8-32505.3 ns)
  20504.8 |########################################
  21104.9 |####################
  21704.9 |
  22304.9 |
  22904.9 |####################
  23505.0 |
  24105.0 |####################
  24705.0 |########################################
  25305.0 |####################
  25905.1 |
  26505.1 |####################
  27105.1 |####################
  27705.1 |####################
  28305.1 |
  28905.2 |####################
  29505.2 |
  30105.2 |####################
  30705.2 |####################
  31305.3 |
  31905.3 |
  (1 below, 3 above range)

kernel_nway (n=18, range 36038.9-46108.8 ns)
  36038.9 |####################
  36542.4 |########################################
  37045.9 |
  37549.4 |
  38052.9 |####################
  38556.4 |####################
  39059.9 |########################################
  39563.4 |
  40066.9 |####################
  40570.4 |
  41073.9 |
  41577.4 |
  42080.9 |
  42584.4 |####################
  43087.9 |
  43591.4 |####################
  44094.9 |########################################
  44598.4 |########################################
  45101.9 |####################
  45605.4 |
  (1 below, 2 above range)

kernel_seq (n=18, range 20272.8-75217.7 ns)
  20272.8 |########################################
  23020.1 |####
  25767.3 |############
  28514.6 |
  31261.8 |####
  34009.0 |
  36756.3 |
  39503.5 |
  42250.8 |
  44998.0 |
  47745.2 |
  50492.5 |
  53239.7 |
  55987.0 |
  58734.2 |
  61481.4 |
  64228.7 |
  66975.9 |
  69723.2 |
  72470.4 |
  (1 below, 2 above range)

nway (n=18, range 54055.6-74011.6 ns)
  54055.6 |##########
  55053.4 |########################################
  56051.2 |##############################
  57049.0 |
  58046.8 |
  59044.6 |##########
  60042.4 |
  61040.2 |####################
  62038.0 |
  63035.8 |
  64033.6 |
  65031.4 |
  66029.2 |##########
  67027.0 |##########
  68024.8 |##########
  69022.6 |
  70020.4 |
  71018.2 |##########
  72016.0 |
  73013.8 |
  (1 below, 2 above range)

sequential (n=18, range 11008.2-21175.2 ns)
  11008.2 |##########################
  11516.6 |##########################
  12024.9 |
  12533.3 |
  13041.6 |
  13550.0 |
  14058.3 |
  14566.7 |
  15075.0 |
  15583.4 |
  16091.7 |
  16600.1 |
  17108.4 |
  17616.8 |
  18125.1 |#############
  18633.5 |##########################
  19141.8 |#############
  19650.2 |########################################
  20158.5 |
  20666.9 |#############
  (2 below, 4 above range)

std_parallel (n=18, range 80831.4-108321.1 ns)
  80831.4 |
  82205.9 |
  83580.4 |########################################
  84954.8 |
  86329.3 |
  87703.8 |#############
  89078.3 |
  90452.8 |#############
  91827.3 |
  93201.7 |#############
  94576.2 |#############
  95950.7 |#############
  97325.2 |#############
  98699.7 |
  100074.2 |#############
  101448.6 |#############
  102823.1 |
  104197.6 |##########################
  105572.1 |
  106946.6 |#############
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=184.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=176.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=146.4% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=137.0% (high variance, measurements may be unstable)
- **kernel_seq**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **kernel_seq**: bridge=198.2% of algo (FFI overhead may distort results)
- **nway**: bridge=143.5% of algo (FFI overhead may distort results)
- **sequential**: CV=25.1% (high variance, measurements may be unstable)
- **sequential**: bridge=209.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=126.2% of algo (FFI overhead may distort results)
