# fold_heavy, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (kernel_nway, kernel_seq) are a dead heat (<1%)

kernel_nway (52.50 us) and kernel_seq (52.84 us) differ by 0.64%, inside the noise, even though the wider field spreads 31.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: kernel_nway** at 52502.1 ns median (-0.9% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.32x (fastest 52502.1 ns, slowest 69052.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 1 |
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
| head_tail | 53506ns | 53138ns | 52290ns | 52956ns | 54937ns | base |
| head_tail_dyn | 53369ns | 53160ns | 52538ns | 52960ns | 54397ns | -0.26% |
| kernel_nway | 52966ns | 52708ns | 52426ns | 52668ns | 53682ns | -1.01% |
| kernel_seq | 53480ns | 53090ns | 52465ns | 52938ns | 54801ns | -0.05% |
| nway | 63512ns | 62733ns | 61929ns | 62696ns | 65526ns | +18.70% |
| sequential | 58252ns | 57608ns | 57222ns | 57581ns | 59772ns | +8.87% |
| std_parallel | 69631ns | 69300ns | 67594ns | 69021ns | 71563ns | +30.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 53263ns | 52114ns | 54628ns | base | 1.230 |
| head_tail_dyn | 53121ns | 52347ns | 54066ns | -0.27% | 1.234 |
| kernel_nway | 52755ns | 52244ns | 53438ns | -0.95% | 1.242 |
| kernel_seq | 53230ns | 52268ns | 54470ns | -0.06% | 1.231 |
| nway | 63210ns | 61717ns | 65139ns | +18.67% | 1.037 |
| sequential | 58041ns | 57028ns | 59516ns | +8.97% | 1.129 |
| std_parallel | 69365ns | 67381ns | 71215ns | +30.23% | 0.945 |

## Performance model

- Peak throughput: **1.258 Gops/s** (head_tail; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.238 | 98.4% |
| head_tail_dyn | 1.239 | 98.5% |
| kernel_nway | 1.248 | 99.3% |
| kernel_seq | 1.240 | 98.6% |
| nway | 1.048 | 83.4% |
| sequential | 1.141 | 90.7% |
| std_parallel | 0.949 | 75.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 53506ns | 53506ns | base |
| head_tail_dyn | 53369ns | 53369ns | -0.26% |
| kernel_nway | 52966ns | 52966ns | -1.01% |
| kernel_seq | 53480ns | 53480ns | -0.05% |
| nway | 63512ns | 63512ns | +18.70% |
| sequential | 58252ns | 58252ns | +8.87% |
| std_parallel | 69631ns | 69631ns | +30.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 52958ns | base | --- | [52379, 53517] | --- | --- | --- | --- |
| head_tail_dyn | 52892ns | no significant difference | [-504, +431]ns | [52567, 53081] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 52502ns | no significant difference | [-790, +119]ns | [52396, 52650] | no | 0.3568 | 0.2379 | 0 |
| kernel_seq | 52840ns | no significant difference | [-373, +808]ns | [52450, 53738] | no | 0.7549 | 0.6291 | 1 |
| nway | 62519ns | +9891.7ns (+18.7%) | [+9402, +11033]ns | [62162, 63819] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 57429ns | +4614.6ns (+8.7%) | [+4171, +4981]ns | [57185, 57990] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 69052ns | +15995.8ns (+30.2%) | [+15537, +16562]ns | [68354, 70573] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 52958ns | -0.6% | -0.9% | -1.1% | +17.4% | +8.9% | +32.3% |
| 1 | 2 | 53469ns | +0.4% | -1.7% | -0.2% | +16.7% | +7.6% | +28.8% |
| 1 | 3 | 52540ns | +0.2% | -0.1% | +0.5% | +20.3% | +9.2% | +30.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.532 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.665 | HIGH- (thermal bounce) |
| kernel_nway | -0.422 | moderate- |
| kernel_seq | -0.539 | HIGH- (thermal bounce) |
| nway | -0.051 | ok |
| sequential | -0.009 | ok |
| std_parallel | -0.100 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 106659.0ns | 53263.2ns | 200.2% | HIGH |
| head_tail_dyn | 106204.6ns | 53121.3ns | 199.9% | HIGH |
| kernel_nway | 105618.3ns | 52754.9ns | 200.2% | HIGH |
| kernel_seq | 106657.6ns | 53230.3ns | 200.4% | HIGH |
| nway | 126037.0ns | 63210.0ns | 199.4% | HIGH |
| sequential | 116203.5ns | 58041.2ns | 200.2% | HIGH |
| std_parallel | 137134.0ns | 69365.0ns | 197.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 52113.9-54628.5 ns)
  52113.9 |########################################
  52239.6 |########################################
  52365.4 |####################
  52491.1 |
  52616.8 |########################################
  52742.5 |####################
  52868.3 |
  52994.0 |####################
  53119.7 |########################################
  53245.5 |####################
  53371.2 |
  53496.9 |
  53622.6 |
  53748.4 |####################
  53874.1 |####################
  53999.8 |
  54125.6 |
  54251.3 |
  54377.0 |
  54502.7 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 52347.2-54066.0 ns)
  52347.2 |########################################
  52433.2 |#############
  52519.1 |#############
  52605.0 |
  52691.0 |##########################
  52776.9 |
  52862.9 |########################################
  52948.8 |
  53034.7 |########################################
  53120.7 |
  53206.6 |
  53292.5 |
  53378.5 |
  53464.4 |
  53550.3 |#############
  53636.3 |#############
  53722.2 |#############
  53808.2 |
  53894.1 |
  53980.0 |
  (1 below, 1 above range)

kernel_nway (n=18, range 52244.4-53437.5 ns)
  52244.4 |#############
  52304.1 |#############
  52363.7 |########################################
  52423.4 |#############
  52483.0 |########################################
  52542.7 |##########################
  52602.4 |#############
  52662.0 |#############
  52721.7 |
  52781.3 |
  52841.0 |
  52900.6 |
  52960.3 |
  53019.9 |
  53079.6 |
  53139.2 |#############
  53198.9 |
  53258.6 |
  53318.2 |
  53377.9 |
  (1 below, 3 above range)

kernel_seq (n=18, range 52268.1-54470.1 ns)
  52268.1 |########################################
  52378.2 |####################
  52488.3 |####################
  52598.4 |####################
  52708.5 |####################
  52818.6 |########################################
  52928.7 |####################
  53038.8 |####################
  53148.9 |
  53259.0 |
  53369.1 |
  53479.2 |
  53589.3 |
  53699.4 |########################################
  53809.5 |####################
  53919.6 |
  54029.7 |
  54139.8 |
  54249.9 |
  54360.0 |
  (2 below, 3 above range)

nway (n=18, range 61716.7-65138.9 ns)
  61716.7 |##########################
  61887.8 |##########################
  62058.9 |
  62230.0 |##########################
  62401.1 |########################################
  62572.2 |#############
  62743.3 |
  62914.4 |#############
  63085.6 |
  63256.7 |
  63427.8 |
  63598.9 |#############
  63770.0 |#############
  63941.1 |
  64112.2 |
  64283.3 |
  64454.4 |
  64625.6 |
  64796.7 |#############
  64967.8 |#############
  (1 below, 2 above range)

sequential (n=18, range 57027.8-59516.0 ns)
  57027.8 |##############################
  57152.2 |########################################
  57276.6 |
  57401.0 |####################
  57525.4 |
  57649.8 |
  57774.3 |##########
  57898.7 |####################
  58023.1 |####################
  58147.5 |
  58271.9 |##########
  58396.3 |
  58520.7 |
  58645.1 |
  58769.5 |
  58893.9 |
  59018.3 |
  59142.8 |
  59267.2 |
  59391.6 |
  (1 below, 2 above range)

std_parallel (n=18, range 67380.5-71214.6 ns)
  67380.5 |#############
  67572.2 |
  67763.9 |#############
  67955.6 |
  68147.3 |#############
  68339.0 |########################################
  68530.7 |#############
  68722.5 |
  68914.2 |#############
  69105.9 |#############
  69297.6 |
  69489.3 |##########################
  69681.0 |
  69872.7 |
  70064.4 |
  70256.1 |
  70447.8 |########################################
  70639.5 |
  70831.2 |
  71022.9 |#############
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=199.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=199.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=200.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=200.0% of algo (FFI overhead may distort results)
- **nway**: bridge=200.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=200.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=198.8% of algo (FFI overhead may distort results)
