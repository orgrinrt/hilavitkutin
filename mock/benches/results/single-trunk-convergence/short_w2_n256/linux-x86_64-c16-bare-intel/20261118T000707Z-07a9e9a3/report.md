# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 4.3x slower than the field

std_parallel (48.46 us) is 4.3x the fastest (11.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.65)

head_tail's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.3x the fastest

Fastest kernel_nway (11.17 us) to slowest std_parallel (48.46 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.3x its best-20%

sequential's best 20% of batches run at 6.67 us but its worst 20% at 21.98 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 11170.6 ns median (-16.6% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.34x (fastest 11170.6 ns, slowest 48455.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 2 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 14359ns | 14173ns | 12928ns | 13920ns | 15734ns | base |
| head_tail_dyn | 15678ns | 14786ns | 12290ns | 14373ns | 19330ns | +9.18% |
| kernel_nway | 12245ns | 11783ns | 10426ns | 11581ns | 14151ns | -14.72% |
| kernel_seq | 11983ns | 12013ns | 9197ns | 11358ns | 14313ns | -16.55% |
| nway | 35172ns | 33262ns | 30820ns | 32578ns | 41238ns | +144.94% |
| sequential | 14915ns | 17292ns | 6806ns | 12597ns | 22447ns | +3.87% |
| std_parallel | 50682ns | 49192ns | 36796ns | 46521ns | 63866ns | +252.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 13605ns | 12376ns | 14789ns | base | 0.019 |
| head_tail_dyn | 14870ns | 11671ns | 18324ns | +9.30% | 0.017 |
| kernel_nway | 11584ns | 9965ns | 13288ns | -14.85% | 0.022 |
| kernel_seq | 11091ns | 8717ns | 13007ns | -18.48% | 0.023 |
| nway | 34686ns | 30408ns | 40630ns | +154.95% | 0.007 |
| sequential | 14613ns | 6670ns | 21979ns | +7.41% | 0.018 |
| std_parallel | 50065ns | 36436ns | 62990ns | +267.99% | 0.005 |

## Performance model

- Peak throughput: **0.038 Gops/s** (sequential; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.019 | 49.8% |
| head_tail_dyn | 0.019 | 48.5% |
| kernel_nway | 0.023 | 59.7% |
| kernel_seq | 0.023 | 59.1% |
| nway | 0.008 | 20.3% |
| sequential | 0.015 | 39.3% |
| std_parallel | 0.005 | 13.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 14359ns | 14359ns | base |
| head_tail_dyn | 15678ns | 15678ns | +9.18% |
| kernel_nway | 12245ns | 12245ns | -14.72% |
| kernel_seq | 11983ns | 11983ns | -16.55% |
| nway | 35172ns | 35172ns | +144.94% |
| sequential | 14915ns | 14915ns | +3.87% |
| std_parallel | 50682ns | 50682ns | +252.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 13395ns | base | --- | [12881, 14447] | --- | --- | --- | --- |
| head_tail_dyn | 13760ns | no significant difference | [-607, +3307]ns | [12628, 16586] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 11171ns | -2253.2ns (-16.8%) | [-2678, -1532]ns | [10779, 11676] | YES | 0.0002 | 0.0001 | 0 |
| kernel_seq | 11289ns | -2020.8ns (-15.1%) | [-4399, -1385]ns | [9815, 11897] | YES | 0.0002 | 0.0001 | 0 |
| nway | 32830ns | +19175.0ns (+143.1%) | [+18076, +21686]ns | [30911, 35916] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 16956ns | no significant difference | [-5705, +6260]ns | [6953, 20303] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 48456ns | +34896.4ns (+260.5%) | [+29937, +43677]ns | [42124, 60000] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14054ns | +27.0% | -10.3% | -7.7% | +123.6% | -12.4% | +300.9% |
| 1 | 2 | 12998ns | +0.7% | -16.1% | -13.1% | +157.3% | +35.9% | +247.1% |
| 1 | 3 | 14280ns | -11.6% | -24.5% | -33.1% | +131.7% | +8.7% | +275.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.648 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.123 | ok |
| kernel_nway | -0.135 | ok |
| kernel_seq | -0.000 | ok |
| nway | -0.270 | moderate- |
| sequential | -0.430 | moderate- |
| std_parallel | -0.632 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 14141.7ns | 13604.9ns | 103.9% | HIGH |
| head_tail_dyn | 15033.6ns | 14869.7ns | 101.1% | HIGH |
| kernel_nway | 11650.3ns | 11584.2ns | 100.6% | HIGH |
| kernel_seq | 11490.3ns | 11091.2ns | 103.6% | HIGH |
| nway | 35027.0ns | 34685.7ns | 101.0% | HIGH |
| sequential | 15031.0ns | 14613.4ns | 102.9% | HIGH |
| std_parallel | 49691.5ns | 50064.5ns | 99.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 12375.7-14789.5 ns)
  12375.7 |
  12496.4 |####################
  12617.1 |####################
  12737.8 |####################
  12858.5 |########################################
  12979.2 |####################
  13099.9 |
  13220.5 |####################
  13341.2 |
  13461.9 |####################
  13582.6 |####################
  13703.3 |
  13824.0 |
  13944.7 |
  14065.3 |####################
  14186.0 |
  14306.7 |
  14427.4 |########################################
  14548.1 |########################################
  14668.8 |####################
  (2 below, 1 above range)

head_tail_dyn (n=18, range 11670.7-18324.0 ns)
  11670.7 |##########################
  12003.4 |#############
  12336.0 |#############
  12668.7 |#############
  13001.4 |########################################
  13334.0 |
  13666.7 |
  13999.4 |#############
  14332.0 |
  14664.7 |
  14997.4 |#############
  15330.0 |
  15662.7 |
  15995.4 |##########################
  16328.0 |
  16660.7 |#############
  16993.4 |
  17326.0 |
  17658.7 |
  17991.4 |#############
  (1 below, 3 above range)

kernel_nway (n=18, range 9965.0-13288.3 ns)
   9965.0 |##########################
  10131.1 |
  10297.3 |#############
  10463.5 |
  10629.6 |#############
  10795.8 |##########################
  10962.0 |#############
  11128.1 |########################################
  11294.3 |##########################
  11460.5 |
  11626.6 |
  11792.8 |#############
  11959.0 |
  12125.1 |
  12291.3 |
  12457.5 |#############
  12623.6 |#############
  12789.8 |
  12956.0 |
  13122.1 |
  (1 below, 2 above range)

kernel_seq (n=18, range 8717.2-13007.2 ns)
   8717.2 |
   8931.7 |
   9146.2 |
   9360.7 |####################
   9575.2 |########################################
   9789.7 |########################################
  10004.2 |
  10218.7 |
  10433.2 |####################
  10647.7 |
  10862.2 |
  11076.7 |####################
  11291.2 |########################################
  11505.7 |
  11720.2 |########################################
  11934.7 |####################
  12149.2 |####################
  12363.7 |
  12578.2 |
  12792.7 |
  (2 below, 3 above range)

nway (n=18, range 30407.5-40630.4 ns)
  30407.5 |########################################
  30918.6 |##########
  31429.8 |##########
  31940.9 |####################
  32452.1 |
  32963.2 |##########
  33474.4 |####################
  33985.5 |
  34496.7 |
  35007.8 |
  35519.0 |##########
  36030.1 |##########
  36541.3 |
  37052.4 |
  37563.6 |
  38074.7 |####################
  38585.8 |
  39097.0 |
  39608.1 |
  40119.3 |
  (1 below, 2 above range)

sequential (n=18, range 6670.0-21978.8 ns)
   6670.0 |########################################
   7435.4 |
   8200.9 |
   8966.3 |
   9731.7 |
  10497.2 |######
  11262.6 |
  12028.1 |
  12793.5 |
  13558.9 |
  14324.4 |
  15089.8 |
  15855.3 |######
  16620.7 |
  17386.2 |######
  18151.6 |
  18917.0 |#############
  19682.5 |######
  20447.9 |######
  21213.4 |######
  (1 below, 3 above range)

std_parallel (n=18, range 36436.4-62990.0 ns)
  36436.4 |########################################
  37764.1 |
  39091.8 |####################
  40419.5 |####################
  41747.2 |####################
  43074.8 |####################
  44402.5 |
  45730.2 |####################
  47057.9 |####################
  48385.5 |####################
  49713.2 |####################
  51040.9 |####################
  52368.6 |
  53696.3 |
  55023.9 |
  56351.6 |
  57679.3 |####################
  59007.0 |
  60334.7 |####################
  61662.3 |####################
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=101.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=102.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=102.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=102.3% of algo (FFI overhead may distort results)
- **nway**: bridge=101.2% of algo (FFI overhead may distort results)
- **sequential**: CV=46.4% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **sequential**: bridge=102.5% of algo (FFI overhead may distort results)
- **std_parallel**: CV=20.6% (high variance, measurements may be unstable)
- **std_parallel**: bridge=105.4% of algo (FFI overhead may distort results)
