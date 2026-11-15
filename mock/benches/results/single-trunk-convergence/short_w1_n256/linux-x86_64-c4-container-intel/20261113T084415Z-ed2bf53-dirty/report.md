# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 135% faster than the next best (head_tail)

sequential (8.92 us) leads head_tail (21.01 us) by 135%, a clear separation rather than a photo finish. CV 27.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 57% (significant)

sequential is -12.07 us (57%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 4.3x slower than the field

std_parallel (38.76 us) is 4.3x the fastest (8.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 27.8%)

sequential wins on median (8.92 us) yet has the highest variance (CV 27.8%), while nway is the steadiest (CV 7.6%, 35.80 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.65)

kernel_seq's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {head_tail, head_tail_dyn, kernel_nway, kernel_seq, nway, std_parallel} (135% apart)

The field splits into a fast tier {sequential} and a slow tier {head_tail, head_tail_dyn, kernel_nway, kernel_seq, nway, std_parallel} with a 135% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.3x the fastest

Fastest sequential (8.92 us) to slowest std_parallel (38.76 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail_dyn is inconsistent: worst-20% is 1.6x its best-20%

head_tail_dyn's best 20% of batches run at 19.97 us but its worst 20% at 31.27 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 8923.8 ns median (-57.5% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.34x (fastest 8923.8 ns, slowest 38756.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 1 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100350026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 21914ns | 21395ns | 19636ns | 21151ns | 24198ns | base |
| head_tail_dyn | 24885ns | 21727ns | 20520ns | 21693ns | 31853ns | +13.56% |
| kernel_nway | 23007ns | 23006ns | 19279ns | 22049ns | 26307ns | +4.99% |
| kernel_seq | 23299ns | 23715ns | 20454ns | 22726ns | 25581ns | +6.32% |
| nway | 37295ns | 36143ns | 34303ns | 36121ns | 40552ns | +70.19% |
| sequential | 9913ns | 9180ns | 8066ns | 8912ns | 12337ns | -54.76% |
| std_parallel | 42392ns | 39158ns | 36175ns | 39112ns | 50421ns | +93.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 21442ns | 19238ns | 23738ns | base | 0.012 |
| head_tail_dyn | 24397ns | 19966ns | 31268ns | +13.78% | 0.010 |
| kernel_nway | 22547ns | 18846ns | 25795ns | +5.15% | 0.011 |
| kernel_seq | 22760ns | 19899ns | 24922ns | +6.14% | 0.011 |
| nway | 36941ns | 33964ns | 40192ns | +72.28% | 0.007 |
| sequential | 9608ns | 7809ns | 11967ns | -55.19% | 0.027 |
| std_parallel | 41829ns | 35671ns | 49691ns | +95.08% | 0.006 |

## Performance model

- Peak throughput: **0.033 Gops/s** (sequential; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.012 | 37.2% |
| head_tail_dyn | 0.012 | 36.6% |
| kernel_nway | 0.011 | 34.6% |
| kernel_seq | 0.011 | 33.8% |
| nway | 0.007 | 21.8% |
| sequential | 0.029 | 87.5% |
| std_parallel | 0.007 | 20.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 21914ns | 21914ns | base |
| head_tail_dyn | 24885ns | 24885ns | +13.56% |
| kernel_nway | 23007ns | 23007ns | +4.99% |
| kernel_seq | 23299ns | 23299ns | +6.32% |
| nway | 37295ns | 37295ns | +70.19% |
| sequential | 9913ns | 9913ns | -54.76% |
| std_parallel | 42392ns | 42392ns | +93.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 21015ns | base | --- | [20068, 22135] | --- | --- | --- | --- |
| head_tail_dyn | 21336ns | no significant difference | [-569, +5694]ns | [20602, 26767] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 22579ns | no significant difference | [-781, +3648]ns | [19596, 24100] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 23124ns | no significant difference | [-375, +2879]ns | [21069, 23580] | no | 0.1444 | 0.0963 | 0 |
| nway | 35796ns | +15719.7ns (+74.8%) | [+13522, +17480]ns | [35036, 37903] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 8924ns | -12070.0ns (-57.4%) | [-13340, -10763]ns | [8177, 10170] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 38756ns | +18581.7ns (+88.4%) | [+16019, +21912]ns | [37978, 44555] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 21746ns | -3.3% | -11.9% | +0.7% | +73.1% | -53.0% | +132.9% |
| 1 | 2 | 20855ns | -1.5% | +20.0% | +14.7% | +70.4% | -59.6% | +83.3% |
| 1 | 3 | 20275ns | +45.4% | +11.4% | +6.1% | +76.5% | -59.7% | +85.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.010 | ok |
| head_tail_dyn | -0.195 | ok |
| kernel_nway | -0.442 | moderate- |
| kernel_seq | -0.651 | HIGH- (thermal bounce) |
| nway | -0.228 | moderate- |
| sequential | -0.109 | ok |
| std_parallel | -0.139 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 21145.2ns | 21442.4ns | 98.6% | HIGH |
| head_tail_dyn | 25055.5ns | 24397.0ns | 102.7% | HIGH |
| kernel_nway | 22915.3ns | 22547.0ns | 101.6% | HIGH |
| kernel_seq | 22845.3ns | 22759.6ns | 100.4% | HIGH |
| nway | 37390.3ns | 36940.6ns | 101.2% | HIGH |
| sequential | 9020.5ns | 9608.2ns | 93.9% | HIGH |
| std_parallel | 43017.1ns | 41829.2ns | 102.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 19237.9-23738.0 ns)
  19237.9 |########################################
  19462.9 |
  19687.9 |
  19912.9 |#############
  20137.9 |##########################
  20363.0 |
  20588.0 |#############
  20813.0 |#############
  21038.0 |#############
  21263.0 |#############
  21488.0 |
  21713.0 |#############
  21938.0 |##########################
  22163.0 |
  22388.0 |
  22613.0 |
  22838.0 |##########################
  23063.0 |
  23288.0 |
  23513.0 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 19965.9-31267.5 ns)
  19965.9 |####################
  20531.0 |########################################
  21096.1 |####################
  21661.2 |
  22226.3 |##########
  22791.3 |##########
  23356.4 |
  23921.5 |
  24486.6 |
  25051.7 |##########
  25616.7 |
  26181.8 |
  26746.9 |
  27312.0 |
  27877.1 |##########
  28442.1 |##########
  29007.2 |
  29572.3 |
  30137.4 |##########
  30702.5 |
  (2 below, 2 above range)

kernel_nway (n=18, range 18846.3-25794.6 ns)
  18846.3 |####################
  19193.7 |########################################
  19541.1 |####################
  19888.5 |
  20235.9 |
  20583.3 |
  20930.8 |
  21278.2 |
  21625.6 |
  21973.0 |########################################
  22320.4 |####################
  22667.9 |########################################
  23015.3 |
  23362.7 |####################
  23710.1 |####################
  24057.5 |
  24404.9 |########################################
  24752.4 |
  25099.8 |####################
  25447.2 |
  (2 below, 2 above range)

kernel_seq (n=18, range 19898.9-24921.5 ns)
  19898.9 |#############
  20150.0 |
  20401.2 |##########################
  20652.3 |#############
  20903.4 |
  21154.6 |#############
  21405.7 |
  21656.8 |#############
  21908.0 |
  22159.1 |
  22410.2 |
  22661.3 |
  22912.5 |##########################
  23163.6 |########################################
  23414.7 |#############
  23665.9 |#############
  23917.0 |#############
  24168.1 |
  24419.3 |#############
  24670.4 |
  (1 below, 2 above range)

nway (n=18, range 33963.8-40192.2 ns)
  33963.8 |
  34275.2 |##########
  34586.6 |####################
  34898.0 |##########
  35209.5 |
  35520.9 |########################################
  35832.3 |
  36143.7 |
  36455.1 |##########
  36766.6 |
  37078.0 |##########
  37389.4 |
  37700.8 |##########
  38012.2 |##########
  38323.7 |
  38635.1 |
  38946.5 |
  39257.9 |##########
  39569.3 |
  39880.8 |
  (2 below, 3 above range)

sequential (n=18, range 7809.1-11967.0 ns)
   7809.1 |########################################
   8017.0 |########################################
   8224.9 |####################
   8432.8 |####################
   8640.7 |####################
   8848.6 |
   9056.5 |####################
   9264.4 |########################################
   9472.3 |
   9680.2 |
   9888.1 |
  10095.9 |########################################
  10303.8 |####################
  10511.7 |####################
  10719.6 |
  10927.5 |
  11135.4 |
  11343.3 |
  11551.2 |####################
  11759.1 |
  (2 below, 1 above range)

std_parallel (n=18, range 35671.0-49690.8 ns)
  35671.0 |
  36372.0 |##########
  37073.0 |##########
  37774.0 |########################################
  38475.0 |####################
  39176.0 |##########
  39877.0 |
  40577.9 |
  41278.9 |##########
  41979.9 |##########
  42680.9 |
  43381.9 |
  44082.9 |
  44783.9 |
  45484.9 |
  46185.8 |##########
  46886.8 |
  47587.8 |##########
  48288.8 |
  48989.8 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=101.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=23.4% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=103.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=99.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.6% of algo (FFI overhead may distort results)
- **nway**: bridge=103.5% of algo (FFI overhead may distort results)
- **sequential**: CV=25.8% (high variance, measurements may be unstable)
- **sequential**: bridge=93.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=102.3% of algo (FFI overhead may distort results)
