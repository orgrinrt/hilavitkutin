# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 11% faster than the next best (head_tail)

kernel_nway (53.50 us) leads head_tail (59.43 us) by 11%, a clear separation rather than a photo finish. CV 12.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.52)

kernel_nway's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### nway is inconsistent: worst-20% is 2.3x its best-20%

nway's best 20% of batches run at 73.69 us but its worst 20% at 167.97 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 53498.9 ns median (-10.0% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 1.78x (fastest 53498.9 ns, slowest 95183.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 2 |
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
| head_tail | 63289ns | 59867ns | 52493ns | 59264ns | 74724ns | base |
| head_tail_dyn | 65253ns | 63008ns | 57390ns | 62919ns | 72686ns | +3.10% |
| kernel_nway | 55914ns | 54318ns | 48387ns | 52627ns | 64607ns | -11.65% |
| kernel_seq | 69063ns | 68703ns | 58427ns | 67831ns | 76227ns | +9.12% |
| nway | 114007ns | 91434ns | 74081ns | 91045ns | 168414ns | +80.14% |
| sequential | 101831ns | 95967ns | 73854ns | 90209ns | 133252ns | +60.90% |
| std_parallel | 82172ns | 77502ns | 71380ns | 77107ns | 95166ns | +29.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 62560ns | 52076ns | 73650ns | base | 1.048 |
| head_tail_dyn | 64644ns | 56971ns | 72107ns | +3.33% | 1.014 |
| kernel_nway | 55240ns | 47699ns | 63796ns | -11.70% | 1.186 |
| kernel_seq | 68382ns | 57976ns | 75433ns | +9.31% | 0.958 |
| nway | 113535ns | 73691ns | 167974ns | +81.48% | 0.577 |
| sequential | 101229ns | 73454ns | 132475ns | +61.81% | 0.647 |
| std_parallel | 81568ns | 70932ns | 94379ns | +30.38% | 0.803 |

## Performance model

- Peak throughput: **1.374 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.103 | 80.3% |
| head_tail_dyn | 1.051 | 76.5% |
| kernel_nway | 1.225 | 89.2% |
| kernel_seq | 0.961 | 70.0% |
| nway | 0.722 | 52.5% |
| sequential | 0.689 | 50.1% |
| std_parallel | 0.850 | 61.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 63289ns | 63289ns | base |
| head_tail_dyn | 65253ns | 65253ns | +3.10% |
| kernel_nway | 55914ns | 55914ns | -11.65% |
| kernel_seq | 69063ns | 69063ns | +9.12% |
| nway | 114007ns | 114007ns | +80.14% |
| sequential | 101831ns | 101831ns | +60.90% |
| std_parallel | 82172ns | 82172ns | +29.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 59432ns | base | --- | [57468, 62295] | --- | --- | --- | --- |
| head_tail_dyn | 62330ns | no significant difference | [-2255, +11711]ns | [59631, 69469] | no | 0.2379 | 0.2379 | 0 |
| kernel_nway | 53499ns | -6819.8ns (-11.5%) | [-10981, -915]ns | [49335, 59731] | YES (adj: no) | 0.1155 | 0.0963 | 0 |
| kernel_seq | 68176ns | no significant difference | [-657, +13806]ns | [65523, 70746] | no | 0.1155 | 0.0963 | 0 |
| nway | 90830ns | +29550.7ns (+49.7%) | [+25828, +40021]ns | [84804, 103486] | YES | 0.0009 | 0.0001 | 0 |
| sequential | 95183ns | +39206.8ns (+66.0%) | [+18873, +60850]ns | [77732, 117052] | YES | 0.0026 | 0.0013 | 0 |
| std_parallel | 77073ns | +17716.5ns (+29.8%) | [+14535, +22636]ns | [74874, 82552] | YES | 0.0026 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 58829ns | +21.0% | -6.7% | +20.3% | +78.0% | +81.3% | +29.6% |
| 1 | 2 | 57790ns | +7.7% | -14.5% | +14.8% | +46.7% | +99.3% | +31.9% |
| 1 | 3 | 67020ns | -12.9% | -10.8% | +1.7% | +33.4% | +11.0% | +23.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.229 | moderate- |
| head_tail_dyn | -0.034 | ok |
| kernel_nway | -0.520 | HIGH- (thermal bounce) |
| kernel_seq | -0.441 | moderate- |
| nway | -0.308 | moderate- |
| sequential | -0.291 | moderate- |
| std_parallel | -0.171 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 403680.3ns | 62559.6ns | 645.3% | HIGH |
| head_tail_dyn | 407279.6ns | 64644.1ns | 630.0% | HIGH |
| kernel_nway | 404706.6ns | 55239.8ns | 732.6% | HIGH |
| kernel_seq | 443256.3ns | 68382.4ns | 648.2% | HIGH |
| nway | 447753.7ns | 113535.3ns | 394.4% | HIGH |
| sequential | 436456.3ns | 101228.8ns | 431.2% | HIGH |
| std_parallel | 414598.2ns | 81567.7ns | 508.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 52076.1-73650.4 ns)
  52076.1 |
  53154.8 |
  54233.5 |########################################
  55312.2 |
  56391.0 |####################
  57469.7 |########################################
  58548.4 |########################################
  59627.1 |####################
  60705.8 |########################################
  61784.5 |########################################
  62863.2 |####################
  63941.9 |
  65020.7 |
  66099.4 |
  67178.1 |
  68256.8 |
  69335.5 |####################
  70414.2 |
  71492.9 |
  72571.7 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 56970.9-72107.0 ns)
  56970.9 |
  57727.7 |#############
  58484.5 |##########################
  59241.3 |
  59998.1 |########################################
  60754.9 |#############
  61511.7 |
  62268.5 |
  63025.3 |#############
  63782.1 |
  64538.9 |
  65295.7 |
  66052.6 |
  66809.4 |#############
  67566.2 |
  68323.0 |
  69079.8 |########################################
  69836.6 |#############
  70593.4 |
  71350.2 |
  (2 below, 3 above range)

kernel_nway (n=18, range 47698.7-63795.9 ns)
  47698.7 |########################################
  48503.6 |####################
  49308.4 |####################
  50113.3 |####################
  50918.2 |
  51723.0 |####################
  52527.9 |####################
  53332.7 |####################
  54137.6 |
  54942.5 |####################
  55747.3 |####################
  56552.2 |
  57357.0 |
  58161.9 |####################
  58966.7 |
  59771.6 |
  60576.5 |####################
  61381.3 |
  62186.2 |
  62991.0 |####################
  (2 below, 3 above range)

kernel_seq (n=18, range 57976.4-75432.5 ns)
  57976.4 |
  58849.2 |
  59722.0 |##########
  60594.8 |
  61467.6 |
  62340.4 |
  63213.2 |
  64086.0 |
  64958.8 |####################
  65831.7 |####################
  66704.5 |####################
  67577.3 |
  68450.1 |
  69322.9 |########################################
  70195.7 |
  71068.5 |##############################
  71941.3 |##########
  72814.1 |
  73686.9 |
  74559.7 |
  (2 below, 1 above range)

nway (n=18, range 73690.6-167974.2 ns)
  73690.6 |
  78404.8 |##########
  83119.0 |##############################
  87833.1 |########################################
  92547.3 |
  97261.5 |##########
  101975.7 |########################################
  106689.9 |##########
  111404.0 |
  116118.2 |
  120832.4 |
  125546.6 |
  130260.8 |
  134974.9 |
  139689.1 |
  144403.3 |
  149117.5 |
  153831.7 |
  158545.8 |
  163260.0 |
  (2 below, 2 above range)

sequential (n=18, range 73453.6-132474.6 ns)
  73453.6 |########################################
  76404.6 |
  79355.7 |####################
  82306.7 |##########
  85257.8 |
  88208.8 |
  91159.9 |##########
  94110.9 |
  97062.0 |##########
  100013.0 |
  102964.1 |
  105915.1 |
  108866.2 |####################
  111817.2 |##########
  114768.3 |
  117719.4 |##########
  120670.4 |
  123621.5 |
  126572.5 |
  129523.6 |##########
  (1 below, 3 above range)

std_parallel (n=18, range 70931.7-94378.9 ns)
  70931.7 |
  72104.0 |########################################
  73276.4 |
  74448.7 |########################################
  75621.1 |##########################
  76793.5 |#############
  77965.8 |
  79138.2 |#############
  80310.6 |##########################
  81482.9 |
  82655.3 |
  83827.6 |#############
  85000.0 |
  86172.4 |
  87344.7 |
  88517.1 |#############
  89689.4 |
  90861.8 |
  92034.2 |
  93206.5 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=649.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=650.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=749.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=655.5% of algo (FFI overhead may distort results)
- **nway**: CV=59.7% (high variance, measurements may be unstable)
- **nway**: bridge=476.6% of algo (FFI overhead may distort results)
- **sequential**: CV=25.5% (high variance, measurements may be unstable)
- **sequential**: bridge=461.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=521.3% of algo (FFI overhead may distort results)
