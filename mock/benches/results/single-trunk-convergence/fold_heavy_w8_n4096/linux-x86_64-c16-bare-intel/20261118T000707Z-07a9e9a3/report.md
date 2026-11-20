# fold_heavy, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 4.1x slower than the field

std_parallel (128.52 us) is 4.1x the fastest (31.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.65)

nway's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, kernel_nway, sequential, head_tail, kernel_seq, nway} vs {std_parallel} (134% apart)

The field splits into a fast tier {head_tail_dyn, kernel_nway, sequential, head_tail, kernel_seq, nway} and a slow tier {std_parallel} with a 134% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.1x the fastest

Fastest head_tail_dyn (31.66 us) to slowest std_parallel (128.52 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader head_tail_dyn vs stability leader kernel_nway (+7% speed for 3.9x steadier)

head_tail_dyn is fastest (31.66 us, CV 43.7%); kernel_nway gives up 7.4% median for 3.9x lower variance (CV 11.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### std_parallel is inconsistent: worst-20% is 5.1x its best-20%

std_parallel's best 20% of batches run at 69.32 us but its worst 20% at 351.14 us (5.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 31658.2 ns median (-27.3% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 4.06x (fastest 31658.2 ns, slowest 128521.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 44623ns | 44084ns | 26613ns | 40042ns | 60501ns | base |
| head_tail_dyn | 38707ns | 32331ns | 24516ns | 31868ns | 56060ns | -13.26% |
| kernel_nway | 36004ns | 34857ns | 31522ns | 34431ns | 40604ns | -19.32% |
| kernel_seq | 51923ns | 49728ns | 39867ns | 47662ns | 64342ns | +16.36% |
| nway | 57123ns | 55554ns | 53646ns | 55327ns | 61557ns | +28.01% |
| sequential | 38597ns | 36963ns | 18574ns | 30626ns | 60566ns | -13.50% |
| std_parallel | 187874ns | 129452ns | 69577ns | 117306ns | 352873ns | +321.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 43627ns | 26058ns | 58970ns | base | 0.094 |
| head_tail_dyn | 37780ns | 24101ns | 54829ns | -13.40% | 0.108 |
| kernel_nway | 34907ns | 30743ns | 38993ns | -19.99% | 0.117 |
| kernel_seq | 50949ns | 39326ns | 62857ns | +16.78% | 0.080 |
| nway | 56492ns | 53080ns | 60822ns | +29.49% | 0.073 |
| sequential | 38287ns | 18431ns | 60064ns | -12.24% | 0.107 |
| std_parallel | 186880ns | 69322ns | 351140ns | +328.36% | 0.022 |

## Performance model

- Peak throughput: **0.222 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.094 | 42.3% |
| head_tail_dyn | 0.129 | 58.2% |
| kernel_nway | 0.120 | 54.2% |
| kernel_seq | 0.084 | 37.9% |
| nway | 0.074 | 33.5% |
| sequential | 0.112 | 50.3% |
| std_parallel | 0.032 | 14.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 44623ns | 44623ns | base |
| head_tail_dyn | 38707ns | 38707ns | -13.26% |
| kernel_nway | 36004ns | 36004ns | -19.32% |
| kernel_seq | 51923ns | 51923ns | +16.36% |
| nway | 57123ns | 57123ns | +28.01% |
| sequential | 38597ns | 38597ns | -13.50% |
| std_parallel | 187874ns | 187874ns | +321.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 43528ns | base | --- | [32626, 50644] | --- | --- | --- | --- |
| head_tail_dyn | 31658ns | no significant difference | [-19100, +9420]ns | [30230, 42267] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 34003ns | no significant difference | [-16560, +1705]ns | [33396, 35081] | no | 0.3568 | 0.2379 | 0 |
| kernel_seq | 48678ns | +7132.9ns (+16.4%) | [+3581, +10055]ns | [43667, 55750] | YES | 0.0151 | 0.0075 | 0 |
| nway | 54984ns | +11141.5ns (+25.6%) | [+5539, +21382]ns | [54446, 55314] | YES | 0.0039 | 0.0013 | 0 |
| sequential | 36673ns | no significant difference | [-15212, +4605]ns | [18864, 56668] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 128522ns | +80839.9ns (+185.7%) | [+61651, +137175]ns | [89298, 187818] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 34114ns | +67.0% | +1.2% | +27.5% | +59.8% | +34.2% | +207.7% |
| 1 | 2 | 39295ns | -24.9% | -13.2% | +32.5% | +40.8% | -33.1% | +189.7% |
| 1 | 3 | 48570ns | -34.9% | -30.2% | +12.2% | +12.6% | -25.0% | +341.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.017 | ok |
| head_tail_dyn | -0.209 | moderate- |
| kernel_nway | -0.019 | ok |
| kernel_seq | -0.063 | ok |
| nway | -0.646 | HIGH- (thermal bounce) |
| sequential | -0.513 | HIGH- (thermal bounce) |
| std_parallel | -0.126 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 45047.1ns | 43627.3ns | 103.3% | HIGH |
| head_tail_dyn | 39012.6ns | 37779.6ns | 103.3% | HIGH |
| kernel_nway | 34642.4ns | 34906.8ns | 99.2% | HIGH |
| kernel_seq | 51794.3ns | 50948.6ns | 101.7% | HIGH |
| nway | 57176.0ns | 56492.0ns | 101.2% | HIGH |
| sequential | 39079.6ns | 38287.3ns | 102.1% | HIGH |
| std_parallel | 190451.9ns | 186880.3ns | 101.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 26058.3-58970.4 ns)
  26058.3 |
  27703.9 |####################
  29349.5 |
  30995.1 |########################################
  32640.7 |####################
  34286.3 |####################
  35931.9 |
  37577.5 |####################
  39223.1 |
  40868.7 |
  42514.3 |########################################
  44159.9 |
  45805.5 |####################
  47451.1 |
  49096.7 |########################################
  50742.3 |########################################
  52387.9 |
  54033.6 |
  55679.2 |
  57324.8 |####################
  (2 below, 2 above range)

head_tail_dyn (n=18, range 24100.9-54829.2 ns)
  24100.9 |
  25637.3 |####################
  27173.7 |##########
  28710.1 |##########
  30246.5 |########################################
  31783.0 |##############################
  33319.4 |
  34855.8 |##########
  36392.2 |
  37928.6 |
  39465.1 |
  41001.5 |
  42537.9 |
  44074.3 |
  45610.7 |
  47147.1 |
  48683.6 |##########
  50220.0 |
  51756.4 |
  53292.8 |
  (1 below, 4 above range)

kernel_nway (n=18, range 30743.0-38992.7 ns)
  30743.0 |
  31155.5 |####################
  31567.9 |
  31980.4 |
  32392.9 |
  32805.4 |##########
  33217.9 |##########
  33630.4 |########################################
  34042.9 |####################
  34455.3 |
  34867.8 |##########
  35280.3 |##########
  35692.8 |##########
  36105.3 |
  36517.8 |
  36930.3 |
  37342.8 |
  37755.2 |
  38167.7 |
  38580.2 |
  (2 below, 3 above range)

kernel_seq (n=18, range 39326.5-62856.6 ns)
  39326.5 |#############
  40503.0 |
  41679.5 |########################################
  42856.0 |
  44032.5 |#############
  45209.0 |#############
  46385.5 |#############
  47562.0 |#############
  48738.5 |##########################
  49915.1 |
  51091.6 |#############
  52268.1 |
  53444.6 |#############
  54621.1 |
  55797.6 |
  56974.1 |#############
  58150.6 |
  59327.1 |#############
  60503.6 |
  61680.1 |
  (1 below, 3 above range)

nway (n=18, range 53079.6-60822.4 ns)
  53079.6 |
  53466.7 |
  53853.9 |########
  54241.0 |########################################
  54628.1 |################
  55015.3 |################################
  55402.4 |################
  55789.6 |
  56176.7 |########
  56563.8 |
  56951.0 |########
  57338.1 |
  57725.3 |
  58112.4 |
  58499.6 |
  58886.7 |
  59273.8 |
  59661.0 |
  60048.1 |
  60435.3 |
  (1 below, 1 above range)

sequential (n=18, range 18431.2-60063.8 ns)
  18431.2 |########################################
  20512.8 |
  22594.4 |
  24676.1 |
  26757.7 |
  28839.3 |
  30920.9 |
  33002.6 |#####
  35084.2 |
  37165.8 |
  39247.5 |#####
  41329.1 |
  43410.7 |
  45492.3 |
  47574.0 |
  49655.6 |
  51737.2 |###########
  53818.9 |
  55900.5 |###########
  57982.1 |#####
  (1 below, 3 above range)

std_parallel (n=18, range 69322.2-351140.1 ns)
  69322.2 |########################################
  83413.1 |#############
  97504.0 |
  111594.9 |########################################
  125685.8 |
  139776.7 |#############
  153867.6 |##########################
  167958.5 |#############
  182049.4 |
  196140.3 |#############
  210231.2 |
  224322.1 |#############
  238413.0 |
  252503.9 |
  266594.8 |
  280685.7 |
  294776.6 |
  308867.4 |
  322958.3 |
  337049.2 |#############
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=32.0% (high variance, measurements may be unstable)
- **head_tail**: bridge=99.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=36.6% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=101.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=101.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.2% of algo (FFI overhead may distort results)
- **nway**: bridge=101.6% of algo (FFI overhead may distort results)
- **sequential**: CV=49.4% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **sequential**: bridge=108.8% of algo (FFI overhead may distort results)
- **std_parallel**: CV=82.6% (high variance, measurements may be unstable)
- **std_parallel**: worst_20/best_20 = 5.1x (possible bimodal distribution)
- **std_parallel**: bridge=106.5% of algo (FFI overhead may distort results)
