# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 29% faster than the next best (head_tail)

kernel_nway (55.76 us) leads head_tail (72.01 us) by 29%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 22% (significant)

kernel_nway is -16.01 us (22%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### head_tail_dyn is an outlier: 2.5x slower than the field

head_tail_dyn (137.16 us) is 2.5x the fastest (55.76 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.64)

head_tail's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: kernel_nway** at 55762.5 ns median (-22.6% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 2.46x (fastest 55762.5 ns, slowest 137160.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 8 |
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
| head_tail | 72130ns | 72652ns | 66929ns | 72536ns | 74122ns | base |
| head_tail_dyn | 137862ns | 137544ns | 135969ns | 137302ns | 139648ns | +91.13% |
| kernel_nway | 56654ns | 56398ns | 53897ns | 55912ns | 59145ns | -21.46% |
| kernel_seq | 96531ns | 96233ns | 94600ns | 95886ns | 98465ns | +33.83% |
| nway | 72977ns | 73312ns | 66950ns | 71953ns | 77526ns | +1.17% |
| sequential | 101247ns | 100775ns | 99442ns | 100607ns | 103108ns | +40.37% |
| std_parallel | 103377ns | 101492ns | 96288ns | 101217ns | 110162ns | +43.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 71400ns | 66114ns | 73303ns | base | 14.686 |
| head_tail_dyn | 137488ns | 135671ns | 139228ns | +92.56% | 7.627 |
| kernel_nway | 55928ns | 53190ns | 58381ns | -21.67% | 18.749 |
| kernel_seq | 96221ns | 94332ns | 98051ns | +34.76% | 10.898 |
| nway | 72309ns | 66389ns | 76720ns | +1.27% | 14.501 |
| sequential | 100934ns | 99222ns | 102759ns | +41.36% | 10.389 |
| std_parallel | 102964ns | 95862ns | 109721ns | +44.21% | 10.184 |

## Performance model

- Peak throughput: **19.714 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 14.561 | 73.9% |
| head_tail_dyn | 7.645 | 38.8% |
| kernel_nway | 18.804 | 95.4% |
| kernel_seq | 10.928 | 55.4% |
| nway | 14.426 | 73.2% |
| sequential | 10.438 | 52.9% |
| std_parallel | 10.368 | 52.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 72130ns | 72130ns | base |
| head_tail_dyn | 137862ns | 137862ns | +91.13% |
| kernel_nway | 56654ns | 56654ns | -21.46% |
| kernel_seq | 96531ns | 96531ns | +33.83% |
| nway | 72977ns | 72977ns | +1.17% |
| sequential | 101247ns | 101247ns | +40.37% |
| std_parallel | 103377ns | 103377ns | +43.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 72015ns | base | --- | [71638, 72756] | --- | --- | --- | --- |
| head_tail_dyn | 137160ns | +65212.5ns (+90.6%) | [+63552, +68273]ns | [136571, 138131] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 55762ns | -16010.4ns (-22.2%) | [-17240, -14012]ns | [54265, 57506] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 95952ns | +24462.5ns (+34.0%) | [+22640, +25765]ns | [94869, 97177] | YES | 0.0000 | 0.0000 | 0 |
| nway | 72688ns | no significant difference | [-3704, +3783]ns | [68812, 75621] | no | 0.8145 | 0.8145 | 0 |
| sequential | 100454ns | +28825.0ns (+40.0%) | [+27750, +30860]ns | [99923, 101123] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 101138ns | +30831.2ns (+42.8%) | [+27258, +34538]ns | [99508, 104794] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 71748ns | +90.8% | -23.9% | +35.2% | -5.1% | +40.4% | +51.2% |
| 1 | 2 | 72579ns | +88.5% | -25.2% | +32.2% | -1.2% | +38.1% | +35.7% |
| 1 | 3 | 71917ns | +91.8% | -19.3% | +31.4% | +5.3% | +39.8% | +40.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.642 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.211 | moderate- |
| kernel_nway | -0.213 | moderate- |
| kernel_seq | -0.005 | ok |
| nway | -0.000 | ok |
| sequential | -0.611 | HIGH- (thermal bounce) |
| std_parallel | -0.330 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 138495.1ns | 71400.0ns | 194.0% | HIGH |
| head_tail_dyn | 137551.6ns | 137488.4ns | 100.0% | HIGH |
| kernel_nway | 112526.4ns | 55927.5ns | 201.2% | HIGH |
| kernel_seq | 95821.3ns | 96221.1ns | 99.6% | HIGH |
| nway | 139547.9ns | 72309.3ns | 193.0% | HIGH |
| sequential | 101341.0ns | 100933.8ns | 100.4% | HIGH |
| std_parallel | 104144.2ns | 102964.1ns | 101.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 66113.9-73302.8 ns)
  66113.9 |
  66473.3 |
  66832.8 |
  67192.2 |
  67551.7 |########
  67911.1 |
  68270.6 |
  68630.0 |
  68989.5 |
  69348.9 |
  69708.3 |
  70067.8 |
  70427.2 |
  70786.7 |
  71146.1 |################
  71505.6 |########
  71865.0 |########################################
  72224.5 |########
  72583.9 |################
  72943.3 |################
  (2 below, 2 above range)

head_tail_dyn (n=18, range 135670.8-139227.8 ns)
  135670.8 |
  135848.7 |####################
  136026.5 |####################
  136204.4 |
  136382.2 |########################################
  136560.1 |####################
  136737.9 |########################################
  136915.8 |####################
  137093.6 |####################
  137271.5 |####################
  137449.3 |####################
  137627.1 |
  137805.0 |####################
  137982.8 |
  138160.7 |####################
  138338.5 |
  138516.4 |
  138694.2 |
  138872.1 |####################
  139049.9 |
  (1 below, 3 above range)

kernel_nway (n=18, range 53190.3-58381.2 ns)
  53190.3 |#############
  53449.8 |#############
  53709.4 |#############
  53968.9 |
  54228.5 |########################################
  54488.0 |
  54747.6 |#############
  55007.1 |
  55266.7 |#############
  55526.2 |
  55785.8 |
  56045.3 |#############
  56304.9 |
  56564.4 |#############
  56824.0 |
  57083.5 |##########################
  57343.1 |
  57602.6 |#############
  57862.2 |
  58121.7 |#############
  (1 below, 3 above range)

kernel_seq (n=18, range 94332.0-98050.7 ns)
  94332.0 |########################################
  94517.9 |####################
  94703.8 |####################
  94889.8 |########################################
  95075.7 |
  95261.7 |
  95447.6 |####################
  95633.5 |
  95819.5 |########################################
  96005.4 |
  96191.3 |
  96377.3 |
  96563.2 |
  96749.1 |####################
  96935.1 |########################################
  97121.0 |####################
  97307.0 |
  97492.9 |
  97678.8 |####################
  97864.8 |
  (1 below, 3 above range)

nway (n=18, range 66388.9-76720.2 ns)
  66388.9 |
  66905.5 |####################
  67422.0 |####################
  67938.6 |####################
  68455.2 |
  68971.7 |####################
  69488.3 |
  70004.8 |
  70521.4 |####################
  71038.0 |####################
  71554.5 |
  72071.1 |
  72587.7 |########################################
  73104.2 |
  73620.8 |
  74137.3 |####################
  74653.9 |########################################
  75170.5 |
  75687.0 |
  76203.6 |########################################
  (2 below, 3 above range)

sequential (n=18, range 99222.2-102759.0 ns)
  99222.2 |
  99399.0 |#############
  99575.9 |##########################
  99752.7 |#############
  99929.6 |#############
  100106.4 |#############
  100283.2 |##########################
  100460.1 |#############
  100636.9 |#############
  100813.8 |
  100990.6 |########################################
  101167.5 |
  101344.3 |#############
  101521.1 |
  101698.0 |
  101874.8 |
  102051.7 |
  102228.5 |#############
  102405.3 |
  102582.2 |
  (1 below, 2 above range)

std_parallel (n=18, range 95862.5-109720.8 ns)
  95862.5 |
  96555.4 |
  97248.3 |########################################
  97941.2 |####################
  98634.2 |
  99327.1 |########################################
  100020.0 |########################################
  100712.9 |########################################
  101405.8 |
  102098.7 |####################
  102791.7 |
  103484.6 |####################
  104177.5 |####################
  104870.4 |####################
  105563.3 |
  106256.2 |
  106949.2 |####################
  107642.1 |
  108335.0 |
  109027.9 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=195.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=200.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.6% of algo (FFI overhead may distort results)
- **nway**: bridge=190.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=101.0% of algo (FFI overhead may distort results)
