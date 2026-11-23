# fold_heavy, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 40% faster than the next best (kernel_nway)

sequential (24.42 us) leads kernel_nway (34.27 us) by 40%, a clear separation rather than a photo finish. CV 85.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 4.6x slower than the field

std_parallel (112.66 us) is 4.6x the fastest (24.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 85.0%)

sequential wins on median (24.42 us) yet has the highest variance (CV 85.0%), while nway is the steadiest (CV 9.8%, 62.71 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.58)

kernel_seq's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.6x the fastest

Fastest sequential (24.42 us) to slowest std_parallel (112.66 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.4x its best-20%

sequential's best 20% of batches run at 18.56 us but its worst 20% at 62.63 us (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 24417.2 ns median (-39.7% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 4.61x (fastest 24417.2 ns, slowest 112661.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112792619 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 42353ns | 41095ns | 26046ns | 38412ns | 56418ns | base |
| head_tail_dyn | 42155ns | 40581ns | 25156ns | 39205ns | 55078ns | -0.47% |
| kernel_nway | 36399ns | 34906ns | 31622ns | 33868ns | 42584ns | -14.06% |
| kernel_seq | 51243ns | 48079ns | 29750ns | 42085ns | 75727ns | +20.99% |
| nway | 64022ns | 63328ns | 57485ns | 61401ns | 71223ns | +51.16% |
| sequential | 36199ns | 24626ns | 18706ns | 24056ns | 63161ns | -14.53% |
| std_parallel | 108292ns | 113414ns | 78578ns | 107587ns | 124207ns | +155.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 41521ns | 25646ns | 55336ns | base | 0.099 |
| head_tail_dyn | 41184ns | 24720ns | 53354ns | -0.81% | 0.099 |
| kernel_nway | 35452ns | 30772ns | 41173ns | -14.62% | 0.116 |
| kernel_seq | 50558ns | 29570ns | 74265ns | +21.76% | 0.081 |
| nway | 63418ns | 56972ns | 70529ns | +52.74% | 0.065 |
| sequential | 35904ns | 18562ns | 62633ns | -13.53% | 0.114 |
| std_parallel | 107424ns | 78110ns | 122874ns | +158.72% | 0.038 |

## Performance model

- Peak throughput: **0.221 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.101 | 45.8% |
| head_tail_dyn | 0.103 | 46.5% |
| kernel_nway | 0.120 | 54.2% |
| kernel_seq | 0.086 | 39.0% |
| nway | 0.065 | 29.6% |
| sequential | 0.168 | 76.0% |
| std_parallel | 0.036 | 16.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 42353ns | 42353ns | base |
| head_tail_dyn | 42155ns | 42155ns | -0.47% |
| kernel_nway | 36399ns | 36399ns | -14.06% |
| kernel_seq | 51243ns | 51243ns | +20.99% |
| nway | 64022ns | 64022ns | +51.16% |
| sequential | 36199ns | 36199ns | -14.53% |
| std_parallel | 108292ns | 108292ns | +155.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 40520ns | base | --- | [32476, 51647] | --- | --- | --- | --- |
| head_tail_dyn | 39893ns | no significant difference | [-5956, +2742]ns | [36549, 44827] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 34267ns | no significant difference | [-12109, +490]ns | [31455, 37426] | no | 0.1925 | 0.0963 | 0 |
| kernel_seq | 47595ns | no significant difference | [-4781, +17460]ns | [34712, 55231] | no | 0.8145 | 0.8145 | 0 |
| nway | 62715ns | +16519.3ns (+40.8%) | [+13759, +31515]ns | [57600, 67019] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 24417ns | no significant difference | [-21943, +10130]ns | [18735, 50582] | no | 0.7210 | 0.4807 | 0 |
| std_parallel | 112661ns | +66723.5ns (+164.7%) | [+62057, +72851]ns | [101654, 117796] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 32476ns | +7.5% | -3.6% | +37.4% | +89.4% | +36.8% | +222.1% |
| 1 | 2 | 44546ns | +4.1% | -25.8% | -12.3% | +28.4% | -41.2% | +161.8% |
| 1 | 3 | 54688ns | -22.8% | -29.9% | -12.4% | +23.5% | -65.5% | +110.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.002 | ok |
| head_tail_dyn | -0.404 | moderate- |
| kernel_nway | -0.052 | ok |
| kernel_seq | -0.576 | HIGH- (thermal bounce) |
| nway | -0.443 | moderate- |
| sequential | -0.038 | ok |
| std_parallel | -0.251 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 42142.5ns | 41521.2ns | 101.5% | HIGH |
| head_tail_dyn | 42640.3ns | 41183.8ns | 103.5% | HIGH |
| kernel_nway | 37106.1ns | 35451.7ns | 104.7% | HIGH |
| kernel_seq | 51394.9ns | 50558.2ns | 101.7% | HIGH |
| nway | 65273.3ns | 63418.0ns | 102.9% | HIGH |
| sequential | 36945.0ns | 35904.1ns | 102.9% | HIGH |
| std_parallel | 107867.7ns | 107424.0ns | 100.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 25646.4-55336.0 ns)
  25646.4 |
  27130.8 |
  28615.3 |########################################
  30099.8 |####################
  31584.3 |########################################
  33068.8 |####################
  34553.2 |
  36037.7 |
  37522.2 |####################
  39006.7 |####################
  40491.2 |####################
  41975.6 |####################
  43460.1 |
  44944.6 |####################
  46429.1 |
  47913.6 |####################
  49398.0 |
  50882.5 |
  52367.0 |
  53851.5 |########################################
  (1 below, 3 above range)

head_tail_dyn (n=18, range 24719.6-53353.8 ns)
  24719.6 |
  26151.3 |
  27583.0 |
  29014.8 |####################
  30446.5 |####################
  31878.2 |
  33309.9 |####################
  34741.6 |####################
  36173.3 |########################################
  37605.0 |########################################
  39036.7 |
  40468.4 |########################################
  41900.1 |########################################
  43331.8 |
  44763.5 |
  46195.2 |####################
  47626.9 |
  49058.6 |
  50490.4 |########################################
  51922.1 |
  (1 below, 2 above range)

kernel_nway (n=18, range 30771.6-41172.8 ns)
  30771.6 |##########################
  31291.7 |########################################
  31811.7 |#############
  32331.8 |
  32851.9 |#############
  33371.9 |
  33892.0 |##########################
  34412.0 |#############
  34932.1 |
  35452.2 |
  35972.2 |#############
  36492.3 |
  37012.3 |#############
  37532.4 |#############
  38052.4 |
  38572.5 |
  39092.6 |#############
  39612.6 |
  40132.7 |
  40652.7 |
  (1 below, 3 above range)

kernel_seq (n=18, range 29570.1-74264.7 ns)
  29570.1 |
  31804.9 |########
  34039.6 |########################################
  36274.3 |
  38509.0 |
  40743.8 |
  42978.5 |########
  45213.2 |########
  47447.9 |################
  49682.7 |########
  51917.4 |
  54152.1 |################
  56386.8 |
  58621.6 |
  60856.3 |
  63091.0 |
  65325.7 |
  67560.5 |
  69795.2 |
  72029.9 |
  (1 below, 4 above range)

nway (n=18, range 56972.2-70529.5 ns)
  56972.2 |########################################
  57650.0 |##########################
  58327.9 |
  59005.8 |
  59683.6 |#############
  60361.5 |
  61039.4 |
  61717.2 |
  62395.1 |##########################
  63073.0 |
  63750.8 |
  64428.7 |
  65106.5 |#############
  65784.4 |#############
  66462.3 |#############
  67140.1 |##########################
  67818.0 |
  68495.9 |
  69173.7 |#############
  69851.6 |
  (2 below, 2 above range)

sequential (n=18, range 18561.9-62633.0 ns)
  18561.9 |########################################
  20765.4 |
  22969.0 |
  25172.5 |
  27376.1 |
  29579.6 |#####
  31783.2 |#####
  33986.8 |
  36190.3 |#####
  38393.9 |
  40597.4 |
  42801.0 |
  45004.5 |
  47208.1 |
  49411.6 |#####
  51615.2 |
  53818.8 |#####
  56022.3 |#####
  58225.9 |
  60429.4 |#####
  (1 below, 2 above range)

std_parallel (n=18, range 78109.7-122874.0 ns)
  78109.7 |
  80347.9 |
  82586.2 |
  84824.4 |#############
  87062.6 |
  89300.8 |
  91539.0 |##########################
  93777.2 |
  96015.4 |
  98253.7 |
  100491.9 |########################################
  102730.1 |
  104968.3 |
  107206.5 |#############
  109444.7 |#############
  111682.9 |
  113921.2 |########################################
  116159.4 |##########################
  118397.6 |#############
  120635.8 |#############
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=27.8% (high variance, measurements may be unstable)
- **head_tail**: bridge=101.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=28.9% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=102.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=104.7% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=39.2% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=100.0% of algo (FFI overhead may distort results)
- **nway**: bridge=99.6% of algo (FFI overhead may distort results)
- **sequential**: CV=57.8% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **sequential**: bridge=108.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=96.2% of algo (FFI overhead may distort results)
