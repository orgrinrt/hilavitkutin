# fold_heavy, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 59% faster than the next best (kernel_seq)

sequential (22.93 us) leads kernel_seq (36.42 us) by 59%, a clear separation rather than a photo finish. CV 27.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 38% (significant)

sequential is -15.35 us (38%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 2.6x slower than the field

std_parallel (59.75 us) is 2.6x the fastest (22.93 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.67)

sequential's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### head_tail_dyn is inconsistent: worst-20% is 1.8x its best-20%

head_tail_dyn's best 20% of batches run at 31.02 us but its worst 20% at 55.80 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 22933.2 ns median (-43.3% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.61x (fastest 22933.2 ns, slowest 59748.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 2 |
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
| head_tail | 45288ns | 40968ns | 34912ns | 40271ns | 58001ns | base |
| head_tail_dyn | 44860ns | 43835ns | 31575ns | 41312ns | 56824ns | -0.95% |
| kernel_nway | 44151ns | 41070ns | 33083ns | 38857ns | 57624ns | -2.51% |
| kernel_seq | 40440ns | 36800ns | 34357ns | 36647ns | 49172ns | -10.70% |
| nway | 64739ns | 57474ns | 51296ns | 58057ns | 81484ns | +42.95% |
| sequential | 27041ns | 23371ns | 22290ns | 23033ns | 35428ns | -40.29% |
| std_parallel | 61315ns | 60417ns | 54206ns | 58406ns | 69233ns | +35.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 44710ns | 34538ns | 57307ns | base | 0.092 |
| head_tail_dyn | 44133ns | 31019ns | 55801ns | -1.29% | 0.093 |
| kernel_nway | 43386ns | 32502ns | 56524ns | -2.96% | 0.094 |
| kernel_seq | 39955ns | 33868ns | 48645ns | -10.63% | 0.103 |
| nway | 63929ns | 50946ns | 79944ns | +42.99% | 0.064 |
| sequential | 26715ns | 22040ns | 35031ns | -40.25% | 0.153 |
| std_parallel | 60828ns | 53724ns | 68723ns | +36.05% | 0.067 |

## Performance model

- Peak throughput: **0.186 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.101 | 54.5% |
| head_tail_dyn | 0.095 | 50.9% |
| kernel_nway | 0.101 | 54.4% |
| kernel_seq | 0.112 | 60.5% |
| nway | 0.072 | 38.6% |
| sequential | 0.179 | 96.1% |
| std_parallel | 0.069 | 36.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 45288ns | 45288ns | base |
| head_tail_dyn | 44860ns | 44860ns | -0.95% |
| kernel_nway | 44151ns | 44151ns | -2.51% |
| kernel_seq | 40440ns | 40440ns | -10.70% |
| nway | 64739ns | 64739ns | +42.95% |
| sequential | 27041ns | 27041ns | -40.29% |
| std_parallel | 61315ns | 61315ns | +35.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 40439ns | base | --- | [38800, 41902] | --- | --- | --- | --- |
| head_tail_dyn | 43265ns | no significant difference | [-7546, +6951]ns | [36581, 47500] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 40501ns | no significant difference | [-4201, +1973]ns | [34154, 41851] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 36423ns | no significant difference | [-6195, +5945]ns | [34746, 44690] | no | 0.5768 | 0.4807 | 0 |
| nway | 57032ns | +17240.1ns (+42.6%) | [+14782, +23415]ns | [55149, 70474] | YES | 0.0003 | 0.0001 | 0 |
| sequential | 22933ns | -15347.1ns (-38.0%) | [-18994, -8403]ns | [22144, 29602] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 59748ns | +18201.8ns (+45.0%) | [+15101, +22327]ns | [54784, 62520] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 42779ns | +14.9% | +3.1% | -19.6% | +67.2% | -48.5% | +43.7% |
| 1 | 2 | 39944ns | -19.3% | +1.9% | +19.8% | +41.1% | -7.5% | +49.5% |
| 1 | 3 | 40177ns | +5.6% | -17.7% | -10.4% | +37.7% | -44.8% | +46.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.211 | moderate- |
| head_tail_dyn | -0.563 | HIGH- (thermal bounce) |
| kernel_nway | -0.031 | ok |
| kernel_seq | -0.659 | HIGH- (thermal bounce) |
| nway | -0.135 | ok |
| sequential | -0.667 | HIGH- (thermal bounce) |
| std_parallel | -0.027 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 43819.6ns | 44709.7ns | 98.0% | HIGH |
| head_tail_dyn | 44092.5ns | 44132.7ns | 99.9% | HIGH |
| kernel_nway | 40887.0ns | 43385.6ns | 94.2% | HIGH |
| kernel_seq | 39744.2ns | 39955.3ns | 99.5% | HIGH |
| nway | 67461.1ns | 63929.0ns | 105.5% | HIGH |
| sequential | 39793.2ns | 26715.1ns | 149.0% | HIGH |
| std_parallel | 59208.0ns | 60827.6ns | 97.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 34538.2-57307.3 ns)
  34538.2 |##########
  35676.7 |##########
  36815.1 |
  37953.6 |########################################
  39092.0 |####################
  40230.5 |##############################
  41369.0 |##############################
  42507.4 |
  43645.9 |
  44784.3 |##########
  45922.8 |
  47061.2 |
  48199.7 |##########
  49338.1 |
  50476.6 |
  51615.0 |
  52753.5 |
  53892.0 |
  55030.4 |
  56168.9 |
  (1 below, 1 above range)

head_tail_dyn (n=18, range 31018.7-55801.0 ns)
  31018.7 |####################
  32257.8 |########################################
  33496.9 |
  34736.0 |
  35975.1 |
  37214.2 |
  38453.4 |
  39692.5 |########################################
  40931.6 |####################
  42170.7 |####################
  43409.8 |####################
  44648.9 |####################
  45888.1 |####################
  47127.2 |########################################
  48366.3 |
  49605.4 |####################
  50844.5 |
  52083.6 |
  53322.8 |
  54561.9 |####################
  (2 below, 2 above range)

kernel_nway (n=18, range 32502.3-56524.4 ns)
  32502.3 |####################
  33703.4 |####################
  34904.5 |
  36105.6 |
  37306.7 |
  38507.8 |##########
  39708.9 |########################################
  40910.1 |####################
  42111.2 |####################
  43312.3 |
  44513.4 |##########
  45714.5 |
  46915.6 |
  48116.7 |##########
  49317.8 |
  50518.9 |
  51720.0 |
  52921.1 |
  54122.2 |
  55323.3 |
  (2 below, 1 above range)

kernel_seq (n=18, range 33867.6-48645.0 ns)
  33867.6 |########################################
  34606.4 |########################################
  35345.3 |########################################
  36084.2 |########################################
  36823.1 |####################
  37561.9 |
  38300.8 |
  39039.7 |
  39778.6 |####################
  40517.4 |
  41256.3 |
  41995.2 |
  42734.0 |####################
  43472.9 |
  44211.8 |
  44950.7 |
  45689.5 |####################
  46428.4 |
  47167.3 |########################################
  47906.2 |
  (2 below, 2 above range)

nway (n=18, range 50945.6-79943.6 ns)
  50945.6 |
  52395.5 |
  53845.4 |########################################
  55295.3 |##############################
  56745.2 |##########
  58195.1 |
  59645.0 |##########
  61094.9 |
  62544.8 |
  63994.7 |
  65444.6 |##########
  66894.5 |##########
  68344.4 |
  69794.3 |
  71244.2 |
  72694.1 |##########
  74144.0 |##########
  75593.9 |
  77043.8 |
  78493.7 |
  (2 below, 3 above range)

sequential (n=18, range 22040.4-35030.6 ns)
  22040.4 |########################################
  22689.9 |####################
  23339.4 |
  23988.9 |
  24638.4 |######
  25288.0 |
  25937.5 |
  26587.0 |
  27236.5 |
  27886.0 |
  28535.5 |
  29185.0 |#############
  29834.5 |
  30484.0 |
  31133.6 |
  31783.1 |
  32432.6 |
  33082.1 |
  33731.6 |
  34381.1 |
  (2 below, 4 above range)

std_parallel (n=18, range 53724.1-68723.5 ns)
  53724.1 |########################################
  54474.1 |##########################
  55224.1 |
  55974.0 |
  56724.0 |
  57474.0 |##########################
  58223.9 |
  58973.9 |#############
  59723.9 |#############
  60473.8 |#############
  61223.8 |#############
  61973.8 |##########################
  62723.7 |#############
  63473.7 |
  64223.7 |
  64973.6 |
  65723.6 |#############
  66473.6 |
  67223.5 |
  67973.5 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=43.2% (high variance, measurements may be unstable)
- **head_tail**: bridge=100.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=25.1% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=100.0% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=43.4% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=103.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.7% of algo (FFI overhead may distort results)
- **nway**: CV=20.2% (high variance, measurements may be unstable)
- **nway**: bridge=108.6% of algo (FFI overhead may distort results)
- **sequential**: CV=23.7% (high variance, measurements may be unstable)
- **sequential**: bridge=110.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=96.4% of algo (FFI overhead may distort results)
