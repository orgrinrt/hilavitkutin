# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway shows alternating (throttle bounce) (autocorr -0.57)

nway's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: head_tail_dyn** at 92379.1 ns median (-2.3% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 1.20x (fastest 92379.1 ns, slowest 111070.9 ns)

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
| head_tail | 94674ns | 94779ns | 92008ns | 94440ns | 96359ns | base |
| head_tail_dyn | 93192ns | 92562ns | 90465ns | 92263ns | 95947ns | -1.57% |
| kernel_nway | 96321ns | 95765ns | 94832ns | 95596ns | 98152ns | +1.74% |
| kernel_seq | 96778ns | 96460ns | 94760ns | 95948ns | 99033ns | +2.22% |
| nway | 106435ns | 106198ns | 104479ns | 105775ns | 108405ns | +12.42% |
| sequential | 100376ns | 100423ns | 99182ns | 99956ns | 101603ns | +6.02% |
| std_parallel | 112210ns | 111273ns | 108299ns | 110850ns | 116206ns | +18.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 94447ns | 91806ns | 96128ns | base | 11.102 |
| head_tail_dyn | 92849ns | 90247ns | 95325ns | -1.69% | 11.293 |
| kernel_nway | 96065ns | 94588ns | 97850ns | +1.71% | 10.915 |
| kernel_seq | 96493ns | 94567ns | 98683ns | +2.17% | 10.867 |
| nway | 106105ns | 104251ns | 107928ns | +12.34% | 9.882 |
| sequential | 100169ns | 98982ns | 101389ns | +6.06% | 10.468 |
| std_parallel | 111949ns | 108090ns | 115880ns | +18.53% | 9.367 |

## Performance model

- Peak throughput: **11.619 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 11.084 | 95.4% |
| head_tail_dyn | 11.351 | 97.7% |
| kernel_nway | 10.980 | 94.5% |
| kernel_seq | 10.919 | 94.0% |
| nway | 9.911 | 85.3% |
| sequential | 10.461 | 90.0% |
| std_parallel | 9.441 | 81.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 94674ns | 94674ns | base |
| head_tail_dyn | 93192ns | 93192ns | -1.57% |
| kernel_nway | 96321ns | 96321ns | +1.74% |
| kernel_seq | 96778ns | 96778ns | +2.22% |
| nway | 106435ns | 106435ns | +12.42% |
| sequential | 100376ns | 100376ns | +6.02% |
| std_parallel | 112210ns | 112210ns | +18.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 94602ns | base | --- | [93571, 95367] | --- | --- | --- | --- |
| head_tail_dyn | 92379ns | -1714.6ns (-1.8%) | [-3154, -273]ns | [91450, 93698] | YES | 0.0309 | 0.0309 | 0 |
| kernel_nway | 95502ns | +1477.1ns (+1.6%) | [+577, +2338]ns | [95144, 96217] | YES | 0.0309 | 0.0309 | 0 |
| kernel_seq | 96029ns | +2043.8ns (+2.2%) | [+427, +3519]ns | [94879, 97165] | YES | 0.0309 | 0.0309 | 0 |
| nway | 105804ns | +11141.7ns (+11.8%) | [+9650, +13275]ns | [104767, 107288] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 100233ns | +6108.3ns (+6.5%) | [+4502, +7158]ns | [99208, 100775] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 111071ns | +17325.0ns (+18.3%) | [+14767, +18404]ns | [109940, 112390] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 95367ns | -0.8% | +1.1% | +0.6% | +11.1% | +3.8% | +16.0% |
| 1 | 2 | 93802ns | -3.1% | +1.8% | +1.0% | +11.9% | +7.2% | +18.9% |
| 1 | 3 | 94183ns | -1.4% | +0.9% | +3.2% | +13.2% | +6.6% | +17.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.316 | moderate- |
| head_tail_dyn | -0.516 | HIGH- (thermal bounce) |
| kernel_nway | -0.020 | ok |
| kernel_seq | -0.504 | HIGH- (thermal bounce) |
| nway | -0.573 | HIGH- (thermal bounce) |
| sequential | -0.236 | moderate- |
| std_parallel | -0.512 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 94875.7ns | 94446.5ns | 100.5% | HIGH |
| head_tail_dyn | 94092.1ns | 92849.3ns | 101.3% | HIGH |
| kernel_nway | 96037.5ns | 96064.6ns | 100.0% | HIGH |
| kernel_seq | 97520.4ns | 96493.1ns | 101.1% | HIGH |
| nway | 106065.0ns | 106104.9ns | 100.0% | HIGH |
| sequential | 100536.3ns | 100168.5ns | 100.4% | HIGH |
| std_parallel | 111458.6ns | 111948.6ns | 99.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 91805.5-96127.8 ns)
  91805.5 |####################
  92021.6 |
  92237.8 |
  92453.9 |
  92670.0 |####################
  92886.1 |
  93102.2 |####################
  93318.3 |####################
  93534.4 |####################
  93750.5 |####################
  93966.7 |
  94182.8 |
  94398.9 |########################################
  94615.0 |####################
  94831.1 |########################################
  95047.2 |
  95263.3 |########################################
  95479.4 |####################
  95695.6 |
  95911.7 |########################################
  (1 below, 1 above range)

head_tail_dyn (n=18, range 90247.2-95325.0 ns)
  90247.2 |####################
  90501.1 |
  90755.0 |
  91008.9 |####################
  91262.8 |####################
  91516.6 |########################################
  91770.5 |
  92024.4 |####################
  92278.3 |########################################
  92532.2 |####################
  92786.1 |
  93040.0 |
  93293.9 |####################
  93547.8 |####################
  93801.7 |####################
  94055.6 |
  94309.4 |####################
  94563.3 |
  94817.2 |####################
  95071.1 |####################
  (2 below, 1 above range)

kernel_nway (n=18, range 94587.5-97850.0 ns)
  94587.5 |##########################
  94750.6 |
  94913.8 |##########################
  95076.9 |
  95240.0 |##########################
  95403.1 |##########################
  95566.3 |########################################
  95729.4 |
  95892.5 |#############
  96055.6 |
  96218.8 |
  96381.9 |#############
  96545.0 |
  96708.1 |
  96871.3 |#############
  97034.4 |
  97197.5 |##########################
  97360.6 |
  97523.8 |
  97686.9 |
  (1 below, 1 above range)

kernel_seq (n=18, range 94566.7-98683.3 ns)
  94566.7 |########################################
  94772.5 |
  94978.3 |#############
  95184.2 |#############
  95390.0 |#############
  95595.8 |#############
  95801.7 |
  96007.5 |
  96213.3 |#############
  96419.2 |
  96625.0 |#############
  96830.8 |
  97036.7 |##########################
  97242.5 |#############
  97448.3 |
  97654.2 |
  97860.0 |
  98065.8 |
  98271.7 |#############
  98477.5 |#############
  (2 below, 2 above range)

nway (n=18, range 104251.4-107928.5 ns)
  104251.4 |
  104435.3 |########################################
  104619.1 |####################
  104803.0 |########################################
  104986.8 |
  105170.7 |####################
  105354.5 |####################
  105538.4 |
  105722.2 |
  105906.1 |
  106089.9 |####################
  106273.8 |
  106457.6 |####################
  106641.5 |
  106825.3 |
  107009.2 |########################################
  107193.0 |
  107376.9 |####################
  107560.7 |
  107744.6 |####################
  (2 below, 3 above range)

sequential (n=18, range 98982.0-101388.9 ns)
  98982.0 |########################################
  99102.3 |########################################
  99222.7 |
  99343.0 |####################
  99463.4 |####################
  99583.7 |
  99704.0 |
  99824.4 |
  99944.7 |
  100065.1 |####################
  100185.4 |
  100305.8 |########################################
  100426.1 |####################
  100546.5 |
  100666.8 |####################
  100787.2 |
  100907.5 |####################
  101027.9 |########################################
  101148.2 |
  101268.6 |
  (2 below, 2 above range)

std_parallel (n=18, range 108090.3-115879.9 ns)
  108090.3 |#############
  108479.8 |
  108869.3 |
  109258.7 |#############
  109648.2 |########################################
  110037.7 |
  110427.2 |#############
  110816.6 |#############
  111206.1 |##########################
  111595.6 |#############
  111985.1 |#############
  112374.6 |##########################
  112764.0 |
  113153.5 |
  113543.0 |
  113932.5 |
  114322.0 |
  114711.4 |#############
  115100.9 |
  115490.4 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=100.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=99.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.7% of algo (FFI overhead may distort results)
- **nway**: bridge=99.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.0% of algo (FFI overhead may distort results)
