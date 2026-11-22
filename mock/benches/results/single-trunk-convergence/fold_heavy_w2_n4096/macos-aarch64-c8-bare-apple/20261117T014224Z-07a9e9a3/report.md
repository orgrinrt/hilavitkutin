# fold_heavy, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 16% faster than the next best (kernel_nway)

kernel_seq (5.63 us) leads kernel_nway (6.51 us) by 16%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 6.0x slower than the field

std_parallel (33.67 us) is 6.0x the fastest (5.63 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.0x the fastest

Fastest kernel_seq (5.63 us) to slowest std_parallel (33.67 us): 6.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 5633.4 ns median (-14.1% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 5.98x (fastest 5633.4 ns, slowest 33670.8 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 7564ns | 7229ns | 6871ns | 7167ns | 8507ns | base |
| head_tail_dyn | 7415ns | 7285ns | 6843ns | 7184ns | 8047ns | -1.97% |
| kernel_nway | 7194ns | 7140ns | 6967ns | 7104ns | 7442ns | -4.90% |
| kernel_seq | 6101ns | 6040ns | 5822ns | 6022ns | 6360ns | -19.34% |
| nway | 21982ns | 21846ns | 21581ns | 21835ns | 22404ns | +190.61% |
| sequential | 13186ns | 13065ns | 12690ns | 13043ns | 13649ns | +74.32% |
| std_parallel | 35480ns | 34033ns | 32049ns | 33860ns | 39626ns | +369.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 6875ns | 6268ns | 7724ns | base | 0.596 |
| head_tail_dyn | 6787ns | 6288ns | 7388ns | -1.27% | 0.603 |
| kernel_nway | 6566ns | 6321ns | 6819ns | -4.49% | 0.624 |
| kernel_seq | 5627ns | 5393ns | 5852ns | -18.16% | 0.728 |
| nway | 21425ns | 21062ns | 21833ns | +211.65% | 0.191 |
| sequential | 13038ns | 12540ns | 13490ns | +89.65% | 0.314 |
| std_parallel | 35078ns | 31651ns | 39209ns | +410.25% | 0.117 |

## Performance model

- Peak throughput: **0.759 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.624 | 82.2% |
| head_tail_dyn | 0.617 | 81.3% |
| kernel_nway | 0.629 | 82.8% |
| kernel_seq | 0.727 | 95.7% |
| nway | 0.192 | 25.3% |
| sequential | 0.317 | 41.8% |
| std_parallel | 0.122 | 16.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 7564ns | 7564ns | base |
| head_tail_dyn | 7415ns | 7415ns | -1.97% |
| kernel_nway | 7194ns | 7194ns | -4.90% |
| kernel_seq | 6101ns | 6101ns | -19.34% |
| nway | 21982ns | 21982ns | +190.61% |
| sequential | 13186ns | 13186ns | +74.32% |
| std_parallel | 35480ns | 35480ns | +369.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 6560ns | base | --- | [6423, 7285] | --- | --- | --- | --- |
| head_tail_dyn | 6635ns | no significant difference | [-627, +546]ns | [6392, 6967] | no | 0.7549 | 0.6291 | 1 |
| kernel_nway | 6512ns | no significant difference | [-675, +65]ns | [6421, 6594] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 5633ns | -922.9ns (-14.1%) | [-1485, -854]ns | [5465, 5710] | YES | 0.0000 | 0.0000 | 0 |
| nway | 21290ns | +14716.6ns (+224.3%) | [+14146, +14919]ns | [21223, 21438] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 12904ns | +6112.6ns (+93.2%) | [+5998, +6362]ns | [12812, 13204] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 33671ns | +26458.3ns (+403.3%) | [+26075, +29646]ns | [32773, 36454] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 6406ns | -0.7% | +2.7% | -15.2% | +238.3% | +100.0% | +491.8% |
| 1 | 2 | 7423ns | -12.2% | -13.7% | -25.8% | +186.7% | +78.3% | +352.5% |
| 1 | 3 | 6531ns | +12.9% | -0.3% | -12.7% | +225.8% | +96.8% | +393.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.658 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.093 | ok |
| kernel_nway | -0.571 | HIGH- (thermal bounce) |
| kernel_seq | -0.042 | ok |
| nway | -0.167 | ok |
| sequential | -0.662 | HIGH- (thermal bounce) |
| std_parallel | -0.055 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 87302.8ns | 6874.8ns | 1269.9% | HIGH |
| head_tail_dyn | 88268.3ns | 6787.3ns | 1300.5% | HIGH |
| kernel_nway | 87723.9ns | 6566.0ns | 1336.0% | HIGH |
| kernel_seq | 88121.3ns | 5626.6ns | 1566.2% | HIGH |
| nway | 102841.4ns | 21425.2ns | 480.0% | HIGH |
| sequential | 90557.6ns | 13038.0ns | 694.6% | HIGH |
| std_parallel | 106545.6ns | 35078.2ns | 303.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 6268.1-7723.6 ns)
   6268.1 |##########################
   6340.8 |#############
   6413.6 |########################################
   6486.4 |##########################
   6559.2 |##########################
   6631.9 |#############
   6704.7 |
   6777.5 |
   6850.3 |
   6923.1 |
   6995.8 |
   7068.6 |
   7141.4 |
   7214.2 |#############
   7286.9 |##########################
   7359.7 |
   7432.5 |
   7505.3 |#############
   7578.0 |
   7650.8 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 6287.5-7388.2 ns)
   6287.5 |########################################
   6342.5 |#############
   6397.6 |#############
   6452.6 |#############
   6507.6 |#############
   6562.7 |#############
   6617.7 |
   6672.8 |#############
   6727.8 |#############
   6782.8 |#############
   6837.9 |#############
   6892.9 |
   6947.9 |
   7003.0 |
   7058.0 |#############
   7113.0 |#############
   7168.1 |
   7223.1 |
   7278.1 |
   7333.2 |
  (1 below, 3 above range)

kernel_nway (n=18, range 6320.8-6818.8 ns)
   6320.8 |#############
   6345.7 |##########################
   6370.6 |#############
   6395.5 |
   6420.4 |
   6445.3 |#############
   6470.2 |#############
   6495.1 |########################################
   6520.0 |#############
   6544.9 |#############
   6569.8 |#############
   6594.7 |#############
   6619.6 |
   6644.5 |##########################
   6669.4 |
   6694.3 |
   6719.2 |
   6744.1 |
   6769.0 |
   6793.9 |
  (1 below, 2 above range)

kernel_seq (n=18, range 5393.1-5852.1 ns)
   5393.1 |########################################
   5416.0 |####################
   5439.0 |####################
   5461.9 |########################################
   5484.9 |
   5507.8 |
   5530.8 |####################
   5553.7 |
   5576.7 |
   5599.6 |####################
   5622.6 |
   5645.5 |########################################
   5668.5 |####################
   5691.4 |####################
   5714.4 |####################
   5737.3 |
   5760.3 |########################################
   5783.2 |
   5806.2 |
   5829.1 |####################
  (1 below, 1 above range)

nway (n=18, range 21062.5-21832.6 ns)
  21062.5 |
  21101.0 |
  21139.5 |##########
  21178.0 |##########
  21216.5 |####################
  21255.0 |########################################
  21293.5 |##########
  21332.1 |
  21370.6 |####################
  21409.1 |
  21447.6 |##########
  21486.1 |
  21524.6 |
  21563.1 |
  21601.6 |##########
  21640.1 |
  21678.6 |
  21717.1 |##########
  21755.6 |
  21794.1 |
  (2 below, 2 above range)

sequential (n=18, range 12540.3-13490.3 ns)
  12540.3 |#############
  12587.8 |
  12635.3 |
  12682.8 |#############
  12730.3 |
  12777.8 |########################################
  12825.3 |#############
  12872.8 |##########################
  12920.3 |
  12967.8 |
  13015.3 |
  13062.8 |
  13110.3 |#############
  13157.8 |##########################
  13205.3 |#############
  13252.8 |##########################
  13300.3 |#############
  13347.8 |
  13395.3 |
  13442.8 |
  (2 below, 1 above range)

std_parallel (n=18, range 31651.4-39209.0 ns)
  31651.4 |
  32029.3 |########################################
  32407.2 |####################
  32785.0 |####################
  33162.9 |########################################
  33540.8 |########################################
  33918.7 |####################
  34296.6 |
  34674.5 |####################
  35052.3 |
  35430.2 |
  35808.1 |####################
  36186.0 |
  36563.9 |####################
  36941.7 |####################
  37319.6 |
  37697.5 |
  38075.4 |
  38453.3 |####################
  38831.2 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=1318.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=1312.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1349.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1562.8% of algo (FFI overhead may distort results)
- **nway**: bridge=488.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=698.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=316.2% of algo (FFI overhead may distort results)
