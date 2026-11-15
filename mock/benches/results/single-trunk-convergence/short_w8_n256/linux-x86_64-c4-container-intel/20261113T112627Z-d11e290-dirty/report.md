# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 177% faster than the next best (kernel_seq)

sequential (7.15 us) leads kernel_seq (19.77 us) by 177%, a clear separation rather than a photo finish. CV 7.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 74% (significant)

sequential is -21.14 us (74%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 23.5x slower than the field

std_parallel (167.67 us) is 23.5x the fastest (7.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.65)

nway's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 23.5x the fastest

Fastest sequential (7.15 us) to slowest std_parallel (167.67 us): 23.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.7x its best-20%

kernel_nway's best 20% of batches run at 59.50 us but its worst 20% at 100.08 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7149.0 ns median (-75.0% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 23.45x (fastest 7149.0 ns, slowest 167668.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 8 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100242360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 29695ns | 28923ns | 27417ns | 28818ns | 32150ns | base |
| head_tail_dyn | 30890ns | 31142ns | 25113ns | 30066ns | 35016ns | +4.02% |
| kernel_nway | 79470ns | 75834ns | 59914ns | 71742ns | 100840ns | +167.62% |
| kernel_seq | 20995ns | 20122ns | 19191ns | 20019ns | 23361ns | -29.30% |
| nway | 72247ns | 74514ns | 57800ns | 72463ns | 79146ns | +143.29% |
| sequential | 7660ns | 7390ns | 7265ns | 7414ns | 8226ns | -74.20% |
| std_parallel | 174729ns | 168115ns | 147447ns | 161732ns | 207867ns | +488.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 29267ns | 26853ns | 31719ns | base | 0.009 |
| head_tail_dyn | 30430ns | 24760ns | 34568ns | +3.97% | 0.008 |
| kernel_nway | 78860ns | 59501ns | 100080ns | +169.45% | 0.003 |
| kernel_seq | 20558ns | 18760ns | 22870ns | -29.76% | 0.012 |
| nway | 71877ns | 57438ns | 78760ns | +145.59% | 0.004 |
| sequential | 7423ns | 7035ns | 7989ns | -74.64% | 0.034 |
| std_parallel | 174129ns | 146890ns | 206967ns | +494.97% | 0.001 |

## Performance model

- Peak throughput: **0.036 Gops/s** (sequential; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.009 | 24.6% |
| head_tail_dyn | 0.008 | 23.1% |
| kernel_nway | 0.003 | 9.3% |
| kernel_seq | 0.013 | 35.6% |
| nway | 0.003 | 9.5% |
| sequential | 0.036 | 98.4% |
| std_parallel | 0.002 | 4.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 29695ns | 29695ns | base |
| head_tail_dyn | 30890ns | 30890ns | +4.02% |
| kernel_nway | 79470ns | 79470ns | +167.62% |
| kernel_seq | 20995ns | 20995ns | -29.30% |
| nway | 72247ns | 72247ns | +143.29% |
| sequential | 7660ns | 7660ns | -74.20% |
| std_parallel | 174729ns | 174729ns | +488.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 28565ns | base | --- | [27963, 30420] | --- | --- | --- | --- |
| head_tail_dyn | 30509ns | no significant difference | [-1358, +4007]ns | [28394, 32842] | no | 0.0963 | 0.0963 | 0 |
| kernel_nway | 75298ns | +46945.1ns (+164.3%) | [+36457, +53369]ns | [64698, 82438] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 19770ns | -8434.6ns (-29.5%) | [-10972, -7304]ns | [19293, 21550] | YES | 0.0000 | 0.0000 | 0 |
| nway | 74153ns | +43909.3ns (+153.7%) | [+39213, +47011]ns | [69201, 76087] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7149ns | -21142.1ns (-74.0%) | [-23095, -20792]ns | [7073, 7471] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 167668ns | +139584.1ns (+488.6%) | [+122102, +162211]ns | [150617, 190226] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 28015ns | -7.8% | +214.4% | -31.2% | +164.7% | -74.8% | +633.8% |
| 1 | 2 | 28255ns | +13.7% | +166.5% | -20.5% | +133.6% | -74.8% | +460.0% |
| 1 | 3 | 30420ns | +4.7% | +99.3% | -35.0% | +148.6% | -75.5% | +408.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.117 | ok |
| head_tail_dyn | -0.191 | ok |
| kernel_nway | -0.001 | ok |
| kernel_seq | -0.653 | HIGH- (thermal bounce) |
| nway | -0.653 | HIGH- (thermal bounce) |
| sequential | -0.104 | ok |
| std_parallel | -0.133 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 33110.1ns | 29266.6ns | 113.1% | HIGH |
| head_tail_dyn | 30844.2ns | 30429.8ns | 101.4% | HIGH |
| kernel_nway | 77493.3ns | 78860.0ns | 98.3% | HIGH |
| kernel_seq | 20586.4ns | 20557.7ns | 100.1% | HIGH |
| nway | 71101.0ns | 71877.3ns | 98.9% | HIGH |
| sequential | 7580.8ns | 7422.9ns | 102.1% | HIGH |
| std_parallel | 170222.2ns | 174128.5ns | 97.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 26853.0-31719.0 ns)
  26853.0 |
  27096.3 |##########
  27339.6 |
  27582.9 |
  27826.2 |########################################
  28069.5 |
  28312.8 |####################
  28556.1 |##########
  28799.4 |##########
  29042.7 |
  29286.0 |
  29529.3 |
  29772.6 |##########
  30015.9 |
  30259.2 |##########
  30502.5 |##########
  30745.8 |
  30989.1 |
  31232.4 |##########
  31475.7 |##########
  (2 below, 2 above range)

head_tail_dyn (n=18, range 24759.7-34568.3 ns)
  24759.7 |#############
  25250.2 |
  25740.6 |
  26231.0 |#############
  26721.5 |
  27211.9 |
  27702.3 |#############
  28192.7 |
  28683.2 |#############
  29173.6 |#############
  29664.0 |
  30154.5 |##########################
  30644.9 |########################################
  31135.3 |
  31625.8 |
  32116.2 |
  32606.6 |##########################
  33097.1 |#############
  33587.5 |
  34077.9 |#############
  (2 below, 2 above range)

kernel_nway (n=18, range 59501.4-100080.5 ns)
  59501.4 |########################################
  61530.3 |####################
  63559.3 |
  65588.2 |####################
  67617.2 |####################
  69646.1 |####################
  71675.1 |
  73704.1 |####################
  75733.0 |####################
  77762.0 |####################
  79790.9 |####################
  81819.9 |########################################
  83848.8 |
  85877.8 |####################
  87906.8 |####################
  89935.7 |
  91964.7 |####################
  93993.6 |
  96022.6 |
  98051.5 |
  (2 below, 1 above range)

kernel_seq (n=18, range 18760.0-22869.8 ns)
  18760.0 |
  18965.5 |########################################
  19171.0 |
  19376.5 |########################################
  19582.0 |#############
  19787.5 |##########################
  19992.9 |
  20198.4 |#############
  20403.9 |
  20609.4 |
  20814.9 |
  21020.4 |
  21225.9 |#############
  21431.4 |
  21636.9 |#############
  21842.4 |
  22047.9 |
  22253.3 |#############
  22458.8 |#############
  22664.3 |
  (2 below, 2 above range)

nway (n=18, range 57437.6-78760.3 ns)
  57437.6 |
  58503.8 |
  59569.9 |
  60636.0 |
  61702.2 |
  62768.3 |##########
  63834.4 |
  64900.6 |
  65966.7 |
  67032.8 |##########
  68099.0 |##########
  69165.1 |##########
  70231.2 |##########
  71297.4 |
  72363.5 |####################
  73429.6 |
  74495.8 |####################
  75561.9 |########################################
  76628.0 |##########
  77694.2 |
  (2 below, 2 above range)

sequential (n=18, range 7034.9-7988.9 ns)
   7034.9 |########################################
   7082.6 |####################
   7130.3 |####################
   7178.0 |
   7225.7 |
   7273.4 |
   7321.1 |
   7368.8 |
   7416.5 |##############################
   7464.2 |##########
   7511.9 |##########
   7559.6 |
   7607.3 |##########
   7655.0 |
   7702.7 |
   7750.4 |
   7798.1 |
   7845.8 |
   7893.5 |
   7941.2 |
  (2 below, 2 above range)

std_parallel (n=18, range 146889.8-206966.8 ns)
  146889.8 |########################################
  149893.6 |##########
  152897.5 |
  155901.3 |##########
  158905.2 |##########
  161909.0 |
  164912.9 |##########
  167916.7 |##########
  170920.6 |####################
  173924.4 |
  176928.3 |
  179932.1 |
  182936.0 |##########
  185939.8 |
  188943.7 |
  191947.5 |
  194951.4 |##########
  197955.2 |
  200959.1 |##########
  203962.9 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=104.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.2% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=30.1% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=102.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=102.3% of algo (FFI overhead may distort results)
- **nway**: bridge=99.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=103.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=95.4% of algo (FFI overhead may distort results)
