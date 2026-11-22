# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 4.4x slower than the field

std_parallel (37.92 us) is 4.4x the fastest (8.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_seq, head_tail) are a dead heat (<1%)

kernel_seq (8.53 us) and head_tail (8.56 us) differ by 0.44%, inside the noise, even though the wider field spreads 344.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.4x the fastest

Fastest kernel_seq (8.53 us) to slowest std_parallel (37.92 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 8527.1 ns median (-0.4% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 4.45x (fastest 8527.1 ns, slowest 37920.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 2 |
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
| head_tail | 9617ns | 9148ns | 8826ns | 9090ns | 10802ns | base |
| head_tail_dyn | 12715ns | 12715ns | 12499ns | 12684ns | 12868ns | +32.21% |
| kernel_nway | 9644ns | 9408ns | 8558ns | 9300ns | 10704ns | +0.29% |
| kernel_seq | 9087ns | 8971ns | 8693ns | 8944ns | 9499ns | -5.51% |
| nway | 23109ns | 22717ns | 22281ns | 22677ns | 24172ns | +140.30% |
| sequential | 15697ns | 15608ns | 15508ns | 15595ns | 15944ns | +63.23% |
| std_parallel | 38491ns | 38300ns | 36187ns | 38131ns | 40183ns | +300.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9038ns | 8278ns | 10206ns | base | 7.252 |
| head_tail_dyn | 12166ns | 12031ns | 12267ns | +34.61% | 5.387 |
| kernel_nway | 9036ns | 8083ns | 10033ns | -0.01% | 7.252 |
| kernel_seq | 8630ns | 8343ns | 8984ns | -4.51% | 7.594 |
| nway | 22537ns | 21703ns | 23593ns | +149.37% | 2.908 |
| sequential | 15517ns | 15328ns | 15765ns | +71.70% | 4.223 |
| std_parallel | 38146ns | 35856ns | 39849ns | +322.08% | 1.718 |

## Performance model

- Peak throughput: **8.108 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.652 | 94.4% |
| head_tail_dyn | 5.389 | 66.5% |
| kernel_nway | 7.444 | 91.8% |
| kernel_seq | 7.686 | 94.8% |
| nway | 2.957 | 36.5% |
| sequential | 4.249 | 52.4% |
| std_parallel | 1.728 | 21.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9617ns | 9617ns | base |
| head_tail_dyn | 12715ns | 12715ns | +32.21% |
| kernel_nway | 9644ns | 9644ns | +0.29% |
| kernel_seq | 9087ns | 9087ns | -5.51% |
| nway | 23109ns | 23109ns | +140.30% |
| sequential | 15697ns | 15697ns | +63.23% |
| std_parallel | 38491ns | 38491ns | +300.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 8565ns | base | --- | [8417, 9121] | --- | --- | --- | --- |
| head_tail_dyn | 12160ns | +3597.9ns (+42.0%) | [+2969, +3702]ns | [12106, 12208] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 8804ns | no significant difference | [-460, +1119]ns | [8508, 9565] | no | 0.5768 | 0.4807 | 0 |
| kernel_seq | 8527ns | no significant difference | [-608, +254]ns | [8419, 8671] | no | 1.0000 | 1.0000 | 0 |
| nway | 22160ns | +13579.1ns (+158.5%) | [+12777, +14046]ns | [21998, 22567] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 15423ns | +6943.8ns (+81.1%) | [+6338, +7238]ns | [15400, 15717] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 37921ns | +28945.8ns (+338.0%) | [+28419, +30567]ns | [37242, 39292] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 8473ns | +43.9% | +16.4% | +5.4% | +180.2% | +82.0% | +342.6% |
| 1 | 2 | 10158ns | +19.7% | -14.4% | -16.8% | +117.6% | +51.4% | +280.5% |
| 1 | 3 | 8417ns | +44.1% | -1.7% | +0.5% | +161.5% | +87.2% | +354.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.666 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.002 | ok |
| kernel_nway | -0.046 | ok |
| kernel_seq | -0.178 | ok |
| nway | -0.140 | ok |
| sequential | -0.230 | moderate- |
| std_parallel | -0.377 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 91161.3ns | 9037.5ns | 1008.7% | HIGH |
| head_tail_dyn | 93926.6ns | 12165.5ns | 772.1% | HIGH |
| kernel_nway | 87809.3ns | 9036.4ns | 971.7% | HIGH |
| kernel_seq | 92243.3ns | 8630.1ns | 1068.9% | HIGH |
| nway | 102888.7ns | 22536.8ns | 456.5% | HIGH |
| sequential | 93328.0ns | 15517.4ns | 601.4% | HIGH |
| std_parallel | 106818.1ns | 38145.8ns | 280.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 8277.8-10205.6 ns)
   8277.8 |################
   8374.2 |########################################
   8470.5 |########
   8566.9 |################
   8663.3 |########
   8759.7 |
   8856.1 |########
   8952.5 |
   9048.9 |
   9145.3 |
   9241.7 |########
   9338.0 |
   9434.4 |
   9530.8 |
   9627.2 |
   9723.6 |
   9820.0 |
   9916.4 |########
  10012.8 |
  10109.2 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 12030.6-12267.4 ns)
  12030.6 |
  12042.4 |
  12054.2 |
  12066.1 |
  12077.9 |####################
  12089.8 |########################################
  12101.6 |####################
  12113.4 |####################
  12125.3 |
  12137.1 |
  12149.0 |########################################
  12160.8 |####################
  12172.6 |
  12184.5 |####################
  12196.3 |########################################
  12208.2 |####################
  12220.0 |
  12231.8 |####################
  12243.7 |
  12255.5 |
  (2 below, 3 above range)

kernel_nway (n=18, range 8083.3-10033.4 ns)
   8083.3 |#############
   8180.8 |#############
   8278.3 |#############
   8375.8 |
   8473.3 |########################################
   8570.8 |#############
   8668.3 |
   8765.8 |##########################
   8863.3 |#############
   8960.8 |
   9058.3 |
   9155.8 |#############
   9253.3 |
   9350.8 |#############
   9448.3 |
   9545.8 |
   9643.3 |#############
   9740.8 |
   9838.3 |
   9935.8 |##########################
  (1 below, 2 above range)

kernel_seq (n=18, range 8343.1-8984.0 ns)
   8343.1 |####################
   8375.1 |########################################
   8407.2 |########################################
   8439.2 |
   8471.3 |########################################
   8503.3 |####################
   8535.4 |########################################
   8567.4 |####################
   8599.5 |####################
   8631.5 |
   8663.5 |
   8695.6 |####################
   8727.6 |
   8759.7 |
   8791.7 |
   8823.8 |
   8855.8 |####################
   8887.9 |
   8919.9 |
   8952.0 |####################
  (1 below, 2 above range)

nway (n=18, range 21702.8-23593.0 ns)
  21702.8 |
  21797.3 |##########################
  21891.8 |
  21986.3 |########################################
  22080.8 |##########################
  22175.4 |##########################
  22269.9 |
  22364.4 |
  22458.9 |##########################
  22553.4 |
  22647.9 |#############
  22742.4 |
  22837.0 |
  22931.5 |
  23026.0 |
  23120.5 |
  23215.0 |
  23309.5 |
  23404.0 |
  23498.5 |
  (2 below, 4 above range)

sequential (n=18, range 15327.8-15764.6 ns)
  15327.8 |
  15349.6 |######
  15371.5 |####################
  15393.3 |
  15415.2 |########################################
  15437.0 |######
  15458.8 |
  15480.7 |
  15502.5 |
  15524.4 |
  15546.2 |
  15568.0 |
  15589.9 |
  15611.7 |
  15633.5 |
  15655.4 |
  15677.2 |
  15699.1 |######
  15720.9 |#############
  15742.7 |
  (1 below, 3 above range)

std_parallel (n=18, range 35855.5-39848.6 ns)
  35855.5 |
  36055.2 |
  36254.8 |
  36454.5 |
  36654.2 |########################################
  36853.8 |####################
  37053.5 |########################################
  37253.1 |####################
  37452.8 |
  37652.4 |########################################
  37852.1 |
  38051.7 |####################
  38251.4 |
  38451.0 |
  38650.7 |####################
  38850.3 |####################
  39050.0 |####################
  39249.7 |####################
  39449.3 |####################
  39649.0 |####################
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=1044.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=766.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=983.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1071.0% of algo (FFI overhead may distort results)
- **nway**: bridge=466.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=602.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=283.8% of algo (FFI overhead may distort results)
