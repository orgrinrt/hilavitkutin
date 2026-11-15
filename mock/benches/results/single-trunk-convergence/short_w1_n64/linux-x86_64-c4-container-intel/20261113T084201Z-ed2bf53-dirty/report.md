# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 86% faster than the next best (head_tail)

sequential (11.07 us) leads head_tail (20.64 us) by 86%, a clear separation rather than a photo finish. CV 26.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 50% (significant)

sequential is -10.23 us (50%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 4.2x slower than the field

std_parallel (46.01 us) is 4.2x the fastest (11.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 26.7%)

sequential wins on median (11.07 us) yet has the highest variance (CV 26.7%), while head_tail is the steadiest (CV 5.8%, 20.64 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.66)

kernel_seq's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {head_tail, kernel_seq, head_tail_dyn, kernel_nway, nway, std_parallel} (86% apart)

The field splits into a fast tier {sequential} and a slow tier {head_tail, kernel_seq, head_tail_dyn, kernel_nway, nway, std_parallel} with a 86% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.2x the fastest

Fastest sequential (11.07 us) to slowest std_parallel (46.01 us): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.8x its best-20%

kernel_nway's best 20% of batches run at 19.85 us but its worst 20% at 35.86 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 11074.3 ns median (-46.3% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 4.15x (fastest 11074.3 ns, slowest 46005.3 ns)

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
| head_tail | 21464ns | 21131ns | 19932ns | 21009ns | 22911ns | base |
| head_tail_dyn | 26272ns | 24504ns | 23140ns | 24310ns | 30782ns | +22.40% |
| kernel_nway | 28460ns | 27711ns | 20425ns | 25817ns | 36443ns | +32.60% |
| kernel_seq | 22871ns | 21627ns | 19705ns | 21576ns | 26395ns | +6.56% |
| nway | 34778ns | 34389ns | 29458ns | 33996ns | 38609ns | +62.03% |
| sequential | 11557ns | 11440ns | 8115ns | 10954ns | 14183ns | -46.15% |
| std_parallel | 46768ns | 46493ns | 40295ns | 44812ns | 52938ns | +117.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 20982ns | 19513ns | 22357ns | base | 0.003 |
| head_tail_dyn | 25692ns | 22488ns | 30264ns | +22.45% | 0.002 |
| kernel_nway | 27897ns | 19851ns | 35858ns | +32.96% | 0.002 |
| kernel_seq | 22403ns | 19299ns | 25843ns | +6.78% | 0.003 |
| nway | 34358ns | 29114ns | 38122ns | +63.76% | 0.002 |
| sequential | 11234ns | 7861ns | 13834ns | -46.46% | 0.006 |
| std_parallel | 46150ns | 39781ns | 52177ns | +119.96% | 0.001 |

## Performance model

- Peak throughput: **0.008 Gops/s** (sequential; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.003 | 38.1% |
| head_tail_dyn | 0.003 | 32.6% |
| kernel_nway | 0.002 | 29.1% |
| kernel_seq | 0.003 | 37.0% |
| nway | 0.002 | 23.1% |
| sequential | 0.006 | 71.0% |
| std_parallel | 0.001 | 17.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 21464ns | 21464ns | base |
| head_tail_dyn | 26272ns | 26272ns | +22.40% |
| kernel_nway | 28460ns | 28460ns | +32.60% |
| kernel_seq | 22871ns | 22871ns | +6.56% |
| nway | 34778ns | 34778ns | +62.03% |
| sequential | 11557ns | 11557ns | -46.15% |
| std_parallel | 46768ns | 46768ns | +117.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 20637ns | base | --- | [20191, 21558] | --- | --- | --- | --- |
| head_tail_dyn | 24106ns | +2862.2ns (+13.9%) | [+2369, +5549]ns | [23272, 25822] | YES | 0.0002 | 0.0001 | 0 |
| kernel_nway | 27012ns | +5836.7ns (+28.3%) | [+968, +11839]ns | [22083, 32157] | YES (adj: no) | 0.1155 | 0.0963 | 0 |
| kernel_seq | 21226ns | no significant difference | [-1096, +4234]ns | [19944, 25123] | no | 0.2379 | 0.2379 | 0 |
| nway | 33972ns | +12429.5ns (+60.2%) | [+12017, +14156]ns | [33136, 36026] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 11074ns | -10234.7ns (-49.6%) | [-11296, -9093]ns | [10494, 11568] | YES | 0.0002 | 0.0001 | 0 |
| std_parallel | 46005ns | +24915.9ns (+120.7%) | [+21765, +27379]ns | [42264, 48391] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 20191ns | +25.4% | +58.6% | -1.7% | +51.7% | -58.8% | +134.0% |
| 1 | 2 | 20893ns | +10.2% | +5.7% | +20.2% | +65.0% | -46.2% | +97.4% |
| 1 | 3 | 21315ns | +15.2% | +34.9% | -4.5% | +75.4% | -45.5% | +129.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.014 | ok |
| head_tail_dyn | -0.595 | HIGH- (thermal bounce) |
| kernel_nway | -0.596 | HIGH- (thermal bounce) |
| kernel_seq | -0.662 | HIGH- (thermal bounce) |
| nway | -0.004 | ok |
| sequential | -0.111 | ok |
| std_parallel | -0.641 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 21859.0ns | 20981.5ns | 104.2% | HIGH |
| head_tail_dyn | 25661.9ns | 25692.3ns | 99.9% | HIGH |
| kernel_nway | 25520.4ns | 27897.2ns | 91.5% | HIGH |
| kernel_seq | 22869.3ns | 22403.3ns | 102.1% | HIGH |
| nway | 34794.9ns | 34358.3ns | 101.3% | HIGH |
| sequential | 10431.8ns | 11233.8ns | 92.9% | HIGH |
| std_parallel | 45899.2ns | 46150.2ns | 99.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 19513.4-22357.0 ns)
  19513.4 |
  19655.6 |####################
  19797.7 |####################
  19939.9 |####################
  20082.1 |
  20224.3 |####################
  20366.5 |########################################
  20508.7 |####################
  20650.8 |####################
  20793.0 |
  20935.2 |
  21077.4 |####################
  21219.6 |
  21361.8 |########################################
  21503.9 |
  21646.1 |####################
  21788.3 |
  21930.5 |########################################
  22072.7 |
  22214.9 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 22487.8-30264.2 ns)
  22487.8 |####################
  22876.6 |##########
  23265.4 |####################
  23654.2 |##########
  24043.0 |########################################
  24431.9 |
  24820.7 |
  25209.5 |##########
  25598.3 |
  25987.1 |##########
  26376.0 |
  26764.8 |
  27153.6 |##########
  27542.4 |
  27931.2 |
  28320.1 |
  28708.9 |
  29097.7 |##########
  29486.5 |
  29875.3 |##########
  (2 below, 1 above range)

kernel_nway (n=18, range 19851.2-35858.0 ns)
  19851.2 |########################################
  20651.6 |
  21451.9 |
  22252.2 |
  23052.6 |##########
  23852.9 |##########
  24653.3 |##########
  25453.6 |##########
  26253.9 |
  27054.3 |
  27854.6 |##########
  28654.9 |
  29455.3 |####################
  30255.6 |
  31056.0 |
  31856.3 |##########
  32656.6 |
  33457.0 |####################
  34257.3 |
  35057.6 |##########
  (1 below, 2 above range)

kernel_seq (n=18, range 19299.3-25842.8 ns)
  19299.3 |########################################
  19626.4 |########################################
  19953.6 |########################################
  20280.8 |####################
  20608.0 |
  20935.1 |####################
  21262.3 |####################
  21589.5 |
  21916.7 |
  22243.8 |
  22571.0 |####################
  22898.2 |
  23225.4 |
  23552.5 |
  23879.7 |
  24206.9 |
  24534.1 |########################################
  24861.2 |
  25188.4 |####################
  25515.6 |
  (1 below, 4 above range)

nway (n=18, range 29114.5-38121.8 ns)
  29114.5 |
  29564.8 |#############
  30015.2 |
  30465.6 |
  30915.9 |
  31366.3 |#############
  31816.7 |
  32267.0 |
  32717.4 |#############
  33167.8 |##########################
  33618.1 |##########################
  34068.5 |#############
  34518.9 |##########################
  34969.2 |
  35419.6 |#############
  35870.0 |#############
  36320.3 |
  36770.7 |
  37221.1 |########################################
  37671.4 |
  (2 below, 1 above range)

sequential (n=18, range 7860.9-13834.2 ns)
   7860.9 |#############
   8159.5 |
   8458.2 |#############
   8756.9 |
   9055.5 |
   9354.2 |
   9652.9 |
   9951.5 |
  10250.2 |#############
  10548.8 |########################################
  10847.5 |#############
  11146.2 |########################################
  11444.8 |########################################
  11743.5 |
  12042.2 |
  12340.8 |
  12639.5 |#############
  12938.2 |
  13236.8 |
  13535.5 |
  (2 below, 2 above range)

std_parallel (n=18, range 39780.8-52177.0 ns)
  39780.8 |
  40400.6 |##########
  41020.4 |####################
  41640.2 |##########
  42260.0 |##########
  42879.8 |
  43499.6 |##########
  44119.5 |##########
  44739.3 |
  45359.1 |
  45978.9 |########################################
  46598.7 |
  47218.5 |
  47838.3 |####################
  48458.1 |
  49077.9 |
  49697.7 |
  50317.5 |
  50937.3 |##########
  51557.2 |##########
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=104.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.3% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=24.1% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=83.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=106.5% of algo (FFI overhead may distort results)
- **nway**: bridge=102.5% of algo (FFI overhead may distort results)
- **sequential**: CV=26.4% (high variance, measurements may be unstable)
- **sequential**: bridge=99.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=95.6% of algo (FFI overhead may distort results)
